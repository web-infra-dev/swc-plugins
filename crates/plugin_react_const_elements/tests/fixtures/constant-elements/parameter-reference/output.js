function render(text) {
  let _div;
  return function() {
    return _div || (_div = <div>{text}</div>);
  };
}
function render() {
  return function(text) {
    return <div>{text}</div>;
  };
}
