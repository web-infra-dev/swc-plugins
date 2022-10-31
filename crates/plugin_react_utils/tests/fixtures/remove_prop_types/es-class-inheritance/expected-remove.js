import _define_property from "@swc/helpers/src/_define_property.mjs";
class PureRenderComponent extends Component {
  
}

class Foo1 extends PureRenderComponent {
  render() {}

}

class Foo2 extends PureRenderComponent {
  render() {}

}

// With no inheritance
export class Foo3 {
  render() {}

}
_define_property(Foo3, "propTypes", {
  foo3: PropTypes.string
});
