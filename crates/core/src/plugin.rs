use std::sync::{Arc, Mutex};

use shared::swc_core::common::{comments::Comments, Mark, SourceMap};

pub struct PluginContext<C: Comments> {
  pub cm: Arc<SourceMap>,
  pub top_level_mark: Mark,
  pub unresolved_mark: Mark,
  pub comments: C,
  pub error_emitter: Arc<Mutex<ErrorEmitter>>,
}

#[derive(Default)]
pub struct ErrorEmitter {
  errors: Vec<shared::anyhow::Error>,
}

impl ErrorEmitter {
  pub fn new() -> Self {
    Self { errors: vec![] }
  }

  pub fn emit_err(&mut self, err: shared::anyhow::Error) {
    self.errors.push(err)
  }
}
