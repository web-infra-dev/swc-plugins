import fp, { camelCase, kebabCase } from "lodash/fp";

export var formatters = {
  camelCase,
  "kebabCase": kebabCase,
  "snakeCase": fp.snakeCase
};
