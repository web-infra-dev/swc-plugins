pub mod config;
pub mod extensions;
pub mod plugin_import;
pub mod plugin_modularize_imports;
pub mod plugin_react_utils;
pub mod plugin_lock_corejs_version;
pub mod plugin_styled_components;
pub mod plugin_emotion;
pub mod plugin_styled_jsx;
pub mod plugin_lodash;

pub use config::TransformConfigNapi;
use napi::{Env, Result};
use shared::hashbrown::HashMap;
use std::collections::HashMap as StdHashMap;

pub trait IntoRawConfig<T> {
  fn into_raw_config(self, env: Env) -> Result<T>;
}

impl<T, S> IntoRawConfig<Option<T>> for Option<S>
where
  S: IntoRawConfig<T>,
{
  fn into_raw_config(self, env: Env) -> Result<Option<T>> {
    match self {
      Some(s) => Ok(Some(s.into_raw_config(env)?)),
      None => Ok(None),
    }
  }
}

impl<T, S> IntoRawConfig<Vec<T>> for Vec<S>
where
  S: IntoRawConfig<T>,
{
  fn into_raw_config(self, env: Env) -> Result<Vec<T>> {
    let mut res = Vec::with_capacity(self.len());

    for item in self {
      res.push(item.into_raw_config(env)?)
    }

    Ok(res)
  }
}

impl<T, K, V> IntoRawConfig<HashMap<K, V>> for HashMap<K, T>
where
  K: Eq + std::hash::Hash,
  T: IntoRawConfig<V>,
{
  fn into_raw_config(self, env: Env) -> Result<HashMap<K, V>> {
    let mut res = HashMap::with_capacity(self.len());

    for (k, v) in self {
      res.insert(k, v.into_raw_config(env)?);
    }

    Ok(res)
  }
}

impl<T, K, V> IntoRawConfig<StdHashMap<K, V>> for StdHashMap<K, T>
where
  K: Eq + std::hash::Hash,
  T: IntoRawConfig<V>,
{
  fn into_raw_config(self, env: Env) -> Result<StdHashMap<K, V>> {
    let mut res = StdHashMap::with_capacity(self.len());

    for (k, v) in self {
      res.insert(k, v.into_raw_config(env)?);
    }

    Ok(res)
  }
}
