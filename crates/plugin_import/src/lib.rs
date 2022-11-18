mod visit;

use handlebars::{Context, Helper, HelperResult, Output, RenderContext, Template};
use heck::ToKebabCase;
use serde::Deserialize;
use shared::swc_core::{
  common::{util::take::Take, BytePos, Span, SyntaxContext, DUMMY_SP},
  ecma::{
    ast::{
      Ident, ImportDecl, ImportDefaultSpecifier, ImportNamedSpecifier, ImportSpecifier, Module,
      ModuleDecl, ModuleExportName, ModuleItem, Str,
    },
    atoms::JsWord,
    visit::{as_folder, Fold, VisitMut, VisitWith},
  },
};

use std::collections::HashSet;
use std::fmt::Debug;

use crate::visit::IdentComponent;

/* ======= Real struct ======= */
#[derive(Debug, Default, Deserialize)]
#[serde(crate = "shared::serde")]
pub struct PluginImportConfig {
  pub from_source: String,
  pub replace_css: Option<ReplaceCssConfig>,
  pub replace_js: Option<ReplaceJsConfig>,
}

#[derive(Default, Deserialize)]
#[serde(crate = "shared::serde")]
pub struct ReplaceJsConfig {
  #[serde(skip)]
  pub replace_expr: Option<Box<dyn Send + Sync + Fn(String) -> Option<String>>>,
  pub template: Option<String>,
  pub ignore_es_component: Option<Vec<String>>,
  pub lower: Option<bool>,
  pub camel2_dash_component_name: Option<bool>,
  pub transform_to_default_import: Option<bool>,
}

impl Debug for ReplaceJsConfig {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&format!(
      "ReplaceJsConfig: {{\nreplace_expr: {:?},\ntemplate: {:?},\nignore_es_component: {:?},\nlower: {:?},\ncamel2_dash_component_name: {:?},\ntransform_to_default_import: {:?},\n}}\n",
      self.replace_expr.as_ref().map(|_| Some("Func")).unwrap_or(None),
      self.template,
      self.ignore_es_component,
      self.lower,
      self.camel2_dash_component_name,
      self.transform_to_default_import
    ))
  }
}

#[derive(Default, Deserialize)]
#[serde(crate = "shared::serde")]
pub struct ReplaceCssConfig {
  #[serde(skip)]
  pub replace_expr: Option<Box<dyn Send + Sync + Fn(String) -> Option<String>>>,
  pub template: Option<String>,
  pub ignore_style_component: Option<Vec<String>>,
  pub lower: Option<bool>,
  pub camel2_dash_component_name: Option<bool>,
}

impl Debug for ReplaceCssConfig {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&format!(
      "ReplaceCssConfig: {{\nreplace_expr: {:?},\ntemplate: {:?},\nignore_style_component: {:?},\nlower: {:?},\ncamel2_dash_component_name: {:?},\n}}\n",
      self.replace_expr.as_ref().map(|_| Some("Func")).unwrap_or(None),
      self.template,
      self.ignore_style_component,
      self.lower,
      self.camel2_dash_component_name,
    ))
  }
}

pub fn plugin_import<'a>(config: &'a Vec<PluginImportConfig>) -> impl Fold + 'a {
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
      if let Some(tpl) = &js_config.template {
        renderer.register_template(
          &(cfg.from_source.clone() + "js"),
          Template::compile(tpl).unwrap(),
        )
      }
    }

    if let Some(css_config) = &cfg.replace_css {
      if let Some(tpl) = &css_config.template {
        renderer.register_template(
          &(cfg.from_source.clone() + "css"),
          Template::compile(tpl).unwrap(),
        )
      }
    }
  });

  as_folder(ImportPlugin { config, renderer })
}

#[derive(Debug)]
struct EsSpec {
  source: String,
  default_spec: String,
  as_name: Option<String>,
  use_default_import: bool,
  mark: u32,
}

pub struct ImportPlugin<'a> {
  pub config: &'a Vec<PluginImportConfig>,
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
                        .and_then(|replace_expr| replace_expr(css_ident.clone()))
                        .or_else(|| {
                          css.template.as_ref().map(|_| {
                            self
                              .renderer
                              .render(&(child_config.from_source.clone() + "css"), &css_ident)
                              .unwrap()
                          })
                        });

                      if let Some(source) = import_css_source {
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
                        .and_then(|replace_expr| replace_expr(js_ident.clone()))
                        .or_else(|| {
                          js_config.template.as_ref().map(|_| {
                            self
                              .renderer
                              .render(&(child_config.from_source.clone() + "js"), &js_ident)
                              .unwrap()
                          })
                        });

                      if let Some(source) = import_es_source {
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
              .filter_map(|(idx, spec)| (!rm_specifier.contains(&idx)).then_some(spec))
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
      .filter_map(|(idx, stmt)| (!specifiers_rm_es.contains(&idx)).then_some(stmt))
      .collect();

    let body = &mut module.body;

    for js_source in specifiers_es {
      let js_source_ref = js_source.source.as_str();
      let dec = ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
        span: DUMMY_SP,
        specifiers: if js_source.use_default_import {
          vec![ImportSpecifier::Default(ImportDefaultSpecifier {
            span: DUMMY_SP,
            local: Ident {
              span: Span::new(
                BytePos::DUMMY,
                BytePos::DUMMY,
                SyntaxContext::from_u32(js_source.mark),
              ),
              sym: JsWord::from(js_source.as_name.unwrap_or(js_source.default_spec).as_str()),
              optional: false,
            },
          })]
        } else {
          vec![ImportSpecifier::Named(ImportNamedSpecifier {
            span: DUMMY_SP,
            imported: if js_source.as_name.is_some() {
              Some(ModuleExportName::Ident(Ident {
                span: DUMMY_SP,
                sym: JsWord::from(js_source.default_spec.as_str()),
                optional: false,
              }))
            } else {
              None
            },
            local: Ident {
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
        src: Box::new(Str {
          span: DUMMY_SP,
          value: JsWord::from(js_source_ref),
          raw: None,
        }),
        type_only: false,
        asserts: None,
      }));
      body.insert(0, dec);
    }

    for css_source in specifiers_css {
      let dec = ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
        span: DUMMY_SP,
        specifiers: vec![],
        src: Box::new(Str {
          span: DUMMY_SP,
          value: JsWord::from(css_source),
          raw: None,
        }),
        type_only: false,
        asserts: None,
      }));
      body.insert(0, dec);
    }
  }
}
