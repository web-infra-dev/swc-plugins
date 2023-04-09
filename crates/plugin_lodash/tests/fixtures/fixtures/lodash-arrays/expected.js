"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
var _interop_require_default = require("@swc/helpers/_/_interop_require_default");
var _partial = /*#__PURE__*/ _interop_require_default._(require("lodash/fp/partial"));
var _toUpper = /*#__PURE__*/ _interop_require_default._(require("lodash/toUpper"));
var _round = /*#__PURE__*/ _interop_require_default._(require("lodash/round"));
var _isString = /*#__PURE__*/ _interop_require_default._(require("lodash/isString"));
var _isNumber = /*#__PURE__*/ _interop_require_default._(require("lodash/isNumber"));
var _cond = /*#__PURE__*/ _interop_require_default._(require("lodash/cond"));
(0, _cond.default)([
    [
        _isNumber.default,
        _round.default
    ],
    [
        _isString.default,
        _toUpper.default
    ]
])(1.8);
(0, _partial.default)(func)([
    _partial.default.placeholder,
    2
])(1);
