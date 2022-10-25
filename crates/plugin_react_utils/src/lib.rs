use shared::serde::{self, Deserialize};
use shared::swc_core::common::Mark;
use shared::swc_core::{
  common::{chain, pass::Either,},
  ecma::{
    transforms::base::pass::noop,
    visit::Fold,
  }
};

mod import_react;
mod remove_effect;

pub use import_react::auto_import_react;
pub use remove_effect::remove_effect;

#[derive(Deserialize, Debug, Clone, Default)]
#[serde(crate = "self::serde")]
pub struct ReactUtilsConfig {
  pub auto_import_react: bool,
  pub rm_effect: bool,
}

pub fn react_utils(config: &ReactUtilsConfig, top_level_mark: Mark) -> impl Fold {
  chain!(
    if config.auto_import_react {
      Either::Left(auto_import_react(top_level_mark))
    } else {
      Either::Right(noop())
    },
    if config.rm_effect {
      Either::Left(remove_effect())
    } else {
      Either::Right(noop())
    }
  )
}
