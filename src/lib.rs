use mdbook::book::{Book, BookItem, Chapter};
use mdbook::errors::Result;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use pulldown_cmark::{CodeBlockKind::*, Event, Options, Parser, Tag};
use std::str::FromStr;

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

#[derive(Debug, PartialEq)]
enum Directive {
    Note,
    Abstract,
    Info,
    Tip,
    Success,
    Question,
    Warning,
    Failure,
    Danger,
    Bug,
    Example,
    Quote,
}

impl FromStr for Directive {
    type Err = ();

    fn from_str(string: &str) -> std::result::Result<Self, ()> {
        match string {
            "note" => Ok(Self::Note),
            "abstract" | "summary" | "tldr" => Ok(Self::Abstract),
            "info" | "todo" => Ok(Self::Info),
            "tip" | "hint" | "important" => Ok(Self::Tip),
            "success" | "check" | "done" => Ok(Self::Success),
            "question" | "help" | "faq" => Ok(Self::Question),
            "warning" | "caution" | "attention" => Ok(Self::Warning),
            "failure" | "fail" | "missing" => Ok(Self::Failure),
            "danger" | "error" => Ok(Self::Danger),
            "bug" => Ok(Self::Bug),
            "example" => Ok(Self::Example),
            "quote" | "cite" => Ok(Self::Quote),
            _ => Err(()),
        }
    }
}

fn parse_info_string(info_string: &str) -> Option<Option<&str>> {
    if info_string == "admonish" {
        return Some(None);
    }

    match info_string.split_once(' ') {
        Some(("admonish", directive)) => Some(Some(directive)),
        _ => None,
    }
}

// https://stackoverflow.com/a/38406885
fn ucfirst(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

fn add_admonish(content: &str) -> Result<String> {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TASKLISTS);

    let mut admonish_blocks = vec![];

    let events = Parser::new_ext(content, opts);
    for (e, span) in events.into_offset_iter() {
        if let Event::Start(Tag::CodeBlock(Fenced(info_string))) = e.clone() {
            let directive_raw = match parse_info_string(info_string.as_ref()) {
                Some(directive) => directive,
                None => continue,
            };
            let directive = directive_raw
                .map(|directive| Directive::from_str(directive).ok())
                .flatten()
                .unwrap_or(Directive::Note);

            const PRE_START: &str = "```";
            const PRE_END: &str = "\n";
            const POST: &str = "```";

            let start_index = span.start + PRE_START.len() + info_string.len() + PRE_END.len();
            let end_index = span.end - POST.len();

            let admonish_content = &content[start_index..end_index];
            let admonish_content = escape_html(admonish_content);
            let admonish_content = admonish_content.trim();
            let directive_title = directive_raw
                .map(ucfirst)
                .unwrap_or_else(|| "Note".to_owned());
            let directive_classname = match directive {
                Directive::Note => "note",
                Directive::Abstract => "abstract",
                Directive::Info => "info",
                Directive::Tip => "tip",
                Directive::Success => "success",
                Directive::Question => "question",
                Directive::Warning => "warning",
                Directive::Failure => "failure",
                Directive::Danger => "danger",
                Directive::Bug => "bug",
                Directive::Example => "example",
                Directive::Quote => "quote",
            };
            let admonish_code = format!(
                r#"<div class="admonition {directive_classname}">
  <p class="admonition-title">{directive_title}</p>
  <p>{admonish_content}</p>
</div>"#
            );
            admonish_blocks.push((span, admonish_code.clone()));
        }
    }

    let mut content = content.to_string();
    for (span, block) in admonish_blocks.iter().rev() {
        let pre_content = &content[..span.start];
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

    use super::*;

    #[test]
    fn test_parse_info_string() {
        assert_eq!(parse_info_string(""), None);
        assert_eq!(parse_info_string("adm"), None);
        assert_eq!(parse_info_string("admonish"), Some(None));
        assert_eq!(parse_info_string("admonish "), Some(Some("")));
        assert_eq!(parse_info_string("admonish unknown"), Some(Some("unknown")));
        assert_eq!(parse_info_string("admonish note"), Some(Some("note")));
        assert_eq!(parse_info_string("admonish warning"), Some(Some("warning")));
    }

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
    fn adds_admonish_directive() {
        let content = r#"# Chapter
```admonish warning
A simple admonition.
```
Text
"#;

        let expected = r#"# Chapter

<div class="admonition warning">
  <p class="admonition-title">Warning</p>
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
