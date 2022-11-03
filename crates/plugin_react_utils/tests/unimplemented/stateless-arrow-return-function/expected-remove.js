import React, { PropTypes } from 'react';
import map from 'lodash/map';

var Message = ({ mapList  }) => {
  return map(mapList, index => {
    return /*#__PURE__*/ React.createElement("div", null);
  });
};

export default Message;
