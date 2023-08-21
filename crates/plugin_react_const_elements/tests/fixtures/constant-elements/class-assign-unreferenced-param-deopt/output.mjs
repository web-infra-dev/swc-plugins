let _div;
import React from 'react';

// Regression test for https://github.com/babel/babel/issues/5552
class BugReport extends React.Component {
  thisWontWork = ({ color })=>{
    let _div2;
    return (data)=>{
      return _div2 || (_div2 = <div color={color}>does not reference data</div>);
    };
  };
  thisWorks = ({ color })=>{
    return (data)=>{
      return <div color={color}>{data}</div>;
    };
  };
  render() {
    return _div || (_div = <div/>);
  }
}
