# mdbook 0.5.x Migration Summary

## Overview

`mdbook-admonish` was migrated from mdbook 0.4.x to mdbook 0.5.x.
Compatibility with mdbook 0.4.x was intentionally dropped as a breaking
change.

## Changes

- Replaced the `mdbook` dependency with `mdbook-preprocessor = "0.5"`.
- Updated the Rust edition to 2024 and the minimum supported Rust version to
  1.88.0.
- Updated preprocessor input parsing to use
  `mdbook_preprocessor::parse_input`.
- Added semver-based mdbook version compatibility warnings.
- Updated `supports_renderer` for the mdbook 0.5 API, which returns
  `Result<bool>`.
- Replaced the removed `Config::get_preprocessor` API with
  `Config::get::<toml::Value>("preprocessor.admonish")`.
- Updated book and test data from the mdbook 0.4 `sections` field to the
  mdbook 0.5 `items` field.
- Removed obsolete `multilingual` and `git-repository-url` book configuration
  fields.
- Added a local `unique_id_from_content` implementation because mdbook 0.5 no
  longer exposes mdbook's previous helper publicly.
- Updated CI and integration tooling to use mdbook 0.5.4.
- Updated the mdbook 0.5 HTML integration snapshot.
- Fixed clippy issues exposed by the newer Rust toolchain.

## Validation

The following checks were completed successfully:

- `cargo build --all-targets`
- `cargo fmt -- --check`
- `cargo clippy --all-targets -- -D warnings`
- End-to-end integration build using mdbook 0.5.4
- Generated integration configuration and custom CSS matched expectations.
- Generated HTML snapshot matched the updated expected output.

The existing `custom::test::verify_against_generated_css` test remains the
only failing unit test. It also fails on the unmodified base revision and
requires the repository's Node/yarn SCSS asset-generation step; it is
unrelated to the mdbook 0.5 migration.

## Installed Tool Versions

- mdbook: 0.5.4
- mdbook-admonish: 1.20.0, rebuilt from this branch
