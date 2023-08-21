"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
var _interop_require_default = require("@swc/helpers/_/_interop_require_default");
var _reject = /*#__PURE__*/ _interop_require_default._(require("lodash/fp/reject"));
var _add = /*#__PURE__*/ _interop_require_default._(require("lodash/fp/add"));
var _take = /*#__PURE__*/ _interop_require_default._(require("lodash/fp/take"));
var _map = /*#__PURE__*/ _interop_require_default._(require("lodash/fp/map"));
var mapper = (0, _map.default)((0, _add.default)(1));
var result = mapper([
    1,
    2,
    3
]);
(0, _take.default)(1, (0, _reject.default)(Boolean, result));
