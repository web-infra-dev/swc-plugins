function Component() {
  let _Counter;
  return ()=>{
    return _Counter || (_Counter = <Counter onClick={(value)=>{
      return value + 1;
    }}/>);
  };
}
