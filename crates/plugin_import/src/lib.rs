mod visit;

use handlebars::{Context, Helper, HelperResult, Output, RenderContext, Template};
use heck::ToKebabCase;
use shared::swc_common::pass::Either;
use shared::swc_common::util::take::Take;

use std::collections::HashSet;

use crate::visit::IdentComponent;
use serde::{self, Serialize};
use shared::napi::{Env, JsFunction, JsString, NapiRaw};
use shared::swc_common::{BytePos, Span, SyntaxContext};
use shared::swc_ecma_ast::{
  self, Ident, ImportDecl, ImportDefaultSpecifier, ImportNamedSpecifier, ImportSpecifier, Module,
  ModuleDecl, ModuleExportName, ModuleItem, Str,
};
use shared::swc_ecma_visit::{as_folder, Fold, VisitMut, VisitWith};
use shared::{napi, napi_derive::napi};
use shared::{swc_atoms::JsWord, swc_common::DUMMY_SP};

pub fn plugin_import<'a>(config: &'a Vec<PluginImportConfig>, env: Option<Env>) -> impl Fold + 'a {
  let mut renderer = handlebars::Handlebars::new();

  renderer.register_helper(
    "kebabCase",
    Box::new(
      |helper: &Helper<'_, '_>,
       _: &'_ handlebars::Handlebars<'_>,
       _: &'_ Context,
       _: &mut RenderContext<'_, '_>,
       out: &mut dyn Output|
       -> HelperResult {
        let param = helper
          .param(0)
          .and_then(|v| v.value().as_str())
          .unwrap_or("");
        out.write(param.to_kebab_case().as_ref())?;
        Ok(())
      },
    ),
  );

  renderer.register_helper(
    "upperCase",
    Box::new(
      |helper: &Helper<'_, '_>,
       _: &'_ handlebars::Handlebars<'_>,
       _: &'_ Context,
       _: &mut RenderContext<'_, '_>,
       out: &mut dyn Output|
       -> HelperResult {
        let param = helper
          .param(0)
          .and_then(|v| v.value().as_str())
          .unwrap_or("");
        out.write(param.to_uppercase().as_ref())?;
        Ok(())
      },
    ),
  );

  renderer.register_helper(
    "lowerCase",
    Box::new(
      |helper: &Helper<'_, '_>,
       _: &'_ handlebars::Handlebars<'_>,
       _: &'_ Context,
       _: &mut RenderContext<'_, '_>,
       out: &mut dyn Output|
       -> HelperResult {
        let param = helper
          .param(0)
          .and_then(|v| v.value().as_str())
          .unwrap_or("");
        out.write(param.to_lowercase().as_ref())?;
        Ok(())
      },
    ),
  );

  config.iter().for_each(|cfg| {
    if let Some(js_config) = &cfg.replace_js {
      if let Some(tpl) = &js_config.replace_tpl {
        renderer.register_template(
          &(cfg.from_source.clone() + "js"),
          Template::compile(tpl).unwrap(),
        )
      }
    }

    if let Some(css_config) = &cfg.replace_css {
      if let Some(tpl) = &css_config.replace_tpl {
        renderer.register_template(
          &(cfg.from_source.clone() + "css"),
          Template::compile(tpl).unwrap(),
        )
      }
    }
  });

  as_folder(ImportPlugin {
    config,
    env,
    renderer,
  })
}

struct EsSpec {
  source: String,
  default_spec: String,
  as_name: Option<String>,
  use_default_import: bool,
  mark: u32,
}

pub struct ImportPlugin<'a> {
  pub config: &'a Vec<PluginImportConfig>,
  pub env: Option<Env>,
  pub renderer: handlebars::Handlebars<'a>,
}

impl<'a> VisitMut for ImportPlugin<'a> {
  fn visit_mut_module(&mut self, module: &mut Module) {
    // use visitor to collect all ident reference, and then remove imported component and type that is never referenced
    let mut visitor = IdentComponent {
      ident_set: HashSet::new(),
      type_ident_set: HashSet::new(),
      in_ts_type_ref: false,
    };
    module.body.visit_with(&mut visitor);

    let ident_referenced = |ident: &Ident| -> bool { visitor.ident_set.contains(&ident.to_id()) };
    let type_ident_referenced =
      |ident: &Ident| -> bool { visitor.type_ident_set.contains(&ident.to_id()) };

    let mut specifiers_css = vec![];
    let mut specifiers_es = vec![];
    let mut specifiers_rm_es = HashSet::new();

    let config = &self.config;

    for (item_index, item) in module.body.iter_mut().enumerate() {
      if let ModuleItem::ModuleDecl(ModuleDecl::Import(var)) = item {
        let source = &*var.src.value;
        if let Some(child_config) = config.iter().find(|&c| c.from_source == source) {
          let mut rm_specifier = HashSet::new();
          for (specifier_idx, specifier) in var.specifiers.iter().enumerate() {
            match specifier {
              ImportSpecifier::Named(ref s) => {
                let imported = s.imported.as_ref().map(|imported| match imported {
                  ModuleExportName::Ident(ident) => ident.sym.to_string(),
                  ModuleExportName::Str(str) => str.value.to_string(),
                });

                let as_name: Option<String> = imported.is_some().then(|| s.local.sym.to_string());
                let ident: String = imported.unwrap_or_else(|| s.local.sym.to_string());

                let mark = s.local.span.ctxt.as_u32();
                if ident_referenced(&s.local) {
                  if let Some(ref css) = child_config.replace_css {
                    let ignore_component = &css.ignore_style_component;
                    let need_lower = css.lower.unwrap_or(false);
                    let camel2dash = css.camel2_dash_component_name.unwrap_or(true);
                    let mut css_ident = ident.clone();
                    if camel2dash {
                      css_ident = css_ident.to_kebab_case();
                    }
                    if need_lower {
                      css_ident = css_ident.to_lowercase()
                    };

                    let mut need_replace = true;
                    if let Some(block_list) = ignore_component {
                      need_replace = !block_list.iter().any(|x| x == &ident);
                    }
                    if need_replace {
                      #[cfg(not(target_arch = "wasm32"))]
                      let import_css_source = css
                        .replace_expr
                        .as_ref()
                        .map(|replace_expr| {
                          call_js(
                            &replace_expr,
                            &[self
                              .env
                              .expect("using js function can only be run on sync api")
                              .create_string(css_ident.as_str())
                              .unwrap()],
                          )
                        })
                        .or_else(|| {
                          css.replace_tpl.as_ref().map(|_| {
                            Either::Left(
                              self
                                .renderer
                                .render(&(child_config.from_source.clone() + "css"), &css_ident)
                                .unwrap(),
                            )
                          })
                        });

                      #[cfg(target_arch = "wasm32")]
                      let import_css_source =
                        Either::Left(css.replace_expr.clone().replace("{}", css_ident.as_str()));

                      if let Some(Either::Left(source)) = import_css_source {
                        specifiers_css.push(source);
                      }
                    }
                  }

                  if let Some(ref js_config) = child_config.replace_js {
                    let ignore_component = &js_config.ignore_es_component;
                    let need_lower = js_config.lower.unwrap_or(false);
                    let camel2dash = js_config.camel2_dash_component_name.unwrap_or(true);
                    let use_default_import = js_config.transform_to_default_import.unwrap_or(true);

                    let mut js_ident = ident.clone();
                    if camel2dash {
                      js_ident = js_ident.to_kebab_case();
                    }
                    if need_lower {
                      js_ident = js_ident.to_lowercase();
                    }

                    let mut need_replace = true;
                    if let Some(block_list) = ignore_component {
                      need_replace = !block_list.iter().any(|x| x == &ident);
                    }
                    if need_replace {
                      #[cfg(not(target_arch = "wasm32"))]
                      let import_es_source = js_config
                        .replace_expr
                        .as_ref()
                        .map(|replace_expr| {
                          call_js(
                            replace_expr,
                            &[self
                              .env
                              .expect("using js function can only be run on sync api")
                              .create_string(&js_ident)
                              .unwrap()],
                          )
                        })
                        .or_else(|| {
                          js_config.replace_tpl.as_ref().map(|_| {
                            Either::Left(
                              self
                                .renderer
                                .render(&(child_config.from_source.clone() + "js"), &js_ident)
                                .unwrap(),
                            )
                          })
                        });
                      #[cfg(target_arch = "wasm32")]
                      let import_es_source = js_config
                        .replace_expr
                        .clone()
                        .replace("{}", js_ident.as_str());
                      if let Some(Either::Left(source)) = import_es_source {
                        specifiers_es.push(EsSpec {
                          source,
                          default_spec: ident,
                          as_name,
                          use_default_import,
                          mark,
                        });
                        rm_specifier.insert(specifier_idx);
                      }
                    }
                  }
                } else if type_ident_referenced(&s.local) {
                  // type referenced
                  continue;
                } else {
                  // not referenced, should tree shaking
                  rm_specifier.insert(specifier_idx);
                }
              }
              ImportSpecifier::Default(ref _s) => {}
              ImportSpecifier::Namespace(ref _ns) => {}
            }
          }
          if rm_specifier.len() == var.specifiers.len() {
            // all specifier remove, just remove whole stmt
            specifiers_rm_es.insert(item_index);
          } else {
            // only remove some specifier
            var.specifiers = var
              .specifiers
              .take()
              .into_iter()
              .enumerate()
              .filter_map(|(idx, spec)| (!rm_specifier.contains(&idx)).then(|| spec))
              .collect();
          }
        }
      }
    }

    module.body = module
      .body
      .take()
      .into_iter()
      .enumerate()
      .filter_map(|(idx, stmt)| (!specifiers_rm_es.contains(&idx)).then(|| stmt))
      .collect();

    let body = &mut module.body;

    for js_source in specifiers_es {
      let js_source_ref = js_source.source.as_str();
      let dec = ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
        span: DUMMY_SP,
        specifiers: if js_source.use_default_import {
          vec![swc_ecma_ast::ImportSpecifier::Default(
            ImportDefaultSpecifier {
              span: DUMMY_SP,
              local: swc_ecma_ast::Ident {
                span: Span::new(
                  BytePos::DUMMY,
                  BytePos::DUMMY,
                  SyntaxContext::from_u32(js_source.mark),
                ),
                sym: JsWord::from(js_source.as_name.unwrap_or(js_source.default_spec).as_str()),
                optional: false,
              },
            },
          )]
        } else {
          vec![swc_ecma_ast::ImportSpecifier::Named(ImportNamedSpecifier {
            span: DUMMY_SP,
            imported: if js_source.as_name.is_some() {
              Some(swc_ecma_ast::ModuleExportName::Ident(swc_ecma_ast::Ident {
                span: DUMMY_SP,
                sym: JsWord::from(js_source.default_spec.as_str()),
                optional: false,
              }))
            } else {
              None
            },
            local: swc_ecma_ast::Ident {
              span: Span::new(
                BytePos::DUMMY,
                BytePos::DUMMY,
                SyntaxContext::from_u32(js_source.mark),
              ),
              sym: JsWord::from(js_source.as_name.unwrap_or(js_source.default_spec).as_str()),
              optional: false,
            },
            is_type_only: false,
          })]
        },
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
      let dec = ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
        span: DUMMY_SP,
        specifiers: vec![],
        src: Str {
          span: DUMMY_SP,
          value: JsWord::from(css_source),
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
  pub replace_js: Option<ReplaceJsConfig>,
}

/*
safety: js_function is Option, if is Some then will be running only in single thread env
 */
unsafe impl Send for PluginImportConfig {}
unsafe impl Sync for PluginImportConfig {}

#[napi(object)]
#[derive(Serialize)]
pub struct ReplaceJsConfig {
  #[serde(skip_serializing)]
  pub replace_expr: Option<JsFunction>,
  pub replace_tpl: Option<String>,
  pub ignore_es_component: Option<Vec<String>>,
  pub lower: Option<bool>,
  pub camel2_dash_component_name: Option<bool>,
  pub transform_to_default_import: Option<bool>,
}

#[napi(object)]
#[derive(Serialize)]
pub struct ReplaceCssConfig {
  pub ignore_style_component: Option<Vec<String>>,
  #[serde(skip_serializing)]
  pub replace_expr: Option<JsFunction>,
  pub replace_tpl: Option<String>,
  pub lower: Option<bool>,
  pub camel2_dash_component_name: Option<bool>,
}

fn call_js(js_fn: &JsFunction, args: &[JsString]) -> Either<String, ()> {
  let js_return = js_fn.call_without_args(None).unwrap();

  match js_return.get_type() {
    Ok(ty) => {
      match ty {
        napi::ValueType::Undefined | napi::ValueType::Null => Either::Right(()),
        napi::ValueType::Boolean => {
          if js_return.coerce_to_bool().unwrap().get_value().unwrap() {
            // return true : invalid
            panic!("replaceExpr return value must be utf-8 string, false, undefined, null, Received true")
          }
          Either::Right(())
        }
        napi::ValueType::String => {
          let res = js_return.coerce_to_string().unwrap();

          let res = res.into_utf8().map_or_else(
            |_| res.into_utf16().unwrap().as_str(),
            |u8_str| u8_str.as_str().map(|s| s.to_string()),
          );

          Either::Left(res.unwrap())
        }
        t => {
          panic!(
            "replaceExpr return value must be utf-8 string, false, undefined, null. Received {}",
            t
          )
        }
      }
    }
    Err(_) => unreachable!(),
  }
}
