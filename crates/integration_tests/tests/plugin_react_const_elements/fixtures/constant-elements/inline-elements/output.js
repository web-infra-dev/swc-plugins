let _foo;
function render() {
  return _foo || (_foo = <foo/>);
}
function render() {
  let _foo2;
  var text = getText();
  return function() {
    return _foo2 || (_foo2 = <foo>{text}</foo>);
  };
}
