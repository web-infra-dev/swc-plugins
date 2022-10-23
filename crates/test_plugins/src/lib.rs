use std::sync::Arc;

use colored::Colorize;
pub use modern_swc_core::{transform, types::Extensions, types::TransformConfig};

use shared::swc_core::{base::Compiler, cached::regex::CachedRegex, common::SourceMap};

pub use modern_swc_core;
use similar::ChangeTag;

// WIP
pub fn run_test<'a>(
  test_name: &str,
  config: &TransformConfig,
  code: &'a str,
  expected: Option<impl AsRef<str>>,
  ignore: Option<CachedRegex>,
) {
  if ignore.map(|re| re.is_match(test_name)).unwrap_or(false) {
    return;
  }

  let cm = Arc::new(SourceMap::default());

  let res = transform(
    Arc::new(Compiler::new(cm)),
    config,
    "test".into(),
    code,
    None,
  );

  if let Err(e) = res {
    if expected.is_some() {
      println!("-----[{}] Transform failed-----", test_name);
      println!("{}", e);
    }
  } else {
    let expected = expected.expect("Not provide expected code");
    let expected = expected.as_ref();

    let res = res.unwrap();

    let expected: String = expected
      .split('\n')
      .map(|s| s.trim().to_string() + "\n")
      .collect();
    let res: String = res
      .code
      .as_str()
      .split('\n')
      .map(|s| s.trim().to_string() + "\n")
      .collect();
    if res.trim() != expected.trim() {
      println!("[{}] fail\n\n", test_name);
      let diff = similar::TextDiff::from_lines(expected.trim(), res.trim());

      for change in diff.iter_all_changes() {
        let sign = match change.tag() {
          ChangeTag::Delete => format!("-{}", change.value().red()),
          ChangeTag::Insert => format!("+{}", change.value().green()),
          ChangeTag::Equal => format!(" {}", change.value()),
        };
        println!("{}", sign);
      }

      println!();
      panic!("[{}] test failed", test_name);
    }
  }
}
