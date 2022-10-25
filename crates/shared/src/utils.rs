use swc_core::{
  common::SyntaxContext,
  ecma::{
    ast::Ident,
    atoms::JsWord,
    visit::{Fold, VisitMut},
  },
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

#[macro_export]
macro_rules! enable_helper {
  ($i:ident) => {{
    $crate::swc_core::ecma::transforms::base::helpers::HELPERS.with(|helpers| {
      helpers.$i();
      helpers.mark()
    })
  }};
}

#[macro_export]
macro_rules! helper_expr {
  ($field_name:ident, $s:tt) => {{
    $crate::helper_expr!($crate::swc_core::common::DUMMY_SP, $field_name, $s)
  }};

  ($span:expr, $field_name:ident, $s:tt) => {{
    let mark = $crate::enable_helper!($field_name);
    let span = $span.apply_mark(mark);
    let external =
      $crate::swc_core::ecma::transforms::base::helpers::HELPERS.with(|helper| helper.external());

    if external {
      $crate::swc_core::ecma::ast::Expr::from($crate::swc_core::ecma::utils::quote_ident!(
        span,
        concat!("_", stringify!($field_name))
      ))
    } else {
      $crate::swc_core::ecma::ast::Expr::from($crate::swc_core::ecma::utils::quote_ident!(
        span,
        concat!("_", $s)
      ))
    }
  }};
}

pub fn change_ident_syntax_context(ctxt: SyntaxContext, name: JsWord) -> impl VisitMut {
  ChangeIdentSyntaxContext::new(ctxt, name)
}

pub struct ChangeIdentSyntaxContext {
  pub ctxt: SyntaxContext,
  pub name: JsWord,
}

impl ChangeIdentSyntaxContext {
  fn new(ctxt: SyntaxContext, name: JsWord) -> Self {
    ChangeIdentSyntaxContext { ctxt, name }
  }
}

impl VisitMut for ChangeIdentSyntaxContext {
  fn visit_mut_ident(&mut self, ident: &mut Ident) {
    if ident.sym == self.name {
      ident.span = ident.span.with_ctxt(self.ctxt);
    }
  }
}
