# mdbook-admonish

A preprocessor for [mdbook](https://github.com/rust-lang-nursery/mdBook) to add Material UI admonishments.

It turns this:

````
```admonish info
A beautifully styled message.
```
````

into this:

![Simple Message](simple-message.png)

Styling is based on [mkdocs-material](https://github.com/squidfunk/mkdocs-material).

## Installation

Install the tool:

```
cargo install mdbook-admonish
```

Then let `mdbook-admonish` add the required files and configuration:

```
mdbook-admonish install path/to/your/book
```

This will add the following configuration to your `book.toml`:

```toml
[preprocessor.admonish]
command = "mdbook-admonish"

[output.html]
additional-css = ["mdbook-admonish.css"]
```

and copy the file `mdbook-admonish.css` into your book's directory.

Then, build your book as usual:

```
mdbook path/to/book
```

## Development

Project design

- Compiled CSS styles are built and committed from SCSS sources. See the `compile_assets` folder for details.
- `mdbook-admonish install` is responsible for delivering additional assets and configuration to a client book.
- `mdbook-admonish` is responsible for preprocessing book data, adding HTML that references compiled classnames.

## Thanks

This utility is heavily drawn from and inspired by other projects, namely:

- [mdbook-mermaid](https://github.com/badboy/mdbook-mermaid)
- [mkdocs-material](https://github.com/squidfunk/mkdocs-material)
- [material-design-icons](https://github.com/google/material-design-icons)

The licences for these projects are included in the `licences` folder.
