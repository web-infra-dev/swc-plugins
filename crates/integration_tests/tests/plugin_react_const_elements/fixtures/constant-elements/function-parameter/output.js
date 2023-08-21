import Foo from 'Foo';

function render(text) {
  let _foo;
  return function() {
    return _foo || (_foo = <foo>{text}</foo>);
  };
}

function createComponent(text) {
  let _Foo;
  return function render() {
    return _Foo || (_Foo = <Foo>{text}</Foo>);
  };
}
