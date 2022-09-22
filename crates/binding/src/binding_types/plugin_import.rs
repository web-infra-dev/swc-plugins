use core::plugin_import::{PluginImportConfig, ReplaceCssConfig, ReplaceJsConfig};

use napi::{Env, JsFunction, JsString, Ref};
use napi_derive::napi;
use shared::serde::Serialize;

use crate::tsfn::ThreadSafeFunction;

use super::FromNapi;

#[napi(object)]
pub struct PluginImportConfigNapi {
  pub from_source: String,
  pub replace_css: Option<ReplaceCssConfigNapi>,
  pub replace_js: Option<ReplaceJsConfigNapi>,
}

#[napi(object)]
#[derive(Serialize)]
#[serde(crate = "shared::serde")]
pub struct ReplaceJsConfigNapi {
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
#[serde(crate = "shared::serde")]
pub struct ReplaceCssConfigNapi {
  pub ignore_style_component: Option<Vec<String>>,
  #[serde(skip_serializing)]
  pub replace_expr: Option<JsFunction>,
  pub replace_tpl: Option<String>,
  pub lower: Option<bool>,
  pub camel2_dash_component_name: Option<bool>,
}

impl FromNapi<PluginImportConfig> for PluginImportConfigNapi {
  fn from_napi(self, env: Env) -> napi::Result<PluginImportConfig> {
    let PluginImportConfigNapi {
      from_source,
      replace_css,
      replace_js,
    } = self;

    Ok(PluginImportConfig {
      from_source,
      replace_css: replace_css.map(|replace_css| {
        let ReplaceCssConfigNapi {
          ignore_style_component,
          replace_expr,
          replace_tpl,
          lower,
          camel2_dash_component_name,
        } = replace_css;

        ReplaceCssConfig {
          ignore_style_component,
          replace_expr: replace_expr.map(|js_function| {
            let js_function = env.create_reference(js_function).unwrap();

            let tsfn = ThreadSafeFunction::<String, Option<String>>::new(env, move |ctx| {
              let env = ctx.env;
              let member = ctx.value;
              let js_string = env.create_string(&member).unwrap();
              let s = call_js(env, &js_function, &[js_string]);
              s
            });

            Box::new(move |s| tsfn.call(s)) as Box<dyn Sync + Send + Fn(String) -> Option<String>>
          }),
          // replace_expr: replace_expr.map(|_| Box::new(|s: String| -> Option<String> { None })),
          replace_tpl,
          lower,
          camel2_dash_component_name,
        }
      }),
      replace_js: replace_js.map(|replace_js| {
        let ReplaceJsConfigNapi {
          ignore_es_component,
          replace_expr,
          replace_tpl,
          lower,
          camel2_dash_component_name,
          transform_to_default_import,
        } = replace_js;

        ReplaceJsConfig {
          ignore_es_component,
          replace_expr: replace_expr.map(|js_function| {
            let js_function = env.create_reference(js_function).unwrap();

            let tsfn = ThreadSafeFunction::<String, Option<String>>::new(env, move |ctx| {
              let env = ctx.env;
              let member = ctx.value;
              let js_string = env.create_string(&member).unwrap();
              let s = call_js(env, &js_function, &[js_string]);
              s
            });

            Box::new(move |s| tsfn.call(s)) as Box<dyn Sync + Send + Fn(String) -> Option<String>>
          }),
          replace_tpl,
          lower,
          camel2_dash_component_name,
          transform_to_default_import,
        }
      }),
    })
  }
}

fn call_js(env: Env, js_fn: &Ref<()>, args: &[JsString]) -> Option<String> {
  let f: JsFunction = env
    .get_reference_value(js_fn)
    .expect("failed to get reference, this may be a internal error");
  let js_return = f.call(None, args).unwrap();

  match js_return.get_type() {
    Ok(ty) => {
      match ty {
        napi::ValueType::Undefined | napi::ValueType::Null => None,
        napi::ValueType::Boolean => {
          if js_return.coerce_to_bool().unwrap().get_value().unwrap() {
            // return true : invalid
            panic!("replaceExpr return value must be utf-8 string, false, undefined, null, Received true")
          }
          None
        }
        napi::ValueType::String => {
          let res = js_return.coerce_to_string().unwrap();

          let res = res.into_utf8().map_or_else(
            |_| res.into_utf16().unwrap().as_str(),
            |u8_str| u8_str.as_str().map(|s| s.to_string()),
          );

          Some(res.unwrap())
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
