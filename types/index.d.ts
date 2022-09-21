/**
 * Internal plugins
 */
export interface Extensions {
  modularizeImports?: import("../crates/modularize_imports").config;
  pluginImport?: import("../crates/plugin_import").config;
  reactUtils?: import("../crates/react_utils").config;
}

export interface TransformConfig {
  /** Raw swc options */
  swc?: import("./swcTypes").Options;
  /** Internal rust-swc Plugins */
  extensions?: Extensions;
}

export interface TransformConfigNapi {
  /** Raw swc options */
  swc?: Buffer;
  /** Internal rust-swc Plugins */
  extensions?: Extensions;
}

export interface Output {
  code: Buffer;
  map?: Buffer;
}

export class Compiler {
  constructor(config: TransformConfigNapi);

  transformSync(filename: string, code: Buffer, map?: Buffer): Output;

  transform(filename: string, code: Buffer, map?: Buffer): Promise<Output>;

  release(): void
}

function minify(
  config: Buffer,
  filename: string,
  code: Buffer,
  map?: Buffer
): Promise<Output>;

function minifySync(
  config: Buffer,
  filename: string,
  code: Buffer,
  map?: Buffer
): Output;
