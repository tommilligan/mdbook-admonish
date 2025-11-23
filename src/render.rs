use std::borrow::Cow;
use std::collections::HashMap;

use crate::{resolve::AdmonitionMeta, types::CssId};

//--------------------------------------------
// FIXME: Copied the code from mdbook as-is...!!

use regex::Regex;
use std::sync::LazyLock;

/// Convert the given string to a valid HTML element ID.
/// The only restriction is that the ID must not contain any ASCII whitespace.
fn normalize_id(content: &str) -> String {
    content
        .chars()
        .filter_map(|ch| {
            if ch.is_alphanumeric() || ch == '_' || ch == '-' {
                Some(ch.to_ascii_lowercase())
            } else if ch.is_whitespace() {
                Some('-')
            } else {
                None
            }
        })
        .collect::<String>()
}

/// Generate an ID for use with anchors which is derived from a "normalised"
/// string.
fn id_from_content(content: &str) -> String {
    let mut content = content.to_string();

    // Skip any tags or html-encoded stuff
    static HTML: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(<.*?>)").unwrap());
    content = HTML.replace_all(&content, "").into();
    const REPL_SUB: &[&str] = &["&lt;", "&gt;", "&amp;", "&#39;", "&quot;"];
    for sub in REPL_SUB {
        content = content.replace(sub, "");
    }

    // Remove spaces and hashes indicating a header
    let trimmed = content.trim().trim_start_matches('#').trim();
    normalize_id(trimmed)
}

/// Generate an ID for use with anchors which is derived from a "normalised"
/// string.
///
/// Each ID returned will be unique, if the same `id_counter` is provided on
/// each call.
pub fn unique_id_from_content(content: &str, id_counter: &mut HashMap<String, usize>) -> String {
    let id = id_from_content(content);

    // If we have headers with the same normalized id, append an incrementing counter
    let id_count = id_counter.entry(id.clone()).or_insert(0);
    let unique_id = match *id_count {
        0 => id,
        id_count => format!("{id}-{id_count}"),
    };
    *id_count += 1;
    unique_id
}
//--------------------------------------------

#[derive(Debug, PartialEq)]
pub(crate) struct Admonition<'a> {
    pub(crate) directive: String,
    pub(crate) title: String,
    pub(crate) content: Cow<'a, str>,
    pub(crate) css_id: CssId,
    pub(crate) additional_classnames: Vec<String>,
    pub(crate) collapsible: bool,
    pub(crate) indent: usize,
}

impl<'a> Admonition<'a> {
    pub(crate) fn new(info: AdmonitionMeta, content: &'a str, indent: usize) -> Self {
        let AdmonitionMeta {
            directive,
            title,
            css_id,
            additional_classnames,
            collapsible,
        } = info;
        Self {
            directive,
            title,
            content: Cow::Borrowed(content),
            css_id,
            additional_classnames,
            collapsible,
            indent,
        }
    }

    pub(crate) fn html(self, id_counter: &mut HashMap<String, usize>) -> String {
        let anchor_id = match &self.css_id {
            CssId::Verbatim(id) => Cow::Borrowed(id.as_str()),
            CssId::Prefix(prefix) => {
                let id = unique_id_from_content(
                    if !self.title.is_empty() {
                        &self.title
                    } else {
                        ANCHOR_ID_DEFAULT
                    },
                    id_counter,
                );

                Cow::Owned(format!("{}{}", prefix, id))
            }
        };

        let title = &self.title;
        let content = &self.content;
        let indent = " ".repeat(self.indent);

        let (titlebar_html, title_id) = if !title.is_empty() {
            let titlebar_element = if self.collapsible { "summary" } else { "div" };
            let title_id = format!("{anchor_id}-title");
            let titlebar_html = Cow::Owned(format!(
                r##"{indent}<{titlebar_element} class="admonition-title">
{indent}<div id="{title_id}">
{indent}
{indent}{title}
{indent}
{indent}</div>
{indent}<a class="admonition-anchor-link" href="#{anchor_id}"></a>
{indent}</{titlebar_element}>
"##
            ));
            (titlebar_html, Some(title_id))
        } else {
            (Cow::Borrowed(""), None)
        };

        let mut classes = vec![
            "admonition".to_owned(),
            format!("admonish-{}", self.directive),
        ];
        classes.extend(self.additional_classnames);
        let classes = classes.join(" ");

        let mut attributes = vec![
            ("id", anchor_id),
            ("class", Cow::Owned(classes)),
            ("role", Cow::Borrowed("note")),
        ];
        if let Some(title_id) = title_id {
            attributes.push(("aria-labelledby", Cow::Owned(title_id)));
        }
        let attributes = join_attributes(&attributes);

        let admonition_element = if self.collapsible { "details" } else { "div" };
        // Notes on the HTML template:
        // - the additional whitespace around the content are deliberate
        //   In line with the commonmark spec, this allows the inner content to be
        //   rendered as markdown paragraphs.
        // - We should not indent the inner content, as it retains the indent
        //   it is written with.
        format!(
            r#"
{indent}<{admonition_element} {attributes}>
{titlebar_html}{indent}<div>

{content}

{indent}</div>
{indent}</{admonition_element}>"#,
        )
    }

    /// Strips all admonish syntax, leaving the plain content of the block.
    pub(crate) fn strip(self) -> String {
        // Add in newlines to preserve line numbering for test output
        // These replace the code fences we stripped out
        format!("\n{}\n", self.content)
    }
}

fn join_attributes(attributes: &[(impl AsRef<str>, impl AsRef<str>)]) -> String {
    let mut buffer = String::new();
    for (key, value) in attributes {
        buffer.push_str(key.as_ref());
        buffer.push_str(r#"=""#);
        buffer.push_str(value.as_ref());
        buffer.push_str(r#"" "#);
    }
    buffer.pop();
    buffer
}

const ANCHOR_ID_DEFAULT: &str = "default";
