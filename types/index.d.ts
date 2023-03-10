/**
 * The type description of Extensions.emotion and Extensions.styledComponent are
 * copied from https://nextjs.org/docs/advanced-features/compiler#emotion
 */
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

  camelToDashComponentName?: boolean; // default to true
  transformToDefaultImport?: boolean;

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

  // Personally think camel2DashComponentName is a bad name as transformToDefaultImport uses differently
  // But for compatibility, both are valid
  camelToDashComponentName?: boolean; // default to true
  camel2DashComponentName?: boolean; // default to true

  transformToDefaultImport?: boolean;

  ignoreEsComponent?: string[];
  ignoreStyleComponent?: string[];
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
    autoImportReact?: boolean;
    removeEffect?: boolean;
    removePropTypes?: {
      mode?: "remove" | "unwrap" | "unsafe-wrap";
      removeImport?: boolean;
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
  modernjsSsrLoaderId?: boolean;
  styledComponent?:
    | boolean
    | {
        displayName?: boolean;
        // Enabled by default.
        ssr?: boolean;
        // Enabled by default.
        fileName?: boolean;
        // Empty by default.
        topLevelImportPaths?: string[];
        // Defaults to ["index"].
        meaninglessFileNames?: string[];
        // Enabled by default.
        cssProp?: boolean;
        // Empty by default.
        namespace?: string;
        // Not supported yet.
        minify?: boolean;
        // Not supported yet.
        transpileTemplateLiterals?: boolean;
        // Not supported yet.
        pure?: boolean;
      };
  emotion?:
    | boolean
    | {
        // default is true. It will be disabled when build type is production.
        sourceMap?: boolean;
        // default is 'dev-only'.
        autoLabel?: "never" | "dev-only" | "always";
        // default is '[local]'.
        // Allowed values: `[local]` `[filename]` and `[dirname]`
        // This option only works when autoLabel is set to 'dev-only' or 'always'.
        // It allows you to define the format of the resulting label.
        // The format is defined via string where variable parts are enclosed in square brackets [].
        // For example labelFormat: "my-classname--[local]", where [local] will be replaced with the name of the variable the result is assigned to.
        labelFormat?: string;
        // default is undefined.
        // This option allows you to tell the compiler what imports it should
        // look at to determine what it should transform so if you re-export
        // Emotion's exports, you can still use transforms.
        importMap?: {
          [packageName: string]: {
            [exportName: string]: {
              canonicalImport?: [string, string];
              styledBaseImport?: [string, string];
            };
          };
        };
      };
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

interface MinifyCssOption {
  sourceMap?: boolean;
  inlineSourceContent?: boolean;
}

export function minifyCss(
  filename: string,
  code: string,
  config: MinifyCssOption
): Promise<Output>;

export function minifyCssSync(
  filename: string,
  code: string,
  config: MinifyCssOption
): Output;
