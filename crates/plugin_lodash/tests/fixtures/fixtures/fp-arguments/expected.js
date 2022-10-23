"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
var _interopRequireDefault = require("@swc/helpers/lib/_interop_require_default.js").default;
var _map = /*#__PURE__*/ _interopRequireDefault(require("lodash/fp/map"));
var _head = /*#__PURE__*/ _interopRequireDefault(require("lodash/fp/head"));
var _compose = /*#__PURE__*/ _interopRequireDefault(require("lodash/fp/compose"));
var _capitalize = /*#__PURE__*/ _interopRequireDefault(require("lodash/fp/capitalize"));
(0, _compose.default)((0, _map.default)(_capitalize.default), _head.default)([]);