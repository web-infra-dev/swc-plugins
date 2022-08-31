use napi::{bindgen_prelude::AsyncTask, JsObject, Status, Task};
use shared::{napi, napi_derive::napi, serde_json, swc::TransformOutput};
use transform::types::TransformConfig;

pub struct Transformer {
  config: String,
  code: String,
}

impl Task for Transformer {
  type Output = TransformOutput;
  type JsValue = JsObject;

  fn compute(&mut self) -> napi::Result<Self::Output> {
    let config: TransformConfig = serde_json::from_str(&self.config).unwrap();

    core::transform::transform(config, &self.code)
      .map_err(|e| napi::Error::new(Status::GenericFailure, e.to_string()))
  }

  fn resolve(&mut self, env: napi::Env, output: Self::Output) -> napi::Result<Self::JsValue> {
    let mut obj: JsObject = env.create_object()?;
    obj.set_named_property("code", env.create_string(&output.code)?)?;
    if let Some(map) = output.map {
      obj.set_named_property("map", env.create_string(&map)?)?;
    }
    Ok(obj)
  }
}

// type def here doesn't matter that much, because @modern-js/swc would override this
#[napi(ts_return_type = "Promise<{ code: string, map?: string }>")]
pub fn transform(config: String, code: String) -> AsyncTask<Transformer> {
  AsyncTask::new(Transformer { config, code })
}
