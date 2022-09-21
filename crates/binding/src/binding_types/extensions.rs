use core::types::Extensions;
use std::collections::HashMap;

use super::plugin_modularize_imports::PackageConfigNapi;
use super::{plugin_react_utils};
use super::{plugin_import, FromNapi};

use napi_derive::napi;
/**
 * Internal plugin
 */
#[derive(Default)]
#[napi(object)]
pub struct ExtensionsNapi {
  pub modularize_imports: Option<HashMap<String, PackageConfigNapi>>,
  pub plugin_import: Option<Vec<plugin_import::PluginImportConfigNapi>>,
  pub react_utils: Option<plugin_react_utils::ReactUtilsConfigNapi>,
}

impl FromNapi<Extensions> for ExtensionsNapi {
  fn from_napi(self, env: napi::Env) -> napi::Result<Extensions> {
    let Self {
      modularize_imports,
      plugin_import,
      react_utils,
    } = self;

    Ok(Extensions {
      modularize_imports: modularize_imports.from_napi(env)?,
      plugin_import: plugin_import.from_napi(env)?,
      react_utils: react_utils.from_napi(env)?,
    })
  }
}
