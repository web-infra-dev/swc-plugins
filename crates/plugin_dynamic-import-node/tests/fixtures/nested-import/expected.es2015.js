function getModule(path) {
  return Promise.resolve().then(function () {
    return _interop_require_wildcard(require("test-module"));
  });
}

getModule().then(function () {});
