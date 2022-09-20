use napi::{bindgen_prelude::AsyncTask, Env, JsObject, Result, Status, Task};
use pass::{
  from_napi_config,
  types::{Extensions, ExtensionsNapi, TransformConfig, TransformConfigNapi},
};
use shared::{
  napi,
  napi_derive::napi,
  serde_json,
  swc::{
    config::{JsMinifyOptions, Options},
    Compiler as SwcCompiler, TransformOutput,
  },
  swc_common::{
    sync::{Lazy, RwLock},
    SourceMap,
  },
};

use std::{
  collections::HashMap,
  sync::{
    atomic::{AtomicU32, Ordering},
    Arc,
  },
};

// ===== Low level internal struct under the hood =====
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
    let swc_options: Options = serde_json::from_str(&config.swc)
      .map_err(|e| napi::Error::new(Status::InvalidArg, e.to_string()))
      .unwrap();

    let ExtensionsNapi {
      modularize_imports,
      plugin_import,
      react_utils,
    } = config.extensions;

    let config = TransformConfig {
      swc: swc_options,
      extensions: Extensions {
        modularize_imports,
        plugin_import: plugin_import.map(|configs| {
          configs
            .into_iter()
            .map(|c| from_napi_config(env, c))
            .collect::<Vec<_>>()
        }),
        react_utils,
      }
    };

    // TODO figure out this ordering
    let id = ID.fetch_add(1, Ordering::Acquire);

    let mut compilers = COMPILERS.write();
    compilers.insert(
      id,
      Compiler {
        config,
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
    env: Env,
    filename: String,
    code: String,
    map: Option<String>,
  ) -> Result<Output> {
    let compilers = COMPILERS.read();

    let compiler = compilers
      .get(&self.id)
      .expect("Compiler is released, maybe you are using compiler after call release()");

    core::transform(
      Some(env),
      compiler.swc_compiler.clone(),
      &compiler.config,
      filename.clone(),
      &code,
      map.clone(),
    )
    .map_err(|e| napi::Error::new(Status::GenericFailure, e.to_string()))
    .map(|transform_output| transform_output.into())
  }

  #[napi]
  pub fn minify(&self, filename: String, config: String, code: String) -> AsyncTask<Minifier> {
    AsyncTask::new(Minifier {
      compiler_id: self.id,
      code,
      filename,
      config: serde_json::from_str(&config).unwrap(),
    })
  }

  #[napi]
  pub fn minify_sync(&self, filename: String, config: String, code: String) -> Result<Output> {
    let compilers = COMPILERS.read();

    let compiler = compilers
      .get(&self.id)
      .expect("Compiler is released, maybe you are using compiler after call release()");
    let js_minify_options = serde_json::from_str(&config).unwrap();

    core::minify(
      compiler.swc_compiler.clone(),
      &js_minify_options,
      filename.clone(),
      code,
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

// Napi boiler plate code

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
      None,
      compiler.swc_compiler.clone(),
      &compiler.config,
      self.filename.clone(),
      &self.code,
      self.map.clone(),
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
  compiler_id: u32,
  code: String,
  filename: String,
  config: JsMinifyOptions,
}

impl Task for Minifier {
  type Output = TransformOutput;
  type JsValue = JsObject;

  fn compute(&mut self) -> napi::Result<Self::Output> {
    let compilers = COMPILERS.read();
    let compiler = compilers
      .get(&self.compiler_id)
      .expect("Compiler is released, maybe you are using compiler after call release()");

    core::minify(
      compiler.swc_compiler.clone(),
      &self.config,
      self.filename.clone(),
      self.code.clone(),
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
