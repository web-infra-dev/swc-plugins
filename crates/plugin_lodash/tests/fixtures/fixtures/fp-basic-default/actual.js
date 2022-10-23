import fp from "lodash/fp";

var mapper = fp.map(fp.add(1));
var result = mapper([]);
fp.take(1, fp.reject(Boolean, result));
