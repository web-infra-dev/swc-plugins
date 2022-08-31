module.exports = {
  rules: {
    'node/no-unsupported-features/node-builtins': 'off',
  },
  ignorePatterns: ['index.d.ts'],
  parserOptions: {
    project: require.resolve('./tsconfig.json'),
  },
};
