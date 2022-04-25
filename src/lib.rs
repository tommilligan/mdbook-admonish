use mdbook::{
    book::{Book, BookItem},
    errors::Result as MdbookResult,
    preprocess::{Preprocessor, PreprocessorContext},
    utils::unique_id_from_content,
};
use pulldown_cmark::{CodeBlockKind::*, Event, Options, Parser, Tag};
use std::{borrow::Cow, str::FromStr};

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
                res = Some(preprocess(&chapter.content).map(|md| {
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
    additional_classnames: Option<Vec<&'a str>>,
}

#[derive(Debug, PartialEq)]
struct AdmonitionInfo<'a> {
    directive: Directive,
    title: Option<String>,
    additional_classnames: Option<Vec<&'a str>>,
}

impl<'a> From<AdmonitionInfoRaw<'a>> for AdmonitionInfo<'a> {
    fn from(other: AdmonitionInfoRaw<'a>) -> Self {
        let AdmonitionInfoRaw {
            directive: raw_directive,
            title,
            additional_classnames,
        } = other;
        let (directive, title) = match (Directive::from_str(raw_directive), title) {
            (Ok(directive), None) => (directive, ucfirst(raw_directive)),
            (Err(_), None) => (Directive::Note, "Note".to_owned()),
            (Ok(directive), Some(title)) => (directive, title),
            (Err(_), Some(title)) => (Directive::Note, title),
        };
        // If the user explicitly gave no title, then disable the title bar
        let title = if title.is_empty() { None } else { Some(title) };
        Self {
            directive,
            title,
            additional_classnames,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Admonition<'a> {
    directive: Directive,
    title: Option<String>,
    content: &'a str,
    additional_classnames: Option<Vec<&'a str>>,
}

impl<'a> Admonition<'a> {
    pub fn new(info: AdmonitionInfo<'a>, content: &'a str) -> Self {
        let AdmonitionInfo {
            directive,
            title,
            additional_classnames,
        } = info;
        Self {
            directive,
            title,
            content,
            additional_classnames,
        }
    }

    fn html(&self, anchor_id: &str) -> String {
        let mut additional_class = Cow::Borrowed(self.directive.classname());
        let title = &self.title;
        let content = &self.content;

        let title_html = title
            .as_ref()
            .map(|title| {
                Cow::Owned(format!(
                    r##"<div class="admonition-title">

{title}

</div>
"##
                ))
            })
            .unwrap_or(Cow::Borrowed(""));

        if let Some(additional_classnames) = &self.additional_classnames {
            let mut buffer = additional_class.into_owned();
            for additional_classname in additional_classnames {
                buffer.push(' ');
                buffer.push_str(additional_classname);
            }

            additional_class = Cow::Owned(buffer);
        }

        // Notes on the HTML template:
        // - the additional whitespace around the content are deliberate
        //   In line with the commonmark spec, this allows the inner content to be
        //   rendered as markdown paragraphs.
        format!(
            r#"<div id="admonition-{anchor_id}" class="admonition {additional_class}">
{title_html}<div>

{content}

</div>
</div>"#,
        )
    }
}

const ADMONISH_BLOCK_KEYWORD: &str = "admonish";
const ANCHOR_ID_DEFAULT: &str = "default";

/// Returns:
/// - `None` if this is not an `admonish` block.
/// - `Some(AdmonitionInfoRaw)` if this is an `admonish` block
fn parse_info_string(info_string: &str) -> Option<AdmonitionInfoRaw> {
    // Get the rest of the info string if this is an admonition
    let directive_title = if info_string == ADMONISH_BLOCK_KEYWORD {
        ""
    } else {
        match info_string.split_once(' ') {
            Some((ADMONISH_BLOCK_KEYWORD, rest)) => rest,
            _ => return None,
        }
    };

    // If we're just given the directive, handle that
    let (directive, title) = directive_title
        .split_once(' ')
        .map(|(directive, title)| (directive, Some(title)))
        .unwrap_or_else(|| (directive_title, None));

    // The title is expected to be a quoted JSON string
    // If parsing fails, output the error message as the title for the user to correct
    let title = title.map(|title| {
        serde_json::from_str::<String>(title)
            .unwrap_or_else(|error| format!("Error parsing JSON string: {error}"))
    });

    // If the directive contains additional classes, parse them out
    const CLASSNAME_SEPARATOR: char = '.';
    let (directive, additional_classnames) = match directive.split_once(CLASSNAME_SEPARATOR) {
        None => (directive, None),
        Some((directive, additional_classnames)) => (
            directive,
            Some(
                additional_classnames
                    .split(CLASSNAME_SEPARATOR)
                    .filter(|additional_classname| !additional_classname.is_empty())
                    .collect(),
            ),
        ),
    };

    Some(AdmonitionInfoRaw {
        directive,
        title,
        additional_classnames,
    })
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
    admonish_content.trim()
}

/// Given the content in the span of the code block, and the info string,
/// return `Some(Admonition)` if the code block is an admonition.
///
/// If the code block is not an admonition, return `None`.
fn parse_admonition<'a>(info_string: &'a str, content: &'a str) -> Option<Admonition<'a>> {
    let info = parse_info_string(info_string)?;
    let info = AdmonitionInfo::from(info);
    let body = extract_admonish_body(content);
    Some(Admonition::new(info, body))
}

fn preprocess(content: &str) -> MdbookResult<String> {
    let mut id_counter = Default::default();
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
            let admonition = match parse_admonition(info_string.as_ref(), span_content) {
                Some(admonition) => admonition,
                None => continue,
            };
            let anchor_id = unique_id_from_content(
                admonition.title.as_deref().unwrap_or(ANCHOR_ID_DEFAULT),
                &mut id_counter,
            );
            admonish_blocks.push((span, admonition.html(&anchor_id)));
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

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    fn prep(content: &str) -> String {
        preprocess(content).unwrap()
    }

    #[test]
    fn test_parse_info_string() {
        assert_eq!(parse_info_string(""), None);
        assert_eq!(parse_info_string("adm"), None);
        assert_eq!(
            parse_info_string("admonish"),
            Some(AdmonitionInfoRaw {
                directive: "",
                title: None,
                additional_classnames: None
            })
        );
        assert_eq!(
            parse_info_string("admonish "),
            Some(AdmonitionInfoRaw {
                directive: "",
                title: None,
                additional_classnames: None
            })
        );
        assert_eq!(
            parse_info_string("admonish unknown"),
            Some(AdmonitionInfoRaw {
                directive: "unknown",
                title: None,
                additional_classnames: None
            })
        );
        assert_eq!(
            parse_info_string("admonish note"),
            Some(AdmonitionInfoRaw {
                directive: "note",
                title: None,
                additional_classnames: None
            })
        );
        assert_eq!(
            parse_info_string("admonish note.additional-classname"),
            Some(AdmonitionInfoRaw {
                directive: "note",
                title: None,
                additional_classnames: Some(vec!["additional-classname"])
            })
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

<div id="admonition-note" class="admonition note">
<div class="admonition-title">

Note

</div>
<div>

A simple admonition.

</div>
</div>
Text
"#;

        assert_eq!(expected, prep(content));
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

<div id="admonition-warning" class="admonition warning">
<div class="admonition-title">

Warning

</div>
<div>

A simple admonition.

</div>
</div>
Text
"#;

        assert_eq!(expected, prep(content));
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

<div id="admonition-read-this" class="admonition warning">
<div class="admonition-title">

Read **this**!

</div>
<div>

A simple admonition.

</div>
</div>
Text
"#;

        assert_eq!(expected, prep(content));
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

        assert_eq!(expected, prep(content));
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

        assert_eq!(expected, prep(content));
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

        assert_eq!(expected, prep(content));
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

<div id="admonition-and-in-the-title" class="admonition note">
<div class="admonition-title">

And "<i>in</i>" the title

</div>
<div>

With <b>html</b> styling.

</div>
</div>
hello
"#;

        assert_eq!(expected, prep(content));
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

<div id="admonition-trademark" class="admonition warning">
<div class="admonition-title">

Trademark™

</div>
<div>

Should be respected

</div>
</div>
hello
"#;

        assert_eq!(expected, prep(content));
    }

    #[test]
    fn block_with_additional_classname() {
        let content = r#"
```admonish tip.my-style.other-style
Will have bonus classnames
```
"#;

        let expected = r#"

<div id="admonition-tip" class="admonition tip my-style other-style">
<div class="admonition-title">

Tip

</div>
<div>

Will have bonus classnames

</div>
</div>
"#;

        assert_eq!(expected, prep(content));
    }

    #[test]
    fn block_with_additional_classname_and_title() {
        let content = r#"
```admonish tip.my-style.other-style "Developers don't want you to know this one weird tip!"
Will have bonus classnames
```
"#;

        let expected = r#"

<div id="admonition-developers-dont-want-you-to-know-this-one-weird-tip" class="admonition tip my-style other-style">
<div class="admonition-title">

Developers don't want you to know this one weird tip!

</div>
<div>

Will have bonus classnames

</div>
</div>
"#;

        assert_eq!(expected, prep(content));
    }

    #[test]
    fn block_with_empty_additional_classnames_title_content() {
        let content = r#"
```admonish .... ""
```
"#;

        let expected = r#"

<div id="admonition-default" class="admonition note">
<div>



</div>
</div>
"#;

        assert_eq!(expected, prep(content));
    }

    #[test]
    fn unique_ids_same_title() {
        let content = r#"
```admonish note "My Note"
Content zero.
```

```admonish note "My Note"
Content one.
```
"#;

        let expected = r#"

<div id="admonition-my-note" class="admonition note">
<div class="admonition-title">

My Note

</div>
<div>

Content zero.

</div>
</div>


<div id="admonition-my-note-1" class="admonition note">
<div class="admonition-title">

My Note

</div>
<div>

Content one.

</div>
</div>
"#;

        assert_eq!(expected, prep(content));
    }
}
