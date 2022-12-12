# SWC executable with some builtin plugins for Node.js users

## Quick start

### Install

Install the plugin by using:

```bash
# npm
npm install @modern-js/swc-plugins

# yarn
yarn add @modern-js/swc-plugins

# pnpm
pnpm install @modern-js/swc-plugins
```

### Usage

#### Transform

```ts
import { Compiler } from '@modern-js/swc-plugins';

const compiler = new Compiler({
  extensions: {
    pluginImport: [{
      fromSource: 'foo';
      replaceJs: {
        template: 'foo/lib/{{ member }}';
      };
    }]
  }
});
const { code, map } = compiler.transform(
  '/projects/my-app/index.js',
  'import { Button } from "foo"\nconsole.log(Button)',
);
```

#### Minify

```ts
import { minify } from '@modern-js/swc-plugins';
import * as fs from 'fs';

const filename = '/projects/my-app/index.js';
const { code, map } = minify(
  filename,
  fs.readFileSync(filename),
  config: JsMinifyOptions,
);

```

## Config

- Type: `PluginConfig`

```ts
type PluginConfig = {
  presetReact?: ReactConfig;
  presetEnv?: EnvConfig;
  jsMinify?: boolean | JsMinifyOptions;
  extensions?: Extensions;
};
```

### presetReact

- Type: [presetReact](https://swc.rs/docs/configuration/compilation#jsctransformreact) in SWC.

Ported from `@babel/preset-react`. The value you passed will be merged with default option.

Default option is:

```ts
{
  runtime: 'automatic',
}
```

### presetEnv

- Type: [presetEnv](https://swc.rs/docs/configuration/supported-browsers#options) in SWC.

Ported from `@babel/preset-env`. The value you passed will be merged with default option.

Default option is:

```ts
{
  targets: '', // automatic get browserslist from your project, so you don't have to set this field
  mode: 'usage',
}
```

### jsMinify

- Type: `boolean` or [compress configuration](https://terser.org/docs/api-reference.html#compress-options).

Default option is: `{ compress: {}, mangle: true }`.

If set it to `false`, then SWC minification will be disabled, if set it to `true` then will it applies default option. If you pass an object, then this object will be merged with default option.

### extensions

- Type: `Object`

Some plugins ported from Babel.

#### extensions.pluginImport

- type

```ts
type PluginImportOptions = Array<{
  fromSource: string;
  replaceJs?: {
    ignoreEsComponent?: string[];
    template?: string;
    replaceExpr?: (member: string) => string | false;
    transformToDefaultImport?: boolean;
  };
  replaceCss?: {
    ignoreStyleComponent?: string[];
    template?: string;
    replaceExpr?: (member: string) => string | false;
  };
}>;
```

Ported from [babel-plugin-import](https://github.com/umijs/babel-plugin-import).

**fromSource**

- Type: `string`

The package that need to be transformed，eg. in `import { a } from 'foo'`, `fromSource` should be `foo`.

**replaceJs.ignoreEsComponent**

- Type: `string[]`
- Default: `[]`

The import specifiers which don't need to be transformed.

**replaceJs.template**

- Type: `string`
- Default: `undefined`

Template that represents replacement, for example:

```ts
import { MyButton as Btn } from "foo";
```

If we set:

```ts
PluginSWC({
  extensions: {
    pluginImport: [
      {
        replaceJs: {
          template: "foo/es/{{member}}",
        },
      },
    ],
  },
});
```

Then the code above will be replaced to code below:

```ts
import Btn from "foo/es/MyButton";
```

We also put some naming conversion functions, take the above example again, if we set it to:

```ts
PluginSWC({
  extensions: {
    pluginImport: [
      {
        replaceJs: {
          template: "foo/es/{{ kebabCase member }}",
        },
      },
    ],
  },
});
```

It will be transformed to code below:

```ts
import Btn from "foo/es/my-button";
```

Besides `kebabCase`, there are also `camelCase`, `snakeCase`, `upperCase`, `lowerCase`.

**replaceJs.replaceExpr**

- Type: `(member: string) => string`
- Default: `undefined`

This is also used to replace import specifiers. The argument is the specifier that imported. eg. `a` in `import { a as b } from 'foo'`.
This function is called by `Rust`，and it needs to be synchronous.
We recommend `template` instead, because call `js` function through `node-api` will cause performance issue. `node-api` invokes `js` function actually put this `js` call inside a queue, and wait for this call to be executed, so if current `js` thread is busy, then this call will block `Rust` thread for a while.

**transformToDefaultImport**

- Type: `boolean`
- Default: `true`

Whether transform specifier to default specifier.

#### extensions.reactUtils

- Type: `Object`

```ts
type ReactUtilsOptions = {
  autoImportReact?: boolean;
  removeEffect?: boolean;
  removePropTypes?: {
    mode?: "remove" | "unwrap" | "unsafe-wrap";
    removeImport?: boolean;
    ignoreFilenames?: string[];
    additionalLibraries?: string[];
    classNameMatchers?: string[];
  };
};
```

Some little help utils for `React`.

**reactUtils.autoImportReact**

- Type: `boolean`

Automatically import `React` as global variable, eg: `import React from 'react'`.
Mostly used for generated `React.createElement`.

**reactUtils.removeEffect**

- Type: `boolean`

Remove `useEffect` call.

**reactUtils.removePropTypes**

- Type:

```ts
type RemovePropTypesOptions = {
  mode?: "remove" | "unwrap" | "unsafe-wrap";
  removeImport?: boolean;
  ignoreFilenames?: string[];
  additionalLibraries?: string[];
  classNameMatchers?: string[];
};
```

Remove `React` runtime type checking. This is ported from [@babel/plugin-react-transform-remove-prop-types](https://github.com/oliviertassinari/babel-plugin-transform-react-remove-prop-types), All the configurations remain the same.

#### extensions.lodash

- Type: `{ cwd?: string; ids?: string;}`
- Default: `{ cwd: process.cwd(), ids: [] }`

Ported from [@babel/plugin-lodash](https://github.com/lodash/babel-plugin-lodash).

#### extensions.modularize_imports

- Type:

```ts
{
  [packageName: string]: {
    transform: string;
    preventFullImport: boolean;
    skipDefaultConversion: boolean;
  }
}
```

More detail on Next.js [modularize-imports](https://nextjs.org/docs/advanced-features/compiler#modularize-imports)

#### extensions.lockCorejsVersion

- Type:

```ts
{
  corejs?: string,
  swcHelpers?: string
}
```

Use this to rewrite `core-js` and `@swc/helpers` import path, this is helpful if you are an author of a library, and that library code contains `@swc/helpers` import, but you don't want your user to specify `@swc/helpers` as dependencies, you can achieve that in the following way.

```ts
{
  extensions: {
    swcHelpers: require('path').dirname(require.resolve('@swc/helpers/package.json'))
  }
}
```

By doing so, the following code:

```ts
import { foo } from '@swc/helpers';
```

will become something like this:

```ts
import { foo } from '/project/node_modules/your-lib/node_modules/@swc/helpers';
```
