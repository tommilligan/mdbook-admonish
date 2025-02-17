# Changelog

## v1.19.0

### Changed

- MSRV (minimum supported rust version) is now 1.76.0 ([#208](https://github.com/tommilligan/mdbook-admonish/pull/208))

### Fixed

- Fixed blocks not rendering correctly in indented list items. Thanks to [@JorelAli](https://github.com/JorelAli) for the bug report! ([#224](https://github.com/tommilligan/mdbook-admonish/pull/224))

## v1.18.0

### Changed

- Add ARIA attributes to generated blocks. Thanks to [@toastal](https://github.com/toastal) for suggesting this feature! ([#195](https://github.com/tommilligan/mdbook-admonish/pull/195))
  - Note: This subtly alters the emitted HTML, and could cause additional styles applied to blocks to break. Native `mdbook-admonish` styles are not affected.

### Fixed

- Fixed some valid configurations producing TOML serialization errors. Thanks to [@DianaNites](https://github.com/DianaNites) for reporting this! ([#197](https://github.com/tommilligan/mdbook-admonish/pull/197))

## v1.17.1

### Fixed

- Removed a stray debug statement ([#186](https://github.com/tommilligan/mdbook-admonish/pull/186))

## v1.17.0

### Changed

- Blocks should have key-value options separated by commas. Existing syntax remains is supported for back-compatibility. See [the documentation on Additional Options](https://tommilligan.github.io/mdbook-admonish/#additional-options) for more details ([#181](https://github.com/tommilligan/mdbook-admonish/pull/181))

### Fixed

- Titles contining `=` will now render correctly. Thanks to [@s00500](https://github.com/s00500) for the bug report! ([#181](https://github.com/tommilligan/mdbook-admonish/pull/181))

## v1.16.0

### Changed

- MSRV (minimum supported rust version) is now 1.74.0 ([#175](https://github.com/tommilligan/mdbook-admonish/pull/175))
- `custom` directives should now be configured under the `directive.custom` option. Existing `custom` configurations are supported for back compatibility ([#179](https://github.com/tommilligan/mdbook-admonish/pull/174))

### Added

- Make blocks `collapsible` on a per-directive basis. Thanks to [@yannickseurin](https://github.com/yannickseurin) for contributing this feature! ([#174](https://github.com/tommilligan/mdbook-admonish/pull/174))

### Fixed

- The `css_id_prefix` option now uses snake case for consistency (kebab case remains supported for back compatibility). Thanks to [@yannickseurin](https://github.com/yannickseurin) for fixing this! ([#173](https://github.com/tommilligan/mdbook-admonish/pull/173))

## 1.15.0

### Added

- Support [custom directives](https://tommilligan.github.io/mdbook-admonish/overview.html#custom-blocks) with the new `mdbook-admonish generate-custom` helper. See the [mdbook-admonish book](https://tommilligan.github.io/mdbook-admonish/overview.html#custom-blocks) for guidance. Thanks to [@Sky9x](https://github.com/Sky9x) for helping design this feature! ([#165](https://github.com/tommilligan/mdbook-admonish/pull/165))

### Fixed

- `additional-css` unix style path normalization. Thanks to [@carlocorradini](https://github.com/carlocorradini) for reporting and fixing! ([#163](https://github.com/tommilligan/mdbook-admonish/pull/163))

## 1.14.0

### Changed

- Styles version updated to `3.0.1`. Run `mdbook-admonish install` to update.

### Added

- You can now set custom CSS ids for admonition blocks with the `id` field. Thanks to [@Sky9x](https://github.com/Sky9x) for contributing this feature! ([#144](https://github.com/tommilligan/mdbook-admonish/pull/144))
  - You can also now customize the CSS id prefix with the config option `default.css_id_prefix`

### Fixed

- Improve rendering of blocks in print/PDF view. Thanks to [@csk111165](https://github.com/csk111165) for the report ([#152](https://github.com/tommilligan/mdbook-admonish/issues/152))
- Fix the default titles for `tldr` and `faq` directives looking bad. They now render as `TL;DR` and `FAQ` by default. Thanks [@joshka](https://github.com/joshka) for fixing this! ([#154](https://github.com/tommilligan/mdbook-admonish/pull/154))

## 1.13.1

### Changed

- Bumped internal `mdbook` version to `0.4.35` ([#142](https://github.com/tommilligan/mdbook-admonish/pull/142))

### Fixed

- Relaxed `clap` dependency to fix compilation error when using other `mdbook-*` plugins. Thanks to [@joshka](https://github.com/joshka) for the [report](https://github.com/tommilligan/mdbook-admonish/pull/141)! ([#142](https://github.com/tommilligan/mdbook-admonish/pull/142))

## 1.13.0

### Changed

- Required styles version is now `^3.0.0` (release `1.13.0`). Run `mdbook-admonish install` to update.
- Internal CSS classnames for directives are now prefixed with `admonish-`, so `warning` is now `admonish-warning`. This avoids a conflict with upstream classnames introduced in `mdbook 0.4.35`. Thanks to [@phoenixr-codes](https://github.com/phoenixr-codes) for the report and fix! ([#139](https://github.com/tommilligan/mdbook-admonish/pull/139))

### Fixed

- Some minor inconsistencies in SCSS (and downstream CSS) styles were fixed by adopting Prettier linting ([#138](https://github.com/tommilligan/mdbook-admonish/pull/138))

## 1.12.1

### Fixed

- Panic when searching for an indent in non-ASCII content. Thanks to [@CoralPink](https://github.com/CoralPink) for the report! ([#128](https://github.com/tommilligan/mdbook-admonish/pull/128))

## 1.12.0

### Added

- Admonitions are now supported when indented inside other elements, such as a list. Thanks to [@mattburgess](https://github.com/mattburgess) for the report! ([#124](https://github.com/tommilligan/mdbook-admonish/pull/124))

## 1.11.1

### Fixed

- Reverted internal dependency upgrades that unintentionally increased MSRV from 1.66.0 in 1.11.0

## 1.11.0 (yanked)

**Note:** This release has been yanked.

It unintentionally increased the MSRV from 1.66.0

### Changed

- `gnu` prebuilt binaries are now built on `ubuntu-20.04` to match `mdbook` binaries. Thanks to [@eitsupi](https://github.com/eitsupi) for the fix! ([#118](https://github.com/tommilligan/mdbook-admonish/pull/118))

### Added

- `aarch64-unknown-linux-musl` prebuilt binary now available ([#119](https://github.com/tommilligan/mdbook-admonish/pull/119))

## 1.10.2

### Fixed

- Fixed `cargo install mdbook-admonish` failing due to an internal dependency mismatch with `mdbook` ([#115](https://github.com/tommilligan/mdbook-admonish/pull/115))

## 1.10.1

### Fixed

- Only restyle `summary` elements generated by `mdbook-admonish`. Thanks to [@ImUrX](https://github.com/ImUrX) for the report and fix! ([#112](https://github.com/tommilligan/mdbook-admonish/pull/112))

## 1.10.0

### Changed

- MSRV (minimum supported rust version) is now 1.66.0 for mdbook v0.4.32 ([#109](https://github.com/tommilligan/mdbook-admonish/pull/109))

### Added

- Support `mdbook test` running doctests inside admonish blocks. Opt-in to this by setting `renderer.test.action_mode = "strip"` ([#109](https://github.com/tommilligan/mdbook-admonish/pull/109))
- Log a warning when an invalid admonish block is encountered ([#109](https://github.com/tommilligan/mdbook-admonish/pull/109))

### Fixed

- Document all `book.toml` configuration options [in the reference](https://tommilligan.github.io/mdbook-admonish/reference.html), some of which were previously undocumened ([#109](https://github.com/tommilligan/mdbook-admonish/pull/109))

## 1.9.0

### Changed

- Styles version updated to `2.0.1`. Run `mdbook-admonish install` to update.
- MSRV (minimum supported rust version) is now 1.64.0 for clap v4 ([#79](https://github.com/tommilligan/mdbook-admonish/pull/79))
- More verbose error messages for invalid TOML configurations ([#79](https://github.com/tommilligan/mdbook-admonish/pull/79))

### Added

- User can set book-wide default for title and collapsible properties ([#84](https://github.com/tommilligan/mdbook-admonish/pull/84)), thanks to [@ShaunSHamilton](https://github.com/ShaunSHamilton)

### Fixed

- Custom installation and CSS directories are now normalized ([#49](https://github.com/tommilligan/mdbook-admonish/pull/49))
- Fix title bars with no text rendering badly ([#83](https://github.com/tommilligan/mdbook-admonish/pull/83)), thanks to [@ShaunSHamilton](https://github.com/ShaunSHamilton)
- Better error message display on crash ([#48](https://github.com/tommilligan/mdbook-admonish/pull/48))
- Better support for commonmark code fence syntax ([#88](https://github.com/tommilligan/mdbook-admonish/pull/88), [#89](https://github.com/tommilligan/mdbook-admonish/pull/89))

## 1.8.0

### Changed

- MSRV (minimum supported rust version) is now 1.60.0 for clap v4

## 1.7.0

### Changed

- Required styles version is now `^2.0.0` (release `1.7.0`). Run `mdbook-admonish install` to update.

### Added

- Support key/value configuration ([#24](https://github.com/tommilligan/mdbook-admonish/pull/24), thanks [@gggto](https://github.com/gggto) and [@schungx](https://github.com/schungx) for design input)
- Support collapsible admonition bodies ([#26](https://github.com/tommilligan/mdbook-admonish/pull/26), thanks [@gggto](https://github.com/gggto) for the suggestion and implementation!)
- Make anchor links hoverable ([#27](https://github.com/tommilligan/mdbook-admonish/pull/27))
- Better handling for misconfigured admonitions ([#25](https://github.com/tommilligan/mdbook-admonish/pull/25))
  - Nicer in-book error messages
  - Option to fail the build instead

## 1.6.0

**Please note:** If updating from an older version, this release requires `mdboook-admonish install` to be rerun after installation.

This behaviour is [documented in the readme here](https://github.com/tommilligan/mdbook-admonish#semantic-versioning), and may appear in any future minor version release.

### Changed

- Required styles version is now `^1.0.0` (release `1.6.0`). Run `mdbook-admonish install` to update.

### Added

- Enforce updating installed styles when required for new features ([#19](https://github.com/tommilligan/mdbook-admonish/pull/19)
- Each admonition has a unique id. Click the header bar to navigate to the anchor link ([#19](https://github.com/tommilligan/mdbook-admonish/pull/19), thanks [@schungx](https://github.com/schungx) for the suggestion)

### Fixed

- Header bar overflow at some zoom levels on Firefox ([#21](https://github.com/tommilligan/mdbook-admonish/pull/21), thanks to [@sgoudham](https://github.com/sgoudham) for the report)

## 1.5.0

### Added

- Admonitions now have an autogenerated `id`, to support anchor links ([#16](https://github.com/tommilligan/mdbook-admonish/pull/16), thanks [@schungx](https://github.com/schungx) for the suggestion)

## 1.4.1

### Changed

- Bumped locked dependency versions (mdbook v0.4.18)

### Packaging

- Support building and releasing binary artefacts.

## 1.4.0

### Added

- Additional classnames can be specified using `directive.classname` syntax
- Support removing the title bar entirely

### Fixed

- Removed superfluous empty `<p>` tags in output

## 1.3.3

### Fixed

- Fixed compilation failure with no default features
- MSRV (minimum supported rust version) documented as 1.58.0

## 1.3.2

### Fixed

- Fixed incorrect admonition title/panic when terminating with non-ascii characters
- Updated readme to note double-JSON string escapes

## 1.3.1

### Fixed

- Flattened indentation of generated HTML, otherwise it's styled as a markdown code block
- Fixed edge cases where the info string changes length when parsed, causing title/body to be incorrectly split

## 1.3.0 (yanked)

**Note:** This release has been yanked.

It unintentionally introduced a serious parsing bug.

### Added

- Add additional examples and images in readme
- Allow markdown styling in title content

### Fixed

- Fix HTML being stripping from body content

## 1.2.0

### Added

- Support custom title text

## 1.1.0

### Added

- CSS rules for the builtin mdbook themes, to adjust card background color

## 1.0.1

### Fixed

- Crate metadata and README wording

## 1.0.0

### Added

- `admonish <admonition type>` support for code fences
- Admonition type support is parity with mkdocs as of release
