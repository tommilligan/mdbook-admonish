## Changelog

## Unreleased

### Added

- Additional classnames can be specified using `directive.classname` syntax

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

## 1.3.0

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
