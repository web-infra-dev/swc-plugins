import { keys } from "lodash";

var o1 = { "a": 1 };
var o2 = { "b": 2, "c": 3 };
var o3 = { ...o1, ...o2 };
var { b: foo, ...bar } = o3;

keys(bar);
