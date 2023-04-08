use std::{
  fmt::Display,
  fs,
  path::{Path, PathBuf},
  sync::Arc,
};

use colored::Colorize;
use swc_core::{base::Compiler, cached::regex::CachedRegex, common::SourceMap};

use crate::utils::show_diff;

pub struct ExpectedInfo {
  name: String,
  content: String,
  config: Option<swc_plugins_collection::types::TransformConfig>,
}

impl ExpectedInfo {
  pub fn new(
    name: String,
    content: String,
    config: Option<swc_plugins_collection::types::TransformConfig>,
  ) -> Self {
    Self {
      name,
      content,
      config,
    }
  }
}

#[derive(Debug, Default)]
pub struct FixtureTester<H>
where
  H: FixtureTesterHook,
{
  pub config: swc_plugins_collection::types::TransformConfig,
  pub hooks: H,
  pub ignore: Vec<CachedRegex>,
  pub config_hash: Option<String>,
}

pub trait FixtureTesterHook {
  fn on_before_resolve(
    &mut self,
    _fixture_path: &Path,
    _config: &mut swc_plugins_collection::types::TransformConfig,
  ) {
  }

  fn on_resolve_actual_file(
    &mut self,
    fixture_path: &Path,
    _config: &mut swc_plugins_collection::types::TransformConfig,
  ) -> String {
    String::from_utf8(fs::read(fixture_path.join("actual.js")).unwrap()).unwrap()
  }

  /// Read all assets
  fn on_resolve_expected_files(
    &mut self,
    fixture_path: &Path,
    _config: &mut swc_plugins_collection::types::TransformConfig,
  ) -> Vec<ExpectedInfo> {
    let expected_path = fixture_path.join("expected.js");
    let expected = fs::read(&expected_path).unwrap();

    let option_path = fixture_path.join("option.json");
    let option = option_path
      .exists()
      .then(|| serde_json::from_slice(fs::read(option_path).unwrap().as_slice()).unwrap());

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
    _config: &mut swc_plugins_collection::types::TransformConfig,
  ) {
  }
}

pub struct BaseFixtureHook;
impl FixtureTesterHook for BaseFixtureHook {}

impl<T> FixtureTesterHook for Option<T>
where
  T: FixtureTesterHook,
{
  fn on_before_resolve(
    &mut self,
    fixture_path: &Path,
    config: &mut swc_plugins_collection::types::TransformConfig,
  ) {
    match self {
      Some(sub) => sub.on_before_resolve(fixture_path, config),
      None => {}
    }
  }

  fn on_resolve_actual_file(
    &mut self,
    fixture_path: &Path,
    config: &mut swc_plugins_collection::types::TransformConfig,
  ) -> String {
    match self {
      Some(sub) => sub.on_resolve_actual_file(fixture_path, config),
      None => String::from_utf8(fs::read(fixture_path.join("actual.js")).unwrap()).unwrap(),
    }
  }

  fn on_resolve_expected_files(
    &mut self,
    fixture_path: &Path,
    config: &mut swc_plugins_collection::types::TransformConfig,
  ) -> Vec<ExpectedInfo> {
    match self {
      Some(sub) => sub.on_resolve_expected_files(fixture_path, config),
      None => {
        let expected_path = fixture_path.join("expected.js");
        let expected = fs::read(&expected_path).unwrap();

        let option_path = fixture_path.join("option.json");
        let option = option_path
          .exists()
          .then(|| serde_json::from_slice(fs::read(option_path).unwrap().as_slice()).unwrap());

        vec![ExpectedInfo::new(
          expected_path.to_string_lossy().to_string(),
          String::from_utf8(expected).unwrap(),
          option,
        )]
      }
    }
  }

  fn on_before_compare(
    &mut self,
    actual: &mut PathBuf,
    expected: &mut PathBuf,
    config: &mut swc_plugins_collection::types::TransformConfig,
  ) {
    match self {
      Some(sub) => sub.on_before_compare(actual, expected, config),
      None => {}
    }
  }
}

impl<H> FixtureTester<H>
where
  H: FixtureTesterHook,
{
  /// You can pass default hook, `BaseFixtureHook`
  pub fn new(
    config: swc_plugins_collection::types::TransformConfig,
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

  pub fn run_test<D: Display>(
    &self,
    test_name: D,
    config: &swc_plugins_collection::types::TransformConfig,
    code: &str,
    expected: Option<impl AsRef<str>>,
  ) {
    use swc_plugins_collection::pass;
    let test_name = test_name.to_string();
    let cm = Arc::new(SourceMap::default());

    let swc_config = &config.swc;
    let extensions_config = &config.extensions;

    let res = swc_plugins_core::transform(
      Arc::new(Compiler::new(cm)),
      swc_config,
      extensions_config,
      "test",
      code,
      None,
      self.config_hash.clone(),
      pass::internal_transform_before_pass,
      pass::internal_transform_after_pass,
    );

    if let Err(e) = res {
      if expected.is_some() {
        println!("-----[{}] Transform failed-----", test_name.red());
        println!("{e}");
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
        show_diff(expected.trim(), res.trim());

        panic!();
      } else {
        println!("{}{}", "Passed: ".green(), test_name);
      }
    }
  }
}
