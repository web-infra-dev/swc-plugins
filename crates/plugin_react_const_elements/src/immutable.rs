use swc_core::{
  common::DUMMY_SP,
  ecma::{
    ast::{
      AssignExpr, AssignOp, BinExpr, BinaryOp, Expr, Ident, JSXAttr, JSXAttrName, JSXElement,
      JSXElementChild, JSXElementName, JSXExpr, JSXExprContainer, PatOrExpr, SpreadElement,
    },
    visit::{noop_visit_mut_type, VisitMut, VisitMutWith},
  },
};

use crate::State;

// Modify jsx in place
// And return if current jsx element is immutable and what jsx elements are replaced
pub fn modify_jsx_if_immutable(jsx_ele: &mut JSXElement, state: &mut State) -> bool {
  // <Home/> <some.foo/>
  let mut self_immutable = match &jsx_ele.opening.name {
    JSXElementName::Ident(ident) => {
      ident.sym.chars().next().unwrap().is_lowercase()
        || state.vars.contains(&ident.to_id())
        || state
          .config
          .immutable_globals
          .contains(&ident.sym.to_string())
    }
    JSXElementName::JSXMemberExpr(_) => false,
    JSXElementName::JSXNamespacedName(_) => false,
  };

  // if name is immutable, check more
  let allow_mutable_props = if let JSXElementName::Ident(ident) = &jsx_ele.opening.name {
    state
      .config
      .allow_mutable_props_on_tags
      .contains(&ident.sym.to_string())
  } else {
    false
  };

  if self_immutable {
    let mut immutable_pass = Immutable {
      state,
      allow_mutable_props,
      immutable: true,
    };
    jsx_ele.opening.attrs.visit_mut_with(&mut immutable_pass);
    self_immutable = immutable_pass.immutable;
  }

  // Check if all child jsx is immutable
  // If this jsx is immutable, and children are immutable, do not hoist children, because the whole jsx should be hoist by the caller
  // eg.
  // <div> <a></a> </div>: <a> is immutable, and <div> is immutable, do not hoist <a> inside <div>, only need to hoist div
  // but if
  // <div {...args}> <a></a> </div>: <a> is immutable, but <div> is not, so hoist child, <div {...args}> { _a || (_a = <a></a>) } </div>
  let children_immutable = are_children_immutable(&mut jsx_ele.children, state, !self_immutable);

  self_immutable && children_immutable
}

pub fn are_children_immutable(
  children: &mut [JSXElementChild],
  state: &mut State,
  allow_hoist_child: bool,
) -> bool {
  children.iter_mut().all(|c| {
    let child_immutable = match c {
      // Literal text and fragments are immutable
      JSXElementChild::JSXText(_) => true,
      JSXElementChild::JSXFragment(jsx_frag) => are_children_immutable(&mut jsx_frag.children, state, false),

      // we can allow raw ident access if is declared as const variable
      JSXElementChild::JSXExprContainer(expr_container) => {
        if let JSXExpr::Expr(expr) = &expr_container.expr &&
          let Expr::Ident(ident) = expr.as_ref() {
            state.vars.contains(&ident.to_id())
        } else {
          false
        }
      },
      JSXElementChild::JSXElement(ele) => modify_jsx_if_immutable(ele, state),
      JSXElementChild::JSXSpreadChild(_) => false,
    };

    if allow_hoist_child && child_immutable {
      match c {
        JSXElementChild::JSXElement(ele) => {
          let name = match &ele.opening.name {
            JSXElementName::Ident(id) => format!("_{}", id.sym),
            _ => unreachable!(),
          };

          let id = state.create_id(Some(name));
          state.candidates.push(id.clone());

          *c = JSXElementChild::JSXExprContainer(JSXExprContainer { span: DUMMY_SP, expr: JSXExpr::Expr(Box::new(hoist_result_ast(id, Expr::JSXElement(ele.clone())))) });
        }
        JSXElementChild::JSXFragment(frag) => {
          let id = state.create_id(None);
          state.candidates.push(id.clone());

          *c = JSXElementChild::JSXExprContainer(JSXExprContainer { span: DUMMY_SP, expr: JSXExpr::Expr(Box::new(hoist_result_ast(id, Expr::JSXFragment(frag.clone())))) });
        },
        _ => {}
      };
    }

    child_immutable
  })
}

// $id || ($id = <div><div/>)
pub fn hoist_result_ast(id: Ident, ele: Expr) -> Expr {
  Expr::Bin(BinExpr {
    span: DUMMY_SP,
    op: BinaryOp::LogicalOr,
    left: Box::new(Expr::Ident(id.clone())),
    right: Box::new(Expr::Assign(AssignExpr {
      span: DUMMY_SP,
      op: AssignOp::Assign,
      left: PatOrExpr::Expr(Box::new(Expr::Ident(id))),
      right: Box::new(ele),
    })),
  })
}

// change:
// <div></div>
// to:
// _div || (_div = <div></div>)
pub fn hoist_jsx(state: &mut State, expr: &mut Expr) {
  match &expr {
    Expr::JSXElement(jsx_ele) => {
      let name: String = match &jsx_ele.opening.name {
        JSXElementName::Ident(jsx_ident) => format!("_{}", jsx_ident.sym),
        _ => unreachable!(),
      };

      let id = state.create_id(Some(name));
      state.candidates.push(id.clone());

      *expr = hoist_result_ast(id, Expr::JSXElement(jsx_ele.clone()));
    }
    Expr::JSXFragment(frag) => {
      let id = state.create_id(None);
      state.candidates.push(id.clone());

      *expr = hoist_result_ast(id, Expr::JSXFragment(frag.clone()));
    }
    _ => {}
  }
}

struct Immutable<'a> {
  state: &'a mut State,
  allow_mutable_props: bool,
  immutable: bool,
}

impl<'a> VisitMut for Immutable<'a> {
  noop_visit_mut_type!();

  fn visit_mut_jsx_attr(&mut self, attr: &mut JSXAttr) {
    if let JSXAttrName::Ident(Ident { sym, .. }) = &attr.name && sym == "ref" {
        self.immutable = false
      }

    attr.visit_mut_children_with(self);
  }

  fn visit_mut_spread_element(&mut self, _: &mut SpreadElement) {
    self.immutable = false
  }

  fn visit_mut_expr(&mut self, expr: &mut Expr) {
    if !self.immutable {
      return;
    }

    match expr {
      Expr::Ident(id) => {
        if !self.state.vars.contains(&id.to_id()) {
          self.immutable = false
        }
      }
      Expr::Lit(_) => {}
      Expr::This(_) => {}
      Expr::Paren(paren) => {
        paren.visit_mut_with(self);
      }
      Expr::Bin(bin) => {
        bin.left.visit_mut_with(self);
        bin.right.visit_mut_with(self);
      }
      Expr::Fn(_) | Expr::Arrow(_) | Expr::Object(_) => {
        if !self.allow_mutable_props {
          self.immutable = false;
        }
      }
      // TODO
      // Expr::JSXElement(jsx) => {
      //   self.immutable = modify_jsx_if_immutable(jsx, self.state);
      // },
      // Expr::JSXFragment(frag) => {
      //   self.immutable = are_children_immutable(&mut frag.children, self.state, false);
      // },
      _ => self.immutable = false,
    };
  }
}
