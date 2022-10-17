Promise.resolve(`${{ "answer": 42 }}`).then(s => _interop_require_wildcard(require(s)));
Promise.resolve(`${["foo", "bar"]}`).then(s => _interop_require_wildcard(require(s)));
Promise.resolve(`${42}`).then(s => _interop_require_wildcard(require(s)));
Promise.resolve(`${void 0}`).then(s => _interop_require_wildcard(require(s)));
Promise.resolve(`${undefined}`).then(s => _interop_require_wildcard(require(s)));
Promise.resolve(`${null}`).then(s => _interop_require_wildcard(require(s)));
Promise.resolve(`${true}`).then(s => _interop_require_wildcard(require(s)));
Promise.resolve(`${Symbol()}`).then(s => _interop_require_wildcard(require(s)));
