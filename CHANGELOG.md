# @modern-js/swc-plugins

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
