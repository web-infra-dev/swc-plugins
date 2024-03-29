let binding;
try {
  binding = require("./index");
} catch (e) {
  console.error(
    "Can't find SWC binary, you can try following ways to solve this:\n1. Upgrade your Node.js version to 14.19, and reinstall dependencies.\n2. Make sure your Node.js matches your computer OS and CPU architecture, you can check that by printing `process.arch` and `process.platform`.\n"
  );
  throw e;
}

exports.minifyCss = binding.minifyCss;
exports.minifyCssSync = binding.minifyCssSync;

const { minify, minifySync, Compiler: RawCompiler } = binding;

class Compiler extends RawCompiler {
  // Do not mutate on rawConfig
  constructor(rawConfig) {
    const config = { ...rawConfig };
    const extensions = { ...config.extensions } || {};

    if (extensions.pluginImport) {
      extensions.pluginImport = transformPluginImport(extensions.pluginImport);
    }

    /**
     * Convert some options to string, let rust to deserialize it to real config,
     */
    optionsToString(extensions);

    delete config.extensions;

    // SWC will crash if use jsc.target and env.targets together after bump
    if (config.jsc?.target && config.env?.targets) {
      console.warn(
        "[SWC] Do not use jsc.target and env.targets together, when used together only env.targets works"
      );
      delete config.jsc.target;
    }

    try {
      super({
        swc: JSON.stringify(config),
        extensions: extensions,
      });
    } catch (e) {
      console.error("[@modern-js/swc-plugins] Failed to initialize config");
      throw e;
    }
  }
}

exports.Compiler = Compiler;

exports.minify = function (filename, code, opt) {
  return minify(JSON.stringify(opt), filename, code);
};

exports.minifySync = function (filename, code, opt) {
  return minifySync(JSON.stringify(opt), filename, code);
};

/**
 *
@type {(pluginImports: import('./types').ImportItem) => import('./types').ImportItemNapi}
 */
function transformPluginImport(pluginImports) {
  return pluginImports.map((pluginImport) => {
    const {
      libraryName,
      libraryDirectory,
      customName,
      customStyleName,
      style,
      styleLibraryDirectory,
      camelToDashComponentName,
      camel2DashComponentName,
      transformToDefaultImport,
      ignoreEsComponent,
      ignoreStyleComponent,
    } = pluginImport;

    const res = {
      libraryName,
      libraryDirectory,

      customNameFn: maybe("function", customName),
      customNameTpl: maybe("string", customName),
      customStyleNameFn: maybe("function", customStyleName),
      customStyleNameTpl: maybe("string", customStyleName),

      style: {
        styleLibraryDirectory: styleLibraryDirectory,
        customFn: maybe("function", style),
        customTpl: style !== "css" ? maybe("string", style) : undefined,
        css: style === "css" ? style : undefined,
        bool: maybe("boolean", style),
      },

      camelToDashComponentName:
        camelToDashComponentName ?? camel2DashComponentName, // default to true
      transformToDefaultImport,

      ignoreEsComponent,
      ignoreStyleComponent,
    };

    return res;
  });
}

function maybe(type, input) {
  return typeof input === type ? input : undefined;
}

function boolToObj(input) {
  if (typeof input === "boolean") {
    return input ? {} : undefined;
  }
  return input;
}

function optionsToString(options) {
  const styledComponents = boolToObj(options.styledComponents);
  const emotion = boolToObj(options.emotion);

  if (styledComponents && typeof styledComponents !== "string") {
    options.styledComponents = JSON.stringify(styledComponents);
  }

  if (emotion && typeof emotion !== "string") {
    options.emotion = JSON.stringify(emotion);
  }
}
