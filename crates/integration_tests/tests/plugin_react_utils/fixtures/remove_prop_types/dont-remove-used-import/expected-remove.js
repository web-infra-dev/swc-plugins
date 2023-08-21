import React, { Component } from 'react';
import PropTypes from 'prop-types';

class Greeting extends Component {
  render() {
    return /*#__PURE__*/ React.createElement("h1", null, "Welcome ", this.props.name, " to ", this.state.appName);
  }
  constructor(props, context){
    super(props, context);
    const appName = context.store.getState().appName;
    this.state = {
      appName: appName
    };
  }
}

export { Greeting as default };
Greeting.contextTypes = {
  store: PropTypes.object
};