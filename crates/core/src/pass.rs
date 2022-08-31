use std::sync::Arc;

use shared::{
  swc_common::{Mark, SourceMap},
  swc_ecma_visit::{VisitMut},
};
use transform::{babel_import, types::TransformConfig};

pub fn internal_transform_pass(
  _cm: Arc<SourceMap>,
  config: &TransformConfig,
  _top_level_mark: Mark,
) -> impl VisitMut + '_ {
  babel_import::plugin_import(config)
}
