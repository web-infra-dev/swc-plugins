function render(Component, text = '') {
  let _Component;
  return function() {
    return _Component || (_Component = <Component text={text}/>);
  };
}
