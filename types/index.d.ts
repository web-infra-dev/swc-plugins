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
  };
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
