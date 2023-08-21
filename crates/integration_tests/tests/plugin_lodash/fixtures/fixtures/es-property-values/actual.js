import _, { camelCase, kebabCase } from "lodash-es";

export var formatters = {
  camelCase,
  "kebabCase": kebabCase,
  "snakeCase": _.snakeCase
};
