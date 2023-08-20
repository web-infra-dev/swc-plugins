// The following code is modified based on
// https://github.com/swc-project/plugins/tree/main/packages/constify/transform/src/lib.rs.
// As we need this plugin not enable ecma_plugin_transform feature of swc_core
//
// Copyright (c) 2021 kdy1(Donny/강동윤)
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use swc_core::ecma::ast::{
  Decl, DefaultDecl, Id, ImportSpecifier, ModuleDecl, Pat, Stmt, VarDeclKind,
};

use crate::utils::StmtLike;

pub trait ConstDecls {
  fn const_declared_by_item(&self) -> Vec<Id>;
}

// impl ConstDecls for Stmt {
//   fn const_declared_by_item(&self) -> Vec<Id> {
//     match self {
//       Stmt::Decl(s) => s.const_declared_by_item(),
//       _ => Default::default(),
//     }
//   }
// }

impl ConstDecls for Decl {
  fn const_declared_by_item(&self) -> Vec<Id> {
    match self {
      Decl::Class(s) => {
        vec![s.ident.to_id()]
      }
      Decl::Fn(s) => {
        vec![s.ident.to_id()]
      }
      Decl::Var(s) => s
        .decls
        .iter()
        .flat_map(|decl| match &decl.name {
          Pat::Ident(id) => vec![id.id.to_id()],
          Pat::Array(array_pat) => array_pat
            .elems
            .iter()
            .filter_map(|ele| {
              ele.as_ref().and_then(|ele| match ele {
                Pat::Ident(id) => Some(id.id.to_id()),
                _ => None,
              })
            })
            .collect(),
          _ => vec![],
        })
        .collect(),

      _ => Default::default(),
    }
  }
}

impl ConstDecls for ModuleDecl {
  fn const_declared_by_item(&self) -> Vec<Id> {
    match self {
      ModuleDecl::Import(s) => {
        let mut buf = vec![];

        for s in s.specifiers.iter() {
          match s {
            ImportSpecifier::Named(s) => {
              buf.push(s.local.to_id());
            }
            ImportSpecifier::Default(s) => {
              buf.push(s.local.to_id());
            }
            ImportSpecifier::Namespace(s) => {
              buf.push(s.local.to_id());
            }
          }
        }

        buf
      }
      ModuleDecl::ExportDecl(s) => s.decl.const_declared_by_item(),
      ModuleDecl::ExportDefaultDecl(s) => match &s.decl {
        DefaultDecl::Class(d) => d.ident.iter().map(|i| i.to_id()).collect(),
        DefaultDecl::Fn(d) => d.ident.iter().map(|i| i.to_id()).collect(),

        _ => Default::default(),
      },
      _ => Default::default(),
    }
  }
}

// impl ConstDecls for ModuleItem {
//   fn const_declared_by_item(&self) -> Vec<Id> {
//     match self {
//       ModuleItem::ModuleDecl(s) => s.const_declared_by_item(),
//       ModuleItem::Stmt(s) => s.const_declared_by_item(),
//     }
//   }
// }

impl<T> ConstDecls for T
where
  T: StmtLike,
{
  fn const_declared_by_item(&self) -> Vec<Id> {
    if let Some(module_decl) = self.as_module_decl() {
      module_decl.const_declared_by_item()
    } else if let Some(stmt) = self.as_stmt() {
      match stmt {
        Stmt::Decl(s) => s.const_declared_by_item(),
        _ => Default::default(),
      }
    } else {
      Default::default()
    }
  }
}
