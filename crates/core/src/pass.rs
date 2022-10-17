use std::sync::Arc;

use crate::types::TransformConfig;
use plugin_dynamic_import_node::dyn_import_node;
use shared::swc_core::{
  common::{chain, pass::Either, SourceMap, comments::SingleThreadedComments},
  ecma::transforms::base::pass::noop,
  ecma::visit::Fold,
};

use plugin_import::plugin_import;
use plugin_modularize_imports::{modularize_imports, Config as ModularizedConfig};
use plugin_react_utils::react_utils;

pub fn internal_transform_pass(
  config: &TransformConfig,
  _cm: Arc<SourceMap>,
) -> impl Fold + '_
{
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
    Either::Left(react_utils(c))
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

  let dyn_import_node = if let Some(dyn_import_node_config) = &extensions.dyn_import_node {
    // Now this just for fn signature, this comment has no op
    let comments = SingleThreadedComments::default();
    Either::Left(dyn_import_node(dyn_import_node_config.clone(), Some(comments)))
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
    dyn_import_node
  )
}
