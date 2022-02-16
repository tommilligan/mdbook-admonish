# mdbook-admonish

A preprocessor for [mdbook][] to add visual support for admonishments.

[mdbook]: https://github.com/rust-lang-nursery/mdBook

It turns this:

````
```admonish
A simple message.
```
````

into this:

![Simple Message](simple-message.png)

in your book.

(Styling based on [mkdocs-material](https://github.com/squidfunk/mkdocs-material))

## Installation

If you want to use only this preprocessor, install the tool:

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
additional-js = ["mdbook-admonish.css"]
```

Additionally it copies the files `mdbook-admonish.css` into your book's directory.

Finally, build your book:

```
mdbook path/to/book
```
