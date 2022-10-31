use std::collections::HashSet;

use shared::swc_core::{
  common::util::take::Take,
  ecma::{
    ast::{BlockStmt, Expr, Id, ImportSpecifier, Module, ModuleDecl, ModuleExportName, ModuleItem},
    visit::{as_folder, Fold, VisitMut, VisitMutWith},
  },
};

use shared::utils::ClearMark;

struct RmUseEffect {
  use_effect_mark: HashSet<Id>, // used for remove useEffect()
  react_mark: HashSet<Id>,      // used for remove React.useEffect()
}

const USE_EFFECT_STR: &str = "useEffect";

impl VisitMut for RmUseEffect {
  fn visit_mut_block_stmt(&mut self, n: &mut BlockStmt) {
    let mut rm_idx_set = HashSet::new();
    for (idx, stmt) in n.stmts.iter().enumerate() {
      if let Some(Some(call_expr)) = stmt.as_expr().map(|expr_stmt| expr_stmt.expr.as_call()) {
        if let Some(callee) = call_expr.callee.as_expr() {
          match &**callee {
            Expr::Member(member) => {
              // check React.useEffect call
              if self.react_mark.is_empty() {
                continue;
              }

              if let Some(obj_ident) = member.obj.as_ident() {
                if self.react_mark.contains(&obj_ident.to_id()) {
                  if let Some(method_ident) = member.prop.as_ident() {
                    if &method_ident.sym == USE_EFFECT_STR {
                      rm_idx_set.insert(idx);
                    }
                  }
                }
              }
            }
            Expr::Ident(ident) => {
              // check useEffect call
              if self.use_effect_mark.contains(&ident.to_id()) {
                rm_idx_set.insert(idx);
              }
            }
            _ => {}
          };
        }
      }
    }
    if !rm_idx_set.is_empty() {
      n.stmts = n
        .stmts
        .take()
        .into_iter()
        .enumerate()
        .filter_map(|(idx, stmt)| {
          if rm_idx_set.contains(&idx) {
            None
          } else {
            Some(stmt)
          }
        })
        .collect();
    }
    n.stmts.visit_mut_with(self);
  }
}

pub fn remove_effect() -> impl Fold + VisitMut {
  as_folder(RmCall {})
}

struct RmCall {}

impl VisitMut for RmCall {
  fn visit_mut_module(&mut self, module: &mut Module) {
    let mut visitor = RmUseEffect {
      react_mark: HashSet::new(),
      use_effect_mark: HashSet::new(),
    };
    for item in &module.body {
      if let ModuleItem::ModuleDecl(ModuleDecl::Import(var)) = item {
        let source = &*var.src.value;
        if source == "react" {
          for specifier in &var.specifiers {
            match specifier {
              ImportSpecifier::Named(named) => {
                match &named.imported {
                  Some(imported) => {
                    let imported_name = match imported {
                      ModuleExportName::Ident(ident) => &ident.sym,
                      ModuleExportName::Str(str) => &str.value,
                    };
                    if imported_name.as_ref() == USE_EFFECT_STR {
                      // import { useEffect as ??? } from 'react'
                      visitor.use_effect_mark.insert(named.local.to_id());
                    }
                  }
                  None => {
                    if named.local.sym.as_ref() == USE_EFFECT_STR {
                      // import { useEffect } from 'react'
                      visitor.use_effect_mark.insert(named.local.to_id());
                    }
                  }
                }
              }
              ImportSpecifier::Default(default) => {
                // import ??? from 'react'
                visitor.react_mark.insert(default.local.to_id());
              }
              ImportSpecifier::Namespace(default) => {
                // import * as ??? from 'react'
                visitor.react_mark.insert(default.local.to_id());
              }
            }
          }
        }
      }
    }

    module.visit_mut_with(&mut visitor);

    #[cfg(not(target_arch = "wasm32"))]
    module.visit_mut_with(&mut ClearMark);
  }
}
