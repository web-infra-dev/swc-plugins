use crate::types::TransformConfig;
use crate::babel_import::plugin_import;
use shared::swc::config::SourceMapsConfig;
use shared::swc::{Compiler, TransformOutput};
use shared::swc_common::input::StringInput;
use shared::swc_common::sync::Lrc;
use shared::swc_common::{FileName, SourceMap};
use shared::swc_ecma_ast::EsVersion;
use shared::swc_ecma_parser::lexer::Lexer;
use shared::swc_ecma_parser::{Parser, Syntax, TsConfig};
use shared::swc_ecma_visit::VisitMutWith;

pub fn transform(
  code: &str,
  config: TransformConfig,
  filename: Option<String>,
  target: Option<String>,
) -> Result<TransformOutput, std::string::String>
{
  let source_filename = filename.unwrap_or_else(|| "test.js".to_string());
  let cm: Lrc<SourceMap> = Default::default();
  let fm = cm.new_source_file(FileName::Custom(source_filename.clone()), code.into());
  let compiler = Compiler::new(cm);

  let lexer = Lexer::new(
    // We want to parse ecmascript
    Syntax::Typescript(TsConfig {
      tsx: true,
      decorators: true,
      dts: false,
      no_early_errors: false,
    }),
    // EsVersion defaults to es5
    EsVersion::Es2016,
    StringInput::from(&*fm),
    None,
  );

  let mut parser = Parser::new_from(lexer);

  let list_error = parser.take_errors();
  if list_error.iter().len() > 0 {
    let err_msg = list_error
      .iter()
      .map(|err| err.kind().msg())
      .collect::<Vec<_>>()
      .join("");
    return Err(err_msg);
  }

  let module_result = parser.parse_module();
  if module_result.is_err() {
    return Err(module_result.err().unwrap().into_kind().msg().to_string());
  }
  let mut module = module_result.unwrap();

  module.visit_mut_with(&mut plugin_import(&config));

  let target_ref = target.unwrap_or_else(|| "".to_string());
  let swc_target = match target_ref.as_str() {
    "" => EsVersion::Es5,
    "ES5" => EsVersion::Es5,
    "ES6" => EsVersion::Es2015,
    _ => EsVersion::Es2020,
  };

  compiler
    .print(
      &module,
      Some(source_filename.as_str()),
      None,
      false,
      swc_target,
      SourceMapsConfig::Bool(true),
      &Default::default(),
      None,
      false,
      None,
      false,
      false,
    )
    .map_err(|err| err.to_string())
}
