use modern_swc_plugins_collection::{pass, types::TransformConfig};
use modern_swc_plugins_core::transform;
use std::sync::Arc;
use swc_core::{base::Compiler, common::sync::Lazy};

use crate::utils::show_diff;

static COMPILER: Lazy<Arc<Compiler>> = Lazy::new(|| Arc::new(Compiler::new(Default::default())));

pub fn test(config: &str, filename: &str, input: &str, expected: &str, hash: Option<String>) {
  let config: TransformConfig = serde_json::from_str(config).unwrap();
  let TransformConfig { swc, extensions } = &config;

  let res = transform(
    COMPILER.clone(),
    swc,
    extensions,
    filename,
    input,
    None,
    hash,
    pass::internal_transform_before_pass,
    pass::internal_transform_after_pass,
  );

  match res {
    Ok(res) => {
      if expected.trim() != res.code.trim() {
        show_diff(expected, &res.code);
        panic!("Test assert failed")
      }
    }
    Err(e) => {
      println!("{}", e);
      panic!("Transform failed");
    }
  };
}
