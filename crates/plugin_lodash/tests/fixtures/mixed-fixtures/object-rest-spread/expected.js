"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
var _interop_require_default = require("@swc/helpers/_/_interop_require_default");
var _object_spread = require("@swc/helpers/_/_object_spread");
var _object_without_properties = require("@swc/helpers/_/_object_without_properties");
var _keys = /*#__PURE__*/ _interop_require_default._(require("lodash/keys"));
var o1 = {
    "a": 1
};
var o2 = {
    "b": 2,
    "c": 3
};
var o3 = _object_spread._({}, o1, o2);
var foo = o3.b, bar = _object_without_properties._(o3, [
    "b"
]);
(0, _keys.default)(bar);
