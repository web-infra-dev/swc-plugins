import _extends from "@swc/helpers/src/_extends.mjs";
import React from 'react';
import { omit } from 'underscore';
import Bar from './bar';
const { PropTypes  } = React;
const propTypes = {
  foo: PropTypes.any
};
export default function Foo(props) {
  const barProps = omit(props, Object.keys(propTypes));
  return /*#__PURE__*/ React.createElement(Bar, _extends({}, barProps));
}
