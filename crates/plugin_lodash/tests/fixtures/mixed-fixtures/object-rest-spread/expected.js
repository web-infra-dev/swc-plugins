"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
var _interopRequireDefault = require("@swc/helpers/lib/_interop_require_default.js").default;
var _objectSpread = require("@swc/helpers/lib/_object_spread.js").default;
var _objectWithoutProperties = require("@swc/helpers/lib/_object_without_properties.js").default;
var _keys = /*#__PURE__*/ _interopRequireDefault(require("lodash/keys"));
var o1 = {
    "a": 1
};
var o2 = {
    "b": 2,
    "c": 3
};
var o3 = _objectSpread({}, o1, o2);
var foo = o3.b, bar = _objectWithoutProperties(o3, [
    "b"
]);
(0, _keys.default)(bar);
