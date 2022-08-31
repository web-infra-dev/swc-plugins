use std::sync::Arc;

use shared::{
  swc_common::{Mark, SourceMap, comments::NoopComments},
  swc_ecma_transforms_react::{react, Options, RefreshOptions, Runtime},
  swc_ecma_visit::VisitMut,
};

pub struct PresetReact {
  pub runtime: Option<Runtime>,

  pub import_source: Option<String>,

  pub pragma: Option<String>,

  pub pragma_frag: Option<String>,

  pub throw_if_namespace: Option<bool>,

  pub development: Option<bool>,

  // default to disabled since this is still considered as experimental by now
  pub hmr: bool,
}

pub fn preset_react(cm: Arc<SourceMap>, options: PresetReact, top_level_mark: Mark) -> impl VisitMut {
  react::<NoopComments>(
    cm,
    None,
    Options {
      runtime: options.runtime,
      import_source: options.import_source,
      pragma: options.pragma,
      pragma_frag: options.pragma_frag,
      throw_if_namespace: options.throw_if_namespace,
      development: options.development,
      refresh: if options.hmr {
        Some(RefreshOptions {
          ..Default::default()
        })
      } else {
        None
      },
      ..Default::default()
    },
    top_level_mark,
  )
}
