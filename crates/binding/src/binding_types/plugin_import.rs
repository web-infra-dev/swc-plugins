use swc_plugins_collection::swc_plugin_import::{CustomTransform, PluginImportConfig, StyleConfig};

use napi::{Env, JsFunction, JsString, Ref, Status};
use napi_derive::napi;

use crate::{thread_safe_function::ThreadSafeFunction, IS_SYNC};

use super::IntoRawConfig;

#[napi(object)]
pub struct StyleConfigNapi {
  pub style_library_directory: Option<String>,
  pub custom_fn: Option<JsFunction>,
  pub custom_tpl: Option<String>,
  pub css: Option<String>,
  pub bool: Option<bool>,
}

impl IntoRawConfig<StyleConfig> for StyleConfigNapi {
  fn into_raw_config(self, env: Env) -> napi::Result<StyleConfig> {
    Ok(if let Some(tpl) = self.custom_tpl {
      StyleConfig::Custom(CustomTransform::Tpl(tpl))
    } else if let Some(f) = self.custom_fn {
      StyleConfig::Custom(CustomTransform::Fn(create_js_fn(env, f)))
    } else if let Some(style_library_directory) = self.style_library_directory {
      StyleConfig::StyleLibraryDirectory(style_library_directory)
    } else if self.css.is_some() {
      StyleConfig::Css
    } else if let Some(bool) = self.bool {
      StyleConfig::Bool(bool)
    } else {
      StyleConfig::None
    })
  }
}

#[napi(object)]
pub struct PluginImportConfigNapi {
  pub library_name: String,
  pub library_directory: Option<String>, // default to `lib`

  pub custom_name_fn: Option<JsFunction>,
  pub custom_name_tpl: Option<String>,

  pub custom_style_name_fn: Option<JsFunction>, // If this is set, `style` option will be ignored
  pub custom_style_name_tpl: Option<String>,    // If this is set, `style` option will be ignored

  pub style: Option<StyleConfigNapi>,

  pub camel_to_dash_component_name: Option<bool>, // default to true
  pub transform_to_default_import: Option<bool>,

  pub ignore_es_component: Option<Vec<String>>,
  pub ignore_style_component: Option<Vec<String>>,
}

/// Wrap for env, to make it impl Send and Sync, we ensure this env won't send between threads, so it's safe
#[derive(Clone, Copy)]
struct SyncEnv(Env);

impl SyncEnv {
  fn get_reference_value<T: napi::NapiValue>(&self, js_ref: &Ref<()>) -> napi::Result<T> {
    IS_SYNC.with(|sync| {
      assert!(
        *sync.borrow(),
        "SyncEnv can only be used in sync Javascript call"
      );
    });
    self.0.get_reference_value(js_ref)
  }

  fn create_string(&self, s: &str) -> napi::Result<JsString> {
    IS_SYNC.with(|sync| {
      assert!(
        *sync.borrow(),
        "SyncEnv can only be used in sync Javascript call. You may use transform and transform_sync the same time!"
      );
    });
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

impl IntoRawConfig<PluginImportConfig> for PluginImportConfigNapi {
  fn into_raw_config(self, env: Env) -> napi::Result<PluginImportConfig> {
    let PluginImportConfigNapi {
      library_name,
      library_directory,
      custom_name_fn,
      custom_name_tpl,

      custom_style_name_fn,
      custom_style_name_tpl,
      style,
      camel_to_dash_component_name,
      transform_to_default_import,
      ignore_es_component,
      ignore_style_component,
    } = self;

    Ok(PluginImportConfig {
      library_name,
      library_directory,
      custom_name: if let Some(tpl) = custom_name_tpl {
        Some(CustomTransform::Tpl(tpl))
      } else {
        custom_name_fn.map(|f| CustomTransform::Fn(create_js_fn(env, f)))
      },
      custom_style_name: if let Some(tpl) = custom_style_name_tpl {
        Some(CustomTransform::Tpl(tpl))
      } else {
        custom_style_name_fn.map(|f| CustomTransform::Fn(create_js_fn(env, f)))
      },
      style: style.into_raw_config(env)?,
      camel_to_dash_component_name,
      transform_to_default_import,
      ignore_es_component,
      ignore_style_component,
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

fn create_js_fn(
  env: Env,
  js_fn: JsFunction,
) -> Box<dyn Sync + Send + Fn(String) -> Option<String>> {
  let js_ref = env.create_reference(js_fn).unwrap();

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

  let wrap_env = SyncEnv(env);
  Box::new(move |s: String| {
    IS_SYNC.with(|is_sync| {
      if *is_sync.borrow() {
        let js_function: JsFunction = wrap_env.get_reference_value(&js_ref).unwrap();
        // sync call
        call_js(&js_function, &[wrap_env.create_string(&s).unwrap()]).unwrap()
      } else {
        tsfn
          .call(s)
          .expect("Error occur while calling pluginImport replace_css.replace_expr() function")
      }
    })
  }) as Box<dyn Sync + Send + Fn(String) -> Option<String>>
}
