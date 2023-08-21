"use strict";
var App = {
  init: function(assets) {
    assets = assets || {};

    if (assets.templates) {
      TemplateStore.init(assets.templates);
    }
  }
};

const FormattedPlural = () => /*#__PURE__*/ React.createElement("div", null);

process.env.NODE_ENV !== 'production' ? void 0 : void 0;
