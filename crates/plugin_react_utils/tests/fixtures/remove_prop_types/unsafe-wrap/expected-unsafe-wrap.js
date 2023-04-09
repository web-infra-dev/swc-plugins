"use strict";
class Foo1 extends React.Component {
    render() {}
}
process.env.NODE_ENV !== "production" ? Foo1.propTypes = {
    bar1: PropTypes.string
} : void 0;
class Foo2 extends React.Component {
    render() {}
}
process.env.NODE_ENV !== "production" ? Foo2.propTypes = {
    bar2: PropTypes.string
} : void 0;
const Foo3 = ()=>/*#__PURE__*/ React.createElement("div", null);
process.env.NODE_ENV !== "production" ? Foo3.propTypes = {
    bar3: PropTypes.string
} : void 0;
