let _div, _div2;
import React from "react";
const Parent = ({})=>{
  return _div || (_div = <div className="parent">
  <Child/>
</div>);
};
export default Parent;
let Child = ()=>{
  return _div2 || (_div2 = <div className="child">
  ChildTextContent
</div>);
};
