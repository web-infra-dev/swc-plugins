function render() {
  let _foo;
  const bar = "bar", renderFoo = ()=>{
    return _foo || (_foo = <foo bar={bar}/>);
  };
  return renderFoo();
}
function render() {
  let _foo2;
  const bar = "bar", renderFoo = ()=>{
    return _foo2 || (_foo2 = <foo bar={bar} baz={baz}/>);
  }, baz = "baz";
  return renderFoo();
}
