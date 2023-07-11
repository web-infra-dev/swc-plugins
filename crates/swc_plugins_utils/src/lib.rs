#![feature(let_chains)]
use std::{borrow::Borrow, path::PathBuf, sync::Arc};

use rustc_hash::FxHashMap as HashMap;
use swc_core::{
  common::{SyntaxContext, DUMMY_SP},
  ecma::{
    ast::{
      AssignExpr, BlockStmt, BlockStmtOrExpr, Callee, Class, ClassDecl, Decl, DefaultDecl, Expr,
      ExprStmt, FnDecl, Function, Id, Ident, Lit, MemberProp, Module, ModuleDecl, ModuleItem, Pat,
      PatOrExpr, Program, ReturnStmt, Stmt, VarDecl, VarDeclKind,
    },
    atoms::JsWord,
    visit::{Fold, Visit, VisitMut, VisitMutWith, VisitWith},
  },
};

#[derive(Clone, Copy)]
pub struct ClearMark;

impl VisitMut for ClearMark {
  fn visit_mut_ident(&mut self, ident: &mut Ident) {
    ident.span.ctxt = SyntaxContext::empty();
  }
}

struct Noop;

impl VisitMut for Noop {}
impl Fold for Noop {}

pub fn noop_pass() -> impl Fold + VisitMut {
  Noop
}

#[macro_export]
macro_rules! enable_helper {
  ($i:ident) => {{
    $crate::swc_core::ecma::transforms::base::helpers::HELPERS.with(|helpers| {
      helpers.$i();
      helpers.mark()
    })
  }};
}

#[macro_export]
macro_rules! helper_expr {
  ($field_name:ident, $s:tt) => {{
    $crate::helper_expr!($crate::swc_core::common::DUMMY_SP, $field_name, $s)
  }};

  ($span:expr, $field_name:ident, $s:tt) => {{
    let mark = $crate::enable_helper!($field_name);
    let span = $span.apply_mark(mark);
    let external =
      $crate::swc_core::ecma::transforms::base::helpers::HELPERS.with(|helper| helper.external());

    if external {
      $crate::swc_core::ecma::ast::Expr::from($crate::swc_core::ecma::utils::quote_ident!(
        span,
        concat!("_", stringify!($field_name))
      ))
    } else {
      $crate::swc_core::ecma::ast::Expr::from($crate::swc_core::ecma::utils::quote_ident!(
        span,
        concat!("_", $s)
      ))
    }
  }};
}

pub fn change_ident_syntax_context(ctxt: SyntaxContext, name: JsWord) -> impl VisitMut {
  ChangeIdentSyntaxContext::new(ctxt, name)
}

pub struct ChangeIdentSyntaxContext {
  pub ctxt: SyntaxContext,
  pub name: JsWord,
}

impl ChangeIdentSyntaxContext {
  fn new(ctxt: SyntaxContext, name: JsWord) -> Self {
    ChangeIdentSyntaxContext { ctxt, name }
  }
}

impl VisitMut for ChangeIdentSyntaxContext {
  fn visit_mut_ident(&mut self, ident: &mut Ident) {
    if ident.sym == self.name {
      ident.span = ident.span.with_ctxt(self.ctxt);
    }
  }
}

pub fn is_esm(program: &Program) -> bool {
  match program {
    Program::Module(module) => module
      .body
      .iter()
      .any(|item| matches!(item, ModuleItem::ModuleDecl(_))),
    Program::Script(_) => false,
  }
}

pub fn contain_ident(id: &Id, expr: &Expr) -> bool {
  struct ContainIdent<'a> {
    id: &'a Id,
    contain: bool,
  }

  impl<'a> Visit for ContainIdent<'a> {
    fn visit_ident(&mut self, ident: &Ident) {
      if self.contain {
        return;
      }
      if &ident.to_id() == self.id {
        self.contain = true;
      }
    }
  }

  let mut contain = ContainIdent { id, contain: false };
  expr.visit_with(&mut contain);
  contain.contain
}

pub struct IdentCount {
  inner: HashMap<Id, usize>,
}
impl Visit for IdentCount {
  fn visit_ident(&mut self, ident: &Ident) {
    self
      .inner
      .entry(ident.to_id())
      .and_modify(|i| *i += 1)
      .or_insert(1);
  }
}
pub fn count_ident(module: &impl VisitWith<IdentCount>) -> HashMap<Id, usize> {
  let mut ident_count = IdentCount {
    inner: Default::default(),
  };
  module.visit_with(&mut ident_count);
  ident_count.inner
}

pub struct RmInvalid;
impl RmInvalid {
  fn rm_stmts(&mut self, stmts: &mut Vec<Stmt>) {
    let mut rm = vec![];
    for (idx, stmt) in stmts.iter_mut().enumerate().rev() {
      if let Stmt::Expr(ExprStmt{ expr , ..}) = stmt && expr.is_invalid() {
        rm.push(idx)
      } else {
        stmt.visit_mut_children_with(self);
      }
    }

    for idx in rm {
      stmts.remove(idx);
    }
  }
}

impl VisitMut for RmInvalid {
  fn visit_mut_module(&mut self, module: &mut Module) {
    let mut rm = vec![];
    for (idx, stmt) in module.body.iter_mut().enumerate().rev() {
      if let ModuleItem::Stmt(Stmt::Expr(ExprStmt{ expr , ..})) = stmt && expr.is_invalid() {
        rm.push(idx)
      } else {
        stmt.visit_mut_children_with(self);
      }
    }

    for idx in rm {
      module.body.remove(idx);
    }

    module.visit_mut_children_with(self);
  }

  fn visit_mut_block_stmt(&mut self, block: &mut BlockStmt) {
    self.rm_stmts(&mut block.stmts);
  }
}
pub fn remove_invalid_expr(module: &mut impl VisitMutWith<RmInvalid>) {
  let mut rm = RmInvalid;
  module.visit_mut_with(&mut rm);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ReactComponentType {
  FC,
  Class,
  None,
}

static COMPONENT_NAME: &str = "Component";
static PURE_COMPONENT_NAME: &str = "PureComponent";

pub fn is_react_component(
  expr: &Expr,
  bindings: Option<&HashMap<Id, BindingInfo>>,
) -> ReactComponentType {
  match expr {
    Expr::Fn(function) => {
      if let Some(body) = &function.function.body && is_return_jsx(body.stmts.iter(), bindings) {
        ReactComponentType::FC
      } else {
        ReactComponentType::None
      }
    },
    Expr::Arrow(array) => {
      let is_return_jsx = match &*array.body {
        BlockStmtOrExpr::BlockStmt(block) => {
          is_return_jsx(block.stmts.iter(), bindings)
        },
        BlockStmtOrExpr::Expr(expr) => {
          is_return_jsx([
            Stmt::Return(ReturnStmt {
                span: DUMMY_SP,
                arg: Some(expr.clone()),
            })
          ].iter(), bindings)
        },
      };

      if is_return_jsx {
        ReactComponentType::FC
      } else {
        ReactComponentType::None
      }
    },
    Expr::Class(class_expr) => {
      let class = &class_expr.class;
      if let Some(super_class) = class.super_class.as_deref() {
        let is = match super_class {
          Expr::Member(_) => {
            match_member(super_class, "React.Component") ||
            match_member(super_class, "React.PureComponent")
          },
          Expr::Ident(ident) => {
            &ident.sym == COMPONENT_NAME || &ident.sym == PURE_COMPONENT_NAME
          },
          _ => false
        };

        if is {
          return ReactComponentType::Class;
        }
      }

      ReactComponentType::None
    },
    _ => ReactComponentType::None
  }
}

pub fn is_react_component_class(
  class: &Class,
  bindings: Option<&HashMap<Id, BindingInfo>>,
) -> bool {
  if let Some(super_class) = class.super_class.as_deref() {
    let is = match super_class {
      Expr::Member(_) => {
        match_member(super_class, "React.Component")
          || match_member(super_class, "React.PureComponent")
      }
      Expr::Ident(super_class) => {
        if &super_class.sym == COMPONENT_NAME || &super_class.sym == PURE_COMPONENT_NAME {
          return true;
        }

        if let Some(bindings) = bindings && bindings.contains_key(&super_class.to_id()) {
          let maybe_class = bindings.get(&super_class.to_id()).unwrap();
          if !maybe_class.re_assigned && maybe_class.init.is_some() {
            let is_react_class = match maybe_class.init.as_ref().unwrap() {
                BindingInitKind::Expr(expr) => is_react_component(expr, Some(bindings)) == ReactComponentType::Class,
                BindingInitKind::Class(class) => is_react_component_class(class, Some(bindings)),
                BindingInitKind::Fn(_) => { false },
            };

            if is_react_class {
              return true;
            }
          }
        }

        false
      }
      _ => false,
    };

    if is {
      return true;
    }
  }

  false
}

pub fn is_return_jsx<'a>(
  stmts: impl Iterator<Item = &'a Stmt>,
  bindings: Option<&HashMap<Id, BindingInfo>>,
) -> bool {
  for stmt in stmts {
    if let Stmt::Return(return_stmt) = stmt {
      if let Some(return_value) = &return_stmt.arg {
        if is_jsx(return_value, bindings) {
          return true;
        }
      }
    }
  }

  false
}

pub fn is_creating_component(expr: &Expr) -> bool {
  match get_real_expr(expr) {
    Expr::Call(call_expr) => match &call_expr.callee {
      Callee::Expr(callee) => match callee.borrow() {
        // React.createElement()
        // React.cloneElement()
        Expr::Member(_) => {
          match_member(callee, "React.createElement") || match_member(callee, "React.cloneElement")
        }

        // createElement()
        // cloneElement()
        Expr::Ident(ident) => &ident.sym == "cloneElement" || &ident.sym == "createElement",
        _ => false,
      },
      _ => false,
    },
    Expr::Cond(cond) => is_creating_component(&cond.alt) || is_creating_component(&cond.cons),
    _ => false,
  }
}

pub fn is_jsx(expr: &Expr, bindings: Option<&HashMap<Id, BindingInfo>>) -> bool {
  if is_creating_component(expr) {
    return true;
  }

  match get_real_expr(expr) {
    Expr::JSXElement(_) | Expr::JSXFragment(_) => true,
    Expr::Cond(cond) => is_jsx(&cond.alt, bindings) || is_jsx(&cond.cons, bindings),
    Expr::Array(array_lit) => array_lit.elems.iter().any(|ele| {
      if let Some(ele) = ele {
        is_jsx(&ele.expr, bindings)
      } else {
        false
      }
    }),
    Expr::Call(call_expr) => match &call_expr.callee {
      Callee::Expr(callee) => {
        let callee = get_real_expr(callee);
        if let Some(ident) = callee.as_ident() && bindings.is_some() && bindings.unwrap().contains_key(&ident.to_id()) {
          if let Some(BindingInitKind::Fn(function)) = &bindings.unwrap().get(&ident.to_id()).unwrap().init {
            if let Some(body) = &function.body {
              return is_return_jsx(body.stmts.iter(), bindings);
            }
          }
        }

        false
      }
      _ => false,
    },
    Expr::Ident(ident) => {
      if let Some(bindings) = bindings && bindings.contains_key(&ident.to_id()) {
        let binding_info = bindings.get(&ident.to_id()).unwrap();

        if let Some(BindingInitKind::Expr(expr)) = &binding_info.init {
          return is_jsx(expr, Some(bindings));
        }
      }

      false
    }
    Expr::Assign(assign) => match &*assign.right {
      Expr::Ident(right) => {
        if let Some(bindings) = bindings && bindings.contains_key(&right.to_id()) {
            let val = bindings.get(&right.to_id()).unwrap();
            if let Some(BindingInitKind::Expr(expr)) = &val.init {
              return is_jsx(expr, Some(bindings))
            }
          }

        false
      }
      _ => is_jsx(&assign.right, bindings),
    },
    _ => false,
  }
}

// `a.b.c`
// `a.b.c` in `swc` is `MemberExpr { MemberExpr { a, b } c }`
pub fn match_member(expr: &Expr, template: &str) -> bool {
  fn recur(expr: &Expr, curr: &[&str]) -> bool {
    if curr.is_empty() {
      return false;
    }

    if curr.len() == 1 {
      return expr.is_ident() && &expr.as_ident().unwrap().sym == *curr.last().unwrap();
    }

    if let Expr::Member(member_expr) = expr {
      match &member_expr.prop {
        MemberProp::Ident(ident) => &ident.sym == *curr.last().unwrap(),
        MemberProp::PrivateName(private_name) => &private_name.id.sym == *curr.last().unwrap(),
        MemberProp::Computed(computed) => {
          if let Expr::Lit(lit) = computed.expr.borrow() {
            match lit {
              Lit::Str(str) => str.value.to_string() == *curr.last().unwrap(),
              Lit::Num(num) => num.value.to_string() == *curr.last().unwrap(),
              _ => false,
            }
          } else {
            false
          }
        }
      }
    } else {
      false
    }
  }

  let segments: Vec<_> = template.split('.').collect();

  recur(expr, &segments)
}

/// If you want to check a value type, eg: obj, this can help you with:
/// (obj)
/// a = obj
pub fn get_real_expr(expr: &Expr) -> &Expr {
  match expr {
    Expr::Assign(assign) => get_real_expr(&assign.right),
    Expr::Paren(paren) => get_real_expr(&paren.expr),
    _ => expr,
  }
}

#[derive(Debug)]
pub enum BindingKind {
  VarDeclKind(VarDeclKind),
  Class,
  Fn,
}

#[derive(Debug)]
pub enum BindingInitKind {
  Expr(Box<Expr>),
  Fn(Box<Function>),
  Class(Box<Class>),
}

#[derive(Debug)]
pub struct BindingInfo {
  pub name: JsWord,
  pub id: Id,
  pub binding_kind: BindingKind,
  pub init: Option<BindingInitKind>,
  pub re_assigned: bool,
}

pub struct CollectBindings {
  pub bindings: HashMap<Id, BindingInfo>,
}

impl CollectBindings {
  fn add_class(&mut self, class_id: &Ident, class: Box<Class>) {
    self.bindings.insert(
      class_id.to_id(),
      BindingInfo {
        name: class_id.sym.clone(),
        id: class_id.to_id(),
        binding_kind: BindingKind::Class,
        init: Some(BindingInitKind::Class(class)),
        re_assigned: false,
      },
    );
  }

  fn add_fn(&mut self, fn_id: &Ident, function: Box<Function>) {
    self.bindings.insert(
      fn_id.to_id(),
      BindingInfo {
        name: fn_id.sym.clone(),
        id: fn_id.to_id(),
        binding_kind: BindingKind::Fn,
        init: Some(BindingInitKind::Fn(function)),
        re_assigned: false,
      },
    );
  }

  fn add_var_decl(&mut self, var_decl: &VarDecl) {
    for decl in &var_decl.decls {
      match &decl.name {
        Pat::Ident(ident) => {
          self.bindings.insert(
            ident.to_id(),
            BindingInfo {
              name: ident.sym.clone(),
              id: ident.to_id(),
              binding_kind: BindingKind::VarDeclKind(var_decl.kind),
              init: decl.init.clone().map(BindingInitKind::Expr),
              re_assigned: false,
            },
          );
        }
        // TODO handle complex var decl
        Pat::Assign(_) => todo!(),
        Pat::Array(_) => {}
        Pat::Rest(_) => {}
        Pat::Object(_) => {}
        Pat::Invalid(_) => {}
        Pat::Expr(_) => {}
      }
    }
  }
}

impl Visit for CollectBindings {
  fn visit_module(&mut self, module: &Module) {
    module.visit_children_with(self);

    for item in &module.body {
      match item {
        ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultDecl(export_default)) => {
          match &export_default.decl {
            DefaultDecl::Class(class) => {
              if let Some(class_id) = &class.ident {
                self.add_class(class_id, class.class.clone());
              }
            }
            DefaultDecl::Fn(fn_decl) => {
              if let Some(fn_id) = &fn_decl.ident {
                self.add_fn(fn_id, fn_decl.function.clone());
              }
            }
            _ => {}
          }
        }
        ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(export_decl)) => {
          match &export_decl.decl {
            Decl::Class(class) => {
              self.add_class(&class.ident, class.class.clone());
            }
            Decl::Fn(function) => {
              self.add_fn(&function.ident, function.function.clone());
            }
            Decl::Var(decl) => {
              self.add_var_decl(decl);
            }
            _ => {}
          };
        }
        _ => {}
      }
    }

    module.visit_with(&mut DetectReAssigned {
      bindings: &mut self.bindings,
    });
  }

  fn visit_fn_decl(&mut self, fn_decl: &FnDecl) {
    self.add_fn(&fn_decl.ident, fn_decl.function.clone())
  }

  fn visit_class_decl(&mut self, class_decl: &ClassDecl) {
    self.add_class(&class_decl.ident, class_decl.class.clone());
  }

  fn visit_var_decl(&mut self, var_decl: &VarDecl) {
    self.add_var_decl(var_decl);
  }
}

pub struct DetectReAssigned<'a> {
  pub bindings: &'a mut HashMap<Id, BindingInfo>,
}

impl<'a> DetectReAssigned<'a> {
  fn reassign_ident(&mut self, ident: &Ident) {
    if let Some(binding_info) = self.bindings.get_mut(&ident.to_id()) {
      binding_info.re_assigned = true;
    }
  }
}

impl<'a> Visit for DetectReAssigned<'a> {
  fn visit_assign_expr(&mut self, assign_expr: &AssignExpr) {
    match &assign_expr.left {
      PatOrExpr::Expr(expr) => {
        if let Expr::Ident(ident) = expr.borrow() {
          self.reassign_ident(ident);
        }
      }
      PatOrExpr::Pat(pat) => {
        match pat.borrow() {
          Pat::Ident(ident) => {
            self.reassign_ident(ident);
          }
          Pat::Expr(expr) => {
            if let Expr::Ident(ident) = expr.borrow() {
              self.reassign_ident(ident);
            }
          }
          // TODO handle complex assign
          _ => {}
        }
      }
    }
  }
}

pub fn collect_bindings(module: &Module) -> HashMap<Id, BindingInfo> {
  let mut collect_bindings = CollectBindings {
    bindings: HashMap::default(),
  };

  module.visit_with(&mut collect_bindings);

  collect_bindings.bindings
}

pub struct PluginContext {
  pub cm: Arc<swc_core::common::SourceMap>,
  pub file: Arc<swc_core::common::SourceFile>,
  pub top_level_mark: swc_core::common::Mark,
  pub unresolved_mark: swc_core::common::Mark,
  pub comments: swc_core::common::comments::SingleThreadedComments,
  pub filename: String,
  pub cwd: PathBuf,

  pub config_hash: Option<String>, // This can be used by plugins to do caching
}

impl std::fmt::Debug for PluginContext {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("PluginContext")
      .field("cm", &"Arc<SourceMap>")
      .field("top_level_mark", &self.top_level_mark)
      .field("unresolved_mark", &self.unresolved_mark)
      .field("comments", &self.comments)
      .field("filename", &self.filename)
      .field("cwd", &self.cwd)
      .field("config_hash", &self.config_hash)
      .finish()
  }
}

#[cfg(test)]
mod test {
  use swc_core::{
    self,
    common::DUMMY_SP,
    ecma::ast::{Module, Program},
    quote,
  };

  use super::{is_esm, is_react_component};
  use crate::{match_member, ReactComponentType};

  #[test]
  fn detect_esm() {
    assert!(is_esm(&Program::Module(Module {
      span: DUMMY_SP,
      body: vec![quote!("import 'a'" as ModuleItem)],
      shebang: None
    })));

    assert!(is_esm(&Program::Module(Module {
      span: DUMMY_SP,
      body: vec![quote!("export const a = 'a'" as ModuleItem)],
      shebang: None
    })));

    assert!(!is_esm(&Program::Module(Module {
      span: DUMMY_SP,
      body: vec![quote!("module.exports = {}" as ModuleItem)],
      shebang: None
    })));
  }

  #[test]
  fn test_match_member() {
    assert!(match_member(&quote!("a.b.c" as Expr), "a.b.c"));
    assert!(!match_member(&quote!("a.b" as Expr), "a.b.c"));
    assert!(!match_member(&quote!("a" as Expr), "a.b.c"));
    assert!(!match_member(&quote!("a.b.c" as Expr), "a.b"));

    assert!(match_member(&quote!("['a'].b.c" as Expr), "a.b.c"));
    assert!(match_member(&quote!("a['b'].c" as Expr), "a.b.c"));
    assert!(match_member(&quote!("a.b['c']" as Expr), "a.b.c"));
  }

  #[test]
  fn test_is_react_component() {
    assert_eq!(
      ReactComponentType::FC,
      is_react_component(
        &quote!(
          "function App() {
      return createElement()
    }" as Expr
        ),
        None
      )
    );

    assert_eq!(
      ReactComponentType::FC,
      is_react_component(
        &quote!(
          "function App() {
      return React.createElement()
    }" as Expr
        ),
        None
      )
    );

    assert_eq!(
      ReactComponentType::FC,
      is_react_component(
        &quote!(
          "() => {
      return React.createElement()
    }" as Expr
        ),
        None
      )
    );

    assert_eq!(
      ReactComponentType::FC,
      is_react_component(
        &quote!(
          "() => {
      return (React.createElement())
    }" as Expr
        ),
        None
      )
    );

    assert_eq!(
      ReactComponentType::FC,
      is_react_component(
        &quote!(
          "() => {
      return (a ? React.createElement() : React.cloneElement())
    }" as Expr
        ),
        None
      )
    );
  }
}
