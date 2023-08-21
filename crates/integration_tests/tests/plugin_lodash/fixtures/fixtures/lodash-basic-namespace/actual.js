import * as _ from "lodash";

var result = _.map([], n => _.add(1, n));
_.take(_.reject(result), 1);
