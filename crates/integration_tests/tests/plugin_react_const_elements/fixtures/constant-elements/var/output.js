function fn(Component) {
  let _Component;
  var data = "prop";
  return ()=>{
    return _Component || (_Component = <Component prop={data}/>);
  };
}
