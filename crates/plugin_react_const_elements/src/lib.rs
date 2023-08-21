#![feature(let_chains)]
use const_decls::ConstDecls;
use immutable::{are_children_immutable, hoist_jsx, modify_jsx_if_immutable};
use serde::Deserialize;
use swc_core::{
  common::{Mark, SyntaxContext, DUMMY_SP},
  ecma::{
    ast::{
      ArrowExpr, BindingIdent, BlockStmt, BlockStmtOrExpr, Class, ClassMember, Decl, Expr,
      Function, Id, Ident, Module, Pat, Program, ReturnStmt, Script, Stmt, VarDecl, VarDeclKind,
      VarDeclarator,
    },
    utils::find_pat_ids,
    visit::{as_folder, noop_visit_mut_type, Fold, VisitMut, VisitMutWith},
  },
};
use utils::{get_immutable_ids, StmtLike};

mod const_decls;
mod immutable;
mod utils;

#[derive(Debug, Default, Deserialize, Clone)]
pub struct ReactConstElementsOptions {
  pub immutable_globals: rustc_hash::FxHashSet<String>,
  pub allow_mutable_props_on_tags: rustc_hash::FxHashSet<String>,
}

pub fn react_const_elements(config: ReactConstElementsOptions) -> impl Fold + VisitMut {
  let pass = ReactConstElements {
    state: State::new(config),
  };
  as_folder(pass)
}

struct ReactConstElements {
  state: State,
}

impl ReactConstElements {
  fn declare_scope_vars(&mut self, ids: Vec<Id>) {
    for id in ids {
      if self.state.immutable_ids.contains(&id) {
        self.state.vars.insert(id);
      }
    }
  }

  fn fn_contains_jsx<'a, T>(&'a mut self, n: &mut T)
  where
    T: VisitMutWith<FnReturnsJsx<'a>>,
  {
    let mut v = FnReturnsJsx {
      state: &mut self.state,
    };

    n.visit_mut_with(&mut v);
  }

  fn visit_mut_stmt_likes<T>(&mut self, stmts: &mut Vec<T>)
  where
    T: StmtLike,
  {
    // Finds all declares
    for stmt in stmts.iter() {
      self.declare_scope_vars(stmt.const_declared_by_item());
    }

    for stmt in stmts.iter_mut() {
      if let Some(stmt) = stmt.as_stmt_mut() {
        self.fn_contains_jsx(stmt)
      } else if let Some(module_decl) = stmt.as_module_decl_mut() {
        self.fn_contains_jsx(module_decl);
      }
    }

    let candidates: Vec<Ident> = std::mem::take(&mut self.state.candidates);

    self.state.candidates = vec![];

    if !candidates.is_empty() {
      stmts.insert(
        0,
        StmtLike::from_stmt(Stmt::Decl(Decl::Var(Box::new(VarDecl {
          span: DUMMY_SP,
          kind: VarDeclKind::Let,
          declare: false,
          decls: candidates
            .iter()
            .map(|candidate| VarDeclarator {
              span: DUMMY_SP,
              name: Pat::Ident(BindingIdent {
                id: candidate.clone(),
                type_ann: None,
              }),
              init: None,
              definite: true,
            })
            .collect(),
        })))),
      );
    }

    // Look into children scopes
    stmts.iter_mut().for_each(|stmt| {
      if let Some(stmt) = stmt.as_stmt_mut() {
        stmt.visit_mut_with(self);
      }
    });
  }
}

impl VisitMut for ReactConstElements {
  noop_visit_mut_type!();

  fn visit_mut_module(&mut self, module: &mut Module) {
    self.visit_mut_stmt_likes(&mut module.body)
  }

  fn visit_mut_script(&mut self, script: &mut Script) {
    self.visit_mut_stmt_likes(&mut script.body)
  }

  fn visit_mut_program(&mut self, program: &mut Program) {
    self.state.immutable_ids = get_immutable_ids(program);
    program.visit_mut_children_with(self);
  }

  fn visit_mut_arrow_expr(&mut self, n: &mut ArrowExpr) {
    self.declare_scope_vars(find_pat_ids(&n.params));

    if let BlockStmtOrExpr::Expr(expr) = n.body.as_ref() {
      *n.body = BlockStmtOrExpr::BlockStmt(BlockStmt {
        span: DUMMY_SP,
        stmts: vec![Stmt::Return(ReturnStmt {
          span: DUMMY_SP,
          arg: Some(expr.clone()),
        })],
      });
    }

    n.visit_mut_children_with(self);
  }

  fn visit_mut_function(&mut self, n: &mut Function) {
    self.declare_scope_vars(find_pat_ids(&n.params));

    n.visit_mut_children_with(self);
  }

  fn visit_mut_stmts(&mut self, n: &mut Vec<Stmt>) {
    self.visit_mut_stmt_likes(n);
  }
}

#[derive(Debug, Default)]
pub struct State {
  config: ReactConstElementsOptions,
  vars: rustc_hash::FxHashSet<Id>,
  candidates: Vec<Ident>,
  ctxt: SyntaxContext,

  next_id: u32,
  used_names: rustc_hash::FxHashSet<String>,

  // all immutable ids in module/script node
  immutable_ids: rustc_hash::FxHashSet<Id>,
}

impl State {
  fn new(config: ReactConstElementsOptions) -> Self {
    Self {
      vars: Default::default(),
      candidates: Default::default(),
      ctxt: DUMMY_SP.apply_mark(Mark::new()).ctxt,
      next_id: 0,
      used_names: Default::default(),
      immutable_ids: Default::default(),
      config,
    }
  }

  fn create_id(&mut self, name: Option<String>) -> Ident {
    self.next_id += 1;

    let name = if let Some(name) = name {
      name
    } else {
      "_SWC_CONST_ELE_FRAG".into()
    };

    if self.used_names.contains(&name) {
      return self.create_id(Some(format!("{}{}", name, self.next_id)));
    }

    self.used_names.insert(name.clone());
    Ident::new(name.into(), DUMMY_SP.with_ctxt(self.ctxt))
  }
}

// This visitor iterates current scope stmts, and find the one contains function which has jsx inside of it
// eg.
// const foo = () => <div></div>
// function foo() { return <div></div> }
// But don't handle  the more deep scopes. for example: foo = () => () => <div></div>, avoid repeated work
struct FnReturnsJsx<'a> {
  state: &'a mut State,
}

impl<'a> VisitMut for FnReturnsJsx<'a> {
  noop_visit_mut_type!();

  fn visit_mut_arrow_expr(&mut self, arrow: &mut ArrowExpr) {
    match arrow.body.as_mut() {
      // foo = ()=>{ return <div></div> }
      BlockStmtOrExpr::BlockStmt(block) => {
        block
          .stmts
          .visit_mut_children_with(&mut JSXHoister::new(self.state));
      }

      // foo = () => <div></div>
      BlockStmtOrExpr::Expr(expr) => {
        expr.visit_mut_with(&mut JSXHoister::new(self.state));
      }
    }
  }

  fn visit_mut_function(&mut self, func: &mut Function) {
    // foo = function () { return <div></div> }
    if let Some(body) = &mut func.body {
      body
        .stmts
        .visit_mut_children_with(&mut JSXHoister::new(self.state));
    }
  }

  fn visit_mut_class(&mut self, class: &mut Class) {
    for member in &mut class.body {
      match member {
        ClassMember::Constructor(constructor) => {
          if let Some(body) = &mut constructor.body {
            body
              .stmts
              .visit_mut_children_with(&mut JSXHoister::new(self.state));
          }
        }
        ClassMember::Method(method) => {
          method.visit_mut_children_with(self);
        }
        ClassMember::PrivateMethod(method) => {
          method.visit_mut_children_with(self);
        }
        ClassMember::ClassProp(prop) => {
          prop.value.visit_mut_with(self);
        }
        ClassMember::PrivateProp(prop) => {
          prop.value.visit_mut_with(self);
        }
        _ => {}
      }
    }
  }
}

// This visitor only cares the current scope
// If jsx in its child scope, do not hoist that
struct JSXHoister<'a> {
  state: &'a mut State,
}

impl<'a> JSXHoister<'a> {
  fn new(state: &'a mut State) -> Self {
    Self { state }
  }
}

impl<'a> VisitMut for JSXHoister<'a> {
  noop_visit_mut_type!();

  // stop looking if visit these, as these will introduce new scopes
  fn visit_mut_function(&mut self, _: &mut Function) {}
  fn visit_mut_arrow_expr(&mut self, _: &mut ArrowExpr) {}

  fn visit_mut_expr(&mut self, expr: &mut Expr) {
    if let Expr::JSXElement(jsx_ele) = expr {
      if modify_jsx_if_immutable(jsx_ele, self.state) {
        hoist_jsx(self.state, expr);
      }
    } else if let Expr::JSXFragment(frag) = expr {
      // if fragment children are all immutable, this fragment can be hoisted
      if are_children_immutable(&mut frag.children, self.state, false) {
        hoist_jsx(self.state, expr);
      }
    } else {
      expr.visit_mut_children_with(self);
    }
  }
}
