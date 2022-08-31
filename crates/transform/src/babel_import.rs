use crate::types::TransformConfig;
use crate::visit::IdentComponent;
use shared::swc_ecma_ast::{
  self, Ident, ImportDecl, ImportDefaultSpecifier, ImportSpecifier, Module, ModuleDecl, ModuleItem,
  Str,
};
use shared::swc_ecma_visit::{VisitMut, VisitWith};
use shared::{swc_atoms::JsWord, swc_common::DUMMY_SP};

pub fn plugin_import<'a>(project_config: &'a TransformConfig) -> ImportPlugin<'a> {
  ImportPlugin { project_config }
}

pub struct EsSpec {
  source: String,
  default_spec: String,
}

pub struct ImportPlugin<'a> {
  pub project_config: &'a TransformConfig,
}

impl<'a> ImportPlugin<'a> {
  fn transform(&self, tpl: &str, _name: &str) -> String {
    let _regex = regex::Regex::new(tpl).unwrap();
    // TODO
    let res = String::new();
    res
  }
}

impl<'a> VisitMut for ImportPlugin<'a> {
  fn visit_mut_module(&mut self, module: &mut Module) {
    let project_config = &self.project_config;
    if project_config.extensions.plugin_import.is_none()
      || project_config.extensions.plugin_import.as_ref().unwrap().is_empty()
    {
      return;
    }
    // let s = serde_json::to_string_pretty(&module).expect("failed to serialize");

    let mut visitor = IdentComponent {
      component_name_jsx_ident: vec![],
      ident_list: vec![],
      ts_type_ident_list: vec![],
    };
    module.body.visit_with(&mut visitor);

    let match_ident = |ident: &Ident| -> bool {
      let name = ident.to_string().replace("#0", "");
      let mark = ident.span.ctxt.as_u32();
      let item = (name, mark);
      visitor.component_name_jsx_ident.contains(&item)
        || (visitor.ident_list.contains(&item) && !visitor.ts_type_ident_list.contains(&item))
    };

    let mut specifiers_css = vec![];
    let mut specifiers_es = vec![];
    let mut specifiers_rm_es = vec![];

    let config = project_config.extensions.plugin_import.as_ref().unwrap();

    for item in &module.body {
      let item_index = module.body.iter().position(|citem| citem == item).unwrap();
      if let ModuleItem::ModuleDecl(ModuleDecl::Import(var)) = item {
        let source = &*var.src.value;
        if let Some(child_config) = config.iter().find(|&c| c.source == source) {
          for specifier in &var.specifiers {
            match specifier {
              ImportSpecifier::Named(ref s) => {
                let ident: String = s.local.sym.to_string();
                if match_ident(&s.local) {
                  // replace style
                  let ignore_component = &child_config.ignore_components;

                  let need_lower = child_config.lower;
                  let css_ident = if need_lower {
                    ident.to_lowercase()
                  } else {
                    ident.clone()
                  };
                  let mut need_replace = true;
                  if let Some(block_list) = ignore_component {
                    need_replace = !block_list.iter().map(|c| c.as_str()).any(|x| x == ident);
                  }
                  if need_replace {
                    let import_css_source = self.transform(&child_config.transform_style, &css_ident);
                    specifiers_css.push(import_css_source);
                  }

                  // replace es
                  let ignore_component = &child_config.ignore_components;

                  let mut js_ident = ident.clone();
                  if child_config.lower {
                    js_ident = ident.to_lowercase();
                  }
                  let mut need_replace = true;
                  if let Some(block_list) = ignore_component {
                    need_replace = !block_list.iter().map(|c| c.as_str()).any(|x| x == ident);
                  }
                  if need_replace {
                    let import_es_source = self.transform(&child_config.transform_es, &js_ident);
                    specifiers_es.push(EsSpec {
                      source: import_es_source,
                      default_spec: ident,
                    });
                    if !specifiers_rm_es.iter().any(|&c| c == item_index) {
                      specifiers_rm_es.push(item_index);
                    }
                  }
                }
              }
              ImportSpecifier::Default(ref _s) => {}
              ImportSpecifier::Namespace(ref _ns) => {}
            }
          }
        }
      }
    }

    let body = &mut module.body;

    let mut index: usize = 0;
    while let Some(i) = specifiers_rm_es.get(index) {
      let rm_index = *i - index;
      body.remove(rm_index);
      index += 1;
    }

    for js_source in specifiers_es {
      let js_source_ref = js_source.source.as_str();
      let dec = ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
        span: DUMMY_SP,
        specifiers: vec![swc_ecma_ast::ImportSpecifier::Default(
          ImportDefaultSpecifier {
            span: DUMMY_SP,
            local: swc_ecma_ast::Ident {
              span: DUMMY_SP,
              sym: JsWord::from(js_source.default_spec.as_str()),
              optional: false,
            },
          },
        )],
        src: Str {
          span: DUMMY_SP,
          value: JsWord::from(js_source_ref),
          raw: None,
        },
        type_only: false,
        asserts: None,
      }));
      body.insert(0, dec);
    }

    for css_source in specifiers_css {
      let css_source_ref = css_source.as_str();
      let dec = ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
        span: DUMMY_SP,
        specifiers: vec![],
        src: Str {
          span: DUMMY_SP,
          value: JsWord::from(css_source_ref),
          raw: None,
        },
        type_only: false,
        asserts: None,
      }));
      body.insert(0, dec);
    }
  }
}
