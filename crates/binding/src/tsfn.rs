use std::{
  ffi::{c_void, CString},
  marker::PhantomData,
  pin::Pin,
  ptr,
  sync::mpsc::{channel, Sender},
};

use napi::{
  check_status,
  sys::{self, ThreadsafeFunctionCallMode},
  threadsafe_function::ThreadSafeCallContext,
  Env,
};

/// Work Flow
/// create_threadsafe_function() to create a `tsfn` pointer
/// call_
unsafe extern "C" fn thread_finalize_cb<T: 'static, R: 'static>(
  _raw_env: sys::napi_env,
  finalize_data: *mut c_void,
  _finalize_hint: *mut c_void,
) {
  // cleanup
  drop(Box::from_raw(
    finalize_data.cast::<fn(ThreadSafeCallContext<(T, Sender<R>)>)>(),
  ));
}

unsafe extern "C" fn call_js_cb<T: 'static, R: 'static, F>(
  raw_env: sys::napi_env,
  _js_callback: sys::napi_value, // null
  context: *mut c_void,          // rust closure
  data: *mut c_void,             // invoke args
) where
  F: Fn(ThreadSafeCallContext<T>) -> R,
{
  // env and/or callback can be null when shutting down
  if raw_env.is_null() {
    return;
  }

  let ctx = context.cast::<F>();

  // Arguments provided by call_napi_threadsafe_function
  let (params, sender): (T, Sender<R>) = *Box::<(T, Sender<R>)>::from_raw(data.cast());

  let res = (*ctx)(ThreadSafeCallContext {
    env: Env::from_raw(raw_env),
    value: params,
  });

  sender
    .send(res)
    .expect("Send threadsafe function return data failed");
}

unsafe impl<Params: 'static, ReturnValue> Send for ThreadSafeFunction<Params, ReturnValue> {}
unsafe impl<Params: 'static, ReturnValue> Sync for ThreadSafeFunction<Params, ReturnValue> {}

pub struct ThreadSafeFunction<Params: 'static, ReturnValue: 'static> {
  pub tsfn: sys::napi_threadsafe_function,
  _params: PhantomData<(Params, ReturnValue)>,
}

// Alloc                        Drop
// closure                      finalize_cb
// *const tsfn                  ThreadSafeFunction get dropped
// call params and sender       inside closure
impl<Params: 'static, ReturnValue: 'static> ThreadSafeFunction<Params, ReturnValue> {
  pub fn new<F: Unpin>(env: Env, callback: F) -> Self
  where
    F: Fn(ThreadSafeCallContext<Params>) -> ReturnValue,
  {
    let cb = Pin::into_inner(Box::pin(callback));
    let cb = Box::into_raw(cb) as *mut c_void;

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
        ptr::null_mut(),
        ptr::null_mut(),
        async_resource_name,
        0,
        1usize,
        cb,
        Some(thread_finalize_cb::<Params, ReturnValue>),
        cb,
        Some(call_js_cb::<Params, ReturnValue, F>),
        &mut tsfn,
      );
      sys::napi_unref_threadsafe_function(env, tsfn);
    };


    Self {
      tsfn,
      _params: PhantomData,
    }
  }

  pub fn call(&self, params: Params) -> ReturnValue {
    let (sender, receiver) = channel::<ReturnValue>();

    unsafe {
      let params_ptr = Box::into_raw(Box::new((params, sender))) as *mut c_void;
      sys::napi_call_threadsafe_function(
        self.tsfn,
        params_ptr,
        ThreadsafeFunctionCallMode::nonblocking,
      );
    }

    // wait for tsfn to finish
    receiver.recv().unwrap()
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
