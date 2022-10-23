import _, { camelCase, kebabCase } from "lodash-compat";

export var formatters = {
  camelCase,
  "kebabCase": kebabCase,
  "snakeCase": _.snakeCase
};
