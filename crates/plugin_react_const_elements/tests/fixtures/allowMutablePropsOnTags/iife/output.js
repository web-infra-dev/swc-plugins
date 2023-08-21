import Counter from 'foo';
function Component() {
  return ()=>{
    return <Counter init={((value)=>{
      return value + prompt("Increment:");
    })(2)}/>;
  };
}
