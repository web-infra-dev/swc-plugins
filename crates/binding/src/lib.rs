mod binding_types;
mod thread_safe_function;
use binding_types::{IntoRawConfig, TransformConfigNapi};
use napi::{bindgen_prelude::AsyncTask, Env, JsObject, Result, Status, Task};

use napi_derive::napi;
use shared::{
  serde_json,
  swc::{
    config::{JsMinifyOptions, TerserSourceMapOption},
    Compiler as SwcCompiler, TransformOutput,
  },
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
        config: IntoRawConfig::into_raw_config(config, env).unwrap(),
        swc_compiler: Arc::new(SwcCompiler::new(Arc::new(SourceMap::default()))),
      },
    );

    Self { id }
  }

  #[napi]
  pub fn transform(
    &self,
    filename: String,
    code: String,
    map: Option<String>,
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
    code: String,
    map: Option<String>,
  ) -> Result<Output> {
    // This hack is for distinguish
    std::env::set_var("MODERN_JS_SWC_SYNC_CALL", "1");

    let compilers = COMPILERS.read();

    let compiler = compilers
      .get(&self.id)
      .expect("Compiler is released, maybe you are using compiler after call release()");

    let res = core::transform(
      compiler.swc_compiler.clone(),
      &compiler.config,
      filename,
      &code,
      map,
    )
    .map_err(|e| napi::Error::new(Status::GenericFailure, e.to_string()))
    .map(|transform_output| transform_output.into());

    std::env::remove_var("MODERN_JS_SWC_SYNC_CALL");

    res
  }

  #[napi]
  pub fn release(&self) {
    let mut compilers = COMPILERS.write();
    compilers.remove(&self.id);
  }
}

#[napi]
pub fn minify(
  config: String,
  filename: String,
  code: String,
  map: Option<String>,
) -> AsyncTask<Minifier> {
  let mut js_minify_options: JsMinifyOptions =
    serde_json::from_slice(&<String as Into<Vec<_>>>::into(config)).unwrap();

  if let Some(m) = map {
    let m: TerserSourceMapOption = serde_json::from_str(&m).expect("Invalid SourceMap string");
    js_minify_options.source_map = m.into();
  }

  AsyncTask::new(Minifier {
    code,
    filename,
    config: js_minify_options,
  })
}

#[napi]
pub fn minify_sync(
  config: String,
  filename: String,
  code: String,
  map: Option<String>,
) -> Result<Output> {
  let mut js_minify_options: JsMinifyOptions =
    serde_json::from_slice(&<String as Into<Vec<_>>>::into(config)).unwrap();

  if let Some(m) = map {
    let m: TerserSourceMapOption = serde_json::from_str(&m).expect("Invalid SourceMap string");
    js_minify_options.source_map = m.into();
  }

  core::minify(&js_minify_options, filename, &code)
    .map_err(|e| napi::Error::new(Status::GenericFailure, e.to_string()))
    .map(|transform_output| transform_output.into())
}

// ======= Napi boiler plate code =======
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

pub struct TransformTask {
  pub compiler_id: u32,
  pub code: String,
  pub filename: String,
  pub map: Option<String>,
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
      &self.code,
      self.map.clone(),
    )
    .map_err(|e| napi::Error::new(Status::GenericFailure, e.to_string()))
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
  code: String,
  filename: String,
  config: JsMinifyOptions,
}

impl Task for Minifier {
  type Output = TransformOutput;
  type JsValue = JsObject;

  fn compute(&mut self) -> napi::Result<Self::Output> {
    core::minify(&self.config, self.filename.clone(), &self.code)
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
