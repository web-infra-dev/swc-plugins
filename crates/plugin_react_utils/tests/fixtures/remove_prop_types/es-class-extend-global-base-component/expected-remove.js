"use strict";
const _define_property = require("@swc/helpers/_/_define_property");
class Foo1 extends GlobalComponent {
  render() {}

}

_define_property._(Foo1, "propTypes", {
  foo1: PropTypes.string
});

class Foo2 extends GlobalComponent {
  render() {}

}

Foo2.propTypes = {
  foo2: PropTypes.string
};
