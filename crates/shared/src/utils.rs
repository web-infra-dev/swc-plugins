use {
  swc_core::{
    common::SyntaxContext,
    ecma::{
      ast::Ident,
      visit::{Fold, VisitMut}
    }
  }
};

#[derive(Clone, Copy)]
pub struct ClearMark;

impl VisitMut for ClearMark {
  fn visit_mut_ident(&mut self, ident: &mut Ident) {
    ident.span.ctxt = SyntaxContext::empty();
  }
}

struct Noop;

impl VisitMut for Noop {}
impl Fold for Noop {}

pub fn noop_pass() -> impl Fold + VisitMut {
  Noop
}
