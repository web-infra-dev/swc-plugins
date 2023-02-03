const { minify, minifySync, Compiler: RawCompiler } = require('./index')

function isDev() {
  return process.env.NODE_ENV === 'development'
}

class Compiler extends RawCompiler {
  constructor(config) {
    const extensions = config.extensions || {};

    if (extensions.pluginImport) {
      extensions.pluginImport = transformPluginImport(extensions.pluginImport)
    }

    extensions.emotion = boolToObj(extensions.emotion)
    extensions.styledComponents = boolToObj(extensions.styledComponents)

    if (extensions.emotion) {
      extensions.emotion = getEmotionOptions(extensions)
    }

    if (extensions.styledComponents) {
      extensions.styledComponents = getStyledComponentsOptions(extensions)
    }

    /**
     * Convert some options to string, let rust to deserialize it to real config,
     */
    optionsToString(extensions)

    delete config.extensions;

    try {
      super({
        swc: JSON.stringify(config),
        extensions
      });
    } catch (e) {
      console.error('[@modern-js/swc-plugins] Failed to initialize config');
      throw e
    }
  }
}

exports.Compiler = Compiler

exports.minify = function (filename, code, opt) {
  return minify(JSON.stringify(opt), filename, code)
}

exports.minifySync = function (filename, code, opt) {
  return minifySync(JSON.stringify(opt), filename, code)
}

/**
 * 
@type {(pluginImports: import('./types').ImportItem) => import('./types').ImportItemNapi}
 */
function transformPluginImport(pluginImports) {
  return pluginImports.map(pluginImport => {
    const {
      libraryName,
      libraryDirectory,
      customName,
      customStyleName,
      style,
      styleLibraryDirectory,
      camelToDashComponentName,
      transformToDefaultImport,
      ignoreEsComponent,
      ignoreStyleComponent,
    } = pluginImport

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

      camelToDashComponentName, // default to true
      transformToDefaultImport,

      ignoreEsComponent,
      ignoreStyleComponent,
    }

    return res;
  })
}

function maybe(type, input) {
  return typeof input === type ? input : undefined
}

function boolToObj(input) {
  if (typeof input === 'boolean') {
    return input ? {} : undefined
  }
  return input
}

function optionsToString(options) {
  const styledComponents = options.styledComponents
  const emotion = options.emotion

  if (styledComponents && typeof styledComponents !== 'string') {
    options.styledComponents = JSON.stringify(styledComponents)
  }

  if (emotion && typeof emotion !== 'string') {
    options.emotion = JSON.stringify(emotion)
  }
}

function getEmotionOptions(config) {
  const emotion = config.emotion
  let autoLabel = false
  switch (config.emotion?.autoLabel) {
    case 'never':
      autoLabel = false
      break
    case 'always':
      autoLabel = true
      break
    case 'dev-only':
    default:
      autoLabel = !!isDev()
      break
  }
  return {
    enabled: true,
    autoLabel,
    importMap: emotion?.importMap,
    labelFormat: emotion?.labelFormat,
    sourcemap: isDev()
      ? emotion?.sourceMap ?? true
      : false,
  }
}

function getStyledComponentsOptions(config) {
  let styledComponentsOptions = config.styledComponents
  return {
    ...styledComponentsOptions,
    displayName: styledComponentsOptions.displayName ?? isDev(),
  }
}

