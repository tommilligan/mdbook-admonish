# mdbook-admonish

[![Latest version](https://img.shields.io/crates/v/mdbook-admonish.svg)](https://crates.io/crates/mdbook-admonish)
[![docs.rs](https://img.shields.io/docsrs/mdbook-admonish)](https://docs.rs/mdbook-admonish)

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

Read the usage and reference [here](https://tommilligan.github.io/mdbook-admonish/), to see the actual examples in action. You can see the source in the [`./book`](./book) subdirectory.

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

#### Custom title

A custom title can be provided, contained in a double quoted JSON string.
Note that JSON escapes must be escaped again - for instance, write `\"` as `\\"`.

````
```admonish warning "Data loss"
The following steps can lead to irrecoverable data corruption.
```
````

![Data Loss](img/data-loss.png)

You can also remove the title bar entirely, by specifying the empty string:

````
```admonish success ""
This will take a while, go and grab a drink of water.
```
````

![No Title Bar](img/no-title-bar.png)

#### Nested Markdown/HTML

Markdown and HTML can be used in the inner content, as you'd expect:

````
```admonish tip "_Referencing_ and <i>dereferencing</i>"
The opposite of *referencing* by using `&` is *dereferencing*, which is
accomplished with the <span style="color: hotpink">dereference operator</span>, `*`.
```
````

![Complex Message](img/complex-message.png)

If you have code blocks you want to include in the content, use [tildes for the outer code fence](https://spec.commonmark.org/0.30/#fenced-code-blocks):

````
~~~admonish bug
This syntax won't work in Python 3:
```python
print "Hello, world!"
```
~~~
````

![Code Bug](img/code-bug.png)

#### Custom styling

If you want to provide custom styling to a specific admonition, you can attach one or more custom classnames:

````
```admonish note.custom-0.custom-1
Styled with my custom CSS class.
```
````

Will yield something like the following HTML, which you can then apply styles to:

```html
<div class="admonition note custom-0 custom-1"
    ...
</div>
```

## Installation

Install the tool:

```bash
cargo install mdbook-admonish
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

Alternatively, pin to a specific version for a reproducible installation:

```bash
cargo install mdbook-admonish --vers "1.5.0" --locked
```

### Semantic Versioning

Guarantees provided are as follows:

- Major versions: Contain breaking changes to the user facing markdown API, or the public API of the crate itself.
- Minor versions: Feature release. May contain changes to generated CSS/HTML requiring `mdbook-admonish install` to be rerun.
  - **Note:** updating acrosss minor versions without running `mdbook-admonish install` to reinstall assets may break your build.
- Patch versions: Bug fixes only.

## Development

Project design

- Compiled CSS styles are built and committed from SCSS sources. See the `compile_assets` folder for details.
- `mdbook-admonish install` is responsible for delivering additional assets and configuration to a client book.
- `mdbook-admonish` is responsible for preprocessing book data, adding HTML that references compiled classnames.

### Scripts to get started

- `./scripts/install` installs other toolchains required for development
- `./scripts/check` runs a full CI check
- `./scripts/rebuild-book` rebuilds the reference book under `./book`. This is useful for integration testing locally.

### Making breaking changes in CSS

To make a breaking change in CSS, you should:

- Update the assets version in `./src/bin/assets/VERSION`
- Update the required assets version specifier in `./src/REQUIRED_ASSETS_VERSION`

You must make the next `mdbook-admonish` crate version at least a **minor** version bump.

### Releasing

Github workflows are setup such that pushing a `vX.Y.Z` tag will trigger a release to be cut.

Once the release is created, copy and paste the relevant section of `CHANGELOG.md` manually to update the description.

## Thanks

This utility is heavily drawn from and inspired by other projects, namely:

- [mdbook-mermaid](https://github.com/badboy/mdbook-mermaid)
- [mkdocs-material](https://github.com/squidfunk/mkdocs-material)
- [material-design-icons](https://github.com/google/material-design-icons)

The licences for these projects are included in the `licences` folder.
