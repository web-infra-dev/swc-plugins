import { _ as _define_property } from "@swc/helpers/_/_define_property";
class PureRenderComponent extends Component {
}
class Foo1 extends PureRenderComponent {
    render() {}
}
_define_property(Foo1, "propTypes", process.env.NODE_ENV !== "production" ? {
    foo1: PropTypes.string.isRequired
} : {});
class Foo2 extends PureRenderComponent {
    render() {}
}
Foo2.propTypes = process.env.NODE_ENV !== "production" ? {
    foo2: PropTypes.string.isRequired
} : {};
// With no inheritance
export class Foo3 {
    render() {}
}
_define_property(Foo3, "propTypes", {
    foo3: PropTypes.string
});
