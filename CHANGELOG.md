# @modern-js/swc-plugins

## 0.6.6

### Patch Changes

- 85766de: chore: bump swc
- 85766de: chore: bump swc

## 0.6.5

### Patch Changes

- b3cecf3: chore: bump swc

## 0.6.4

### Patch Changes

- f2c7812: chore: update swc_core
- 5ff9680: fix: use files instead of npmignore

## 0.6.3

### Patch Changes

- 71b639e: fix: adapte syntax based on filename, do not transform core-js-pure

## 0.6.2

### Patch Changes

- 4791282: chore: publish

## 0.6.1

### Patch Changes

- a3e1f6b: feat: add wasm plugins

## 0.6.0

### Minor Changes

- 962d82a: feat: add plugin-react-const-elements

### Patch Changes

- bc6ac50: fix: dont modify original config
- 77ebf43: chore: bump swc
- c26ff21: chore: bump swc
- 2bb898e: chore: bump swc
- 88c3a0d: refactor: move tests to integration_tests crate

## 0.5.7

### Patch Changes

- d7d7c48: chore: bump swc

## 0.5.6

### Patch Changes

- 2b75e9a: chore: upgrade rust, optimize binary size
- b2d9a83: feat: bump swc_core automatically
- f9f49be: fix: fix mangle options type def

## 0.5.5

### Patch Changes

- 691ad81: fix: remove pnpm engine limitation

## 0.5.4

### Patch Changes

- 699354f: feat: enable concurrent minifier

## 0.5.3

### Patch Changes

- 9e68e36: fix(ci): disable wasm support for aarch64-windows, fix the unexpected git tag in ci
- 494272d: feat: enable wasm feature

## 0.5.2

### Patch Changes

- f6a92c2: feat: enable wasm feature, upgrade swc_core

## 0.5.1

### Patch Changes

- c22c44b: chore: upgrade swc_core

## 0.5.0

### Minor Changes

- fb5dfd3: refactor: remove auto detect esmodule

### Patch Changes

- 668e31e: chore: bump dependencies
- 668e31e: chore: upgrade swc_core
- 3810c93: fix: plugin ssr-loader-id should appy `modify_loader_call` for it's children
- d2dfd89: fix(CI): enable corepack in docker

## 0.4.0

### Minor Changes

- 40a7e83: feat: add ssr-loader-id plugin
- 1835d9e: feat: add config-routes plugin

## 0.3.5

### Patch Changes

- 8f7254c: fix(plugin-lodash): invalid local name

## 0.3.4

### Patch Changes

- aa14f49: fix: extensions.styledComponents typo
- 0b9835a: fix: incorrect lodash.ids type

## 0.3.3

### Patch Changes

- e2247ed: chore: bump dependencies

## 0.3.2

### Patch Changes

- 583e238: fix: we must use same comments obj to parser, transform && we must put the loadable-components in after pass.
- 11d6883: chore: update dependencies
- 0dc8f4c: chore: bump swc_core

## 0.3.1

### Patch Changes

- 018f6d8: feat: add peer dep @swc/helpers

## 0.3.0

### Minor Changes

- fb154c1: feat: add loadable-componnets-plugin

### Patch Changes

- fb154c1: chore: bump swc
- fb154c1: fix: add loadable compnents plugins ts type.
- fb154c1: fix: set thread-local when uses css minify
- fb154c1: chore: upgrade swc
- fb154c1: refactor: run multi-platform compilation only when we are ready to release
- fb154c1: feat: stay compatible with babel-plugin-import

## 0.2.0

### Minor Changes

- 1ab9cef: feat: add css minify

## 0.1.0

### Minor Changes

- ba1b5f3: feat: separate the plugin from the core layer
  feat: 将插件从 core 层分离出去
- 2941240: feat: prepare for publish

### Patch Changes

- 404a3a5: fix: fix README type
- c4eadbc: fix(modern-swc-binding): export more binding struct
- c48c5e0: fix: sink more types in swc_plugins_collections
- 0ec9f4f: feat: Add changeset to CI
- 1399722: feat: add error message and advice if not find any binary
- 082f93e: fix: fix wrong rust toolchain on CI
- 08de761: feat: support publish crate on CI
