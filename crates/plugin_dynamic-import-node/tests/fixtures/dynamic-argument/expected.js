Promise.resolve(`${MODULE}`).then(s => _interop_require_wildcard(require(s)));

let i = 0;
Promise.resolve(`${i++}`).then(s => _interop_require_wildcard(require(s)));

Promise.resolve(`${fn()}`).then(s => _interop_require_wildcard(require(s)));

async () => Promise.resolve(`${await "x"}`).then(s => _interop_require_wildcard(require(s)));

function* f() {
  Promise.resolve(`${yield "x"}`).then(s => _interop_require_wildcard(require(s)));
}
