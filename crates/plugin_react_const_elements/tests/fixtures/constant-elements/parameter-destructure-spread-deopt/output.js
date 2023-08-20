import Component from 'foo';
function render({ text, className, id, ...props }) {
  return ()=>{
    return <Component text={text} className={className} id={id} {...props}/>;
  };
}
