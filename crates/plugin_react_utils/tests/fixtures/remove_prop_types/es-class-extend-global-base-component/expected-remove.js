"use strict";
const _defineProperty = require("@swc/helpers/lib/_define_property.js").default;
class Foo1 extends GlobalComponent {
  render() {}

}

_defineProperty(Foo1, "propTypes", {
  foo1: PropTypes.string
});

class Foo2 extends GlobalComponent {
  render() {}

}

Foo2.propTypes = {
  foo2: PropTypes.string
};
