function render({ text, className, id }) {
  let _div;
  return ()=>{
    return _div || (_div = <div text={text} className={className} id={id}/>);
  };
}
