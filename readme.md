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
      libraryName: 'foo'
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
type pluginImport = {
  libraryName: string;
  libraryDirectory?: string;

  style?: boolean | "css" | string | ((name: string) => string | undefined);
  styleLibraryDirectory?: string;

  camelToDashComponentName?: bool; // default to true
  transformToDefaultImport?: bool;

  customName?: string | ((name: string) => string | undefined);
  customStyleName?: string | ((name: string) => string | undefined);

  ignoreEsComponent?: string[];
  ignoreStyleComponent?: string[];
}[];
```

Ported from [babel-plugin-import](https://github.com/umijs/babel-plugin-import).

**libraryName**

- Type: `string`

The package that need to be transformed，eg. in `import { a } from 'foo'`, `**libraryName**` should be `foo`.

**libraryDirectory**

- Type: `string`
- Default: `lib`

The path prefix that to import. For example `Button` will be transformed to `some-lib/lib/button`.

**style**

- Type: `'css' | string | boolean | ((input: string) => string | undefined)`
- Default: `undefined`

If this field is not `undefined` or `false`, the plugin will import style for the given component path.

For example, let's say the original code is `import { Button } from 'foo'`.
If `style` is set to:
`true`, code will be extended by: `import 'foo/lib/button/style'`.
`'css`, code will be extended by: `import 'foo/lib/button/style/css'`.
`(path) => path + 'less'`, code will be extended by: `import 'foo/lib/button.less'`.

**styleLibraryDirectory**

- Type: `string`
- Default: `undefined`

If this field is set, `style` will be ignored.

This field decides the path of style to import, for example, set this to `'styles'`, `import { Button } from 'foo'` will become:

```ts
import Button from 'foo/lib/button';
import 'foo/styles/button';
```

**camelToDashComponentName**

- Type: `boolean`
- Default: `true`

Wether to transform specifier to kebab case when added to import path. Eg: `myText` to `foo/lib/my-text`.

**transformToDefaultImport**

- Type: `boolean`
- Default: `true`

Wether to transform this import expression to default import. Eg: `import { Button } from 'foo'` will be transformed to `import { Button } from 'foo/lib/button';`.

**customName**

- Type: `string | ((name: string) => string | undefined)`
- Default: `undefined`

You can use this to customize the transformation.

Assume the original code is:

```ts
import { Button, Input } from 'foo';
```

And set `customName` to:

```ts
const customName = (name: string) => {
  if (name === 'Button') {
    return undefined
  } else {
    return `foo/es/components/${name.toLowercase()}`
  }
}
```

Result:

```ts
import { Button } from 'foo';
import Input from 'foo/es/components/input';
```

**customStyleName**

- Type: `string | ((name: string) => string | undefined)`
- Default: `undefined`

The same with `customName`, but just for style import expression.

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

Note there is a small difference that `lodash-compat` is currently deprecated so we do not support `lodash-compat` package.

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

#### extensions.styledComponent

- Type:

```ts
boolean | {
  displayName?: boolean;
  // Enabled by default.
  ssr?: boolean;
  // Enabled by default.
  fileName?: boolean;
  // Empty by default.
  topLevelImportPaths?: string[];
  // Defaults to ["index"].
  meaninglessFileNames?: string[];
  // Enabled by default.
  cssProp?: boolean;
  // Empty by default.
  namespace?: string;
  // Not supported yet.
  minify?: boolean;
  // Not supported yet.
  transpileTemplateLiterals?: boolean;
  // Not supported yet.
  pure?: boolean;
};
```

More detail at https://nextjs.org/docs/advanced-features/compiler#styled-components

#### extensions.emotion

- Type:

```ts
boolean | {
  // default is true. It will be disabled when build type is production.
  sourceMap?: boolean,
  // default is 'dev-only'.
  autoLabel?: 'never' | 'dev-only' | 'always',
  // default is '[local]'.
  // Allowed values: `[local]` `[filename]` and `[dirname]`
  // This option only works when autoLabel is set to 'dev-only' or 'always'.
  // It allows you to define the format of the resulting label.
  // The format is defined via string where variable parts are enclosed in square brackets [].
  // For example labelFormat: "my-classname--[local]", where [local] will be replaced with the name of the variable the result is assigned to.
  labelFormat?: string,
  // default is undefined.
  // This option allows you to tell the compiler what imports it should
  // look at to determine what it should transform so if you re-export
  // Emotion's exports, you can still use transforms.
  importMap?: {
    [packageName: string]: {
      [exportName: string]: {
        canonicalImport?: [string, string],
        styledBaseImport?: [string, string],
      }
    }
  },
},
```

More detail at https://nextjs.org/docs/advanced-features/compiler#emotion

#### extensions.modernjsSsrLoaderId

- Type: `boolean`
- Default: `undefined`

Enable some transform only needed by Modern.js
