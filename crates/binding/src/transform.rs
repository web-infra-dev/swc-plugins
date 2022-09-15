use std::sync::Arc;

use napi::{bindgen_prelude::AsyncTask, Env, JsObject, Status, Task};
use pass::types::{TransformConfig, TransformConfigNapi};
use shared::{
  napi,
  napi_derive::napi,
  serde_json,
  swc::{config::Options, TransformOutput},
  swc_common::sync::{Lazy, ReadGuard, RwLock},
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
pub fn transform_sync(
  env: Env,
  code: String,
  filename: String,
  map: Option<String>,
  config: Option<TransformConfigNapi>,
) -> napi::Result<Output> {
  let config = config.map(|config| {
    let swc_options: Options = serde_json::from_str(&config.swc)
      .map_err(|e| napi::Error::new(Status::InvalidArg, e.to_string()))
      .unwrap();

    TransformConfig {
      swc: swc_options,
      extensions: config.extensions,
    }
  });

  if let Some(config) = config {
    let mut cached_option = CACHED_OPTION.write();
    *cached_option = Some(config);
  };

  core::transform::transform(
    Some(env),
    &code,
    filename,
    map,
    get_option_from_cache()
      .as_ref()
      .expect("Failed to get options, you may forget to provide transform options"),
  )
  .map_err(|e| napi::Error::new(Status::GenericFailure, e.to_string()))
  .map(|transform_output| transform_output.into())
}

pub struct TransformTask {
  code: String,
  filename: String,
  map: Option<String>,
}

impl Task for TransformTask {
  type Output = TransformOutput;
  type JsValue = JsObject;
  fn compute(&mut self) -> napi::Result<Self::Output> {
    core::transform::transform(
      None,
      &self.code,
      self.filename.clone(),
      self.map.clone(),
      get_option_from_cache()
        .as_ref()
        .expect("Failed to get options, you may forget to provide transform options"),
    )
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

static CACHED_OPTION: Lazy<Arc<RwLock<Option<TransformConfig>>>> =
  Lazy::new(|| Arc::new(RwLock::new(None)));

#[napi]
pub fn transform(
  code: String,
  filename: String,
  map: Option<String>,
  config: Option<TransformConfigNapi>,
) -> AsyncTask<TransformTask> {
  if let Some(config) = config {
    let swc_options: Options = serde_json::from_str(&config.swc)
      .map_err(|e| napi::Error::new(Status::InvalidArg, e.to_string()))
      .unwrap();

    let config = TransformConfig {
      swc: swc_options,
      extensions: config.extensions,
    };

    let mut cached_options = CACHED_OPTION.write();
    *cached_options = Some(config);
  }

  AsyncTask::new(TransformTask {
    code,
    filename,
    map,
  })
}

fn get_option_from_cache() -> ReadGuard<'static, Option<TransformConfig>> {
  CACHED_OPTION.read()
}
