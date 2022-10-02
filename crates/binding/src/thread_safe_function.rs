use crossbeam_channel::{unbounded, Receiver, Sender};
use std::{
  ffi::{c_void, CString},
  marker::PhantomData,
  ptr,
};

use napi::{
  check_status,
  sys::{self, ThreadsafeFunctionCallMode},
  Env, JsFunction, NapiRaw, NapiValue, Status,
};

pub struct CallContext<T> {
  pub value: T,
  pub callback: JsFunction,
  pub env: Env,
}

unsafe impl<Params: 'static, ReturnValue> Send for ThreadSafeFunction<Params, ReturnValue> {}
unsafe impl<Params: 'static, ReturnValue> Sync for ThreadSafeFunction<Params, ReturnValue> {}

pub struct ThreadSafeFunction<Params: 'static, ReturnValue: 'static> {
  tsfn: sys::napi_threadsafe_function,
  tx: Sender<napi::Result<ReturnValue>>,
  rx: Receiver<napi::Result<ReturnValue>>,
  _params: PhantomData<(Params, ReturnValue)>,
}

// Alloc:                       Drop at:
// closure                      finalize_cb
// tsfn                         ThreadSafeFunction impl dropped
impl<Params: 'static, ReturnValue: 'static> ThreadSafeFunction<Params, ReturnValue> {
  pub fn new<F: Unpin>(env: Env, func: JsFunction, callback: F) -> Self
  where
    F: Fn(CallContext<Params>) -> napi::Result<ReturnValue>,
  {
    let (tx, rx) = unbounded();

    let cb = Box::into_raw(Box::new(callback)) as *mut c_void;

    let mut tsfn = ptr::null_mut();
    let mut async_resource_name = ptr::null_mut();
    let s = "napi_threadsafe_function";
    let len = s.len();
    let s = CString::new(s).unwrap();
    let env = env.raw();
    check_status!(unsafe {
      sys::napi_create_string_utf8(env, s.as_ptr(), len, &mut async_resource_name)
    })
    .unwrap();

    unsafe {
      sys::napi_create_threadsafe_function(
        env,
        func.raw(),
        ptr::null_mut(),
        async_resource_name,
        0,
        1usize,
        cb,
        Some(thread_finalize_cb::<Params, ReturnValue, F>),
        cb,
        Some(call_js_cb::<Params, ReturnValue, F>),
        &mut tsfn,
      );

      // mark this to tell nodejs process could exit before `tsfn` gets destroyed
      sys::napi_unref_threadsafe_function(env, tsfn);
    };

    Self {
      tsfn,
      tx,
      rx,
      _params: PhantomData,
    }
  }

  pub fn call(&self, params: Params) -> napi::Result<ReturnValue> {
    unsafe {
      let params_ptr = Box::into_raw(Box::new((params, self.tx.clone()))) as *mut c_void;

      sys::napi_call_threadsafe_function(
        self.tsfn,
        params_ptr,
        ThreadsafeFunctionCallMode::nonblocking,
      );
    }

    // wait for tsfn to finish
    self.rx.recv().unwrap()
  }
}

impl<Params: 'static, ReturnValue: 'static> Drop for ThreadSafeFunction<Params, ReturnValue> {
  fn drop(&mut self) {
    unsafe {
      check_status!(sys::napi_release_threadsafe_function(
        self.tsfn,
        sys::ThreadsafeFunctionReleaseMode::release
      ))
      .unwrap();
    }
  }
}

/// Work Flow
/// create_threadsafe_function() to create a `tsfn` pointer
unsafe extern "C" fn thread_finalize_cb<T: 'static, R: 'static, F>(
  _raw_env: sys::napi_env,
  finalize_data: *mut c_void,
  _finalize_hint: *mut c_void,
) where
  F: Fn(CallContext<T>) -> napi::Result<R>,
{
  // cleanup
  drop(Box::from_raw(finalize_data.cast::<F>()));
}

unsafe extern "C" fn call_js_cb<T: 'static, R: 'static, F>(
  raw_env: sys::napi_env,
  js_callback: sys::napi_value, // null
  context: *mut c_void,         // rust closure
  data: *mut c_void,            // invoke args
) where
  F: Fn(CallContext<T>) -> napi::Result<R>,
{
  // ctx is the callback passed into the ThreadSafeFunction::new()
  let callback = context.cast::<F>();
  // Arguments provided by call_napi_threadsafe_function
  let (params, sender): (T, Sender<napi::Result<R>>) =
    *Box::<(T, Sender<napi::Result<R>>)>::from_raw(data.cast());

  // env and/or callback can be null when shutting down
  if raw_env.is_null() || js_callback.is_null() {
    sender
      .send(Err(napi::Error::new(
        Status::Cancelled,
        "Internal error while calling threadsafe function".into(),
      )))
      .expect("Send threadsafe function return data failed");
    return;
  }

  let res = (*callback)(CallContext {
    env: Env::from_raw(raw_env),
    value: params,
    callback: JsFunction::from_raw(raw_env, js_callback).unwrap(),
  });

  sender
    .send(res)
    .expect("Send threadsafe function return data failed");
}
