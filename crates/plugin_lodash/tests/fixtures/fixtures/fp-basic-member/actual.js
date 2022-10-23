import { add, map, reject, take } from "lodash/fp";

var mapper = map(add(1));
var result = mapper([]);
take(1, reject(Boolean, result));
