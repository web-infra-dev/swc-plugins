use std::sync::Arc;

use colored::Colorize;
pub use modern_swc_core::{transform, types::Extensions, types::TransformConfig};

use shared::swc_core::{base::Compiler, common::SourceMap};

pub use modern_swc_core;
use similar::ChangeTag;

// WIP
pub fn run_test(test_name: &str, config: &TransformConfig, code: &str, expected: &str) {
  let cm = Arc::new(SourceMap::default());

  let res = transform(
    Arc::new(Compiler::new(cm)),
    config,
    "test".into(),
    code,
    None,
  );

  if let Err(e) = res {
    println!("-----[{}] Transform failed-----", test_name);
    println!("{}", e);
  } else {
    let res = res.unwrap();
    if res.code.as_str().trim() != expected.trim() {
      println!("[{}] fail\n\n", test_name);
      let diff = similar::TextDiff::from_lines(expected.trim(), res.code.as_str().trim());

      for change in diff.iter_all_changes() {
        let sign = match change.tag() {
          ChangeTag::Delete => format!("-{}", change.value().red()),
          ChangeTag::Insert => format!("+{}", change.value().green()),
          ChangeTag::Equal => format!(" {}", change.value()),
        };
        print!("{}", sign);
      }

      println!();
      panic!("[{}] test failed", test_name);
    }
  }
}
