function getModule(path) {
  return Promise.resolve().then(() => _interop_require_wildcard(require("test-module")));
}

getModule().then(() => {});
