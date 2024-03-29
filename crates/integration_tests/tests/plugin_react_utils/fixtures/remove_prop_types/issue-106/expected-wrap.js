import React, { Component, createElement } from 'react';
import PropTypes from 'prop-types';

class RouteNode extends Component {
  render() {
    const { previousRoute, route } = this.state;
    return /*#__PURE__*/ createElement(RouteSegment, Object.assign({}, this.props, route, previousRoute));
  }

} 

const storeName = 'storeName';
RouteNode.wrappedComponent.propTypes = process.env.NODE_ENV !== "production" ? {
  [storeName]: PropTypes.object.isRequired
} : {};

class BaseLink extends Component {
  render() {
    return /*#__PURE__*/ React.createElement('a', {
      href,
      className,
      onClick
    }, this.props.children);
  }

}

BaseLink.propTypes = process.env.NODE_ENV !== "production" ? {
  routeOptions: PropTypes.object,
  [storeName]: PropTypes.object,
  route: PropTypes.object,
  ['previousRoute']: PropTypes.object
} : {};
