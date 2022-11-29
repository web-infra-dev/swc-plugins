use std::{
  fmt::Display,
  fs,
  path::{Path, PathBuf},
  sync::Arc,
};

use colored::Colorize;

use shared::swc_core::{base::Compiler, cached::regex::CachedRegex, common::SourceMap};

use similar::ChangeTag;
pub use swc_plugins_core;

pub struct ExpectedInfo {
  name: String,
  content: String,
  config: Option<swc_plugins_core::types::TransformConfig>,
}

impl ExpectedInfo {
  pub fn new(
    name: String,
    content: String,
    config: Option<swc_plugins_core::types::TransformConfig>,
  ) -> Self {
    Self {
      name,
      content,
      config,
    }
  }
}

pub struct FixtureTester<H>
where
  H: FixtureTesterHook,
{
  pub config: swc_plugins_core::types::TransformConfig,
  pub hooks: H,
  pub ignore: Vec<CachedRegex>,
  pub config_hash: Option<String>,
}

pub trait FixtureTesterHook {
  fn on_resolve_actual_file(
    &mut self,
    fixture_path: &Path,
    _config: &mut swc_plugins_core::types::TransformConfig,
  ) -> String {
    String::from_utf8(fs::read(&fixture_path.join("actual.js")).unwrap()).unwrap()
  }

  /// Read all assets
  fn on_resolve_expected_files(
    &mut self,
    fixture_path: &Path,
    _config: &mut swc_plugins_core::types::TransformConfig,
  ) -> Vec<ExpectedInfo> {
    let expected_path = fixture_path.join("expected.js");
    let expected = fs::read(&expected_path).unwrap();

    let option_path = fixture_path.join("option.json");
    let option = option_path
      .exists()
      .then(|| shared::serde_json::from_slice(fs::read(option_path).unwrap().as_slice()).unwrap());

    vec![ExpectedInfo::new(
      expected_path.to_string_lossy().to_string(),
      String::from_utf8(expected).unwrap(),
      option,
    )]
  }

  fn on_before_compare(
    &mut self,
    _actual: &mut PathBuf,
    _expected: &mut PathBuf,
    _config: &mut swc_plugins_core::types::TransformConfig,
  ) {
  }
}

pub struct BaseFixtureHook;
impl FixtureTesterHook for BaseFixtureHook {}

impl<H> FixtureTester<H>
where
  H: FixtureTesterHook,
{
  /// You can pass default hook, `BaseFixtureHook`
  pub fn new(
    config: swc_plugins_core::types::TransformConfig,
    hooks: H,
    ignore: Vec<CachedRegex>,
    config_hash: Option<String>,
  ) -> Self {
    Self {
      config,
      hooks,
      ignore,
      config_hash,
    }
  }

  pub fn fixtures(&mut self, fixture_path: &Path) {
    let fixtures = fs::read_dir(fixture_path).unwrap();
    for fixture in fixtures {
      let fixture = fixture.unwrap().path();

      self.internal_run_fixture_tests(&fixture);
    }
  }

  fn internal_run_fixture_tests(&mut self, path: &Path) {
    let tests = fs::read_dir(path).unwrap();

    for test_dir in tests {
      let test_dir = test_dir.unwrap().path();

      if self
        .ignore
        .iter()
        .any(|re| re.is_match(&test_dir.to_string_lossy()))
      {
        continue;
      }

      let actual_content = self
        .hooks
        .on_resolve_actual_file(&test_dir, &mut self.config);

      let expected_files = self
        .hooks
        .on_resolve_expected_files(&test_dir, &mut self.config);

      for expected_info in expected_files {
        if let Some(opt) = &expected_info.config {
          self.run_test(
            &expected_info.name,
            opt,
            &actual_content,
            Some(&expected_info.content),
          )
        } else {
          self.run_test(
            &expected_info.name,
            &self.config,
            &actual_content,
            Some(&expected_info.content),
          )
        }
      }
    }
  }

  pub fn run_test<'a, D: Display>(
    &self,
    test_name: D,
    config: &swc_plugins_core::types::TransformConfig,
    code: &'a str,
    expected: Option<impl AsRef<str>>,
  ) {
    let test_name = test_name.to_string();
    let cm = Arc::new(SourceMap::default());

    let res = swc_plugins_core::transform(
      Arc::new(Compiler::new(cm)),
      config,
      "test".into(),
      code,
      None,
      self.config_hash.clone(),
    );

    if let Err(e) = res {
      if expected.is_some() {
        println!("-----[{}] Transform failed-----", test_name.red());
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
        println!("[{}] test failed", test_name.red());
        let diff = similar::TextDiff::from_lines(expected.trim(), res.trim());

        for change in diff.iter_all_changes() {
          let sign = match change.tag() {
            ChangeTag::Delete => format!("-{}", change.value().red()),
            ChangeTag::Insert => format!("+{}", change.value().green()),
            ChangeTag::Equal => format!(" {}", change.value()),
          };
          print!("{}", sign);
        }

        panic!();
      } else {
        println!("{}{}", "Passed: ".green(), test_name);
      }
    }
  }
}
