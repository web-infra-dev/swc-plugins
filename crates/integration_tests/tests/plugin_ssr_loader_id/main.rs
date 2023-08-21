use std::{path::PathBuf, sync::Arc};

use swc_core::{
  common::{comments::SingleThreadedComments, FileName, Mark, SourceMap},
  ecma::parser::Syntax,
};
use swc_plugins_collection::plugin_ssr_loader_id::{plugin_ssr_loader_id, SSRLoaderIdConfig};
use swc_plugins_utils::PluginContext;

#[test]
fn ssr_loader_id() {
  let cm = Arc::new(SourceMap::default());
  integration_tests::testing::test_transform(
    Syntax::Es(Default::default()),
    |_| {
      plugin_ssr_loader_id(
        &SSRLoaderIdConfig {
          runtime_package_name: "@modern-js/runtime".to_string(),
          function_use_loader_name: Some("useLoader".to_string()),
          function_use_static_loader_name: None,
          function_create_container_name: None,
        },
        &PluginContext {
          cm: cm.clone(),
          file: cm.new_source_file(FileName::Anon, "".into()),
          top_level_mark: Mark::new(),
          unresolved_mark: Mark::new(),
          comments: SingleThreadedComments::default(),
          filename: "/root/a.js".into(),
          cwd: PathBuf::from("/root"),
          config_hash: None,
        },
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
