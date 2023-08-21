import Counter from 'foo';
function Component() {
  let _Counter;
  return ()=>{
    return _Counter || (_Counter = <Counter onClick={(value)=>{
      return value + prompt("Increment:");
    }}/>);
  };
}
