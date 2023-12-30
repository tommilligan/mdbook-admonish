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

    pub(crate) fn html(&self, id_counter: &mut HashMap<String, usize>) -> String {
        let anchor_id = match &self.css_id {
            CssId::Verbatim(id) => Cow::Borrowed(id),
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

        let title_block = if self.collapsible { "summary" } else { "div" };

        let title_html = if !title.is_empty() {
            Cow::Owned(format!(
                r##"{indent}<{title_block} class="admonition-title">
{indent}
{indent}{title}
{indent}
{indent}<a class="admonition-anchor-link" href="#{anchor_id}"></a>
{indent}</{title_block}>
"##
            ))
        } else {
            Cow::Borrowed("")
        };

        let mut additional_class = format!("admonish-{}", self.directive);
        if !self.additional_classnames.is_empty() {
            for additional_classname in &self.additional_classnames {
                additional_class.push(' ');
                additional_class.push_str(additional_classname);
            }
        }

        let admonition_block = if self.collapsible { "details" } else { "div" };
        // Notes on the HTML template:
        // - the additional whitespace around the content are deliberate
        //   In line with the commonmark spec, this allows the inner content to be
        //   rendered as markdown paragraphs.
        format!(
            r#"
{indent}<{admonition_block} id="{anchor_id}" class="admonition {additional_class}">
{title_html}{indent}<div>
{indent}
{indent}{content}
{indent}
{indent}</div>
{indent}</{admonition_block}>"#,
        )
    }

    /// Strips all admonish syntax, leaving the plain content of the block.
    pub(crate) fn strip(&self) -> String {
        // Add in newlines to preserve line numbering for test output
        // These replace the code fences we stripped out
        format!("\n{}\n", self.content)
    }
}

const ANCHOR_ID_DEFAULT: &str = "default";
