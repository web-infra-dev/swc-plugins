use std::{path::PathBuf, sync::Arc};

use shared::{
  anyhow::Result,
  serde_json,
  swc_core::{
    base::{
      config::{self, ModuleConfig},
      try_with_handler, Compiler, HandlerOpts, TransformOutput,
    },
    common::{errors::ColorConfig, FileName, Mark, GLOBALS},
    ecma::{
      ast::EsVersion,
      parser::{Syntax, TsConfig},
      // TODO current version too low
      // transforms::module::common_js::Config
    },
  },
};
use swc_plugins_utils::is_esm;

use crate::pass::{internal_transform_after_pass, internal_transform_before_pass};
use crate::types::TransformConfig;

pub fn transform(
  compiler: Arc<Compiler>,
  config: &TransformConfig,
  filename: String,
  code: &str,
  input_source_map: Option<String>,
) -> Result<TransformOutput> {
  GLOBALS.set(&Default::default(), || {
    let cm = compiler.cm.clone();

    try_with_handler(
      cm.clone(),
      HandlerOpts {
        color: ColorConfig::Never,
        skip_filename: false,
      },
      |handler| {
        compiler.run_transform(handler, true, || {
          let fm = cm.new_source_file(FileName::Real(PathBuf::from(&filename)), code.to_string());

          let mut swc_config = config::Options {
            ..config.swc.clone()
          };
          swc_config.config.input_source_map = input_source_map.map(config::InputSourceMap::Str);
          swc_config.filename = filename;

          let top_level_mark = swc_config.top_level_mark.unwrap_or_else(Mark::new);
          let unresolved_mark = Mark::new();

          swc_config.top_level_mark = Some(top_level_mark);

          // Need auto detect esm
          if swc_config.config.module.is_none() {
            let program = Some(compiler.parse_js(
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
              config::IsModule::Bool(true),
              None,
            )?);

            swc_config.config.module = Some(if is_esm(program.as_ref().unwrap()) {
              ModuleConfig::Es6
            } else {
              ModuleConfig::CommonJs(
                serde_json::from_str(
                  r#"{
                    "ignoreDynamic": true
                  }"#,
                )
                .unwrap(),
              )
            });
          }

          compiler.process_js_with_custom_pass(
            fm,
            None,
            handler,
            &swc_config,
            // TODO pass comments to internal pass
            |_, comments| {
              internal_transform_before_pass(
                config,
                cm.clone(),
                top_level_mark,
                unresolved_mark,
                comments.clone(),
              )
            },
            |_, comments| {
              internal_transform_after_pass(
                config,
                cm.clone(),
                top_level_mark,
                unresolved_mark,
                comments.clone(),
              )
            },
          )
        })
      },
    )
  })
}
