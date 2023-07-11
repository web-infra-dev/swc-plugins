use std::{path::Path, sync::Arc};

use modularize_imports::{modularize_imports, Config as ModularizedConfig};
use plugin_config_routes::plugin_config_routes;
use plugin_lock_corejs_version::lock_corejs_version;
use plugin_ssr_loader_id::plugin_ssr_loader_id;
use swc_core::{
  base::config::Options,
  common::{chain, comments::Comments, pass::Either, FileName},
  ecma::visit::Fold,
  ecma::{transforms::base::pass::noop, visit::as_folder},
};
use swc_plugin_import::plugin_import;
use swc_plugin_loadable_components::loadable_transform;
use swc_plugin_lodash::plugin_lodash;
use swc_plugin_react_utils::react_utils;
use swc_plugins_utils::PluginContext;

use crate::types::Extensions;

pub fn internal_transform_before_pass<'a>(
  extensions: &'a Extensions,
  swc_config: &Options,
  plugin_context: Arc<PluginContext>,
) -> impl Fold + 'a {
  let modularize_imports = extensions
    .modularize_imports
    .as_ref()
    .map(|config| {
      Either::Left(modularize_imports(ModularizedConfig {
        packages: config.clone(),
      }))
    })
    .unwrap_or_else(|| Either::Right(noop()));

  let plugin_import = extensions
    .plugin_import
    .as_ref()
    .map(|config| Either::Left(plugin_import(config)))
    .unwrap_or_else(|| Either::Right(noop()));

  let react_utils = if let Some(c) = &extensions.react_utils {
    Either::Left(react_utils(c, plugin_context.clone()))
  } else {
    Either::Right(noop())
  };

  let lodash = if let Some(ref config) = extensions.lodash {
    Either::Left(plugin_lodash(config, plugin_context.clone()))
  } else {
    Either::Right(noop())
  };

  let ssr_loader_id = if let Some(ref config) = extensions.ssr_loader_id {
    Either::Left(plugin_ssr_loader_id(config, plugin_context.clone()))
  } else {
    Either::Right(noop())
  };

  let config_routes = if let Some(ref config) = extensions.config_routes {
    Either::Left(plugin_config_routes(config))
  } else {
    Either::Right(noop())
  };

  let emotion = if let Some(emotion_options) = &extensions.emotion {
    Either::Left(swc_emotion::emotion(
      emotion_options.clone(),
      Path::new(swc_config.filename.as_str()),
      plugin_context.file.src_hash as u32,
      plugin_context.cm.clone(),
      plugin_context.comments.clone(),
    ))
  } else {
    Either::Right(noop())
  };

  let styled_jsx = if *extensions.styled_jsx.as_ref().unwrap_or(&false) {
    Either::Left(styled_jsx::visitor::styled_jsx(
      plugin_context.cm.clone(),
      FileName::Real(swc_config.filename.clone().into()),
    ))
  } else {
    Either::Right(noop())
  };

  let styled_components = if let Some(config) = &extensions.styled_components {
    Either::Left(styled_components::styled_components(
      plugin_context.file.name.clone(),
      plugin_context.file.src_hash,
      config.clone(),
    ))
  } else {
    Either::Right(noop())
  };

  chain!(
    modularize_imports,
    plugin_import,
    react_utils,
    lodash,
    ssr_loader_id,
    config_routes,
    emotion,
    styled_jsx,
    styled_components,
  )
}

pub fn internal_transform_after_pass<'a>(
  extensions: &Extensions,
  _swc_config: &Options,
  plugin_context: Arc<PluginContext>,
) -> impl Fold + 'a {
  let lock_core_js = if let Some(config) = &extensions.lock_corejs_version {
    Either::Left(lock_corejs_version(
      config.corejs.clone(),
      config.swc_helpers.clone(),
    ))
  } else {
    Either::Right(noop())
  };

  let loadable_components = if extensions.loadable_components.unwrap_or(false) {
    Either::Left(plugin_loadable_components(plugin_context.comments.clone()))
  } else {
    Either::Right(noop())
  };

  chain!(lock_core_js, loadable_components)
}

fn plugin_loadable_components<C: Comments>(comments: C) -> impl Fold {
  as_folder(loadable_transform(comments))
}
