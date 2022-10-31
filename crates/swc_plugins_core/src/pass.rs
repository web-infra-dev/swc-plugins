use std::sync::Arc;

use crate::types::TransformConfig;
use plugin_lodash::plugin_lodash;
use shared::{
  swc_core::{
    common::{chain, comments::SingleThreadedComments, pass::Either, Mark, SourceMap},
    ecma::transforms::base::pass::noop,
    ecma::visit::Fold,
  },
  PluginContext,
};

use plugin_import::plugin_import;
use plugin_modularize_imports::{modularize_imports, Config as ModularizedConfig};
use plugin_react_utils::react_utils;

pub fn internal_transform_pass(
  config: &TransformConfig,
  cm: Arc<SourceMap>,
  top_level_mark: Mark,
  unresolved_mark: Mark,
  comments: SingleThreadedComments,
) -> impl Fold + '_ {
  let plugin_context = Arc::new(PluginContext {
    cm,
    top_level_mark,
    unresolved_mark,
    comments,
  });

  let extensions = &config.extensions;

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

  let lock_core_js = if let Some(lock_core_js_config) = &extensions.lock_corejs_version {
    Either::Left(plugin_lock_corejs_version::lock_corejs_version(
      lock_core_js_config.corejs_path.to_string(),
    ))
  } else {
    Either::Right(noop())
  };

  let lodash = if let Some(ref config) = extensions.lodash {
    Either::Left(plugin_lodash(config, plugin_context))
  } else {
    Either::Right(noop())
  };

  // let emotion = if let Some(emotion_options) = &extensions.emotion {
  //   Either::Left(swc_emotion::emotion(
  //     emotion_options.clone(),
  //     Path::new(config.swc.filename.as_str()),
  //     cm.clone(),
  //     SingleThreadedComments::default(),
  //   ))
  // } else {
  //   Either::Right(noop())
  // };

  // let styled_jsx = if *extensions.styled_jsx.as_ref().unwrap_or(&false) {
  //   Either::Left(styled_jsx(cm, FileName::Real(config.swc.filename.clone().into())))
  // } else {
  //   Either::Right(noop())
  // };

  chain!(
    modularize_imports,
    plugin_import,
    react_utils,
    lock_core_js,
    lodash
  )
}
