/**
 * Internal plugins
 */
export interface Extensions {
  modularizeImports?: import("../crates/modularize_imports").config;
  pluginImport?: import("../crates/plugin_import").config;
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

  minify(
    config: string,
    filename: string,
    code: string,
    map?: string
  ): Promise<{ code: string; map?: string }>;

  minifySync(
    config: string,
    filename: string,
    code: string,
    map?: string
  ): { code: string; map?: string };
}
