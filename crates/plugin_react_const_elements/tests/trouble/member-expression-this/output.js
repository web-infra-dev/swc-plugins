let _span;
class Component extends React.Component {
  subComponent = ()=>{
    return _span || (_span = <span>Sub Component</span>);
  };
  render = ()=>{
    return <_this.subComponent/>
  };
}
