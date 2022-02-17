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
