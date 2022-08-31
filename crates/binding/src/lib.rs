pub mod minify;
pub mod transform;

use napi::{bindgen_prelude::Buffer, Env, JsObject};
use shared::napi;

pub fn buffer2string(b: Buffer) -> String {
  let s: Vec<u8> = b.into();
  String::from_utf8(s).unwrap()
}

pub fn create_output(env: Env, code: &str, map: Option<impl AsRef<str>>) -> napi::Result<JsObject> {
  let mut obj = env.create_object()?;
  obj.set_named_property("code", env.create_string(code)?)?;
  if let Some(map) = map {
    obj.set_named_property("map", env.create_string(map.as_ref())?)?;
  }

  Ok(obj)
}
