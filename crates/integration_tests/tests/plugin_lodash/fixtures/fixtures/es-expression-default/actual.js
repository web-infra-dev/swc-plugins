import _ from "lodash-es";

var func1 = _.identity || _.noop;
var func2 = _.noop ? _.map : _.filter;

_.noop;

(bool ? _.omit : _.pick)(object);
