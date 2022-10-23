"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
var _interopRequireDefault = require("@swc/helpers/lib/_interop_require_default.js").default;
var _reject = /*#__PURE__*/ _interopRequireDefault(require("lodash/reject"));
var _add = /*#__PURE__*/ _interopRequireDefault(require("lodash/add"));
var _take = /*#__PURE__*/ _interopRequireDefault(require("lodash/take"));
var _map = /*#__PURE__*/ _interopRequireDefault(require("lodash/map"));
var result = (0, _map.default)([], function(n) {
    return (0, _add.default)(1, n);
});
(0, _take.default)((0, _reject.default)(result), 1);