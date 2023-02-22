use std::sync::Arc;
use swc_core::{base::Compiler, common::sync::Lazy};
use swc_plugins_core::{transform, types::TransformConfig};

use crate::utils::show_diff;

static COMPILER: Lazy<Arc<Compiler>> = Lazy::new(|| Arc::new(Compiler::new(Default::default())));

pub fn test(config: &str, filename: &str, input: &str, expected: &str, hash: Option<String>) {
  let config: TransformConfig = serde_json::from_str(config).unwrap();

  let res = transform(COMPILER.clone(), &config, filename, input, None, hash);

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
