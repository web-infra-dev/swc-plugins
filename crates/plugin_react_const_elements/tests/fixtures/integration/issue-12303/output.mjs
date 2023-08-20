function Foo({ outsetArrows, ...rest }) {
  let _div;
  return useMemo(()=>{
    return _div || (_div = <div outsetArrows={outsetArrows}/>);
  }, [
    outsetArrows
  ]);
}
