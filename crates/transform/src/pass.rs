use crate::types::TransformConfig;
use either::Either;
use shared::{swc_common::chain, swc_ecma_transforms_base::pass::noop, swc_ecma_visit::Fold};

use modularize_imports::{modularize_imports, Config as ModularizedConfig};
use plugin_import::plugin_import;

pub fn internal_transform_pass(config: &TransformConfig) -> impl Fold + '_ {
  let modularize_imports = config
    .extensions
    .modularize_imports
    .as_ref()
    .map(|config| {
      Either::Left(modularize_imports(ModularizedConfig {
        packages: config.clone(),
      }))
    })
    .unwrap_or(Either::Right(noop()));

  let plugin_import = config
    .extensions
    .plugin_import
    .as_ref()
    .map(|config| Either::Left(plugin_import(config)))
    .unwrap_or(Either::Right(noop()));

  chain!(modularize_imports, plugin_import)
}
