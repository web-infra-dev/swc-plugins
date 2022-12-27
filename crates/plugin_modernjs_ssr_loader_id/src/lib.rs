use shared::{
  swc_core::{
    self,
    common::DUMMY_SP,
    ecma::{
      ast::{CallExpr, Expr, Id, ImportDecl, ImportSpecifier, Lit, Module, ModuleExportName, Str},
      atoms::JsWord,
      visit::{as_folder, Fold, Visit, VisitMut, VisitMutWith, VisitWith},
    },
    quote,
  },
  PluginContext,
};
use std::sync::Arc;

const RUNTIME_PACKAGE_NAME: &str = "@modern-js/runtime";
const USE_LOADER: &str = "useLoader";

pub struct PluginModernJsSSRLoaderId {
  plugin_ctx: Arc<PluginContext>,
  hash: Option<md5::Digest>,
  use_loader_local: Option<Id>,
  idx: usize,
}

impl PluginModernJsSSRLoaderId {
  fn reset_state(&mut self) {
    self.hash = None;
    self.use_loader_local = None;
    self.idx = 0;
  }

  fn get_hash(&mut self) -> md5::Digest {
    if self.hash.is_none() {
      let cwd = self.plugin_ctx.cwd.to_str().unwrap();
      let filename = if self.plugin_ctx.filename.starts_with(cwd) {
        &self.plugin_ctx.filename[cwd.len()..]
      } else {
        &self.plugin_ctx.filename
      };

      self.hash = Some(md5::compute(filename));
    }

    self.hash.unwrap()
  }

  fn get_id(&mut self) -> String {
    let idx = self.idx;
    self.idx += 1;

    format!("{:x}_{}", self.get_hash(), idx)
  }

  fn get_self_invoke_expr(&mut self, loader: &Expr) -> Expr {
    quote!(
      "(function() {
        var innerLoader = $loader;
        innerLoader.id = $id;
        return innerLoader;
      })()" as Expr,
      loader: Expr = loader.clone(),
      id: Expr = Expr::Lit(Lit::Str(Str {
        span: DUMMY_SP,
        value: JsWord::from(self.get_id()),
        raw: None
      })),
    )
  }
}

impl VisitMut for PluginModernJsSSRLoaderId {
  fn visit_mut_module(&mut self, module: &mut Module) {
    self.reset_state();

    let mut use_loader_analyzer = UseLoaderAnalyzer(None);
    module.visit_with(&mut use_loader_analyzer);

    if let Some(use_loader_local) = use_loader_analyzer.0 {
      self.hash = Some(self.get_hash());
      self.use_loader_local = Some(use_loader_local);
      module.visit_mut_children_with(self);
    }
  }

  fn visit_mut_call_expr(&mut self, call_expr: &mut CallExpr) {
    // check if useLoader()
    if let Some(use_loader_local) = &mut self.use_loader_local &&
    let Some(callee) = call_expr.callee.as_mut_expr() &&
    let Some(callee) = callee.as_mut_ident() &&
    &callee.to_id() == use_loader_local &&
    let Some(arg) = call_expr.args.get_mut(0) {
      *arg.expr = self.get_self_invoke_expr(&arg.expr);
    }
  }
}

struct UseLoaderAnalyzer(Option<Id>);

impl Visit for UseLoaderAnalyzer {
  fn visit_import_decl(&mut self, import_decl: &ImportDecl) {
    if &import_decl.src.value != RUNTIME_PACKAGE_NAME {
      return;
    }

    for specifier in &import_decl.specifiers {
      if let ImportSpecifier::Named(ref named_specifier) = specifier {
        let imported = if let Some(ref imported) = named_specifier.imported {
          match imported {
            ModuleExportName::Ident(id) => Some(id),
            _ => None,
          }
        } else {
          Some(&named_specifier.local)
        };

        if let Some(imported) = imported && &imported.sym == USE_LOADER {
          self.0 = Some(named_specifier.local.to_id());
        }
      }
    }
  }
}

pub fn plugin_modernjs_ssr_loader_id(plugin_ctx: Arc<PluginContext>) -> impl Fold {
  as_folder(PluginModernJsSSRLoaderId {
    plugin_ctx,
    hash: None,
    use_loader_local: None,
    idx: 0,
  })
}

#[cfg(test)]
mod test {
  use std::{path::PathBuf, sync::Arc};

  use shared::{
    swc_core::{
      common::{comments::SingleThreadedComments, Mark, SourceMap},
      ecma::parser::Syntax,
    },
    PluginContext,
  };

  use crate::plugin_modernjs_ssr_loader_id;

  #[test]
  fn test() {
    let cm = Arc::new(SourceMap::default());
    shared::swc_ecma_transforms_testing::test_transform(
      Syntax::Es(Default::default()),
      |_| {
        plugin_modernjs_ssr_loader_id(Arc::new(PluginContext {
          cm,
          top_level_mark: Mark::new(),
          unresolved_mark: Mark::new(),
          comments: SingleThreadedComments::default(),
          filename: "/root/a.js".into(),
          cwd: PathBuf::from("/root"),
          config_hash: None,
          is_source_esm: true,
        }))
      },
      "import { useLoader } from '@modern-js/runtime';useLoader(foo);useLoader(bar)",
      "import { useLoader } from '@modern-js/runtime';
      useLoader(function(){
        var innerLoader = foo;
        innerLoader.id = \"29e70e1822232ad34a331c74d9588977_0\";
        return innerLoader;
      }());
      useLoader(function() {
        var innerLoader = bar;
        innerLoader.id = \"29e70e1822232ad34a331c74d9588977_1\";
        return innerLoader;
    }());",
      true,
    );
  }
}
