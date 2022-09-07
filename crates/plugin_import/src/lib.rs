mod visit;

use crate::visit::IdentComponent;
use serde::{self, Serialize};
use shared::napi::{Env, JsFunction};
use shared::swc_ecma_ast::{
  self, Ident, ImportDecl, ImportDefaultSpecifier, ImportSpecifier, Module, ModuleDecl, ModuleItem,
  Str,
};
use shared::swc_ecma_visit::{as_folder, Fold, VisitMut, VisitWith};
use shared::{napi, napi_derive::napi};
use shared::{swc_atoms::JsWord, swc_common::DUMMY_SP};

pub fn plugin_import<'a>(config: Vec<PluginImportConfig>, env: Env) -> impl Fold + 'a {
  as_folder(ImportPlugin { config, env })
}

pub struct EsSpec {
  source: String,
  default_spec: String,
}

pub struct ImportPlugin {
  pub config: Vec<PluginImportConfig>,
  pub env: Env,
}

impl VisitMut for ImportPlugin {
  fn visit_mut_module(&mut self, module: &mut Module) {
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

    let config = &self.config;

    for item in &module.body {
      let item_index = module.body.iter().position(|citem| citem == item).unwrap();
      if let ModuleItem::ModuleDecl(ModuleDecl::Import(var)) = item {
        let source = &*var.src.value;
        if let Some(child_config) = config.iter().find(|&c| c.from_source == source) {
          for specifier in &var.specifiers {
            match specifier {
              ImportSpecifier::Named(ref s) => {
                let ident: String = s.local.sym.to_string();
                if match_ident(&s.local) {

                  if let Some(ref css) = child_config.replace_css {
                    let ignore_component = &css.ignore_style_component;
                    let need_lower = css.lower.unwrap_or(false);
                    let css_ident = if need_lower {
                      camel2snake(&ident)
                    } else {
                      ident.clone()
                    };
                    let mut need_replace = true;
                    if let Some(block_list) = ignore_component {
                      need_replace = !block_list.iter().map(|c| c.as_str()).any(|x| x == ident);
                    }
                    if need_replace {
                      let import_css_source = css
                        .replace_expr
                        .call(None, &[self.env.create_string(css_ident.as_str()).unwrap()])
                        .unwrap()
                        .coerce_to_string()
                        .unwrap()
                        .into_utf8()
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .to_string();
                      specifiers_css.push(import_css_source);
                    }
                  }

                  if let Some(ref js_config) = child_config.replace_js {
                    let ignore_component = &js_config.ignore_es_component;
                    let need_lower = js_config.lower.unwrap_or(false);
                    let js_ident = if need_lower {
                      camel2snake(&ident)
                    } else {
                      ident.clone()
                    };
                    let mut need_replace = true;
                    if let Some(block_list) = ignore_component {
                      need_replace = !block_list.iter().map(|c| c.as_str()).any(|x| x == ident);
                    }
                    if need_replace {
                      let import_es_source = js_config
                        .replace_expr
                        .call(None, &[self.env.create_string(js_ident.as_str()).unwrap()])
                        .unwrap()
                        .coerce_to_string()
                        .unwrap()
                        .into_utf8()
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .to_string();
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

#[napi(object)]
pub struct PluginImportConfig {
  pub from_source: String,
  pub replace_css: Option<ReplaceCssConfig>,
  pub replace_js: Option<ReplaceSpecConfig>,
}

#[napi(object)]
#[derive(Serialize)]
pub struct ReplaceSpecConfig {
  #[serde(skip_serializing)]
  pub replace_expr: JsFunction,
  pub ignore_es_component: Option<Vec<String>>,
  pub lower: Option<bool>,
}

#[napi(object)]
#[derive(Serialize)]
pub struct ReplaceCssConfig {
  pub ignore_style_component: Option<Vec<String>>,
  #[serde(skip_serializing)]
  pub replace_expr: JsFunction,
  pub lower: Option<bool>,
}

// camelCase -> snake_case
fn camel2snake(raw: &str) -> String {
  let mut first = true;

  raw
    .chars()
    .map(|c| {
      let s = if c.is_uppercase() {
        let mut s = c.to_lowercase().to_string();
        if !first {
          s = format!("-{}", s);
        }
        first = false;
        s
      } else {
        c.into()
      };
      s
    })
    .collect()
}
