use serde::Deserialize;
use shared::{
  helper_expr,
  swc_core::{
    self,
    common::{comments::Comments, Span, DUMMY_SP},
    ecma::{
      ast::{Callee, Expr, ExprOrSpread, Ident, Lit, Tpl, TplElement},
      atoms::JsWord,
      utils::ExprFactory,
      visit::{as_folder, Fold, VisitMut, VisitMutWith},
    },
    quote,
  },
};

#[derive(Debug, Deserialize, Clone)]
pub struct DynImportNodeConfig {
  pub interop: bool,
}

pub struct DynImportNode<C>
where
  C: Comments,
{
  pub interop: bool,
  pub comments: Option<C>,
}

pub fn dyn_import_node<C>(config: DynImportNodeConfig, comments: Option<C>) -> impl Fold
where
  C: Comments,
{
  as_folder(DynImportNode {
    interop: config.interop,
    comments,
  })
}

fn string_lit(args: &Vec<ExprOrSpread>) -> bool {
  if args.len() != 1 {
    return false;
  }

  matches!(&*args[0].expr, Expr::Lit(Lit::Str(_)))
}

impl<C> VisitMut for DynImportNode<C>
where
  C: Comments,
{
  fn visit_mut_expr(&mut self, expr: &mut Expr) {
    if let Expr::Call(call_expr) = expr {
      if let Callee::Import(_) = call_expr.callee {
        // argument that passed to require()
        let (is_string_lit, import_specifier) = string_lit(&call_expr.args)
          .then(|| (true, *call_expr.args[0].expr.clone())) // for string lit argument, we simply using require('foo')
          .unwrap_or_else(|| {
            // for expr, we wrap it in template string: `${foo()}`
            (
              false,
              Expr::Tpl(Tpl {
                span: DUMMY_SP,
                exprs: vec![call_expr.args[0].expr.clone()],
                quasis: vec![
                  TplElement {
                    span: swc_core::common::DUMMY_SP,
                    tail: false,
                    cooked: None,
                    raw: "".into(),
                  },
                  TplElement {
                    span: swc_core::common::DUMMY_SP,
                    tail: true,
                    cooked: None,
                    raw: "".into(),
                  },
                ],
              }),
            )
          });
        let tpl = if is_string_lit {
          // Promise.resolve().then(() => require(import_specifier))
          let import_specifier = quote!("require($s)" as Expr, s: Expr = import_specifier);

          quote!(
            "Promise.resolve().then(() => $import_specifier)" as Box<Expr>,
            import_specifier: Expr = if self.interop {
              interop_expr(import_specifier, self.comments.as_ref())
            } else {
              import_specifier
            },
          )
        } else {
          // Promise.resolve(specifier).then(ident => require(ident))
          let ident = Ident::new(JsWord::from("s"), DUMMY_SP);
          let require_expr = quote!("require($ident)" as Expr, ident: Ident = ident.clone());

          quote!(
            "Promise.resolve($specifier).then($ident => $require_expr)" as Box<Expr>,
            specifier: Expr = import_specifier,
            ident: Ident = ident,
            require_expr: Expr = if self.interop {
              interop_expr(require_expr, self.comments.as_ref())
            } else {
              require_expr
            }
          )
        };

        *expr = *tpl
      }
    }
    expr.visit_mut_children_with(self);
  }
}

fn interop_expr(e: Expr, comment: Option<&impl Comments>) -> Expr {
  let span = if let Some(c) = comment {
    let s = Span::dummy_with_cmt();
    c.add_pure_comment(s.lo);
    s
  } else {
    DUMMY_SP
  };

  helper_expr!(interop_require_wildcard, "interopRequireWildcard").as_call(
    span,
    vec![ExprOrSpread {
      spread: None,
      expr: Box::new(e),
    }],
  )
}

#[cfg(test)]
mod test {
  use shared::{
    swc_core::common::comments::SingleThreadedComments, swc_ecma_transforms_testing::test_transform,
  };

  use crate::{dyn_import_node, DynImportNodeConfig};

  #[test]
  fn dynamic_and_literal() {
    test_transform(
      Default::default(),
      |_| dyn_import_node::<SingleThreadedComments>(DynImportNodeConfig { interop: false }, None),
      "import('foo')",
      "Promise.resolve().then(() => require('foo'));",
      true,
    );
  }

  #[test]
  fn interop() {
    test_transform(
      Default::default(),
      |_| dyn_import_node::<SingleThreadedComments>(DynImportNodeConfig { interop: true }, None),
      "import('foo')",
      "Promise.resolve().then(() => _interopRequireWildcard(require('foo')));",
      true,
    );

    test_transform(
      Default::default(),
      |_| dyn_import_node::<SingleThreadedComments>(DynImportNodeConfig { interop: false }, None),
      "import([1, 2])",
      "Promise.resolve(`${[1, 2]}`).then((s) => require(s));",
      true,
    );
  }
}
