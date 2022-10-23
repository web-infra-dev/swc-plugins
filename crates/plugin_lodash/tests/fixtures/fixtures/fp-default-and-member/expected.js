"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
var _interopRequireDefault = require("@swc/helpers/lib/_interop_require_default.js").default;
var _reject = /*#__PURE__*/ _interopRequireDefault(require("lodash/fp/reject"));
var _add = /*#__PURE__*/ _interopRequireDefault(require("lodash/fp/add"));
var _take = /*#__PURE__*/ _interopRequireDefault(require("lodash/fp/take"));
var _map = /*#__PURE__*/ _interopRequireDefault(require("lodash/fp/map"));
var mapper = (0, _map.default)((0, _add.default)(1));
var result = mapper([
    1,
    2,
    3
]);
(0, _take.default)(1, (0, _reject.default)(Boolean, result));