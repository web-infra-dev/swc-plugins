use std::{path::PathBuf, sync::Arc};

use anyhow::Result;
use swc_core::{
  base::{
    config::{self, Options},
    try_with_handler, Compiler, HandlerOpts, TransformOutput,
  },
  common::{comments::SingleThreadedComments, errors::ColorConfig, FileName, Mark, GLOBALS},
  ecma::{
    ast::EsVersion,
    parser::{Syntax, TsConfig},
    visit::Fold,
    // TODO current version too low
    // transforms::module::common_js::Config
  },
};
use swc_plugins_utils::PluginContext;

use crate::TransformFn;

/// As we initialize plugins at each transform,
/// Some plugins need very heavy work on the first
/// time, and if we can cache it, we should get better
/// performance.
/// A `config_hash` is the unique key representing a
/// specific `TransformConfig`.
/// transform don't care how you create this hash.
///
/// If you call `transform` from `nodejs`, this config hash
/// is unique for each `binding::Compiler`.
#[allow(clippy::too_many_arguments)]
pub fn transform<'a, E, P1, P2>(
  compiler: Arc<Compiler>,
  swc_config: &Options,
  extensions_config: &'a E,
  filename: impl Into<String>,
  code: &str,
  input_source_map: Option<String>,
  config_hash: Option<String>,
  transform_before_pass: TransformFn<'a, E, P1>,
  transform_after_pass: TransformFn<'a, E, P2>,
) -> Result<TransformOutput>
where
  E: 'a,
  P1: Fold + 'a,
  P2: Fold + 'a,
{
  GLOBALS.set(&Default::default(), || {
    let cm = compiler.cm.clone();
    let filename: String = filename.into();

    try_with_handler(
      cm.clone(),
      HandlerOpts {
        color: ColorConfig::Never,
        skip_filename: false,
      },
      |handler| {
        compiler.run_transform(handler, true, || {
          let cm_filename = if filename.is_empty() {
            FileName::Anon
          } else {
            FileName::Real(PathBuf::from(filename.clone()))
          };

          let fm = cm.new_source_file(cm_filename, code.to_string());

          let mut swc_config = swc_config.clone();
          swc_config.config.input_source_map = input_source_map.map(config::InputSourceMap::Str);
          swc_config.filename = filename.clone();

          let top_level_mark = swc_config.top_level_mark.unwrap_or_else(Mark::new);
          let unresolved_mark = Mark::new();

          swc_config.top_level_mark = Some(top_level_mark);
          let comments = SingleThreadedComments::default();
          let is_module = swc_config
            .config
            .is_module
            .unwrap_or(config::IsModule::Unknown);

          let program = compiler.parse_js(
            fm.clone(),
            handler,
            swc_config.config.jsc.target.unwrap_or(EsVersion::Es2022),
            swc_config.config.jsc.syntax.unwrap_or_else(|| {
              Syntax::Typescript(TsConfig {
                tsx: true,
                decorators: true,
                ..Default::default()
              })
            }),
            is_module,
            Some(&comments),
          )?;

          // TODO comments can be pass to `process_js_with_custom_pass` in next swc version
          let plugin_context = Arc::new(PluginContext {
            cm,
            file: fm.clone(),
            top_level_mark,
            unresolved_mark,
            comments: comments.clone(),
            config_hash,
            filename,
            cwd: swc_config.cwd.clone(),
          });

          compiler.process_js_with_custom_pass(
            fm,
            Some(program),
            handler,
            &swc_config,
            comments,
            // TODO pass comments to internal pass in next swc versions
            |_| transform_before_pass(extensions_config, &swc_config, plugin_context.clone()),
            |_| transform_after_pass(extensions_config, &swc_config, plugin_context.clone()),
          )
        })
      },
    )
  })
}
