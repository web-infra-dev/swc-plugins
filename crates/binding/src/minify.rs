use napi::{bindgen_prelude::AsyncTask, JsObject, Status, Task};
use shared::{
  napi,
  napi_derive::napi,
  serde_json,
  swc::{config::JsMinifyOptions, TransformOutput},
};

use crate::create_output;

pub struct Minifier {
  code: String,
  filename: String,
  config: JsMinifyOptions,
}

impl Task for Minifier {
  type Output = TransformOutput;
  type JsValue = JsObject;

  fn compute(&mut self) -> napi::Result<Self::Output> {
    core::minify::minify(
      &self.config,
      self.filename.clone(),
      self.code.clone(),
    )
    .map_err(|e| napi::Error::new(Status::GenericFailure, e.to_string()))
  }

  fn resolve(&mut self, env: napi::Env, output: Self::Output) -> napi::Result<Self::JsValue> {
    create_output(env, &output.code, output.map.as_ref())
  }
}

// type def here doesn't matter that much, because @modern-js/swc would override this
#[napi(ts_return_type = "Promise<{ code: string, map?: string }>")]
pub fn minify(
  config: String,
  filename: String,
  code: String,
) -> AsyncTask<Minifier> {
  AsyncTask::new(Minifier {
    code,
    filename,
    config: serde_json::from_str(&config).unwrap(),
  })
}
