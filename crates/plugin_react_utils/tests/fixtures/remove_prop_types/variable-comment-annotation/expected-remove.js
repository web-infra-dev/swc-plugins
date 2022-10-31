import React from 'react';
import { connect } from 'react-redux';
import FooComponent from './FooComponent';
const Foo = connect(()=>{}, ()=>{})(FooComponent);
export default Foo;
