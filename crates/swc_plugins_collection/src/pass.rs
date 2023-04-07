use std::{path::Path, sync::Arc};

use modularize_imports::{modularize_imports, Config as ModularizedConfig};
use plugin_lock_corejs_version::lock_corejs_version;
use plugin_remove_es_module_mark::remove_es_module_mark;
use swc_core::{
  base::config::{ModuleConfig, Options},
  common::{chain, pass::Either, FileName},
  ecma::visit::Fold,
  ecma::{transforms::base::pass::noop, visit::as_folder},
  plugin::proxies::PluginCommentsProxy,
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

  let modernjs_ssr_loader_id = if *extensions.modernjs_ssr_loader_id.as_ref().unwrap_or(&false) {
    Either::Left(
      plugin_modernjs_ssr_loader_id::plugin_modernjs_ssr_loader_id(plugin_context.clone()),
    )
  } else {
    Either::Right(noop())
  };

  let loadable_components = if extensions.loadable_components.unwrap_or(false) {
    Either::Left(plugin_loadable_components())
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
    modernjs_ssr_loader_id,
    emotion,
    styled_jsx,
    styled_components,
    loadable_components
  )
}

pub fn internal_transform_after_pass<'a>(
  extensions: &Extensions,
  swc_config: &Options,
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

  let remove_es_module_mark = if let Some(ModuleConfig::CommonJs(_)) = swc_config.config.module && !plugin_context.is_source_esm {
    Either::Left(remove_es_module_mark())
  } else {
    Either::Right(noop())
  };

  chain!(lock_core_js, remove_es_module_mark)
}

fn plugin_loadable_components() -> impl Fold {
  as_folder(loadable_transform(PluginCommentsProxy))
}
