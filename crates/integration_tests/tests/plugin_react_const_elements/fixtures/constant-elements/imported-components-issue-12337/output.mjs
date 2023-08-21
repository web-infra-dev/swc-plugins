let _div;
import React from "react";
import OtherComponent from "./components/other-component";
export default function App() {
  return _div || (_div = <div>
      <LazyComponent/>
      <OtherComponent/>
    </div>);
}
const LazyComponent = React.lazy(()=>{
  return import("./components/lazy-component");
});
