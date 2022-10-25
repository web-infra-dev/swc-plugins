use std::sync::Arc;

use shared::swc_core::common::{comments::Comments, Mark, SourceMap};

pub struct PluginContext<C: Comments> {
  pub top_level_mark: Mark,
  pub unresolved_mark: Mark,
  pub comments: C,
  pub cm: Arc<SourceMap>,
}
