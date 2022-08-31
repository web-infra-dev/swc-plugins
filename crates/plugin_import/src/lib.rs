mod visit;

use crate::visit::IdentComponent;
use handlebars::Handlebars;
use serde::{self, Deserialize, Serialize};
use shared::swc_ecma_ast::{
  self, Ident, ImportDecl, ImportDefaultSpecifier, ImportSpecifier, Module, ModuleDecl, ModuleItem,
  Str,
};
use shared::swc_ecma_visit::{as_folder, Fold, VisitMut, VisitWith};
use shared::{swc_atoms::JsWord, swc_common::DUMMY_SP};

pub fn plugin_import<'a>(config: &'a Vec<PluginImportItem>) -> impl Fold + 'a {
  let config = config
    .iter()
    .map(|item| {
      let mut handlebars = Handlebars::new();

      handlebars
        .register_template_string(&format!("{}-es", item.source), &item.transform_es)
        .unwrap();
      if let Some(ref transform_style) = item.transform_style {
        handlebars
          .register_template_string(&format!("{}-style", item.source), transform_style)
          .unwrap();
      }

      (item, handlebars)
    })
    .collect();

  as_folder(ImportPlugin { config })
}

pub struct EsSpec {
  source: String,
  default_spec: String,
}

pub struct ImportPlugin<'a> {
  pub config: Vec<(&'a PluginImportItem, Handlebars<'a>)>,
}

impl<'a> ImportPlugin<'a> {
  fn transform(&self, handlebars: &'a Handlebars, tpl: &str, member: &str) -> String {
    #[derive(Serialize)]
    struct Data<'a> {
      member: &'a str,
    }
    let data = Data { member };
    handlebars
      .render(tpl, &data)
      .expect("render template failed")
  }
}

impl<'a> VisitMut for ImportPlugin<'a> {
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
        if let Some((child_config, handlebars)) = config.iter().find(|&(c, _)| c.source == source) {
          for specifier in &var.specifiers {
            match specifier {
              ImportSpecifier::Named(ref s) => {
                let ident: String = s.local.sym.to_string();
                if match_ident(&s.local) {
                  // replace style
                  let ignore_component = &child_config.ignore_components;

                  let css_ident = if child_config.snake_case {
                    camel2snake(&ident)
                  } else {
                    ident.clone()
                  };
                  let mut need_replace = true;
                  if let Some(block_list) = ignore_component {
                    need_replace = !block_list.iter().map(|c| c.as_str()).any(|x| x == ident);
                  }
                  if need_replace && child_config.transform_style.is_some() {
                    let import_css_source =
                      self.transform(handlebars, &format!("{}-style", source), &css_ident);
                    specifiers_css.push(import_css_source);
                  }

                  // replace es
                  let ignore_component = &child_config.ignore_components;

                  let mut js_ident = ident.clone();
                  if child_config.snake_case {
                    js_ident = camel2snake(&js_ident);
                  }
                  let mut need_replace = true;
                  if let Some(block_list) = ignore_component {
                    need_replace = !block_list.iter().map(|c| c.as_str()).any(|x| x == ident);
                  }
                  if need_replace {
                    let import_es_source =
                      self.transform(handlebars, &format!("{}-es", source), &js_ident);
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

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all="camelCase")]
pub struct PluginImportItem {
  pub source: String,

  pub transform_es: String, // template syntax: foo/lib/{{member}}
  #[serde(default)]
  pub transform_style: Option<String>, // template syntax: foo/lib/{{member}}

  #[serde(default)]
  pub ignore_components: Option<Vec<String>>,
  #[serde(default)]
  pub snake_case: bool,
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
