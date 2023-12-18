# Contributing

## Workflow

Please submit a PR from a **new branch** in your fork.
Please do not submit a PR from your fork's `main` branch, as it makes collaborating on/editing the branch a pain.

## Project design

- Compiled CSS styles are built and committed from SCSS sources. See the `compile_assets` folder for details.
- `mdbook-admonish install` is responsible for delivering additional assets and configuration to a client book.
- `mdbook-admonish` is responsible for preprocessing book data, adding HTML that references compiled classnames.

## Scripts to get started

- `./scripts/install` installs other toolchains required for development
- `./scripts/check` runs a full CI check
- `./scripts/rebuild-book` rebuilds the reference book under `./book`. This is useful for integration testing locally.

## Making breaking changes in CSS

To make a breaking change in CSS, you should:

- Update the assets version in `./src/bin/assets/VERSION`
- Update the required assets version specifier in `./src/REQUIRED_ASSETS_VERSION`

You must make the next `mdbook-admonish` crate version at least a **minor** version bump.

## Releasing

Github workflows are setup such that pushing a `vX.Y.Z` tag will trigger a release to be cut.
