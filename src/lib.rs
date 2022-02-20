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
                res = Some(Admonish::preprocess(chapter).map(|md| {
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
struct AdmonitionInfoRaw<'a> {
    directive: &'a str,
    title: Option<String>,
}

#[derive(Debug, PartialEq)]
struct AdmonitionInfo<'a> {
    directive: Directive,
    title: Cow<'a, str>,
}

impl<'a> Default for AdmonitionInfo<'a> {
    fn default() -> Self {
        Self {
            directive: Directive::Note,
            title: Cow::Borrowed("Note"),
        }
    }
}

impl<'a> TryFrom<AdmonitionInfoRaw<'a>> for AdmonitionInfo<'a> {
    type Error = ();

    fn try_from(other: AdmonitionInfoRaw<'a>) -> Result<Self, ()> {
        let directive = Directive::from_str(other.directive)?;
        Ok(Self {
            directive,
            title: other
                .title
                .map(Cow::Owned)
                .unwrap_or_else(|| Cow::Owned(ucfirst(other.directive))),
        })
    }
}

#[derive(Debug, PartialEq)]
struct Admonition<'a> {
    directive: Directive,
    title: Cow<'a, str>,
    content: &'a str,
}

impl<'a> Admonition<'a> {
    pub fn new(info: AdmonitionInfo<'a>, content: &'a str) -> Self {
        let AdmonitionInfo { directive, title } = info;
        Self {
            directive,
            title,
            content,
        }
    }

    fn html(&self) -> String {
        let directive_classname = self.directive.classname();
        let title = &self.title;
        let content = &self.content;

        // Notes on the HTML template:
        // - the additional whitespace around the content are deliberate
        //   In line with the commonmark spec, this allows the inner content to be
        //   rendered as markdown paragraphs.
        // - <p> nested in <div> is deliberate
        //   - If plain text is given, it is contained in the <p> tag
        //   - If markdown is given, it is rendered into a new <p> tag.
        //     This leads to it escaping the template <p> tag, and to apply
        //     styling we contain in in the outer <div>.
        format!(
            r#"<div class="admonition {directive_classname}">
<div class="admonition-title">
<p>

{title}

</p>
</div>
<div>
<p>

{content}

</p>
</div>
</div>"#,
        )
    }
}

/// Returns:
/// - `None` if this is not an `admonish` block.
/// - `Some(None)` if this is an `admonish` block, but no further configuration was given
/// - `Some(AdmonitionInfoRaw)` if this is an `admonish` block, and further configuration was given
fn parse_info_string(info_string: &str) -> Option<Option<AdmonitionInfoRaw>> {
    if info_string == "admonish" {
        return Some(None);
    }

    let directive_title = match info_string.split_once(' ') {
        Some(("admonish", rest)) => rest,
        _ => return None,
    };

    let info = if let Some((directive, title)) = directive_title.split_once(' ') {
        // The title is expected to be a quoted JSON string
        let title: String = serde_json::from_str(title)
            .unwrap_or_else(|error| format!("Error parsing JSON string: {error}"));
        AdmonitionInfoRaw {
            directive,
            title: Some(title),
        }
    } else {
        AdmonitionInfoRaw {
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

fn extract_admonish_body(content: &str) -> &str {
    const PRE_END: char = '\n';
    const POST: &str = "```";

    // We can't trust the info string length to find the start of the body
    // it may change length if it contains HTML or character escapes.
    //
    // So we scan for the first newline and use that.
    // If gods forbid it doesn't exist for some reason, just include the whole info string.
    let start_index = content
        // Start one character _after_ the newline
        .find(PRE_END)
        .map(|index| index + 1)
        .unwrap_or_default();
    let end_index = content.len() - POST.len();

    let admonish_content = &content[start_index..end_index];
    // The newline after a code block is technically optional, so we have to
    // trim it off dynamically.
    let admonish_content = admonish_content.trim();
    admonish_content
}

fn preprocess(content: &str) -> MdbookResult<String> {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TASKLISTS);

    let mut admonish_blocks = vec![];

    let events = Parser::new_ext(content, opts);
    for (e, span) in events.into_offset_iter() {
        if let Event::Start(Tag::CodeBlock(Fenced(info_string))) = e.clone() {
            let span_content = &content[span.start..span.end];
            let admonition_content = extract_admonish_body(span_content);
            let info = match parse_info_string(info_string.as_ref()) {
                Some(info) => info,
                None => continue,
            };
            let info = info
                .map(|info| AdmonitionInfo::try_from(info).unwrap_or_default())
                .unwrap_or_default();
            let admonition = Admonition::new(info, admonition_content);
            admonish_blocks.push((span, admonition.html()));
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
    fn preprocess(chapter: &mut Chapter) -> MdbookResult<String> {
        preprocess(&chapter.content)
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
            Some(Some(AdmonitionInfoRaw {
                directive: "",
                title: None,
            }))
        );
        assert_eq!(
            parse_info_string("admonish unknown"),
            Some(Some(AdmonitionInfoRaw {
                directive: "unknown",
                title: None
            }))
        );
        assert_eq!(
            parse_info_string("admonish note"),
            Some(Some(AdmonitionInfoRaw {
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
<div class="admonition-title">
<p>

Note

</p>
</div>
<div>
<p>

A simple admonition.

</p>
</div>
</div>
Text
"#;

        assert_eq!(expected, preprocess(content).unwrap());
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
<div class="admonition-title">
<p>

Warning

</p>
</div>
<div>
<p>

A simple admonition.

</p>
</div>
</div>
Text
"#;

        assert_eq!(expected, preprocess(content).unwrap());
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
<div class="admonition-title">
<p>

Read **this**!

</p>
</div>
<div>
<p>

A simple admonition.

</p>
</div>
</div>
Text
"#;

        assert_eq!(expected, preprocess(content).unwrap());
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

        assert_eq!(expected, preprocess(content).unwrap());
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

        assert_eq!(expected, preprocess(content).unwrap());
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

        assert_eq!(expected, preprocess(content).unwrap());
    }

    #[test]
    fn info_string_that_changes_length_when_parsed() {
        let content = r#"
```admonish note "And \\"<i>in</i>\\" the title"
With <b>html</b> styling.
```
hello
"#;

        let expected = r#"

<div class="admonition note">
<div class="admonition-title">
<p>

And "<i>in</i>" the title

</p>
</div>
<div>
<p>

With <b>html</b> styling.

</p>
</div>
</div>
hello
"#;

        assert_eq!(expected, preprocess(content).unwrap());
    }

    #[test]
    fn info_string_ending_in_symbol() {
        let content = r#"
```admonish warning "Trademark™"
Should be respected
```
hello
"#;

        let expected = r#"

<div class="admonition warning">
<div class="admonition-title">
<p>

Trademark™

</p>
</div>
<div>
<p>

Should be respected

</p>
</div>
</div>
hello
"#;

        assert_eq!(expected, preprocess(content).unwrap());
    }
}
