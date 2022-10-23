import _, { camelCase, kebabCase } from "lodash";

export var formatters = {
  camelCase,
  "kebabCase": kebabCase,
  "snakeCase": _.snakeCase
};
