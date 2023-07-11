import { _ as _define_property } from "@swc/helpers/_/_define_property";
class Foo1 extends Component {
    render() {}
}
_define_property(Foo1, "propTypes", process.env.NODE_ENV !== "production" ? {
    foo1: PropTypes.string
} : {});
class Foo2 extends Component {
    render() {}
}
Foo2.propTypes = process.env.NODE_ENV !== "production" ? {
    foo2: PropTypes.string
} : {};
