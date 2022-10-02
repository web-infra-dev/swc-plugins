use core::plugin_import::{PluginImportConfig, ReplaceCssConfig, ReplaceJsConfig};

use napi::{Env, JsFunction, JsString, Ref, Status};
use napi_derive::napi;
use shared::serde::Serialize;

use crate::{thread_safe_function::ThreadSafeFunction, IS_SYNC};

use super::IntoRawConfig;

#[napi(object)]
pub struct PluginImportConfigNapi {
  pub from_source: String,
  pub replace_css: Option<ReplaceCssConfigNapi>,
  pub replace_js: Option<ReplaceJsConfigNapi>,
}

#[derive(Clone, Copy)]
struct SyncEnv(Env);

impl SyncEnv {
  fn get_reference_value<T: napi::NapiValue>(&self, js_ref: &Ref<()>) -> napi::Result<T> {
    self.0.get_reference_value(js_ref)
  }

  fn create_string(&self, s: &str) -> napi::Result<JsString> {
    self.0.create_string(s)
  }
}

// Safety: Only use this in sync call
unsafe impl Send for SyncEnv {}
unsafe impl Sync for SyncEnv {}

impl From<SyncEnv> for Env {
  fn from(e: SyncEnv) -> Self {
    e.0
  }
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

impl IntoRawConfig<PluginImportConfig> for PluginImportConfigNapi {
  fn into_raw_config(self, env: Env) -> napi::Result<PluginImportConfig> {
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
            let wrap_env = SyncEnv(env);
            let js_ref = env.create_reference(js_function).unwrap();

            let tsfn = ThreadSafeFunction::<String, Option<String>>::new(
              env,
              env.get_reference_value(&js_ref).unwrap(),
              move |ctx| {
                let env = ctx.env;
                let member = ctx.value;
                let js_function = ctx.callback;

                let js_string = env.create_string(&member)?;
                call_js(&js_function, &[js_string])
              },
            );

            Box::new(move |s: String| {
              IS_SYNC.with(|is_sync| {
                if *is_sync.borrow() {
                  let js_function: JsFunction = wrap_env.get_reference_value(&js_ref).unwrap();
                  // sync call
                  call_js(&js_function, &[wrap_env.create_string(&s).unwrap()]).unwrap()
                } else {
                  tsfn.call(s).expect(
                    "Error occur while calling pluginImport replace_css.replace_expr() function",
                  )
                }
              })
            }) as Box<dyn Sync + Send + Fn(String) -> Option<String>>
          }),
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
            let wrap_env = SyncEnv(env);
            let js_ref = env.create_reference(js_function).unwrap();

            let tsfn = ThreadSafeFunction::<String, Option<String>>::new(
              env,
              env.get_reference_value(&js_ref).unwrap(),
              move |ctx| {
                let env = ctx.env;
                let member = ctx.value;
                let js_function = ctx.callback;

                let js_string = env.create_string(&member)?;
                call_js(&js_function, &[js_string])
              },
            );

            Box::new(move |s: String| {
              IS_SYNC.with(|is_sync| {
                if *is_sync.borrow() {
                  let js_function: JsFunction = wrap_env.get_reference_value(&js_ref).unwrap();
                  // sync call
                  call_js(&js_function, &[wrap_env.create_string(&s).unwrap()]).unwrap()
                } else {
                  tsfn.call(s).expect(
                    "Error occur while calling pluginImport replace_css.replace_expr() function",
                  )
                }
              })
            }) as Box<dyn Sync + Send + Fn(String) -> Option<String>>
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

fn call_js(js_fn: &JsFunction, args: &[JsString]) -> napi::Result<Option<String>> {
  let js_return = js_fn.call(None, args)?;

  match js_return.get_type() {
    Ok(ty) => {
      match ty {
        napi::ValueType::Undefined | napi::ValueType::Null => Ok(None),
        napi::ValueType::Boolean => {
          if js_return.coerce_to_bool()?.get_value()? {
            // return true : invalid
            return Err(napi::Error::new(
              Status::GenericFailure,
              "functions of pluginImport replaceExpr can only strictly return false or string"
                .into(),
            ));
          }
          Ok(None)
        }
        napi::ValueType::String => {
          let res = js_return.coerce_to_string()?;

          let res = res.into_utf8().map_or_else(
            |_| res.into_utf16()?.as_str(),
            |u8_str| u8_str.as_str().map(|s| s.to_string()),
          );

          Ok(Some(res.unwrap()))
        }
        ty => {
          Err(napi::Error::new(
            Status::GenericFailure,
            format!(
              "functions of pluginImport replaceExpr can only strictly return false or string. Received: {}", ty),
          ))
        }
      }
    }
    Err(_) => unreachable!(),
  }
}
