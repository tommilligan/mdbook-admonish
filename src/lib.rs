use mdbook::book::{Book, BookItem, Chapter};
use mdbook::errors::Result;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use pulldown_cmark::{CodeBlockKind::*, Event, Options, Parser, Tag};

pub struct Admonish;

impl Preprocessor for Admonish {
    fn name(&self) -> &str {
        "admonish"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        let mut res = None;
        book.for_each_mut(|item: &mut BookItem| {
            if let Some(Err(_)) = res {
                return;
            }

            if let BookItem::Chapter(ref mut chapter) = *item {
                res = Some(Admonish::add_admonish(chapter).map(|md| {
                    chapter.content = md;
                }));
            }
        });

        res.unwrap_or(Ok(())).map(|_| book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html"
    }
}

fn escape_html(s: &str) -> String {
    let mut output = String::new();
    for c in s.chars() {
        match c {
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '"' => output.push_str("&quot;"),
            '&' => output.push_str("&amp;"),
            _ => output.push(c),
        }
    }
    output
}

fn add_admonish(content: &str) -> Result<String> {
    let mut admonish_content = String::new();
    let mut in_admonish_block = false;

    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TASKLISTS);

    let mut admonish_start = 0..0;

    let mut admonish_blocks = vec![];

    let events = Parser::new_ext(content, opts);
    for (e, span) in events.into_offset_iter() {
        if let Event::Start(Tag::CodeBlock(Fenced(code))) = e.clone() {
            log::debug!("e={:?}, span={:?}", e, span);
            if &*code == "admonish" {
                admonish_start = span;
                in_admonish_block = true;
                admonish_content.clear();
            }
            continue;
        }

        if !in_admonish_block {
            continue;
        }

        if let Event::End(Tag::CodeBlock(Fenced(code))) = e {
            assert_eq!(
                "admonish", &*code,
                "After an opening admonish code block we expect it to close again"
            );
            in_admonish_block = false;
            let pre = "```admonish\n";
            let post = "```";

            let admonish_content =
                &content[admonish_start.start + pre.len()..span.end - post.len()];
            let admonish_content = escape_html(admonish_content);
            let admonish_content = admonish_content.trim();
            let admonish_code = format!(
                r#"<div class="admonition note">
  <p class="admonition-title">Note</p>
  <p>{admonish_content}</p>
</div>"#
            );
            admonish_blocks.push((admonish_start.start..span.end, admonish_code.clone()));
        }
    }

    let mut content = content.to_string();
    for (span, block) in admonish_blocks.iter().rev() {
        let pre_content = &content[0..span.start];
        let post_content = &content[span.end..];
        content = format!("{}\n{}{}", pre_content, block, post_content);
    }
    Ok(content)
}

impl Admonish {
    fn add_admonish(chapter: &mut Chapter) -> Result<String> {
        add_admonish(&chapter.content)
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::add_admonish;

    #[test]
    fn adds_admonish() {
        let content = r#"# Chapter
```admonish
A simple admonition.
```
Text
"#;

        let expected = r#"# Chapter

<div class="admonition note">
  <p class="admonition-title">Note</p>
  <p>A simple admonition.</p>
</div>
Text
"#;

        assert_eq!(expected, add_admonish(content).unwrap());
    }

    #[test]
    fn leaves_tables_untouched() {
        // Regression test.
        // Previously we forgot to enable the same markdwon extensions as mdbook itself.

        let content = r#"# Heading
| Head 1 | Head 2 |
|--------|--------|
| Row 1  | Row 2  |
"#;

        let expected = r#"# Heading
| Head 1 | Head 2 |
|--------|--------|
| Row 1  | Row 2  |
"#;

        assert_eq!(expected, add_admonish(content).unwrap());
    }

    #[test]
    fn leaves_html_untouched() {
        // Regression test.
        // Don't remove important newlines for syntax nested inside HTML

        let content = r#"# Heading
<del>
*foo*
</del>
"#;

        let expected = r#"# Heading
<del>
*foo*
</del>
"#;

        assert_eq!(expected, add_admonish(content).unwrap());
    }

    #[test]
    fn html_in_list() {
        // Regression test.
        // Don't remove important newlines for syntax nested inside HTML

        let content = r#"# Heading
1. paragraph 1
   ```
   code 1
   ```
2. paragraph 2
"#;

        let expected = r#"# Heading
1. paragraph 1
   ```
   code 1
   ```
2. paragraph 2
"#;

        assert_eq!(expected, add_admonish(content).unwrap());
    }

    #[test]
    fn escape_in_admonish_block() {
        env_logger::init();
        let content = r#"
```admonish
classDiagram
    class PingUploader {
        <<interface>>
        +Upload() UploadResult
    }
```
hello
"#;

        let expected = r#"

<div class="admonition note">
  <p class="admonition-title">Note</p>
  <p>classDiagram
    class PingUploader {
        &lt;&lt;interface&gt;&gt;
        +Upload() UploadResult
    }</p>
</div>
hello
"#;

        assert_eq!(expected, add_admonish(content).unwrap());
    }
}
