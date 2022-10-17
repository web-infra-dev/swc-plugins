"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
var _interopRequireWildcard = require("@swc/helpers/lib/_interop_require_wildcard.js").default;
Promise.resolve().then(function() {
    return /*#__PURE__*/ _interopRequireWildcard(require("test-module"));
}).then(function() {
    return Promise.resolve().then(function() {
        return /*#__PURE__*/ _interopRequireWildcard(require("test-module-2"));
    });
});
Promise.all([
    Promise.resolve().then(function() {
        return /*#__PURE__*/ _interopRequireWildcard(require("test-1"));
    }),
    Promise.resolve().then(function() {
        return /*#__PURE__*/ _interopRequireWildcard(require("test-2"));
    }),
    Promise.resolve().then(function() {
        return /*#__PURE__*/ _interopRequireWildcard(require("test-3"));
    })
]).then(function() {});