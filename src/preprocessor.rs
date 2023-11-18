use anyhow::{anyhow, Result};
use mdbook::{
    book::{Book, BookItem},
    errors::Result as MdbookResult,
    preprocess::{Preprocessor, PreprocessorContext},
};

use crate::{
    book_config::{admonish_config_from_context, Config, RenderMode},
    markdown::preprocess,
    types::RenderTextMode,
};

pub struct Admonish;

impl Preprocessor for Admonish {
    fn name(&self) -> &str {
        "admonish"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> MdbookResult<Book> {
        let config = admonish_config_from_context(ctx)?;
        ensure_compatible_assets_version(&config)?;

        let on_failure = config.on_failure;
        let custom_flavours = config.custom;
        let admonition_defaults = config.default;

        // TODO remove
        eprintln!("loaded custom flavours: {custom_flavours:#?}");

        // Load what rendering we should do from config, falling back to a default
        let render_mode = config
            .renderer
            .get(&ctx.renderer)
            .and_then(|renderer| renderer.render_mode)
            .unwrap_or_else(|| {
                // By default only render html for the html renderer
                // For everything else, do nothing
                if &ctx.renderer == "html" {
                    RenderMode::Html
                } else {
                    RenderMode::Preserve
                }
            });
        let render_text_mode = match render_mode {
            RenderMode::Preserve => return Ok(book),
            RenderMode::Html => RenderTextMode::Html,
            RenderMode::Strip => RenderTextMode::Strip,
        };

        let mut res = None;
        book.for_each_mut(|item: &mut BookItem| {
            if let Some(Err(_)) = res {
                return;
            }

            if let BookItem::Chapter(ref mut chapter) = *item {
                res = Some(
                    preprocess(
                        &chapter.content,
                        on_failure,
                        // TODO fix
                        custom_flavours.clone(),
                        &admonition_defaults,
                        render_text_mode,
                    )
                    .map(|md| {
                        chapter.content = md;
                    }),
                );
            }
        });

        res.unwrap_or(Ok(())).map(|_| book)
    }

    fn supports_renderer(&self, _renderer: &str) -> bool {
        // We support all renderers, but will only actually take action
        // if configured to do so - or, if it's the html renderer
        true
    }
}

fn ensure_compatible_assets_version(config: &Config) -> Result<()> {
    use semver::{Version, VersionReq};

    const REQUIRES_ASSETS_VERSION: &str = std::include_str!("./REQUIRED_ASSETS_VERSION");
    let requirement = VersionReq::parse(REQUIRES_ASSETS_VERSION.trim()).unwrap();

    const USER_ACTION: &str = "Please run `mdbook-admonish install` to update installed assets.";
    const DOCS_REFERENCE: &str = "For more information, see: https://github.com/tommilligan/mdbook-admonish#semantic-versioning";

    let version = match &config.assets_version {
        Some(version) => version,
        None => {
            return Err(anyhow!(
                r#"ERROR:
  Incompatible assets installed: required mdbook-admonish assets version '{requirement}', but did not find a version.
  {USER_ACTION}
  {DOCS_REFERENCE}"#
            ))
        }
    };

    let version = Version::parse(version).unwrap();

    if !requirement.matches(&version) {
        return Err(anyhow!(
            r#"ERROR:
  Incompatible assets installed: required mdbook-admonish assets version '{requirement}', but found '{version}'.
  {USER_ACTION}
  {DOCS_REFERENCE}"#
        ));
    };
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};

    fn mock_book(content: &str) -> Book {
        serde_json::from_value(json!({
            "sections": [
                {
                    "Chapter": {
                        "name": "Chapter 1",
                        "content": content,
                        "number": [1],
                        "sub_items": [],
                        "path": "chapter_1.md",
                        "source_path": "chapter_1.md",
                        "parent_names": []
                    }
                }
            ],
            "__non_exhaustive": null
        }))
        .unwrap()
    }

    fn mock_context(admonish: &Value, renderer: &str) -> PreprocessorContext {
        let value = json!({
            "root": "/path/to/book",
            "config": {
                "book": {
                    "authors": ["AUTHOR"],
                    "language": "en",
                    "multilingual": false,
                    "src": "src",
                    "title": "TITLE"
                },
                "preprocessor": {
                    "admonish": admonish,
                }
            },
            "renderer": renderer,
            "mdbook_version": "0.4.21"
        });

        serde_json::from_value(value).unwrap()
    }

    #[test]
    fn run_html() {
        let content = r#"
````admonish title="Title"
```rust
let x = 10;
x = 20;
```
````
"#;
        let expected_content = r##"

<div id="admonition-title" class="admonition admonish-note">
<div class="admonition-title">

Title

<a class="admonition-anchor-link" href="#admonition-title"></a>
</div>
<div>

```rust
let x = 10;
x = 20;
```

</div>
</div>
"##;

        let ctx = mock_context(
            &json!({
                "assets_version": "4.0.0"
            }),
            "html",
        );
        let book = mock_book(content);
        let expected_book = mock_book(expected_content);

        assert_eq!(Admonish.run(&ctx, book).unwrap(), expected_book)
    }

    #[test]
    fn run_test_preserves_by_default() {
        let content = r#"
````admonish title="Title"
```rust
let x = 10;
x = 20;
```
````
"#;
        let ctx = mock_context(
            &json!({
                "assets_version": "4.0.0"
            }),
            "test",
        );
        let book = mock_book(content);
        let expected_book = book.clone();

        assert_eq!(Admonish.run(&ctx, book).unwrap(), expected_book)
    }

    #[test]
    fn run_test_can_strip() {
        let content = r#"
````admonish title="Title"
```rust
let x = 10;
x = 20;
```
````
"#;
        let expected_content = r#"

```rust
let x = 10;
x = 20;
```

"#;
        let ctx = mock_context(
            &json!({
                "assets_version": "4.0.0",
                "renderer": {
                    "test": {
                        "render_mode": "strip",
                    },
                },
            }),
            "test",
        );
        let book = mock_book(content);
        let expected_book = mock_book(expected_content);

        assert_eq!(Admonish.run(&ctx, book).unwrap(), expected_book)
    }
}
