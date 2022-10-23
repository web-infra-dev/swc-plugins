import _ from "lodash-compat";

var result = _.map([], n => _.add(1, n));
_.take(_.reject(result), 1);
