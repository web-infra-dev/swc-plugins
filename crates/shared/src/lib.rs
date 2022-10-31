pub mod utils;
use std::sync::Arc;

// reexports some same version libs
pub use anyhow;
pub use serde_json;
pub use swc_core;
use swc_core::common::{comments::SingleThreadedComments, Mark, SourceMap};
pub use swc_core::ecma::transforms::testing as swc_ecma_transforms_testing;
pub use testing;
pub extern crate serde;
pub use ahash;

pub struct PluginContext {
  pub cm: Arc<SourceMap>,
  pub top_level_mark: Mark,
  pub unresolved_mark: Mark,
  pub comments: SingleThreadedComments,
}
