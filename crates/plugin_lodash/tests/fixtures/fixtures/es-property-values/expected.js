"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
Object.defineProperty(exports, "formatters", {
    enumerable: true,
    get: function() {
        return formatters;
    }
});
var _interop_require_default = require("@swc/helpers/_/_interop_require_default");
var _snakeCase = /*#__PURE__*/ _interop_require_default._(require("lodash-es/snakeCase"));
var _kebabCase = /*#__PURE__*/ _interop_require_default._(require("lodash-es/kebabCase"));
var _camelCase = /*#__PURE__*/ _interop_require_default._(require("lodash-es/camelCase"));
var formatters = {
    camelCase: _camelCase.default,
    "kebabCase": _kebabCase.default,
    "snakeCase": _snakeCase.default
};
