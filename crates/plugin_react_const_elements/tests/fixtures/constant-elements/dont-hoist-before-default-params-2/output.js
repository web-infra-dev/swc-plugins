function render(title = '') {
  let _Component;
  return ()=>{
    return _Component || (_Component = <Component title={title}/>);
  };
}
