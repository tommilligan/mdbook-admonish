use mdbook::utils::unique_id_from_content;
use std::borrow::Cow;
use std::collections::HashMap;

use crate::{resolve::AdmonitionMeta, types::CssId};

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
        format!(
            r#"
{indent}<{admonition_element} {attributes}>
{titlebar_html}{indent}<div>
{indent}
{indent}{content}
{indent}
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
