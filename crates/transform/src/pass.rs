use crate::types::TransformConfig;
use either::Either;
use napi::Env;
use shared::{swc_common::chain, swc_ecma_transforms_base::pass::noop, swc_ecma_visit::Fold};

use modularize_imports::{modularize_imports, Config as ModularizedConfig};
use plugin_import::plugin_import;

pub fn internal_transform_pass(env: Option<Env>, config: &mut TransformConfig) -> impl Fold + '_ {
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
    .take()
    .map(|config| Either::Left(plugin_import(config, env.unwrap())))
    .unwrap_or(Either::Right(noop()));

  chain!(modularize_imports, plugin_import)
}
