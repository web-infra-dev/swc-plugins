use std::{fmt::Debug, path::PathBuf, sync::Arc};

// reexports some same version libs

#[cfg(feature = "anyhow")]
pub use anyhow;

#[cfg(feature = "serde_json")]
pub use serde_json;

#[cfg(feature = "serde")]
pub extern crate serde;

#[cfg(feature = "ahash")]
pub use ahash;

#[cfg(feature = "dashmap")]
pub use dashmap;

#[cfg(feature = "plugin_context")]
pub struct PluginContext {
  pub cm: Arc<swc_core::common::SourceMap>,
  pub file: Arc<swc_core::common::SourceFile>,
  pub top_level_mark: swc_core::common::Mark,
  pub unresolved_mark: swc_core::common::Mark,
  pub comments: swc_core::common::comments::SingleThreadedComments,
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
