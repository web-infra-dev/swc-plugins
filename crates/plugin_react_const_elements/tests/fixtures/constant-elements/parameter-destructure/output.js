import Component from 'foo';
function render({ text }) {
  let _Component;
  return ()=>{
    return _Component || (_Component = <Component text={text}/>);
  };
}
