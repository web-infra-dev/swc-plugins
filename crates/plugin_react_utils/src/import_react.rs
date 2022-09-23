use shared::swc_atoms::JsWord;
use shared::swc_common::DUMMY_SP;
use shared::swc_ecma_ast::{
  Ident, ImportDecl, ImportDefaultSpecifier, ImportSpecifier, Module, ModuleDecl, ModuleItem, Str,
};
use shared::swc_ecma_visit::{as_folder, Fold, VisitMut};

struct ImportReact;

impl VisitMut for ImportReact {
  fn visit_mut_module(&mut self, module: &mut Module) {
    let mut need_add = true;

    for item in &module.body {
      if let ModuleItem::ModuleDecl(ModuleDecl::Import(var)) = item {
        let source = &var.src.value;
        if source == "react" {
          for specifier in &var.specifiers {
            match specifier {
              ImportSpecifier::Named(ref _s) => {}
              ImportSpecifier::Default(ref s) => {
                if &s.local.sym == "React" {
                  need_add = false;
                }
              }
              ImportSpecifier::Namespace(ref _s) => {}
            }
          }
        }
      }
    }

    if need_add {
      let body = &mut module.body;
      let dec = ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
        span: DUMMY_SP,
        specifiers: vec![ImportSpecifier::Default(ImportDefaultSpecifier {
          span: DUMMY_SP,
          local: Ident {
            span: DUMMY_SP,
            sym: JsWord::from("React"),
            optional: false,
          },
        })],
        src: Str {
          span: DUMMY_SP,
          value: JsWord::from("react"),
          raw: None,
        },
        type_only: false,
        asserts: None,
      }));
      body.insert(0, dec);
    }
  }
}

pub fn auto_import_react() -> impl Fold + VisitMut {
  as_folder(ImportReact)
}
