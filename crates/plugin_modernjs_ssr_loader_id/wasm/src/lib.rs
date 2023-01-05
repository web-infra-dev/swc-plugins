use plugin_modernjs_ssr_loader_id::plugin_modernjs_ssr_loader_id;
use std::{sync::Arc, env::current_dir};
use swc_core::{
  ecma::{ast::Program, visit::FoldWith},
  plugin::{plugin_transform, proxies::TransformPluginProgramMetadata}, common::{SourceMap, Mark, comments::SingleThreadedComments},
};

#[plugin_transform]
fn transform(program: Program, meta: TransformPluginProgramMetadata) -> Program {
  let cm = Arc::new(SourceMap::default());
  let plugin_context = Arc::new(shared::PluginContext {
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
