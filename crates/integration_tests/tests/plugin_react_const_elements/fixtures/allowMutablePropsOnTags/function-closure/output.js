import Counter from 'foo';
function Component({ increment }) {
  let _Counter;
  return ()=>{
    return _Counter || (_Counter = <Counter onClick={(value)=>{
      return value + increment;
    }}/>);
  };
}
