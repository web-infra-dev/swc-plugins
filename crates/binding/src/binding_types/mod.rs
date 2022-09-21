pub mod config;
pub mod extensions;
pub mod plugin_import;
pub mod plugin_modularize_imports;
pub mod plugin_react_utils;

use std::{
  collections::HashMap,
};

pub use config::TransformConfigNapi;
use napi::{Env, Result};
pub trait FromNapi<T> {
  fn from_napi(self, env: Env) -> Result<T>;
}

impl<T, S> FromNapi<Option<T>> for Option<S>
where
  S: FromNapi<T>,
{
  fn from_napi(self, env: Env) -> Result<Option<T>> {
    match self {
      Some(s) => Ok(Some(s.from_napi(env)?)),
      None => Ok(None),
    }
  }
}

impl<T, S> FromNapi<Vec<T>> for Vec<S>
where
  S: FromNapi<T>,
{
  fn from_napi(self, env: Env) -> Result<Vec<T>> {
    let mut orig = self.into_iter();
    let mut res = Vec::with_capacity(orig.len());

    while let Some(item) = orig.next() {
      res.push(item.from_napi(env)?)
    }

    Ok(res)
  }
}

impl<T, K, V> FromNapi<HashMap<K, V>> for HashMap<K, T>
where
  K: Eq + std::hash::Hash,
  T: FromNapi<V>,
{
  fn from_napi(self, env: Env) -> Result<HashMap<K, V>> {
    let mut orig = self.into_iter();
    let mut res = HashMap::with_capacity(orig.len());

    while let Some((k, v)) = orig.next() {
      res.insert(k, v.from_napi(env)?);
    }

    Ok(res)
  }
}
