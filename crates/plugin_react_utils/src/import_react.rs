use swc_core::{
  common::{Mark, Span, DUMMY_SP},
  ecma::{
    ast::{
      Ident, ImportDecl, ImportDefaultSpecifier, ImportSpecifier, Module, ModuleDecl, ModuleItem,
      Str,
    },
    atoms::JsWord,
    visit::{as_folder, Fold, VisitMut, VisitMutWith},
  },
};
use swc_plugins_utils::change_ident_syntax_context;

struct ImportReact {
  top_level_mark: Mark,
}

impl VisitMut for ImportReact {
  fn visit_mut_module(&mut self, module: &mut Module) {
    let mut need_import = true;

    for item in &module.body {
      if let ModuleItem::ModuleDecl(ModuleDecl::Import(var)) = item {
        let source = &var.src.value;
        if source == "react" {
          for specifier in &var.specifiers {
            if let ImportSpecifier::Default(_) = specifier {
              // default import already exist
              need_import = false;
              break;
            }
          }
        }
      }
    }

    if need_import {
      let local_span = Span::dummy_with_cmt().apply_mark(self.top_level_mark);

      module.visit_mut_children_with(&mut change_ident_syntax_context(
        local_span.ctxt,
        "React".into(),
      ));

      let body = &mut module.body;
      let dec = ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
        span: DUMMY_SP,
        specifiers: vec![ImportSpecifier::Default(ImportDefaultSpecifier {
          span: DUMMY_SP,
          local: Ident {
            span: local_span,
            sym: JsWord::from("React"),
            optional: false,
          },
        })],
        src: Box::new(Str {
          span: DUMMY_SP,
          value: JsWord::from("react"),
          raw: None,
        }),
        type_only: false,
        asserts: None,
      }));
      body.insert(0, dec);
    }
  }
}

pub fn auto_import_react(top_level_mark: Mark) -> impl Fold + VisitMut {
  as_folder(ImportReact { top_level_mark })
}
