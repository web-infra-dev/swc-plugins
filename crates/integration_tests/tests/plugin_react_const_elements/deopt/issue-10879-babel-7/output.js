"use strict";

import React from 'react';

const namespace = {
  MyComponent: (props)=>{
    return props.name;
  }
};
const buildTest = function buildTest(name) {
  let _MyComponent;
  const { MyComponent } = namespace;
  return function () {
    return _MyComponent || (_MyComponent = /*#__PURE__*/_react["default"].createElement(MyComponent, {
      name: name
    }));
  };
};
