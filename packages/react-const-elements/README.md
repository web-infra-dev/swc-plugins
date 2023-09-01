# Quick Start

## Install

```bash
npm i swc-plugin-react-const-elements-plugin
```

Add this to your config

```json
{
  "jsc": {
    "experimental": {
      "plugins": [
        [
          "@modern-js/swc-react-const-elements-plugin",
          {}
        ]
      ]
    }
  }
}
```

## Options

immutableGlobals

- type: string[]

By default all global components are considered mutable, so ```<Comp />``` is never hoisted, if you are sure ```Comp``` is immutable, you can add it to `immutableGlobals` like:

```json
{
  "jsc": {
    "experimental": {
      "plugins": [
        [
          "@modern-js/swc-react-const-elements-plugin",
          {
            "immutableGlobals": ["Comp"]
          }
        ]
      ]
    }
  }
}
```

allowMutablePropsOnTags

- type: string[]

By default all expression and object literal are considered mutable, so ```<Card info={{ name: 'modern.js' }} />``` is never hoisted, if you are sure ```Card``` is fine if it has mutable props, you can add it to `allowMutablePropsOnTags` like:

```json
{
  "jsc": {
    "experimental": {
      "plugins": [
        [
          "@modern-js/swc-react-const-elements-plugin",
          {
            "allowMutablePropsOnTags": ["Card"]
          }
        ]
      ]
    }
  }
}
```
