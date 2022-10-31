use std::sync::Arc;

use colored::Colorize;

use shared::swc_core::{base::Compiler, cached::regex::CachedRegex, common::SourceMap};

use similar::ChangeTag;
pub use swc_plugins_core;

// WIP
pub fn run_test<'a>(
  test_name: &str,
  config: &swc_plugins_core::types::TransformConfig,
  code: &'a str,
  expected: Option<impl AsRef<str>>,
  ignore: Option<CachedRegex>,
) {
  if ignore.map(|re| re.is_match(test_name)).unwrap_or(false) {
    return;
  }

  let cm = Arc::new(SourceMap::default());

  let res = swc_plugins_core::transform(
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
      .filter(|s| s != "\n" && s != ";\n")
      .collect();
    let res: String = res
      .code
      .as_str()
      .split('\n')
      .map(|s| s.trim().to_string() + "\n")
      .filter(|s| s != "\n" && s != ";\n")
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
