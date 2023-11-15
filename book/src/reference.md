# Reference

<!-- toc -->

## `book.toml` configuration

See below for all configuration options available to add in `book.toml`.

The options should all be nested under `preprocessor.admonish`; for example:

```toml
[preprocessor.admonish]
on_failure = "bail"

[preprocessor.admonish.default]
collapsible = true

[preprocessor.admonish.renderer.test]
render_mode = "strip"
```

### `on_failure`

Optional. Default value: `continue`.

The action to take when an invalid `admonish` block is encountered:

- `continue` (default): Continue processing future blocks, do not fail the build. If rendering to HTML, an error message will be displayed in the book output.
- `bail`: Abort the build.

### `default`

Optional.

Default values to use, when not provided in an `admonish` block explicitly.

Subfields:

- `default.title` (optional): Title to use for blocks. Defaults to the directive used in titlecase.
- `default.collapsible` (optional, default: `false`): Make blocks collapsible by default when set to `true`.
- `default.css_id_prefix` (optional, default: `"admonition-"`): The default css id prefix to add to the id of all blocks. Ignored on blocks with an `id` field.

### `renderer`

````admonish tip
It is recommended that you set:

```toml
[preprocessor.admonish.renderer.test]
render_mode = "strip"
```

This allows `mdbook test` to find and test rust examples within `admonish` blocks.

This will be the default behaviour in the next `mdbook-admonish` major version.
````

Optional.

Additional settings to apply, depending on the renderer that is running.

The most common renderers used are:

- `html`: Used by `mdbook build` to build the final book output.
- `test`: Used by `mdbook test` to find and run doctests.

Subfields:

- `renderer.<renderer_name>.render_mode` (optional): The action `mdbook-admonish` should take when running with this renderer.
  - Valid values:
    - `html`: Convert `admonish` blocks into HTML output.
    - `preserve`: Do nothing. Leave the book untouched.
    - `strip`: Strip `admonish`-specific syntax, leaving the inner content untouched.
  - Default values:
    - For the `html` renderer, the default value is `html`.
    - For all other renderers, the default value is `preserve`.

### `command`

Required.

Used by `mdbook` to know how to call the `mdbook-admonish` plugin.

Running this command with the `--version` flag from your shell should work, for the plugin to function.

### `assets_version`

Optional.

This is automatically updated by `mdbook-admonish install` and should not be edited.

## Directives

All supported directives are listed below.

`note`

```admonish note
Rust is a multi-paradigm, general-purpose programming language designed for performance and safety, especially safe concurrency.
```

`abstract`, `summary`, `tldr`

```admonish abstract
Rust is a multi-paradigm, general-purpose programming language designed for performance and safety, especially safe concurrency.
```

`info`, `todo`

```admonish info
Rust is a multi-paradigm, general-purpose programming language designed for performance and safety, especially safe concurrency.
```

`tip`, `hint`, `important`

```admonish tip
Rust is a multi-paradigm, general-purpose programming language designed for performance and safety, especially safe concurrency.
```

`success`, `check`, `done`

```admonish success
Rust is a multi-paradigm, general-purpose programming language designed for performance and safety, especially safe concurrency.
```

`question`, `help`, `faq`

```admonish question
Rust is a multi-paradigm, general-purpose programming language designed for performance and safety, especially safe concurrency.
```

`warning`, `caution`, `attention`

```admonish warning
Rust is a multi-paradigm, general-purpose programming language designed for performance and safety, especially safe concurrency.
```

`failure`, `fail`, `missing`

```admonish failure
Rust is a multi-paradigm, general-purpose programming language designed for performance and safety, especially safe concurrency.
```

`danger`, `error`

```admonish danger
Rust is a multi-paradigm, general-purpose programming language designed for performance and safety, especially safe concurrency.
```

`bug`

```admonish bug
Rust is a multi-paradigm, general-purpose programming language designed for performance and safety, especially safe concurrency.
```

`example`

```admonish example
Rust is a multi-paradigm, general-purpose programming language designed for performance and safety, especially safe concurrency.
```

`quote`, `cite`

```admonish quote
Rust is a multi-paradigm, general-purpose programming language designed for performance and safety, especially safe concurrency.
```
