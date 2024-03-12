#![feature(let_chains)]
use swc_core::ecma::{
  ast::{CallExpr, Expr, ExprStmt, MemberProp, Module, ModuleItem, Stmt},
  visit::{as_folder, Fold, VisitMut},
};

pub struct RemoveEsModuleMark {}

impl VisitMut for RemoveEsModuleMark {
  fn visit_mut_module(&mut self, module: &mut Module) {
    let mut rm_index = None;
    for (idx, item) in module.body.iter().enumerate() {
      if is_es_module_mark(item) {
        rm_index = Some(idx);
      }
    }

    if let Some(idx) = rm_index {
      module.body.remove(idx);
    }
  }
}

fn is_es_module_mark(item: &ModuleItem) -> bool {
  if let ModuleItem::Stmt(Stmt::Expr(ExprStmt { expr, .. })) = item
    && let Expr::Call(CallExpr { callee, .. }) = &**expr
    && let Some(callee) = callee.as_expr()
    && let Expr::Member(member_expr) = &**callee
    && let Expr::Ident(obj) = &*member_expr.obj
    && let MemberProp::Ident(prop) = &member_expr.prop
    && &obj.sym == "Object"
    && &prop.sym == "defineProperty"
  {
    true
  } else {
    false
  }
}

pub fn remove_es_module_mark() -> impl Fold {
  as_folder(RemoveEsModuleMark {})
}

#[cfg(test)]
mod test {
  use swc_core::{self, quote};

  use crate::is_es_module_mark;

  #[test]
  fn t() {
    assert!(is_es_module_mark(&swc_core::ecma::ast::ModuleItem::Stmt(
      quote!(r#"Object.defineProperty(exports, "__esModule", { value: true });"# as Stmt)
    )));
  }
}
