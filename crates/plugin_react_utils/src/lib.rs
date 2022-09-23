use shared::serde::{self, Deserialize};
use shared::swc_common::chain;
use shared::swc_common::pass::Either;
use shared::swc_ecma_transforms_base::pass::noop;
use shared::swc_ecma_visit::Fold;

mod import_react;
mod remove_effect;

pub use import_react::auto_import_react;
pub use remove_effect::remove_effect;

#[derive(Deserialize, Debug)]
#[serde(crate = "self::serde")]
pub struct ReactUtilsConfig {
  pub auto_import_react: Option<bool>,
  pub rm_effect: Option<bool>,
}

pub fn react_utils(config: &ReactUtilsConfig) -> impl Fold {
  chain!(
    if config.auto_import_react.unwrap_or(false) {
      Either::Left(auto_import_react())
    } else {
      Either::Right(noop())
    },
    if config.rm_effect.unwrap_or(false) {
      Either::Left(remove_effect())
    } else {
      Either::Right(noop())
    }
  )
}
