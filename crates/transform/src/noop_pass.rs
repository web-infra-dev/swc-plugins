use shared::swc_ecma_visit::VisitMut;

pub struct Noop;

impl VisitMut for Noop {}

pub fn noop_pass() -> Noop {
  Noop
}
