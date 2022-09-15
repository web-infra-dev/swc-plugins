/**
 * Internal plugins
 */
export interface Extensions<Async extends boolean> {
  modularizeImports?: import("../crates/modularize_imports").config;
  pluginImport?: import("../crates/plugin_import").config<Async>;
}

export interface TransformConfig<Async extends boolean> {
  /** Raw swc options */
  swc: import("./swcTypes").Options;
  /** Internal rust-swc Plugins */
  extensions: Extensions<Async>;
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
  config: TransformConfig<false>,
  code: string
): Output;

export function transform(
  config: TransformConfig<true>,
  code: string
): Promise<Output>;
