"use strict";
const Foo1 = ()=>/*#__PURE__*/ React.createElement("div", null);

const Foo2 = ()=>{
  return /*#__PURE__*/ React.createElement("div", null);
};

const Foo3 = function() {
  switch (true) {
    case true:
      if (true) {
        return /*#__PURE__*/ React.createElement("div", null);
      } else {
        return /*#__PURE__*/ React.createElement("span", null);
      }

      break;
  }
};

function Foo4() {
  return /*#__PURE__*/ React.createElement("div", null);
}

function Foo5() {
  const bar5 = function() {
    return /*#__PURE__*/ React.createElement("div", null);
  };

  return bar5();
}

function Foo6() {
  var result = bar6();
  return result;

  function bar6() {
    return /*#__PURE__*/ React.createElement("div", null);
  }
}

function Foo7() {
  const shallow = {
    shallowMember() {
      return /*#__PURE__*/ React.createElement("div", null);
    }

  };
  return shallow.shallowMember();
}

function Foo8() {
  const obj = {
    deep: {
      member() {
        return /*#__PURE__*/ React.createElement("div", null);
      }

    }
  };
  return obj.deep.member();
}

const Foo9 = () => React.createElement("div", null);

const Foo10 = () => {
  return React.createElement("div", null);
};

const Foo11 = () => true && /*#__PURE__*/ React.createElement("div", null);

function Foo12(props) {
  return React.cloneElement(props.children);
}

const Foo13 = React.memo(()=>true && /*#__PURE__*/ React.createElement("div", null));
