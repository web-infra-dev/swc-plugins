function AComponent() {
  let _div, _CComponent;
  const CComponent = ()=>{ 
    return _div || (_div = <div/>);
  };
  return <BComponent/>;
  function BComponent() {
    return _CComponent || (_CComponent = <CComponent/>);
  }
}
