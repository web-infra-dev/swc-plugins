use swc_core::{ecma::ast::{ModuleDecl, ModuleItem, Stmt}, css::visit::VisitMut};

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
