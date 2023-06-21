# Contributing Guide

## Setup

### Install pnpm

```bash
# Enable pnpm with corepack, only available on Node.js >= `v14.19.0`
corepack enable
```

### Install Dependencies

```bash
pnpm install
```

## Development

Run `build:dev` command to generate the binary dynamic link library compiled from the Rust code into the current directory for debugging purposes.

```bash
pnpm build:dev
```

The `index.js` will is will try to load the corresponding binary from the current directory in this case, see [NAPI - Getting started](https://napi.rs/docs/introduction/getting-started#package-installed-in-users-node_modules).

## Testing

```bash
# Run Rust test cases
cargo test

# Run Node.js test cases
pnpm run vitest
```

## Submitting Changes

### Add a Changeset

This repo is using [Changesets](https://github.com/changesets/changesets) to manage the versioning and changelogs.

If you've changed some packages, you need add a new changeset for the changes. Please run `change` command to select the changed packages and add the changeset info.

```sh
pnpm run changeset
```

### Format of PR titles

The format of PR titles follow Conventional Commits.

An example:

```
feat(plugin-swc): Add `xxx` config
^    ^    ^
|    |    |__ Subject
|    |_______ Scope
|____________ Type
```

## Publishing

1. Run [Release Pull Request](https://github.com/web-infra-dev/swc-plugins/actions/workflows/release-pull-request.yml) action to create a release pull request.
2. Run [Release](https://github.com/web-infra-dev/swc-plugins/actions/workflows/release.yml) action the release the packages and crates.
