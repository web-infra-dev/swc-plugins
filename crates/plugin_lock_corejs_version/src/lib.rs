use shared::{
  swc_atoms::JsWord,
  swc_common::DUMMY_SP,
  swc_ecma_ast::{CallExpr, Callee, ImportDecl, Lit, Str},
  swc_ecma_visit::{as_folder, Fold, VisitMut},
};

pub struct LockCoreJsVersion {
  corejs_path: String,
}

static COREJS: &str = "core-js";

impl VisitMut for LockCoreJsVersion {
  fn visit_mut_import_decl(&mut self, decl: &mut ImportDecl) {
    if decl.src.value.contains(COREJS) {
      let locked_src = change_specifier(&decl.src.value, &self.corejs_path);
      decl.src = Str {
        span: DUMMY_SP,
        value: JsWord::from(locked_src.as_str()),
        raw: None,
      }
    }
  }

  fn visit_mut_call_expr(&mut self, call_expr: &mut CallExpr) {
    match &call_expr.callee {
      Callee::Import(_) => {
        // import('core-js')
        if let Some(Lit::Str(specifier)) = call_expr.args[0].expr.as_mut_lit() {
          if specifier.value.contains(COREJS) {
            let locked_path = change_specifier(&specifier.value, &self.corejs_path);

            *specifier = Str {
              span: DUMMY_SP,
              value: JsWord::from(locked_path.as_str()),
              raw: None,
            }
          }
        }
      }
      Callee::Expr(expr) => {
        if let Some(id) = expr.as_ident() {
          if &id.sym != "require" {
            return;
          }

          // require('core-js')
          if let Some(Lit::Str(specifier)) = call_expr.args[0].expr.as_mut_lit() {
            if specifier.value.contains(COREJS) {
              let locked_path = change_specifier(&specifier.value, &self.corejs_path);
              specifier.value = JsWord::from(locked_path.as_str());
              specifier.span = DUMMY_SP;
            }
          }
        }
      }
      _ => {}
    }
  }
}

pub fn change_specifier(input: &str, corejs_path: &str) -> String {
  input.replace(COREJS, corejs_path)
}

pub fn lock_corejs_version(corejs_path: String) -> impl Fold {
  as_folder(LockCoreJsVersion { corejs_path })
}

#[cfg(test)]
mod test {
  use shared::swc_ecma_transforms_testing::test_transform;

  use crate::lock_corejs_version;

  #[test]
  fn test() {
    test_transform(
      Default::default(),
      |_| lock_corejs_version("node_modules/core-js".into()),
      r#"import "core-js";"#,
      r#"import "node_modules/core-js";"#,
      true,
    )
  }
}
