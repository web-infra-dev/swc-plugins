use napi::{bindgen_prelude::AsyncTask, JsUndefined, Task};
use napi_derive::napi;

#[napi]
/// terminate the process when node.js process.exit
pub fn terminate_process(code: Option<i32>) -> AsyncTask<Exit> {
  AsyncTask::new(Exit(code))
}

pub struct Exit(Option<i32>);

#[napi]
impl Task for Exit {
  type Output = ();

  type JsValue = JsUndefined;

  fn compute(&mut self) -> napi::Result<Self::Output> {
    let code = self.0.unwrap_or(0);
    std::process::exit(code);
  }

  fn resolve(&mut self, env: napi::Env, _output: Self::Output) -> napi::Result<Self::JsValue> {
    env.get_undefined()
  }
}
