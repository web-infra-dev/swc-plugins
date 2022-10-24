use mappings::{build_mappings, Mappings, Package};
use shared::{swc_core::{
  self,
  common::{Mark, Span, DUMMY_SP},
  ecma::{
    ast::{
      CallExpr, ExportNamedSpecifier, ExportSpecifier, Expr, Id, Ident, ImportDecl,
      ImportDefaultSpecifier, ImportSpecifier, MemberProp, Module, ModuleDecl, ModuleExportName,
      ModuleItem, NamedExport, Str,
    },
    atoms::JsWord,
    utils::undefined,
    visit::{as_folder, Fold, VisitMut, VisitMutWith},
  },
  quote,
}, hashbrown::{HashMap, HashSet}};
use std::{
  ops::Deref,
  path::PathBuf,
};

mod error;
mod mappings;

extern crate glob;

#[derive(Debug, Default)]
pub struct PluginLodashConfig {
  pub cwd: PathBuf,
  pub ids: Vec<String>,
}

pub fn plugin_lodash(config: &PluginLodashConfig, top_level_mark: Mark) -> impl Fold {
  let mut ids = vec!["lodash".into(), "lodash-es".into(), "lodash-compat".into()];
  config.ids.iter().for_each(|id| {
    if !ids.contains(id) {
      ids.push(id.into());
    }
  });

  let mappings = build_mappings(ids.iter().map(|s| s.as_str()), &config.cwd).unwrap();
  let mut pkg_map = HashMap::default();

  for (id, module_map) in &mappings {
    for base in module_map.keys() {
      // Key is lodash, lodash/fp
      // `base` could be empty
      pkg_map.insert(
        {
          if base.is_empty() {
            JsWord::from(id.as_str())
          } else {
            JsWord::from(format!("{}/{}", &id, &base).as_str())
          }
        },
        Package::new(&config.cwd, id, base).unwrap(),
      );
    }
  }

  as_folder(PluginLodash {
    cwd: config.cwd.clone(),
    top_level_mark,
    mappings,
    pkg_map,
    imported_names: Default::default(),
    namespace_map: Default::default(),
    imports: Default::default(),
    exports: Default::default(),
    lodash_vars: Default::default(),
  })
}

#[derive(Debug, Default)]
pub struct PluginLodash {
  pub cwd: PathBuf,
  pkg_map: HashMap<JsWord, Package>,
  mappings: Mappings,

  top_level_mark: Mark,

  // HashMap<(module_id, local_id, imported), imported_ident>
  imported_names: HashMap<(JsWord, JsWord, JsWord), Id>,

  // Eg: "_" -> "lodash"
  namespace_map: HashMap<Id, JsWord>,
  imports: Vec<ImportDecl>,
  exports: Vec<Id>,

  lodash_vars: HashSet<Id>,
}

impl PluginLodash {
  fn add_import(&mut self, source: JsWord, local: Id, imported: Option<Id>) -> Id {
    let (imported_name, _) = imported.unwrap_or_else(|| local.clone());

    if let Some(id) =
      self
        .imported_names
        .get(&(source.clone(), local.0.clone(), imported_name.clone()))
    {
      return id.clone();
    }

    let pkg = self.pkg_map.get(&source).unwrap();
    let import_path = pkg
      .find_module(&self.mappings, &imported_name)
      .unwrap_or_else(|| {
        panic!(
          "Cannot find appropriate import path to: {}, in package: {}",
          imported_name, source
        )
      });
    let new_source = format!("{}/{}", pkg.id, import_path.display());

    self
      .imported_names
      .insert((source, local.0.clone(), imported_name), local.clone());

    self.imports.push(ImportDecl {
      span: DUMMY_SP,
      specifiers: vec![ImportSpecifier::Default(ImportDefaultSpecifier {
        span: DUMMY_SP,
        local: local.clone().into(),
      })],
      src: Box::new(Str {
        span: DUMMY_SP,
        value: JsWord::from(new_source),
        raw: None,
      }),
      type_only: false,
      asserts: None,
    });

    self.lodash_vars.insert(local.clone());
    local
  }
}

impl VisitMut for PluginLodash {
  // Clear states
  fn visit_mut_module(&mut self, module: &mut Module) {
    // First check all import decl, this can collect namespaces identifier,
    // Form `import _ from 'foo';` we can know that '_' is namespace identifier,
    // So that we can replace `_.map` into `map`
    for module_item in &mut module.body {
      match module_item {
        ModuleItem::ModuleDecl(ModuleDecl::Import(import)) => {
          import.visit_mut_with(self);
        }
        ModuleItem::ModuleDecl(ModuleDecl::ExportNamed(export_named)) => {
          export_named.visit_mut_with(self);
        }
        _ => {}
      }
    }

    for module_item in &mut module.body {
      // Skip import decl as we already visited
      if !matches!(module_item, ModuleItem::ModuleDecl(ModuleDecl::Import(_)))
        && !matches!(
          module_item,
          ModuleItem::ModuleDecl(ModuleDecl::ExportNamed(_))
        )
      {
        module_item.visit_mut_children_with(self);
      }
    }

    for import in &self.imports {
      module.body.insert(
        0,
        ModuleItem::ModuleDecl(ModuleDecl::Import(import.clone())),
      );
    }

    for export in &self.exports {
      module
        .body
        .push(ModuleItem::ModuleDecl(ModuleDecl::ExportNamed(
          NamedExport {
            span: DUMMY_SP,
            specifiers: vec![ExportSpecifier::Named(ExportNamedSpecifier {
              span: DUMMY_SP,
              orig: ModuleExportName::Ident(export.clone().into()),
              exported: None,
              is_type_only: false,
            })],
            src: None,
            type_only: false,
            asserts: None,
          },
        )))
    }

    module.visit_mut_with(&mut PostProcess {
      pkg_map: &self.pkg_map,
      namespaces: &self.namespace_map,
      lodash_vars: &self.lodash_vars,
      in_lodash_call: None,
    });
  }

  fn visit_mut_import_decl(&mut self, import_decl: &mut ImportDecl) {
    let source = &import_decl.src.value;

    if self.pkg_map.get(source).is_none() {
      return;
    }

    let specifiers = &import_decl.specifiers;
    for spec in specifiers {
      match spec {
        ImportSpecifier::Named(named_import) => {
          // Check import { default as id } from 'lodash';
          if let Some(ModuleExportName::Ident(id)) = &named_import.imported {
            if id.sym.to_string() == "default" {
              self.namespace_map.insert(id.to_id(), source.clone());
              continue;
            }
          }

          self.add_import(
            source.clone(),
            named_import.local.to_id(),
            imported_to_id(named_import.imported.clone()),
          );
        }
        ImportSpecifier::Default(default_spec) => {
          self
            .namespace_map
            .insert(default_spec.local.to_id(), source.clone());
        }
        ImportSpecifier::Namespace(namespace) => {
          self
            .namespace_map
            .insert(namespace.local.to_id(), source.clone());
        }
      };
    }
  }

  fn visit_mut_expr(&mut self, expr: &mut Expr) {
    // check for `_.map()`
    if let Expr::Member(member_expr) = expr {
      if let Expr::Ident(id) = &*member_expr.obj {
        if let (Some(source), MemberProp::Ident(prop)) =
          (self.namespace_map.get(&id.to_id()), &member_expr.prop)
        {
          // Check if this import should be replaced
          if self.pkg_map.get(source).is_some() {
            // Convert _.map() -> map#0()
            let local_name = format!("{}#{}", &prop.sym, self.imported_names.len());
            let local = Ident::new(
              JsWord::from(local_name),
              Span::dummy_with_cmt().apply_mark(self.top_level_mark),
            );

            let local = self.add_import(
              source.clone(),
              local.to_id(),
              Some(Ident::new(prop.sym.clone(), DUMMY_SP).to_id()),
            );
            *expr = Expr::Ident(local.into());
          }
        }
      }
    }

    expr.visit_mut_children_with(self);
  }

  // BEFORE
  // export { map, add } from 'lodash'
  // AFTER
  // import map from 'lodash/map'
  // import add from 'lodash/add'
  // export { map, add }
  fn visit_mut_named_export(&mut self, named_export: &mut NamedExport) {
    if let Some(source) = &named_export.src {
      if self.pkg_map.get(&source.value).is_some() {
        for spec in &named_export.specifiers {
          if let ExportSpecifier::Named(named_spec) = spec {
            let (local, imported) = if named_spec.exported.is_some() {
              // export { orig as exported } from 'lodash'
              (
                named_spec.exported.clone().unwrap(),
                Some(named_spec.orig.clone()),
              )
            } else {
              // export { orig } from 'lodash'
              (named_spec.orig.clone(), None)
            };

            let added_ident = self.add_import(
              source.value.clone(),
              export_name_to_ident(local).to_id(),
              imported.map(export_name_to_ident).map(|i| i.to_id()),
            );
            self.exports.push(added_ident);
          }
        }
      }
    }
  }
}

fn imported_to_id(imported: Option<ModuleExportName>) -> Option<Id> {
  imported.and_then(|module| match module {
    ModuleExportName::Ident(ident) => Some(ident.to_id()),
    ModuleExportName::Str(_) => None,
  })
}

// Remove useless import decl and export decl
// Replace every lodash global variable with (void 0)
struct PostProcess<'a> {
  pkg_map: &'a HashMap<JsWord, Package>,
  namespaces: &'a HashMap<Id, JsWord>,
  lodash_vars: &'a HashSet<Id>,
  in_lodash_call: Option<Id>,
}

impl<'a> PostProcess<'a> {
  fn is_from_lodash(&self, id: &Id) -> bool {
    self.lodash_vars.contains(id)
  }
}

impl<'a> VisitMut for PostProcess<'a> {
  fn visit_mut_module(&mut self, module: &mut Module) {
    let mut removed = vec![];

    for (idx, module_item) in module.body.iter().enumerate() {
      if let ModuleItem::ModuleDecl(ModuleDecl::Import(import_decl)) = module_item {
        if self.pkg_map.get(&import_decl.src.value).is_some() {
          removed.push(idx);
        }
      } else if let ModuleItem::ModuleDecl(ModuleDecl::ExportNamed(named_export)) = module_item {
        if let Some(source) = &named_export.src {
          if self.pkg_map.get(&source.value).is_some() {
            removed.push(idx);
          }
        }
      }
    }

    for idx in removed.iter().rev() {
      module.body.remove(*idx);
    }

    module.visit_mut_children_with(self);
  }

  fn visit_mut_call_expr(&mut self, call_expr: &mut CallExpr) {
    let prev = self.in_lodash_call.clone();
    if let Some(Expr::Ident(ident)) = call_expr.callee.as_expr().map(Deref::deref) {
      let id = ident.to_id();
      if self.is_from_lodash(&id) {
        // Callee is imported from lodash
        // Replace all arguments
        self.in_lodash_call = Some(id);
      }
    }
    call_expr.visit_mut_children_with(self);
    self.in_lodash_call = prev;
  }

  // `_; map(_)` ===>>> `void 0; map(map.place_holder)`
  fn visit_mut_expr(&mut self, expr: &mut Expr) {
    let prev = self.in_lodash_call.clone();
    if let Some(id) = detect_curried_fn(expr) {
      self.in_lodash_call = Some(id);
    }

    // Replace _ with void 0 or placeholder
    if let Expr::Ident(ident) = &expr {
      if self.namespaces.get(&ident.to_id()).is_some() {
        if let Some(id) = &self.in_lodash_call {
          *expr = quote!("$id.placeholder" as Expr, id: Ident = id.clone().into());
        } else {
          *expr = *undefined(DUMMY_SP);
        }
      }
    }

    expr.visit_mut_children_with(self);

    self.in_lodash_call = prev;
  }
}

fn export_name_to_ident(export_name: ModuleExportName) -> Ident {
  match export_name {
    ModuleExportName::Ident(ident) => ident,
    ModuleExportName::Str(_) => panic!("Export name cannot be string literal"),
  }
}

// partial(func)(_);
// ^^^^^^^-------^: Some(Id[partial])
fn detect_curried_fn(expr: &Expr) -> Option<Id> {
  if let Expr::Call(call_expr) = expr {
    let inner = call_expr
      .callee
      .as_expr()
      .map(Deref::deref)
      .and_then(detect_curried_fn);

    if let Some(inner) = inner {
      Some(inner)
    } else if let Some(Expr::Call(call_expr)) = call_expr
    .callee
    .as_expr()
    .map(Deref::deref)
    {
      if let Some(Expr::Ident(ident)) = call_expr.callee.as_expr().map(Deref::deref) {
        Some(ident.to_id())
      } else {
        None
      }
    } else {
      None
    }
  } else {
    None
  }
}

#[test]
fn test_detect_curry() {
  let expr = quote!(
    "curryFn(func)([_])" as Expr
  );

  assert!(detect_curried_fn(&expr).is_some());
  assert_eq!("curryFn", detect_curried_fn(&expr).unwrap().0.to_string());

  let expr = quote!(
    "curryFn()()()(func)([_])" as Expr
  );

  assert!(detect_curried_fn(&expr).is_some());
  assert_eq!("curryFn", detect_curried_fn(&expr).unwrap().0.to_string());

  let expr = quote!(
    "curryFn([_])" as Expr
  );

  assert!(detect_curried_fn(&expr).is_none());
}
