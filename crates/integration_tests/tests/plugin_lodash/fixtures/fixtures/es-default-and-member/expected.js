"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
var _interop_require_default = require("@swc/helpers/_/_interop_require_default");
var _reject = /*#__PURE__*/ _interop_require_default._(require("lodash-es/reject"));
var _add = /*#__PURE__*/ _interop_require_default._(require("lodash-es/add"));
var _take = /*#__PURE__*/ _interop_require_default._(require("lodash-es/take"));
var _map = /*#__PURE__*/ _interop_require_default._(require("lodash-es/map"));
var result = (0, _map.default)([], function(n) {
    return (0, _add.default)(1, n);
});
(0, _take.default)((0, _reject.default)(result), 1);
