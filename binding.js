const { minify, minifySync, Compiler: RawCompiler } = require('./index')

class Compiler extends RawCompiler {
  constructor(config) {
    const extensions = config.extensions;
    delete config.extensions;

    super({
      swc: JSON.stringify(config),
      extensions: extensions || {}
    });
  }
}

exports.Compiler = Compiler

exports.minify = function (filename, code, opt) {
  return minify(JSON.stringify(opt), filename, code)
}

exports.minifySync = function (filename, code, opt) {
  return minifySync(JSON.stringify(opt), filename, code)
}
