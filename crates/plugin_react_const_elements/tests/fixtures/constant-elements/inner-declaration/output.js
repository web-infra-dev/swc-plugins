function render() {
  let _foo;
  var text = getText();
  return function() {
    return _foo || (_foo = <foo>{text}</foo>);
  };
}
