function render(text) {
  let _div;
  text += "yes";
  return function() {
    return _div || (_div = <div>{text}</div>);
  };
}
