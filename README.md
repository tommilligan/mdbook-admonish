# mdbook-admonish

[![Latest version](https://img.shields.io/crates/v/mdbook-admonish.svg)](https://crates.io/crates/mdbook-admonish)
[![docs.rs](https://img.shields.io/badge/docs-available-brightgreen)](https://tommilligan.github.io/mdbook-admonish/)

A preprocessor for [mdbook](https://github.com/rust-lang-nursery/mdBook) to add [Material Design](https://material.io/design) admonishments, based on the [mkdocs-material](https://squidfunk.github.io/mkdocs-material/reference/admonitions/) implementation.

It turns this:

````
```admonish info
A beautifully styled message.
```
````

into this:

![Simple Message](img/simple-message.png)

## Examples

Read the documentation [here](https://tommilligan.github.io/mdbook-admonish/), to see the actual examples in action. You can see the source in the [`./book`](./book) subdirectory.

Other projects using mdbook-admonish:

- [The Rhai Book](https://rhai.rs/book/)

## Usage

Use any [fenced code-block](https://spec.commonmark.org/0.30/#fenced-code-blocks) as you normally would, but annotate it with `admonish <admonition type>`:

````
```admonish example
My example is the best!
```
````

![Best Example](img/best-example.png)

See the [reference page](https://tommilligan.github.io/mdbook-admonish/reference.html) for a list of supported admonitions. You'll find:

- `info`
- `warning`
- `danger`
- `example`

and quite a few more!

You can also leave out the admonition type altogether, in which case it will default to `note`:

````
```admonish
A plain note.
```
````

![Plain Note](img/plain-note.png)

### Additional Options

See the [`mdbook-admonish` book](https://tommilligan.github.io/mdbook-admonish/) for additional options, such as:

- Custom titles
- Custom styling
- Collapsible blocks

## Installation

Install the tool:

```bash
cargo install mdbook-admonish

# If you get compilation/installation errors, try a locked installation
cargo install mdbook-admonish --locked
```

Then let `mdbook-admonish` add the required files and configuration:

```bash
# Note: this may need to be rerun for new minor versions of mdbook-admonish
# see the 'Semantic Versioning' section below for details.
mdbook-admonish install path/to/your/book

# optionally, specify a directory where CSS files live, relative to the book root
mdbook-admonish install --css-dir ./assets/css .
```

This will add the following configuration to your `book.toml`:

```toml
[preprocessor.admonish]
command = "mdbook-admonish"

[output.html]
additional-css = ["./mdbook-admonish.css"]
```

and copy the file `mdbook-admonish.css` into your book's directory.

Then, build your book as usual:

```bash
mdbook path/to/book
```

### Reproducible builds

For a reproducible build suitable for use in CI or scripts, please:

- Pin to a specific version
- Install with lockfile dependencies
- Always install the latest CSS assets

```bash
cargo install mdbook-admonish --vers "1.5.0" --locked
mdbook-admonish install path/to/your/book
```

The Minimum Supported Rust Version (MSRV) is documented in `Cargo.toml`, and noted in the `CHANGELOG.md`. We aims to support around six months of stable Rust.

### Updates

**Please note**, when updating your version of `mdbook-admonish`, updated styles will not be applied unless you rerun `mdbook-admonish install` to update the additional CSS files in your book.

`mdbook` will fail the build if you require newer assets than you have installed:

```log
2022-04-26 12:27:52 [INFO] (mdbook::book): Book building has started
ERROR:
  Incompatible assets installed: required mdbook-admonish assets version '^2.0.0', but found '1.0.0'.
  Please run `mdbook-admonish install` to update installed assets.
2022-04-26 12:27:52 [ERROR] (mdbook::utils): Error: The "admonish" preprocessor exited unsuccessfully with exit status: 1 status
```

If you want to update across minor versions without breakage, you should always run `mdbook-admonish install`.

### Process included files

You can ensure that content inlined with `{{#include}}` is also processed by [setting the `after` option](https://rust-lang.github.io/mdBook/format/configuration/preprocessors.html#require-a-certain-order):

```toml
[preprocessor.admonish]
after = ["links"]
```

This will expand `include` directives, before expanding `admonish` blocks.

### Semantic Versioning

Guarantees provided are as follows:

- Major versions: Contain breaking changes to the user facing markdown API, or the public API of the crate itself.
- Minor versions: Feature release. May contain changes to generated CSS/HTML requiring `mdbook-admonish install` to be rerun.
  - **Note:** updating acrosss minor versions without running `mdbook-admonish install` to reinstall assets may break your build.
  - This is due to limitations in the `mdbook` preprocessor architecture. Relevant issues that may alleviate this:
    - https://github.com/rust-lang/mdBook/issues/1222
    - https://github.com/rust-lang/mdBook/issues/1687
    - https://github.com/rust-lang/mdBook/issues/1689
- Patch versions: Bug fixes only.

## Development

See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines on developing.

## Thanks

This utility is heavily drawn from and inspired by other projects, namely:

- [mdbook-mermaid](https://github.com/badboy/mdbook-mermaid)
- [mkdocs-material](https://github.com/squidfunk/mkdocs-material)
- [material-design-icons](https://github.com/google/material-design-icons)

The licences for these projects are included in the `licences` folder.
