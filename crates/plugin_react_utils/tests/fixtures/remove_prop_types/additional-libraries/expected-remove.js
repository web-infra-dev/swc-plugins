import React, { Component } from 'react';
class Greeting extends Component {
  render() {
    return /*#__PURE__*/ React.createElement("h1", null, "Welcome ", this.props.name, " and ", this.props.friends.join(', '), " to ", this.state.appName);
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