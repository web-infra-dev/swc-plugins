use plugin_modernjs_ssr_loader_id::plugin_modernjs_ssr_loader_id;
use std::{env::current_dir, sync::Arc};
use swc_core::{
  common::{comments::SingleThreadedComments, Mark, SourceMap},
  ecma::{ast::Program, visit::FoldWith},
  plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

#[plugin_transform]
fn transform(program: Program, meta: TransformPluginProgramMetadata) -> Program {
  let cm = Arc::new(SourceMap::default());
  let plugin_context = Arc::new(modern_swc_plugins_utils::PluginContext {
    cm,
    top_level_mark: Mark::new(),
    unresolved_mark: meta.unresolved_mark,
    comments: SingleThreadedComments::default(),
    filename: "test.js".into(),
    cwd: current_dir().unwrap(),
    config_hash: None,
    is_source_esm: true,
  });
  let mut v = plugin_modernjs_ssr_loader_id(plugin_context);
  program.fold_with(&mut v)
}
