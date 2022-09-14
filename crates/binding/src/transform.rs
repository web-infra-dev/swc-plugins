use napi::{bindgen_prelude::AsyncTask, Env, JsObject, Status, Task};
use pass::types::{TransformConfig, TransformConfigNapi};
use shared::{
  napi,
  napi_derive::napi,
  serde_json,
  swc::{config::Options, TransformOutput},
};

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
pub fn transform_sync(env: Env, config: TransformConfigNapi, code: String) -> napi::Result<Output> {
  // swc do not impl napi, so we have to make that
  let swc_options: Options = serde_json::from_str(&config.swc)
    .map_err(|e| napi::Error::new(Status::InvalidArg, e.to_string()))?;

  let config = TransformConfig {
    swc: swc_options,
    extensions: &config.extensions,
  };

  core::transform::transform(Some(env), config, &code)
    .map_err(|e| napi::Error::new(Status::GenericFailure, e.to_string()))
    .map(|transform_output| transform_output.into())
}

pub struct TransformTask {
  config: TransformConfigNapi,
  code: String,
}

impl Task for TransformTask {
  type Output = TransformOutput;
  type JsValue = JsObject;
  fn compute(&mut self) -> napi::Result<Self::Output> {
    // swc do not impl napi, so we have to make that
    let swc_options: Options = serde_json::from_str(&self.config.swc)
      .map_err(|e| napi::Error::new(Status::InvalidArg, e.to_string()))?;

    let config = TransformConfig {
      swc: swc_options,
      extensions: &self.config.extensions,
    };

    core::transform::transform(None, config, &self.code)
      .map_err(|e| napi::Error::new(Status::GenericFailure, e.to_string()))
      .map(|transform_output| transform_output.into())
  }

  fn resolve(&mut self, env: Env, output: Self::Output) -> napi::Result<Self::JsValue> {
    let mut obj = env.create_object()?;
    obj.set_named_property("code", env.create_string(&output.code)?)?;
    if let Some(map) = output.map {
      obj.set_named_property("map", env.create_string(&map)?)?;
    }
    Ok(obj)
  }
}

#[napi]
pub fn transform(config: TransformConfigNapi, code: String) -> AsyncTask<TransformTask> {
  AsyncTask::new(TransformTask { config, code })
}
