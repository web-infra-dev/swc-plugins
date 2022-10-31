import _define_property from "@swc/helpers/src/_define_property.mjs";
import BaseComponent from 'components/base';
class Foo extends BaseComponent {
  render() {}

}

_define_property(Foo, "propTypes", {
  foo: PropTypes.string.isRequired
});
