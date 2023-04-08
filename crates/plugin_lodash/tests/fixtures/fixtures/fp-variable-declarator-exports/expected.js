"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
function _export(target, all) {
    for(var name in all)Object.defineProperty(target, name, {
        enumerable: true,
        get: all[name]
    });
}
_export(exports, {
    case1: function() {
        return case1;
    },
    case2: function() {
        return case2;
    },
    case3: function() {
        return case3;
    }
});
var _interop_require_default = require("@swc/helpers/_/_interop_require_default");
var _kebabCase = /*#__PURE__*/ _interop_require_default._(require("lodash/fp/kebabCase"));
var _camelCase = /*#__PURE__*/ _interop_require_default._(require("lodash/fp/camelCase"));
var _string = require("string");
var case1 = _camelCase.default;
var case2 = _kebabCase.default;
var case3 = _string.snakeCase;
