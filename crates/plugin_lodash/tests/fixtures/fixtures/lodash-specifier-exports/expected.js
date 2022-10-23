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
    isObject: function() {
        return _isObject.default;
    },
    map: function() {
        return _map.default;
    },
    foo: function() {
        return _isObject.default;
    },
    bar: function() {
        return _foo.default;
    }
});
var _interopRequireDefault = require("@swc/helpers/lib/_interop_require_default.js").default;
var _map = /*#__PURE__*/ _interopRequireDefault(require("lodash/fp/map"));
var _isObject = /*#__PURE__*/ _interopRequireDefault(require("lodash/isObject"));
var _foo = /*#__PURE__*/ _interopRequireDefault(require("foo"));
isObject(a);