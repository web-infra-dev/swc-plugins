use napi::{Env, Status};
use shared::{
  napi,
  napi_derive::napi,
  serde_json,
  swc::{config::Options, TransformOutput},
};
use transform::types::{TransformConfig, TransformConfigNapi};

#[derive(Debug)]
#[napi(object)]
pub struct Output {
  pub code: String,
  pub map: Option<String>,
}

impl From<TransformOutput> for Output {
  fn from(o: TransformOutput) -> Self {
    Self {
      code: o.code,
      map: o.map,
    }
  }
}

#[napi]
pub fn transform(env: Env, config: TransformConfigNapi, code: String) -> napi::Result<Output> {
  // swc do not impl napi, so we have to make that
  let swc_options: Options = serde_json::from_str(&config.swc)
    .map_err(|e| napi::Error::new(Status::InvalidArg, e.to_string()))?;

  let config = TransformConfig {
    swc: swc_options,
    extensions: config.extensions,
  };

  core::transform::transform(Some(env), config, &code)
    .map_err(|e| napi::Error::new(Status::GenericFailure, e.to_string()))
    .map(|transform_output| transform_output.into())
}
