import { JsMinifyOptions, Options } from "./swcTypes";
export * from "./swcTypes";

/**
 * Internal plugins
 */
export interface ImportItemNapi {
  libraryName: string;
  libraryDirectory?: string;
  customNameFn?: (name: string) => string | undefined;
  customNameTpl?: string;
  customStyleNameFn?: (name: string) => string | undefined;
  customStyleNameTpl?: string;
  style?: {
    styleLibraryDirectory?: string;
    customFn?: (name: string) => string | undefined;
    customTpl?: string;
    css?: "";
    bool?: boolean;
  };

  camelToDashComponentName?: bool; // default to true
  transformToDefaultImport?: bool;

  ignoreEsComponent?: string[];
  ignoreStyleComponent?: string[];
}

// Exposed to user, to keep this the same with babel-plugin-import
export interface ImportItem {
  libraryName: string;
  libraryDirectory?: string;

  customName?: string | ((name: string) => string | undefined);
  customStyleName?: string | ((name: string) => string | undefined);

  styleLibraryDirectory?: string;
  style?: boolean | "css" | string | ((name: string) => string | undefined);

  camelToDashComponentName?: bool; // default to true
  transformToDefaultImport?: bool;

  ignoreEsComponent?: string[];
  ignoreStyleComponent?: string[];
}

export interface PackageConfig {
  transform: string;
  preventFullImport: boolean;
  skipDefaultConversion: boolean;
}

export interface ExtensionsInternal {
  modularizeImports?: Record<string, PackageConfig>;
  pluginImport?: ImportItemNapi[];
  reactUtils?: {
    autoImportReact?: boolean;
    removeEffect?: boolean;
    removePropTypes?: {
      mode?: "remove" | "unwrap" | "unsafe-wrap";
      removeImport?: bool;
      ignoreFilenames?: String[];
      additionalLibraries?: String[];
      classNameMatchers?: String[];
    };
  };
  lockCorejsVersion?: {
    corejs?: string;
    swcHelpers?: string;
  };
  lodash?: {
    cwd?: string;
    ids?: string;
  };
}

export interface Extensions extends ExtensionsInternal {
  pluginImport?: ImportItem[];
}

export interface Output {
  code: string;
  map?: string;
}

export interface TransformConfig extends Options {
  extensions?: Extensions;
}

export class Compiler {
  constructor(config: TransformConfig);

  transformSync(filename: string, code: string, map?: string): Output;

  transform(filename: string, code: string, map?: string): Promise<Output>;

  release(): void;
}

export function minify(
  filename: string,
  code: string,
  config: JsMinifyOptions
): Promise<Output>;

export function minifySync(
  filename: string,
  code: string,
  config: JsMinifyOptions
): Output;
