mod binding_types;
mod tsfn;
use binding_types::{FromNapi, TransformConfigNapi};
use napi::{
  bindgen_prelude::{AsyncTask, Buffer},
  Env, JsObject, Result, Status, Task,
};

use napi_derive::napi;
use shared::{
  serde_json,
  swc::{config::JsMinifyOptions, Compiler as SwcCompiler, TransformOutput},
  swc_common::{
    sync::{Lazy, RwLock},
    SourceMap,
  },
};

use core::types::TransformConfig;
use std::{
  collections::HashMap,
  sync::{
    atomic::{AtomicU32, Ordering},

    Arc,
  },
};

// ===== Internal Rust struct under the hood =====
pub struct Compiler {
  pub config: TransformConfig,
  pub swc_compiler: Arc<SwcCompiler>,
}

// js id -> Rust Compiler
pub static COMPILERS: Lazy<Arc<RwLock<HashMap<u32, Compiler>>>> =
  Lazy::new(|| Arc::new(RwLock::new(HashMap::new())));

static ID: AtomicU32 = AtomicU32::new(0);

// ========== API exposed to js ==========
// this id maps to a real struct in COMPILERS

#[napi(js_name = "Compiler")]
pub struct JsCompiler {
  pub id: u32,
}

#[napi]
impl JsCompiler {
  #[napi(constructor)]
  pub fn new(env: Env, config: TransformConfigNapi) -> Self {
    let id = ID.fetch_add(1, Ordering::Relaxed);

    let mut compilers = COMPILERS.write();
    compilers.insert(
      id,
      Compiler {
        config: FromNapi::from_napi(config, env)
        .unwrap(),
        swc_compiler: Arc::new(SwcCompiler::new(Arc::new(SourceMap::default()))),
      },
    );

    Self { id }
  }

  #[napi]
  pub fn transform(
    &self,
    filename: String,
    code: Buffer,
    map: Option<Buffer>,
  ) -> AsyncTask<TransformTask> {
    AsyncTask::new(TransformTask {
      code,
      filename,
      map,
      compiler_id: self.id,
    })
  }

  #[napi]
  pub fn transform_sync(
    &self,
    filename: String,
    code: Buffer,
    map: Option<Buffer>,
  ) -> Result<Output> {
    let compilers = COMPILERS.read();

    let compiler = compilers
      .get(&self.id)
      .expect("Compiler is released, maybe you are using compiler after call release()");

    core::transform(
      compiler.swc_compiler.clone(),
      &compiler.config,
      filename.clone(),
      std::str::from_utf8(code.to_vec().as_slice()).unwrap(),
      map.map(|m| String::from_utf8(m.to_vec()).unwrap()),
    )
    .map_err(|e| napi::Error::new(Status::GenericFailure, e.to_string()))
    .map(|transform_output| transform_output.into())
  }

  #[napi]
  pub fn release(&self) {
    let mut compilers = COMPILERS.write();
    compilers.remove(&self.id);
  }
}

#[napi]
pub fn minify(config: Buffer, filename: String, code: Buffer) -> AsyncTask<Minifier> {
  AsyncTask::new(Minifier {
    code,
    filename,
    config: serde_json::from_slice(&<Buffer as Into<Vec<_>>>::into(config)).unwrap(),
  })
}

#[napi]
pub fn minify_sync(config: Buffer, filename: String, code: Buffer) -> Result<Output> {
  let js_minify_options = serde_json::from_slice(&<Buffer as Into<Vec<_>>>::into(config)).unwrap();

  core::minify(
    &js_minify_options,
    filename.clone(),
    std::str::from_utf8(code.to_vec().as_slice()).unwrap(),
  )
  .map_err(|e| napi::Error::new(Status::GenericFailure, e.to_string()))
  .map(|transform_output| transform_output.into())
}

// ======= Napi boiler plate code =======
#[napi(object)]
pub struct Output {
  pub code: Buffer,
  pub map: Option<Buffer>,
}

impl From<TransformOutput> for Output {
  fn from(o: TransformOutput) -> Self {
    Self {
      code: o.code.into_bytes().into(),
      map: o.map.map(|m| m.into_bytes().into()),
    }
  }
}

pub struct TransformTask {
  pub compiler_id: u32,
  pub code: Buffer,
  pub filename: String,
  pub map: Option<Buffer>,
}

impl Task for TransformTask {
  type Output = TransformOutput;
  type JsValue = JsObject;
  fn compute(&mut self) -> napi::Result<Self::Output> {
    let compilers = COMPILERS.read();

    let compiler = compilers
      .get(&self.compiler_id)
      .expect("Compiler is released, maybe you are using compiler after call release()");

    core::transform(
      compiler.swc_compiler.clone(),
      &compiler.config,
      self.filename.clone(),
      std::str::from_utf8(self.code.to_vec().as_slice()).unwrap(),
      self
        .map
        .as_ref()
        .map(|map_buf| String::from_utf8(map_buf.to_vec()).unwrap()),
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

pub struct Minifier {
  code: Buffer,
  filename: String,
  config: JsMinifyOptions,
}

impl Task for Minifier {
  type Output = TransformOutput;
  type JsValue = JsObject;

  fn compute(&mut self) -> napi::Result<Self::Output> {
    core::minify(
      &self.config,
      self.filename.clone(),
      std::str::from_utf8(self.code.to_vec().as_slice()).unwrap(),
    )
    .map_err(|e| napi::Error::new(Status::GenericFailure, e.to_string()))
  }

  fn resolve(&mut self, env: napi::Env, output: Self::Output) -> napi::Result<Self::JsValue> {
    let mut obj = env.create_object()?;
    obj.set_named_property("code", env.create_string(&output.code)?)?;
    if let Some(map) = output.map {
      obj.set_named_property("map", env.create_string(&map)?)?;
    }

    Ok(obj)
  }
}
