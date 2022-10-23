use std::fmt::Display;

#[derive(Debug)]
pub enum ResolveErrorKind {
  ModuleNotFound,
  ShouldIgnore,
}

#[derive(Debug)]
pub struct ResolveError {
  pub msg: String,
  pub kind: ResolveErrorKind,
}

impl Display for ResolveError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&format!(
      "Error [{}]: {}",
      self.msg,
      match self.kind {
        ResolveErrorKind::ModuleNotFound => "Module not found",
        ResolveErrorKind::ShouldIgnore => "This module found but should be ignored",
      }
    ))
  }
}

impl std::error::Error for ResolveError {}

impl ResolveError {
  pub fn new(msg: String, kind: ResolveErrorKind) -> Self {
    Self { msg, kind }
  }
}
