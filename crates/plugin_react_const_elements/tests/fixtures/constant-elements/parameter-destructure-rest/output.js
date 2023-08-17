function render({ text, className, id, ...props }) {
  let _div;
  // intentionally ignoring props
  return ()=>{
    return _div || (_div = <div text={text} className={className} id={id}/>);
  };
}
