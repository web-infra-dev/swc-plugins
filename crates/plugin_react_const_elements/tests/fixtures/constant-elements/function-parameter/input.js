import Foo from 'Foo'

function render(text) {
  return function () {
    return <foo>{text}</foo>;
  };
}

function createComponent(text) {
  return function render() {
    return <Foo>{text}</Foo>;
  };
}
