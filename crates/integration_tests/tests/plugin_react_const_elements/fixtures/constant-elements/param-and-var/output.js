function fn(Component, obj) {
  let _Component;
  var data = obj.data;
  return ()=>{
    return _Component || (_Component = <Component prop={data}/>);
  };
}
