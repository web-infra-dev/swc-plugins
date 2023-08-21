import Counter from 'foo';
function Component() {
  return () => {
    return <Counter onClick={value => value + prompt("Increment:")}/>;
  };
}
