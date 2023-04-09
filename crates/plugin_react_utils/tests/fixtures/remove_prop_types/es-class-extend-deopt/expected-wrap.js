import { _ as _define_property } from "@swc/helpers/_/_define_property";
import BaseComponent from 'components/base';
class Foo extends BaseComponent {
    render() {}
}
_define_property(Foo, "propTypes", {
    foo: PropTypes.string.isRequired
});
