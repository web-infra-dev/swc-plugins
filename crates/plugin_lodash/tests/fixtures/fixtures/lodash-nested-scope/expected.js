"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
var _interopRequireDefault = require("@swc/helpers/lib/_interop_require_default.js").default;
var _merge = /*#__PURE__*/ _interopRequireDefault(require("lodash/merge"));
function foo(object) {
    return (0, _merge.default)(object, {
        "a": 1
    });
}