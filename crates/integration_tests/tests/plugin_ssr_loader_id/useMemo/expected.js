import { useLoader } from "@modern-js/runtime";
import { memo } from "react";
var Wrap = memo(function(props) {
    useLoader(function() {
        var innerLoader = function() {
            console.log("wrap");
            return Promise.resolve({});
        };
        innerLoader.id = "6a66e22043a0ad5e33bc19f7345c89db_0";
        return innerLoader;
    }(), {});
    return <div>wrap header{props.children}</div>;
});
export default Wrap;
