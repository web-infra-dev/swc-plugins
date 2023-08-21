(function() {
  let _div, _div2;
  const AppItem = ()=>{
    return _div || (_div = <div>child</div>);
  };
  class App extends React.Component {
    render() {
      return _div2 || (_div2 = <div>
          <p>Parent</p>
          <AppItem/>
        </div>);
    }
  }
});
