/**
 * Internal plugins
 */
export interface Extensions {
  modularizeImports?: import("../crates/plugin_modularize_imports").config;
  pluginImport?: import("../crates/plugin_import").config;
  reactUtils?: import("../crates/plugin_react_utils").config;
  lockCorejsVersion?: {
    corejsPath: string
  }
}

export interface TransformConfig {
  /** Raw swc options */
  swc?: import("./swcTypes").Options;
  /** Internal rust-swc Plugins */
  extensions?: Extensions;
}

export interface TransformConfigNapi {
  /** Raw swc options */
  swc?: string;
  /** Internal rust-swc Plugins */
  extensions?: Extensions;
}

export interface Output {
  code: string;
  map?: string;
}

export class Compiler {
  constructor(config: TransformConfigNapi);

  transformSync(filename: string, code: string, map?: string): Output;

  transform(filename: string, code: string, map?: string): Promise<Output>;

  release(): void
}

function minify(
  config: string,
  filename: string,
  code: string,
  map?: string
): Promise<Output>;

function minifySync(
  config: string,
  filename: string,
  code: string,
  map?: string
): Output;
