use anyhow::{anyhow, Result};
use std::{path::PathBuf, sync::Arc};
use swc_core::{
  base::{config::JsMinifyOptions, try_with_handler, Compiler, HandlerOpts, TransformOutput},
  common::{
    errors::ColorConfig, source_map::SourceMapGenConfig, sync::Lazy, FileName, Globals,
    SourceMap, GLOBALS, SourceFile,
  },
  css::{
    ast::Stylesheet,
    codegen::{
      writer::basic::{BasicCssWriter, BasicCssWriterConfig},
      CodeGenerator, CodegenConfig, Emit,
    },
    minifier::minify as swc_minify_css,
    parser::{parse_file, parser::ParserConfig},
  },
};

static COMPILER: Lazy<Arc<Compiler>> = Lazy::new(|| Arc::new(Compiler::new(Default::default())));

pub fn minify(
  config: &JsMinifyOptions,
  filename: impl Into<String>,
  src: &str,
) -> Result<TransformOutput> {
  GLOBALS.set(&Globals::default(), || {
    let cm = COMPILER.cm.clone();
    let filename: String = filename.into();
    let filename = if filename.is_empty() {
      FileName::Anon
    } else {
      FileName::Real(PathBuf::from(filename))
    };
    let fm = cm.new_source_file(filename, src.to_string());

    try_with_handler(
      cm,
      HandlerOpts {
        color: ColorConfig::Never,
        skip_filename: false,
      },
      |handler| COMPILER.minify(fm, handler, config),
    )
  })
}

pub fn minify_css(config: &CssMinifyOptions, filename: &str, src: &str) -> Result<TransformOutput> {
  GLOBALS.set(&Globals::default(), || {
    let cm = Arc::new(SourceMap::default());
  
    let fm = cm.new_source_file(FileName::Real(filename.into()), src.into());
  
    let mut ast = parse(filename, fm)?;
    swc_minify_css(&mut ast, Default::default());
    codegen(&cm, filename, &ast, config)
  })
}

fn parse(filename: &str, fm: Arc<SourceFile>) -> Result<Stylesheet> {
  let mut errors = vec![];
  parse_file(
    &fm,
    ParserConfig {
      allow_wrong_line_comments: true,
      css_modules: false,
      legacy_nesting: true,
      ..Default::default()
    },
    &mut errors,
  )
  .map_err(|e| {
    anyhow!(format!(
      "Failed to parse CSS file: {}:\n{}",
      filename,
      e.message()
    ))
  })
}

impl SourceMapGenConfig for CssMinifyOptions {
  fn inline_sources_content(&self, _f: &FileName) -> bool {
    self.inline_source_content
  }

  fn file_name_to_source(&self, f: &FileName) -> String {
    f.to_string()
  }
}

fn codegen(cm: &SourceMap, filename: &str, ast: &Stylesheet, option: &CssMinifyOptions) -> Result<TransformOutput> {
  let mut output = String::new();
  let mut src_map = option.source_map.then(Vec::new);

  let wr = BasicCssWriter::new(
    &mut output,
    src_map.as_mut(),
    BasicCssWriterConfig::default(),
  );

  let mut codegen = CodeGenerator::new(wr, CodegenConfig { minify: true });

  codegen
    .emit(ast)
    .map_err(|e| anyhow!(format!("Failed to generate css for {filename}:\n{e}")))?;
  let out_map = src_map
    .as_ref()
    .map(|m| cm.build_source_map_with_config(m, None, option));

  Ok(TransformOutput {
    code: output,
    map: out_map.map(|src_map| {
      let mut output = vec![];
      src_map.to_writer(&mut output).unwrap();
      String::from_utf8(output).unwrap()
    }),
  })
}

pub struct CssMinifyOptions {
  pub source_map: bool,
  pub inline_source_content: bool,
}
