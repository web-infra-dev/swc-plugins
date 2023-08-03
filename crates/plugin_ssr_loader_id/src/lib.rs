#![feature(let_chains)]
#![allow(clippy::arc_with_non_send_sync)]
use std::sync::Arc;

use serde::Deserialize;
use swc_core::{
  common::DUMMY_SP,
  ecma::{
    ast::{
      CallExpr, Expr, FnExpr, Id, ImportDecl, ImportSpecifier, KeyValueProp, Lit, PropName, Str,
    },
    atoms::JsWord,
    visit::{as_folder, noop_visit_mut_type, Fold, VisitMut, VisitMutWith},
  },
  quote,
};
use swc_plugins_utils::PluginContext;

#[derive(Debug, Deserialize, Default, Clone)]
pub struct SSRLoaderIdConfig {
  pub runtime_package_name: String,
  pub function_use_loader_name: Option<String>,
  pub function_use_static_loader_name: Option<String>,
  pub function_create_container_name: Option<String>,
}

pub struct PluginSSRLoaderId {
  config: SSRLoaderIdConfig,
  state: SSRLoaderIdState,
}

impl<'a> PluginSSRLoaderId {
  pub fn new(filename: String, cwd: &'a str, config: SSRLoaderIdConfig) -> Self {
    let hash = {
      let filename = if let Some(name) = filename.strip_prefix(cwd) {
        name
      } else {
        filename.as_str()
      };
      md5::compute(filename)
    };

    Self {
      config,
      state: SSRLoaderIdState {
        idx: 0,
        hash,
        use_loader: None,
        create_container: None,
        use_static_loader: None,
      },
    }
  }

  fn generate_id(&mut self) -> String {
    format!("{:?}_{}", self.state.hash, {
      let number = self.state.idx;
      self.state.idx += 1;
      number
    })
  }

  fn create_loader_expression(&mut self, loader: &Expr) -> Expr {
    quote!(
      "(function() {
        var innerLoader = $loader;
        innerLoader.id = $id;
        return innerLoader;
      })()" as Expr,
      loader: Expr = loader.clone(),
      id: Expr = Expr::Lit(Lit::Str(Str {
        span: DUMMY_SP,
        value: JsWord::from(self.generate_id()),
        raw: None
      })),
    )
  }

  fn check_is_duplicate_inner_loader(&self, loader: &Expr) -> Option<()> {
    let func_body = &loader.as_fn_expr()?.function.body;
    match func_body {
      Some(body) => {
        let inner_loader_decl = body.stmts.get(0)?.as_decl()?.as_var()?;
        let decl_ident = &inner_loader_decl.decls.get(0)?.name.as_ident()?.id;

        let inner_loader_return = body.stmts.get(2)?.as_return_stmt()?;
        let return_ident = inner_loader_return.arg.as_ref()?.as_ident()?;

        if decl_ident.sym.eq("innerLoader") && return_ident.sym.eq("innerLoader") {
          return Some(());
        }
        None
      }
      None => None,
    }
  }

  fn modify_loader_call_expr(
    &mut self,
    call_expr: &mut CallExpr,
    import_func_name: &Option<Id>,
  ) -> Option<()> {
    let Some(func_name) = import_func_name else {
      return None;
    };
    let loader_name = func_name.0.to_lowercase();

    let callee = call_expr
      .callee
      .as_expr()?
      .as_ident()
      .map(|ident| ident.to_id())?;
    if !callee.eq(func_name) {
      return None;
    }

    let arg0 = call_expr.args.get_mut(0)?;

    if arg0.expr.is_fn_expr()
      || arg0.expr.is_call()
      || arg0.expr.is_ident()
      || arg0.expr.is_member()
      || arg0.expr.is_arrow()
    {
      if self.check_is_duplicate_inner_loader(&arg0.expr).is_none() {
        *arg0.expr = self.create_loader_expression(&arg0.expr);
      }
      return Some(());
    }

    println!("{loader_name} 中 loaderId 生成失败，请检查 {loader_name}");
    panic!("please check the usage of {callee:?}");
  }

  fn modify_create_container_call_expr(
    &mut self,
    call_expr: &mut CallExpr,
    import_func_name: &Option<Id>,
  ) -> Option<()> {
    use swc_core::ecma::ast::Prop;

    let Some(func_name) = import_func_name else {
      return None;
    };

    let callee = call_expr
      .callee
      .as_expr()?
      .as_ident()
      .map(|ident| ident.to_id())?;
    if !callee.eq(func_name) {
      return None;
    }

    let arg0_props = call_expr
      .args
      .get_mut(0)?
      .expr
      .as_mut_object()?
      .props
      .as_mut_slice();

    fn get_name_from_prop_key(prop: &PropName) -> Option<&str> {
      prop
        .as_ident()
        .map(|ident| ident.sym.as_ref())
        .or_else(|| prop.as_str().map(|str| str.value.as_ref()))
    }

    const LOADER_SLICE: [&str; 2] = ["loader", "staticLoader"];
    arg0_props
      .iter_mut()
      .filter_map(|prop| prop.as_mut_prop())
      .for_each(|prop| {
        let prop = prop.as_mut();
        match prop {
          Prop::Method(method) => {
            let name = get_name_from_prop_key(&method.key);
            let func_expr = &Expr::Fn(FnExpr {
              ident: None,
              function: method.function.clone(),
            });
            if let Some(name) = name &&
              LOADER_SLICE.contains(&name) &&
              self.check_is_duplicate_inner_loader(func_expr).is_none()
                    {
                        let key_value_prop = Prop::KeyValue(KeyValueProp {
                            key: method.key.clone(),
                            value: Box::new(self.create_loader_expression(func_expr)),
                        });
                        *prop = key_value_prop;
                    }
          }
          Prop::KeyValue(key_value) => {
            let name = get_name_from_prop_key(&key_value.key);

            if  let Some(name) = name &&
                    LOADER_SLICE.contains(&name) &&
                    self.check_is_duplicate_inner_loader(&key_value.value).is_none()
                    {
                        *key_value.value = self.create_loader_expression(&key_value.value);
                    }
          }
          _ => (),
        }
      });

    Some(())
  }
}

impl VisitMut for PluginSSRLoaderId {
  noop_visit_mut_type!();

  fn visit_mut_import_decl(&mut self, import_decl: &mut ImportDecl) {
    use swc_core::ecma::ast::ModuleExportName;
    let config = &self.config;
    let state = &mut self.state;
    if import_decl.src.value != config.runtime_package_name {
      return;
    }

    let has_use_loader = if config.function_use_loader_name.is_some() {
      state.use_loader.is_some()
    } else {
      true
    };
    let has_use_static_loader = if config.function_use_static_loader_name.is_some() {
      state.use_static_loader.is_some()
    } else {
      true
    };
    let has_create_container = if config.function_create_container_name.is_some() {
      state.create_container.is_some()
    } else {
      true
    };

    if has_use_loader && has_use_static_loader && has_create_container {
      return;
    }

    import_decl.specifiers.iter().for_each(|specifier| {
            let ImportSpecifier::Named(imported_spec) = specifier else {
                return;
            };

            let local_name = &imported_spec.local;
            let import_name = imported_spec
                .imported
                .as_ref()
                .and_then(|module_decl| match module_decl {
                    ModuleExportName::Ident(id) => Some(id),
                    ModuleExportName::Str(_) => None,
                })
                .unwrap_or(local_name);
            if let Some(function_use_loader_name) = &config.function_use_loader_name &&
                   import_name.sym.eq(function_use_loader_name) &&
                   state.use_loader.is_none() { state.use_loader = Some(local_name.to_id()); }

            if let Some(function_use_static_loader_name) = &config.function_use_static_loader_name &&
                   import_name.sym.eq(function_use_static_loader_name) &&
                   state.use_static_loader.is_none() { state.use_static_loader = Some(local_name.to_id()); }

            if let Some(function_create_container_name) = &config.function_create_container_name &&
                   import_name.sym.eq(function_create_container_name) &&
                   state.create_container.is_none() { state.create_container = Some(local_name.to_id()); }
    })
  }

  fn visit_mut_call_expr(&mut self, call_expr: &mut CallExpr) {
    // FIXME: clone the Option<Id>, because Rust: cannot borrow `*self` as mutable because it is also borrowed as immutable
    let state = &self.state;
    let (use_loader, use_static_loader, create_container) = (
      state.use_loader.clone(),
      state.use_static_loader.clone(),
      state.create_container.clone(),
    );

    self.modify_loader_call_expr(call_expr, &use_loader);
    self.modify_loader_call_expr(call_expr, &use_static_loader);
    self.modify_create_container_call_expr(call_expr, &create_container);

    call_expr.visit_mut_children_with(self);
  }
}

struct SSRLoaderIdState {
  idx: usize,
  hash: md5::Digest,
  use_loader: Option<Id>,
  create_container: Option<Id>,
  use_static_loader: Option<Id>,
}

pub fn plugin_ssr_loader_id(
  config: &SSRLoaderIdConfig,
  plugin_ctx: Arc<PluginContext>,
) -> impl Fold + VisitMut {
  let (filename, cwd) = (
    plugin_ctx.filename.clone(),
    plugin_ctx
      .cwd
      .as_os_str()
      .to_str()
      .expect("Cannot get plugin cwd root path"),
  );

  as_folder(PluginSSRLoaderId::new(filename, cwd, config.clone()))
}

#[cfg(test)]
mod test {
  use std::{path::PathBuf, sync::Arc};

  use swc_core::{
    common::{comments::SingleThreadedComments, FileName, Mark, SourceMap},
    ecma::parser::Syntax,
  };
  use swc_plugins_utils::PluginContext;

  use super::plugin_ssr_loader_id;

  #[test]
  fn test_plugin() {
    let cm = Arc::new(SourceMap::default());
    test_plugins::testing::test_transform(
      Syntax::Es(Default::default()),
      |_| {
        plugin_ssr_loader_id(
          &crate::SSRLoaderIdConfig {
            runtime_package_name: "@modern-js/runtime".to_string(),
            function_use_loader_name: Some("useLoader".to_string()),
            function_use_static_loader_name: None,
            function_create_container_name: None,
          },
          Arc::new(PluginContext {
            cm: cm.clone(),
            file: cm.new_source_file(FileName::Anon, "".into()),
            top_level_mark: Mark::new(),
            unresolved_mark: Mark::new(),
            comments: SingleThreadedComments::default(),
            filename: "/root/a.js".into(),
            cwd: PathBuf::from("/root"),
            config_hash: None,
          }),
        )
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
