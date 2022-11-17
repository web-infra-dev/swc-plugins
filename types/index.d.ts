import { JsMinifyOptions, Options } from "./swcTypes";
export * from './swcTypes'

/**
 * Internal plugins
 */
 export interface ImportItem {
  fromSource: string;
  replaceJs?: {
    ignoreEsComponent?: string[];
    replaceExpr?: (member: string) => (string | false);
    replaceTpl?: string;
    lower?: boolean;
    camel2DashComponentName?: boolean;
    transformToDefaultImport?: boolean;
  };
  replaceCss?: {
    ignoreStyleComponent?: string[];
    replaceExpr?: (member: string) => (string | false);
    replaceTpl?: string;
    lower?: boolean;
    camel2DashComponentName?: boolean;
  };
}

export interface PackageConfig {
  transform: string;
  preventFullImport: boolean;
  skipDefaultConversion: boolean;
}

export interface Extensions {
  modularizeImports?: Record<string, PackageConfig>;
  pluginImport?: ImportItem[];
  reactUtils?: {
    autoImportReact?: boolean,
    rmEffect?: boolean,
    rmPropTypes?: {
      mode?: "remove" | "unwrap" | "unsafe-wrap",
      removeImport?: bool,
      ignoreFilenames?: String[],
      additionalLibraries?: String[],
      classNameMatchers?: String[],
    }
  };
  lockCorejsVersion?: {
    corejs?: string,
    swcHelpers?: string
  },
  lodash?: {
    cwd?: string,
    ids?: string,
  }
}

export interface Output {
  code: string;
  map?: string;
}

export interface TransformConfig extends Options {
  extensions?: Extensions
}

export class Compiler {
  constructor(config: TransformConfig);

  transformSync(filename: string, code: string, map?: string): Output;

  transform(filename: string, code: string, map?: string): Promise<Output>;

  release(): void
}

export function minify(
  filename: string,
  code: string,
  config: JsMinifyOptions,
): Promise<Output>;

export function minifySync(
  filename: string,
  code: string,
  config: JsMinifyOptions,
): Output;
