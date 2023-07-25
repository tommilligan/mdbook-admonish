# mdbook-admonish

<!-- toc -->

## Intoduction

[![Latest version](https://img.shields.io/crates/v/mdbook-admonish.svg)](https://crates.io/crates/mdbook-admonish)
[![docs.rs](https://img.shields.io/docsrs/mdbook-admonish)](https://docs.rs/mdbook-admonish)

A preprocessor for [mdbook](https://github.com/rust-lang/mdBook) to add [Material Design](https://material.io/design) admonishments, based on the [mkdocs-material](https://squidfunk.github.io/mkdocs-material/reference/admonitions/) implementation.

It turns this:

````
```admonish info
A beautifully styled message.
```
````

into this:

```admonish info
A beautifully styled message.
```

## Usage

### A basic `admonish` block

Use any [fenced code-block](https://spec.commonmark.org/0.30/#fenced-code-blocks) as you normally would, but annotate it with `admonish <admonition type>`:

````
```admonish example
My example is the best!
```
````

```admonish example
My example is the best!
```

See the [list of directives](./reference.md#directives) for a full list of supported admonitions. You'll find:

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

```admonish
A plain note.
```

### Invalid blocks

By default, if an `admonish` block cannot be parsed, an error will be rendered in the output:

````
```admonish title="\j"
This block will error
```
````

```admonish title="\j"
This block will error
```

You can also configure the build to fail loudly, by setting `on_failure = "bail"` in `book.toml`. See the [configuration reference](./reference.md#booktoml-configuration) for more details.

### Additional Options

You can pass additional options to each block. The options are structured as TOML key-value pairs.

Note that some options can be passed globally, through the `default` section in `book.toml`. See the [configuration reference](./reference.mdbooktoml-configuration) for more details.

#### Custom title

A custom title can be provided, contained in a double quoted TOML string.
Note that TOML escapes must be escaped again - for instance, write `\"` as `\\"`.

````
```admonish warning title="Data loss"
The following steps can lead to irrecoverable data corruption.
```
````

```admonish warning title="Data loss"
The following steps can lead to irrecoverable data corruption.
```

You can also remove the title bar entirely, by specifying the empty string:

````
```admonish success title=""
This will take a while, go and grab a drink of water.
```
````

```admonish success title=""
This will take a while, go and grab a drink of water.
```

#### Nested Markdown/HTML

Markdown and HTML can be used in the inner content, as you'd expect:

````
```admonish tip title="_Referencing_ and <i>dereferencing</i>"
The opposite of *referencing* by using `&` is *dereferencing*, which is
accomplished with the <span style="color: hotpink">dereference operator</span>, `*`.
```
````

```admonish tip title="_Referencing_ and <i>dereferencing</i>"
The opposite of *referencing* by using `&` is *dereferencing*, which is
accomplished with the <span style="color: hotpink">dereference operator</span>, `*`.
```

If you have code blocks you want to include in the content, use [tildes for the outer code fence](https://spec.commonmark.org/0.30/#fenced-code-blocks):

````
~~~admonish bug
This syntax won't work in Python 3:
```python
print "Hello, world!"
```
~~~
````

```admonish bug
This syntax won't work in Python 3:
~~~python
print "Hello, world!"
~~~
```

#### Custom styling

If you want to provide custom styling to a specific admonition, you can attach one or more custom classnames:

````
```admonish note class="custom-0 custom-1"
Styled with my custom CSS class.
```
````

Will yield something like the following HTML, which you can then apply styles to:

```html
<div class="admonition note custom-0 custom-1"
    ...
</div>
```

#### Collapsible

For a block to be initially collapsible, and then be openable, set `collapsible=true`:

````
```admonish collapsible=true
Content will be hidden initially.
```
````

Will yield something like the following HTML, which you can then apply styles to:

```admonish collapsible=true
Content will be hidden initially.
```
