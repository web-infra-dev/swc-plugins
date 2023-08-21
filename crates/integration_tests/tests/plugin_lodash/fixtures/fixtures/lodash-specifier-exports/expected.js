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
var _interop_require_default = require("@swc/helpers/_/_interop_require_default");
var _map = /*#__PURE__*/ _interop_require_default._(require("lodash/fp/map"));
var _isObject = /*#__PURE__*/ _interop_require_default._(require("lodash/isObject"));
var _foo = /*#__PURE__*/ _interop_require_default._(require("foo"));
isObject(a);
