"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
var _interopRequireDefault = require("@swc/helpers/lib/_interop_require_default.js").default;
var _pick = /*#__PURE__*/ _interopRequireDefault(require("lodash-es/pick"));
var _omit = /*#__PURE__*/ _interopRequireDefault(require("lodash-es/omit"));
var _noop = /*#__PURE__*/ _interopRequireDefault(require("lodash-es/noop"));
var _filter = /*#__PURE__*/ _interopRequireDefault(require("lodash-es/filter"));
var _map = /*#__PURE__*/ _interopRequireDefault(require("lodash-es/map"));
var _identity = /*#__PURE__*/ _interopRequireDefault(require("lodash-es/identity"));
var func1 = _identity.default || _noop.default;
var func2 = _noop.default ? _map.default : _filter.default;
_noop.default;
(bool ? _omit.default : _pick.default)(object);