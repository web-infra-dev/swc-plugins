"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
var _interopRequireDefault = require("@swc/helpers/lib/_interop_require_default.js").default;
var _reject = /*#__PURE__*/ _interopRequireDefault(require("lodash/reject"));
var _add = /*#__PURE__*/ _interopRequireDefault(require("lodash/add"));
var _reject1 = /*#__PURE__*/ _interopRequireDefault(require("lodash/fp/reject"));
var _add1 = /*#__PURE__*/ _interopRequireDefault(require("lodash/fp/add"));
var _take = /*#__PURE__*/ _interopRequireDefault(require("lodash/fp/take"));
var _map = /*#__PURE__*/ _interopRequireDefault(require("lodash/fp/map"));
var _take1 = /*#__PURE__*/ _interopRequireDefault(require("lodash/take"));
var _map1 = /*#__PURE__*/ _interopRequireDefault(require("lodash/map"));
var mapper = (0, _map.default)((0, _add1.default)(1));
var result = mapper([]);
(0, _take.default)(1, (0, _reject1.default)(Boolean, result));
var result2 = (0, _map1.default)([], function(n) {
    return (0, _add.default)(1, n);
});
(0, _take1.default)((0, _reject.default)(result2), 1);
