use mdbook::book::{Book, BookItem, Chapter};
use mdbook::errors::Result as MdbookResult;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use pulldown_cmark::{CodeBlockKind::*, Event, Options, Parser, Tag};
use std::borrow::Cow;
use std::str::FromStr;

pub struct Admonish;

impl Preprocessor for Admonish {
    fn name(&self) -> &str {
        "admonish"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> MdbookResult<Book> {
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

    fn from_str(string: &str) -> Result<Self, ()> {
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

impl Directive {
    fn classname(&self) -> &'static str {
        match self {
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
        }
    }
}

#[derive(Debug, PartialEq)]
struct AdmonitionInfo<'a> {
    directive: &'a str,
    title: Option<&'a str>,
}

#[derive(Debug, PartialEq)]
struct Admonition<'a> {
    directive: Directive,
    title: Cow<'a, str>,
}

impl<'a> Default for Admonition<'a> {
    fn default() -> Self {
        Self {
            directive: Directive::Note,
            title: Cow::Borrowed("Note"),
        }
    }
}

impl<'a> TryFrom<AdmonitionInfo<'a>> for Admonition<'a> {
    type Error = ();

    fn try_from(other: AdmonitionInfo<'a>) -> Result<Self, ()> {
        let directive = Directive::from_str(other.directive)?;
        Ok(Self {
            directive,
            title: other
                .title
                .map(Cow::Borrowed)
                .unwrap_or_else(|| Cow::Owned(ucfirst(other.directive))),
        })
    }
}

/// Returns:
/// - `None` if this is not an `admonish` block.
/// - `Some(None)` if this is an `admonish` block, but no further configuration was given
/// - `Some(AdmonitionInfo)` if this is an `admonish` block, and further configuration was given
fn parse_info_string(info_string: &str) -> Option<Option<AdmonitionInfo>> {
    if info_string == "admonish" {
        return Some(None);
    }

    let directive_title = match info_string.split_once(' ') {
        Some(("admonish", rest)) => rest,
        _ => return None,
    };

    let info = if let Some((directive, title)) = directive_title.split_once(' ') {
        // The title is expected to be a quoted JSON string
        let title = serde_json::from_str(title).ok();
        AdmonitionInfo { directive, title }
    } else {
        AdmonitionInfo {
            directive: directive_title,
            title: None,
        }
    };

    Some(Some(info))
}

/// Make the first letter of `input` upppercase.
///
/// source: https://stackoverflow.com/a/38406885
fn ucfirst(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

fn add_admonish(content: &str) -> MdbookResult<String> {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TASKLISTS);

    let mut admonish_blocks = vec![];

    let events = Parser::new_ext(content, opts);
    for (e, span) in events.into_offset_iter() {
        if let Event::Start(Tag::CodeBlock(Fenced(info_string))) = e.clone() {
            let info = match parse_info_string(info_string.as_ref()) {
                Some(info) => info,
                None => continue,
            };
            let admonition = info
                .map(|info| Admonition::try_from(info).unwrap_or_default())
                .unwrap_or_default();

            const PRE_START: &str = "```";
            const PRE_END: &str = "\n";
            const POST: &str = "```";

            let start_index = span.start + PRE_START.len() + info_string.len() + PRE_END.len();
            let end_index = span.end - POST.len();

            let admonish_content = &content[start_index..end_index];
            let admonish_content = escape_html(admonish_content);
            let admonish_content = admonish_content.trim();

            // Note that the additional whitespace around the content are deliberate
            // In line with the commonmark spec, this allows the inner content to be
            // rendered as markdown again.
            let admonish_code = format!(
                r#"<div class="admonition {directive_classname}">
  <p class="admonition-title">{directive_title}</p>
  <p>

  {admonish_content}

  </p>
</div>"#,
                directive_classname = admonition.directive.classname(),
                directive_title = admonition.title,
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
    fn add_admonish(chapter: &mut Chapter) -> MdbookResult<String> {
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
        assert_eq!(
            parse_info_string("admonish "),
            Some(Some(AdmonitionInfo {
                directive: "",
                title: None,
            }))
        );
        assert_eq!(
            parse_info_string("admonish unknown"),
            Some(Some(AdmonitionInfo {
                directive: "unknown",
                title: None
            }))
        );
        assert_eq!(
            parse_info_string("admonish note"),
            Some(Some(AdmonitionInfo {
                directive: "note",
                title: None
            }))
        );
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
  <p>

  A simple admonition.

  </p>
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
  <p>

  A simple admonition.

  </p>
</div>
Text
"#;

        assert_eq!(expected, add_admonish(content).unwrap());
    }

    #[test]
    fn adds_admonish_directive_title() {
        let content = r#"# Chapter
```admonish warning "Read **this**!"
A simple admonition.
```
Text
"#;

        let expected = r#"# Chapter

<div class="admonition warning">
  <p class="admonition-title">Read **this**!</p>
  <p>

  A simple admonition.

  </p>
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
  <p>

  classDiagram
    class PingUploader {
        &lt;&lt;interface&gt;&gt;
        +Upload() UploadResult
    }

  </p>
</div>
hello
"#;

        assert_eq!(expected, add_admonish(content).unwrap());
    }
}
