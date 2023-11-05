use std::collections::hash_map::Entry;

use rustc_hash::{FxHashMap, FxHashSet};
use swc_core::ecma::{
  ast::{Id, ModuleDecl, ModuleItem, Stmt, VarDeclKind},
  usage_analyzer::analyzer::{
    analyze_with_storage,
    storage::{ScopeDataLike, Storage, VarDataLike},
    UsageAnalyzer,
  },
  visit::VisitWith,
};

pub trait StmtLike {
  fn as_stmt(&self) -> Option<&Stmt>;
  fn as_stmt_mut(&mut self) -> Option<&mut Stmt>;

  fn as_module_decl(&self) -> Option<&ModuleDecl>;
  fn as_module_decl_mut(&mut self) -> Option<&mut ModuleDecl>;

  fn from_stmt(stmt: Stmt) -> Self;
}

impl StmtLike for ModuleItem {
  fn as_stmt(&self) -> Option<&Stmt> {
    self.as_stmt()
  }

  fn as_stmt_mut(&mut self) -> Option<&mut Stmt> {
    self.as_mut_stmt()
  }

  fn as_module_decl(&self) -> Option<&ModuleDecl> {
    self.as_module_decl()
  }

  fn as_module_decl_mut(&mut self) -> Option<&mut ModuleDecl> {
    self.as_mut_module_decl()
  }

  fn from_stmt(stmt: Stmt) -> Self {
    Self::Stmt(stmt)
  }
}

impl StmtLike for Stmt {
  fn as_stmt(&self) -> Option<&Stmt> {
    Some(self)
  }

  fn as_stmt_mut(&mut self) -> Option<&mut Stmt> {
    Some(self)
  }

  fn as_module_decl(&self) -> Option<&ModuleDecl> {
    None
  }

  fn as_module_decl_mut(&mut self) -> Option<&mut ModuleDecl> {
    None
  }

  fn from_stmt(stmt: Stmt) -> Self {
    stmt
  }
}

pub fn get_immutable_ids(n: &impl VisitWith<UsageAnalyzer<ImmutableVar>>) -> FxHashSet<Id> {
  let storage = analyze_with_storage(n, None);

  storage
    .vars
    .into_iter()
    .filter_map(|(id, info)| if !info.reassigned { Some(id) } else { None })
    .collect()
}

#[derive(Clone, Default, Debug)]
pub struct DummyScope;

impl ScopeDataLike for DummyScope {
  fn add_declared_symbol(&mut self, _id: &swc_core::ecma::ast::Ident) {}

  fn merge(&mut self, _other: Self, _is_child: bool) {}

  fn mark_used_arguments(&mut self) {}

  fn mark_eval_called(&mut self) {}

  fn mark_with_stmt(&mut self) {}
}

#[derive(Debug)]
pub struct ImmutableVar {
  vars: FxHashMap<Id, VarInfo>,
  scopes: Vec<DummyScope>,
}

impl Default for ImmutableVar {
  fn default() -> Self {
    Self {
      vars: Default::default(),
      scopes: vec![DummyScope],
    }
  }
}

impl Storage for ImmutableVar {
  type ScopeData = DummyScope;

  type VarData = VarInfo;

  fn scope(&mut self, _ctxt: swc_core::common::SyntaxContext) -> &mut Self::ScopeData {
    self.scopes.get_mut(0).unwrap()
  }

  fn top_scope(&mut self) -> &mut Self::ScopeData {
    self.scopes.get_mut(0).unwrap()
  }

  fn var_or_default(&mut self, id: swc_core::ecma::ast::Id) -> &mut Self::VarData {
    self.vars.entry(id).or_default()
  }

  fn merge(&mut self, _kind: swc_core::ecma::usage_analyzer::analyzer::ScopeKind, child: Self) {
    self.vars.reserve(child.vars.len());

    for (id, var_info) in child.vars {
      match self.vars.entry(id) {
        Entry::Occupied(mut info) => {
          let info = info.get_mut();

          info.initialized |= var_info.initialized;
          info.is_fn_param |= var_info.is_fn_param;
          info.assign_count += var_info.assign_count;

          info.reassigned = var_info.reassigned || info.assign_count > 1;
        }
        Entry::Vacant(v) => {
          v.insert(var_info);
        }
      }
    }
  }

  fn declare_decl(
    &mut self,
    ctx: swc_core::ecma::usage_analyzer::analyzer::Ctx,
    i: &swc_core::ecma::ast::Ident,
    has_init: bool,
    kind: Option<VarDeclKind>,
  ) -> &mut Self::VarData {
    let var = self.vars.entry(i.to_id()).or_default();

    var.initialized = has_init || ctx.in_pat_of_param;

    if var.initialized {
      var.assign_count += 1;
    }

    var.kind = kind;

    var
  }

  fn get_initialized_cnt(&self) -> usize {
    self
      .vars
      .values()
      .map(|var| var.initialized)
      .filter(|it| *it)
      .count()
  }

  fn truncate_initialized_cnt(&mut self, _len: usize) {}

  fn mark_property_mutation(&mut self, _id: Id) {}

  fn report_assign(
    &mut self,
    _ctx: swc_core::ecma::usage_analyzer::analyzer::Ctx,
    i: Id,
    _is_op: bool,
  ) {
    let var = self.vars.entry(i).or_default();

    if var.initialized {
      var.reassigned = true;
    }

    var.assign_count += 1;
  }

  fn report_usage(&mut self, _ctx: swc_core::ecma::usage_analyzer::analyzer::Ctx, _i: Id) {}
}

#[derive(Debug, Default)]
pub struct VarInfo {
  is_fn_param: bool,
  initialized: bool,
  reassigned: bool,
  kind: Option<VarDeclKind>,
  assign_count: u32,
}

impl VarDataLike for VarInfo {
  fn mark_declared_as_fn_param(&mut self) {
    self.is_fn_param = true
  }

  fn mark_declared_as_fn_decl(&mut self) {}

  fn mark_declared_as_fn_expr(&mut self) {}

  fn mark_declared_as_for_init(&mut self) {}

  fn mark_has_property_access(&mut self) {}

  fn mark_used_as_callee(&mut self) {}

  fn mark_used_as_arg(&mut self) {}

  fn mark_indexed_with_dynamic_key(&mut self) {}

  fn add_accessed_property(&mut self, _name: swc_core::ecma::atoms::JsWord) {}

  fn mark_used_as_ref(&mut self) {}

  fn add_infects_to(&mut self, _other: swc_core::ecma::usage_analyzer::alias::Access) {}

  fn prevent_inline(&mut self) {}

  fn mark_as_exported(&mut self) {}

  fn mark_initialized_with_safe_value(&mut self) {
    self.initialized = true;
  }

  fn mark_as_pure_fn(&mut self) {}

  fn mark_used_above_decl(&mut self) {}

  fn mark_used_recursively(&mut self) {}
}
