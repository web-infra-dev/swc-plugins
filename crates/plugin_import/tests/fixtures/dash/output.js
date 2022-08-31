import "foo/style/aa-bb";
import AaBb from "foo/lib/es/aa-bb";
console.log(AaBb);
export const App = ()=><AaBb ></AaBb>;
(function iife() {
    const AaBb = "";
    console.log(AaBb);
})();
