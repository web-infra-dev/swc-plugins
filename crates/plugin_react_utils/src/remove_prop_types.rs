use std::{
  borrow::{Borrow, BorrowMut},
  collections::{HashMap, HashSet},
  ops::DerefMut,
  sync::Arc,
};

use shared::{
  serde::Deserialize,
  swc_core::{
    self,
    cached::regex::CachedRegex,
    common::{comments::Comments, util::take::Take, Spanned, DUMMY_SP},
    ecma::{
      ast::{
        AssignExpr, BlockStmt, Class, ClassDecl, ClassExpr, ClassMember, ClassProp, Decl,
        DefaultDecl, ExportDecl, ExportDefaultDecl, ExportDefaultExpr, Expr, ExprStmt, FnDecl, Id,
        Ident, ImportDecl, ImportSpecifier, Module, ModuleDecl, ModuleItem, PropName, Stmt,
        VarDecl,
      },
      atoms::JsWord,
      visit::{as_folder, Fold, Visit, VisitMut, VisitMutWith, VisitWith},
    },
    quote,
  },
  utils::{
    collect_bindings, contain_ident, count_ident, is_react_component, is_react_component_class,
    is_return_jsx, remove_invalid_expr, BindingInfo, ReactComponentType,
  },
  PluginContext,
};

pub fn react_remove_prop_types(
  config: &ReactRemovePropTypeConfig,
  plugin_context: Arc<PluginContext>,
) -> impl Fold + '_ {
  if config.remove_import && !matches!(config.mode, Mode::Removal) {
    panic!(
      r#"react-remove-prop-type: removeImport = true and mode != "remove" can not be used at the same time."#
    );
  }

  as_folder(ReactRemovePropTypes {
    config,
    comments: plugin_context.comments.clone(),
    components: Default::default(),
    namespaces: Default::default(),
    inserts: Default::default(),
    bindings: Default::default(),
  })
}

#[derive(Debug, Clone, Deserialize)]
#[serde(crate = "shared::serde")]
pub enum Mode {
  Removal,
  Wrap,
  UnsafeWrap,
}

impl From<String> for Mode {
  fn from(mode: String) -> Self {
    match mode.as_str() {
      "remove" => Self::Removal,
      "wrap" => Self::Wrap,
      "unsafe-wrap" => Self::UnsafeWrap,
      _ => Self::Removal,
    }
  }
}

impl Default for Mode {
  fn default() -> Self {
    Self::Removal
  }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(crate = "shared::serde", rename_all = "camelCase")]
pub struct ReactRemovePropTypeConfig {
  #[serde(default)]
  pub mode: Mode,
  #[serde(default)]
  pub remove_import: bool,
  #[serde(default)]
  pub ignore_filenames: Vec<CachedRegex>,
  #[serde(default)]
  pub additional_libraries: Vec<CachedRegex>,
  #[serde(default)]
  pub class_name_matchers: Vec<CachedRegex>,
}

impl Default for ReactRemovePropTypeConfig {
  fn default() -> Self {
    Self {
      mode: Default::default(),
      remove_import: true,
      ignore_filenames: Default::default(),
      additional_libraries: Default::default(),
      class_name_matchers: Default::default(),
    }
  }
}

impl ReactRemovePropTypeConfig {
  fn is_match_library(&self, name: &str) -> bool {
    if name == "prop-types" {
      return true;
    }

    return self
      .additional_libraries
      .iter()
      .any(|lib| lib.is_match(name));
  }
}

#[derive(Debug)]
pub struct ReactRemovePropTypes<'a, C>
where
  C: Comments,
{
  pub config: &'a ReactRemovePropTypeConfig,
  comments: C,

  components: HashSet<Id>,
  namespaces: HashSet<Id>,
  inserts: Vec<(usize, ModuleItem)>,
  bindings: HashMap<Id, BindingInfo>,
}

impl<'a, C> ReactRemovePropTypes<'a, C>
where
  C: Comments,
{
  fn visit_stmts(&mut self, mut stmts: Vec<&mut Stmt>) {
    for (idx, stmt) in stmts.iter_mut().enumerate().rev() {
      match stmt {
        Stmt::Expr(ExprStmt { expr, .. }) => {
          if let Expr::Assign(assign_expr) = expr.borrow_mut() {
            let left = &assign_expr.left;

            // App.propTypes = {...}
            if let Some(Expr::Member(member_decl)) = left.as_expr() {
              if let Expr::Ident(obj) = &*member_decl.obj {
                if self.is_react_class(&obj.to_id())
                  && (member_decl.prop.is_ident()
                    && &member_decl.prop.as_ident().unwrap().sym == "propTypes")
                {
                  self.remove_expr(expr);
                }
              }
            }
          }
        }
        Stmt::Decl(Decl::Class(class_decl)) => {
          // class App { static propTypes: {...} }
          self.remove_class(Some(&class_decl.ident), &mut class_decl.class, idx);
        }
        _ => {}
      }
    }
  }

  fn remove_expr(&mut self, expr: &mut Expr) {
    match expr {
      Expr::Assign(assign_expr) => {
        match &self.config.mode {
          Mode::Removal => {
            expr.take();
          }

          // left = condition ? right : {};
          Mode::Wrap => {
            let value = *assign_expr.right.clone();
            assign_expr.right = quote!(
              r#"process.env.NODE_ENV !== "production" ? $value : {}"# as Box<Expr>,
              value: Expr = value
            );
          }

          // condition ? left = right : void 0;
          Mode::UnsafeWrap => {
            let assign: AssignExpr = assign_expr.clone();

            *expr = quote!(
              r#"process.env.NODE_ENV !== "production" ? $assign : void 0"# as Expr,
              assign: Expr = Expr::Assign(assign)
            );
          }
        }
      }
      _ => match &self.config.mode {
        Mode::Removal => {
          expr.take();
        }
        _ => {
          *expr = quote!(
            r#"process.env.NODE_ENV !== "production" ? $e : {}"# as Expr,
            e: Expr = expr.clone()
          );
        }
      },
    }
  }

  fn remove_class(&mut self, ident: Option<&Ident>, class: &mut Class, idx: usize) {
    // Remove static propTypes
    if !is_react_component_class(class, Some(&self.bindings)) {
      return;
    }

    let mut removal = None;
    for (idx, field) in class.body.iter().enumerate().rev() {
      match field {
        ClassMember::ClassProp(property) => {
          if !property.is_static {
            continue;
          }
          if let PropName::Ident(property_ident) = &property.key {
            if &property_ident.sym == PROP_TYPES {
              removal = Some((idx, property.value.clone()));
              break;
            }
          }
        }
        ClassMember::PrivateProp(property) => {
          if !property.is_static {
            continue;
          }
          if &property.key.id.sym == PROP_TYPES {
            removal = Some((idx, property.value.clone()));
            break;
          }
        }
        _ => {}
      }
    }

    if let Some((removal_idx, old_val)) = removal {
      match &self.config.mode {
        Mode::Removal => {
          class.body.remove(removal_idx);
        }
        Mode::Wrap => {
          class.body[removal_idx] = ClassMember::ClassProp(ClassProp {
            span: DUMMY_SP,
            key: PropName::Ident(Ident::new(JsWord::from(PROP_TYPES), DUMMY_SP)),
            value: Some(quote!(
              r#"process.env.NODE_ENV !== "production" ? $obj : {}"# as Box<Expr>,
              obj: Expr = old_val.map(|item| *item).unwrap_or(quote!("{}" as Expr))
            )),
            type_ann: None,
            is_static: true,
            decorators: Default::default(),
            accessibility: None,
            is_abstract: false,
            is_optional: false,
            is_override: false,
            readonly: false,
            declare: false,
            definite: false,
          })
        }
        Mode::UnsafeWrap => {
          if let Some(ident) = ident {
            self.inserts.push((
              idx + 1,
              ModuleItem::Stmt(Stmt::Expr(ExprStmt {
                span: DUMMY_SP,
                expr: Box::new(quote!(
                  r#"process.env.NODE_ENV !== "production" ? $obj.propTypes = $value : void 0"#
                    as Expr,
                  obj = ident.clone(),
                  value: Expr = old_val.map(|item| *item).unwrap_or(quote!("{}" as Expr))
                )),
              })),
            ));
            class.body.remove(removal_idx);
          } else {
            class.body[removal_idx] = ClassMember::ClassProp(ClassProp {
              span: DUMMY_SP,
              key: PropName::Ident(Ident::new(JsWord::from(PROP_TYPES), DUMMY_SP)),
              value: Some(quote!(
                r#"process.env.NODE_ENV !== "production" ? $obj : void 0"# as Box<Expr>,
                obj: Expr = old_val.map(|item| *item).unwrap_or(quote!("{}" as Expr))
              )),
              type_ann: None,
              is_static: true,
              decorators: Default::default(),
              accessibility: None,
              is_abstract: false,
              is_optional: false,
              is_override: false,
              readonly: false,
              declare: false,
              definite: false,
            })
          }
        }
      }
    }
  }

  fn is_react_class(&self, name: &Id) -> bool {
    self.components.contains(name)
  }
}

static PROP_TYPES: &str = "propTypes";

impl<'a, C> VisitMut for ReactRemovePropTypes<'a, C>
where
  C: Comments,
{
  fn visit_mut_module(&mut self, module: &mut Module) {
    self.bindings = collect_bindings(module);

    // 1.
    // Find all React class or function components, store in components
    // Find all namespaces, store in namespaces
    let mut collector = CollectReactComponent {
      config: self.config,
      ids: Default::default(),
      namespaces: Default::default(),
      bindings: &self.bindings,
    };

    module.visit_with(&mut collector);

    self.components = collector.ids;
    self.namespaces = collector.namespaces;

    // 2.
    // Replace top level App.propTypes = {}, class App{ static propTypes = {} }...
    self.visit_stmts(
      module
        .body
        .iter_mut()
        .filter_map(|item| item.as_mut_stmt())
        .collect(),
    );

    // 3.
    // Handle export declarations
    // export const App = () => <></>
    for (idx, item) in module.body.iter_mut().enumerate().rev() {
      match item {
        ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(ExportDecl {
          decl: Decl::Class(class_decl),
          ..
        })) => {
          self.remove_class(Some(&class_decl.ident), &mut class_decl.class, idx);
        }
        ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultDecl(ExportDefaultDecl {
          decl: DefaultDecl::Class(class_expr),
          ..
        })) => {
          self.remove_class(class_expr.ident.as_ref(), &mut class_expr.class, idx);
        }
        ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultExpr(ExportDefaultExpr {
          expr, ..
        })) => {
          if let Expr::Class(class_expr) = expr.deref_mut() {
            self.remove_class(class_expr.ident.as_ref(), &mut class_expr.class, idx);
          }
        }
        _ => {}
      }
    }

    // 4.
    // We only have checked top level App.propTypes = {}, but not any function body or other block stmt
    module.visit_mut_children_with(self);

    for (idx, stmt) in &self.inserts {
      module.body.insert(*idx, stmt.clone());
    }

    // 5.
    // Last we remove useless import
    if self.config.remove_import {
      let ident_counts = count_ident(module);
      for stmt in &mut module.body {
        if let ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
          specifiers, src, ..
        })) = stmt
        {
          if self.config.is_match_library(&src.value) {
            let mut removed = vec![];
            for (idx, spec) in specifiers.iter_mut().enumerate().rev() {
              match spec {
                ImportSpecifier::Named(named) => {
                  if *ident_counts.get(&named.local.to_id()).unwrap() == 1 {
                    removed.push(idx);
                  }
                }
                ImportSpecifier::Default(default_import) => {
                  if *ident_counts.get(&default_import.local.to_id()).unwrap() == 1 {
                    removed.push(idx);
                  }
                }
                ImportSpecifier::Namespace(namespace) => {
                  if *ident_counts.get(&namespace.local.to_id()).unwrap() == 1 {
                    removed.push(idx);
                  }
                }
              }
            }

            if removed.len() == specifiers.len() {
              stmt.take();
            } else {
              for idx in removed {
                specifiers.remove(idx);
              }
            }
          }
        }
      }
    }

    // Finalize
    // remove any Expr::invalid
    remove_invalid_expr(module);
  }

  // var sharedPropType = PropTypes.number;
  fn visit_mut_var_decl(&mut self, var_decl: &mut VarDecl) {
    for decl in &mut var_decl.decls {
      if let Some(init) = decl.init.as_deref_mut() {
        if !self.namespaces.iter().any(|id| contain_ident(id, init)) {
          continue;
        }

        self.remove_expr(init);
      }
    }
  }

  // a.b.c /* remove-proptypes */ = PropTypes.number
  fn visit_mut_expr(&mut self, expr: &mut Expr) {
    if let Expr::Assign(assign) = expr {
      if let Some(comments) = self.comments.get_trailing(assign.left.span().hi) &&
      comments.iter().all(|comment| {comment.text.trim() == "remove-proptypes"})
      {
        self.remove_expr(expr);
      }
    }

    expr.visit_mut_children_with(self);
  }

  fn visit_mut_block_stmt(&mut self, block_stmt: &mut BlockStmt) {
    self.visit_stmts(block_stmt.stmts.iter_mut().collect());
    block_stmt.visit_mut_children_with(self);
  }
}

struct CollectReactComponent<'a> {
  config: &'a ReactRemovePropTypeConfig,
  ids: HashSet<Id>,
  namespaces: HashSet<Id>,
  bindings: &'a HashMap<Id, BindingInfo>,
}

impl<'a> CollectReactComponent<'a> {
  fn check_custom_class(&mut self, ident: &Ident, class: &Class) {
    let key = ident.to_id();
    if let Some(super_class) = class.super_class.as_deref() {
      if let Expr::Ident(super_class) = super_class && self.config.class_name_matchers.iter().any(|it| it.is_match(&super_class.sym)) {
        self.ids.insert(key);
      }
    }
  }
}

impl<'a> Visit for CollectReactComponent<'a> {
  fn visit_import_decl(&mut self, import_decl: &ImportDecl) {
    if !self.config.is_match_library(&import_decl.src.value) {
      return;
    }

    for spec in &import_decl.specifiers {
      match spec {
        ImportSpecifier::Default(default) => {
          self.namespaces.insert(default.local.to_id());
        }
        ImportSpecifier::Namespace(namespace) => {
          self.namespaces.insert(namespace.local.to_id());
        }
        _ => {}
      }
    }
  }

  fn visit_export_default_decl(&mut self, export_default: &ExportDefaultDecl) {
    match &export_default.decl {
      DefaultDecl::Class(class) => {
        if class.ident.is_none() {
          return;
        }

        let class_ident = class.ident.as_ref().unwrap();

        if is_react_component_class(&class.class, Some(self.bindings)) {
          self.ids.insert(class_ident.to_id());
        } else {
          self.check_custom_class(class_ident, &class.class);
        }
      }
      DefaultDecl::Fn(function) => {
        if function.ident.is_none() {
          return;
        }

        let fn_id = function.ident.as_ref().unwrap();

        if function.function.body.is_some()
          && is_return_jsx(
            function.function.body.as_ref().unwrap().stmts.iter(),
            Some(self.bindings),
          )
        {
          self.ids.insert(fn_id.to_id());
        }
      }
      _ => {}
    }
  }

  fn visit_export_decl(&mut self, export_decl: &ExportDecl) {
    match &export_decl.decl {
      Decl::Class(class_decl) => {
        if is_react_component_class(&class_decl.class, Some(&self.bindings)) {
          self.ids.insert(class_decl.ident.to_id());
        } else {
          self.check_custom_class(&class_decl.ident, &class_decl.class);
        }
      }
      Decl::Fn(function) => {
        if function
          .function
          .body
          .as_ref()
          .map(|body| is_return_jsx(body.stmts.iter(), Some(self.bindings)))
          .unwrap_or(false)
        {
          self.ids.insert(function.ident.to_id());
        }
      }
      Decl::Var(var_decl) => {
        var_decl.visit_with(self);
      }
      _ => {}
    }
  }

  // var App = () => { ... }
  // var App = function () { ... }
  fn visit_var_decl(&mut self, var_decl: &VarDecl) {
    for decl in &var_decl.decls {
      if decl.init.is_none() {
        continue;
      }

      let init = decl.init.as_deref().unwrap();

      let name = decl.name.as_ident();
      if name.is_none() {
        continue;
      }

      if matches!(
        is_react_component(init, Some(self.bindings)),
        ReactComponentType::FC | ReactComponentType::Class
      ) {
        let ident = &name.as_ref().unwrap().id;
        self.ids.insert(ident.to_id());
      }
    }
  }

  // App = () => { ... }
  // App = function() { ... }
  fn visit_assign_expr(&mut self, assign_expr: &AssignExpr) {
    if assign_expr.left.as_ident().is_none() {
      return;
    }

    if matches!(
      is_react_component(&assign_expr.right, Some(self.bindings)),
      ReactComponentType::FC | ReactComponentType::Class
    ) {
      let ident = assign_expr.left.as_ident().unwrap();
      self.ids.insert(ident.to_id());
    }
  }

  // function App() { ... }
  fn visit_fn_decl(&mut self, fn_decl: &FnDecl) {
    let id = &fn_decl.ident;

    if let Some(block_stmt) = &fn_decl.function.body && is_return_jsx(block_stmt.stmts.iter(), Some(self.bindings)) {
      self.ids.insert(id.to_id());
    }
  }

  // class App extends React.Component {}
  // class App extends Component {}
  fn visit_class_decl(&mut self, class_decl: &ClassDecl) {
    if is_react_component_class(&class_decl.class, Some(self.bindings)) {
      self.ids.insert(class_decl.ident.to_id());
    } else {
      self.check_custom_class(&class_decl.ident, &class_decl.class);
    }
  }

  // class App extends React.Component {}
  // class App extends Component {}
  fn visit_class_expr(&mut self, class_expr: &ClassExpr) {
    if class_expr.ident.is_none() {
      return;
    }

    if is_react_component_class(&class_expr.class, Some(self.bindings)) {
      self.ids.insert(class_expr.ident.as_ref().unwrap().to_id());
    } else {
      self.check_custom_class(class_expr.ident.as_ref().unwrap(), &class_expr.class);
    }
  }
}

struct RemoveImports<'a> {
  config: &'a ReactRemovePropTypeConfig,
  ids: HashMap<Id, usize>,
}

impl<'a> VisitMut for RemoveImports<'a> {
  fn visit_mut_module(&mut self, module: &mut Module) {
    module.visit_mut_children_with(self);

    let mut removed = vec![];
    for (idx, module_item) in module.body.iter().enumerate().rev() {
      if let ModuleItem::ModuleDecl(ModuleDecl::Import(import_decl)) = module_item {
        if self.config.is_match_library(&import_decl.src.value) {
          let safely_remove = import_decl.specifiers.iter().all(|spec| match spec {
            ImportSpecifier::Named(named_import) => {
              self.ids.get(&named_import.local.to_id()).unwrap() == &1
            }
            ImportSpecifier::Default(default_import) => {
              self.ids.get(&default_import.local.to_id()).unwrap() == &1
            }
            ImportSpecifier::Namespace(namespace_import) => {
              self.ids.get(&namespace_import.local.to_id()).unwrap() == &1
            }
          });

          if safely_remove {
            removed.push(idx);
          }
        }
      }
    }

    for idx in removed {
      module.body.remove(idx);
    }
  }

  fn visit_mut_ident(&mut self, ident: &mut Ident) {
    self
      .ids
      .entry(ident.to_id())
      .and_modify(|count| *count += 1)
      .or_insert(1);
  }
}
