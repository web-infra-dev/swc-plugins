use std::{fmt::Debug, path::PathBuf, sync::Arc};

// reexports some same version libs
pub use anyhow;
pub use serde_json;
pub use swc_core;
use swc_core::common::{comments::SingleThreadedComments, Mark, SourceMap};
pub use swc_core::ecma::transforms::testing as swc_ecma_transforms_testing;
pub use testing;
pub extern crate serde;
pub use ahash;
pub use dashmap;

pub struct PluginContext {
  pub cm: Arc<SourceMap>,
  pub top_level_mark: Mark,
  pub unresolved_mark: Mark,
  pub comments: SingleThreadedComments,
  pub filename: String,
  pub cwd: PathBuf,

  pub config_hash: Option<String>, // This can be used by plugins to do caching

  // Use this to determine if we should remove __esModule mark in pure commonjs module
  // Remove this when SWC fix https://github.com/swc-project/swc/issues/6500
  pub is_source_esm: bool,
}

impl Debug for PluginContext {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("PluginContext")
      .field("cm", &"Arc<SourceMap>")
      .field("top_level_mark", &self.top_level_mark)
      .field("unresolved_mark", &self.unresolved_mark)
      .field("comments", &self.comments)
      .field("filename", &self.filename)
      .field("cwd", &self.cwd)
      .field("config_hash", &self.config_hash)
      .field("is_source_esm", &self.is_source_esm)
      .finish()
  }
}
