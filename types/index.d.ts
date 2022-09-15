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

export function minify(
  config: string,
  filename: string,
  code: string,
  map?: string
): Promise<{ code: string; map?: string }>;

export interface Output {
  code: string;
  map?: string;
}

export function transformSync(
  code: string,
  filename: string,
  map?: string,
  config?: TransformConfig
): Output;

export function transform(
  code: string,
  filename: string,
  map?: string,
  config?: TransformConfig
): Promise<Output>;
