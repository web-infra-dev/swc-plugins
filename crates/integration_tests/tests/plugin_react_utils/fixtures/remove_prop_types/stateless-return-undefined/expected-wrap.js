import React, { PropTypes } from 'react';
var Message = ({ isFetching, isSuccess, isFailure, errorMsg })=>{
    let messageType;
    let messageTxt;
    if (isFetching) {
        messageType = 'warning';
        messageTxt = 'Pending call...';
    } else if (isSuccess) {
        messageType = 'success';
        messageTxt = 'Repo pushed successfully';
    } else if (isFailure) {
        messageType = 'danger';
        messageTxt = 'Something wrong occured';
    }
    if (messageTxt === null) {
        return;
    }
    return /*#__PURE__*/ React.createElement("div", {
        className: 'alert alert-' + messageType,
        role: "alert"
    }, messageTxt);
};
Message.propTypes = process.env.NODE_ENV !== "production" ? {
    isFetching: PropTypes.bool.isRequired,
    isSuccess: PropTypes.bool.isRequired,
    isFailure: PropTypes.bool.isRequired,
    errorMsg: PropTypes.string.isRequired
} : {};
export default Message;
