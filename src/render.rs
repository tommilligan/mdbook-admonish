use mdbook::utils::unique_id_from_content;
use std::borrow::Cow;
use std::collections::HashMap;

pub use crate::preprocessor::Admonish;
use crate::{resolve::AdmonitionMeta, types::Directive};

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
pub(crate) struct Admonition<'a> {
    pub(crate) directive: Directive,
    pub(crate) title: String,
    pub(crate) content: Cow<'a, str>,
    pub(crate) additional_classnames: Vec<String>,
    pub(crate) collapsible: bool,
    pub(crate) indent: usize,
}

impl<'a> Admonition<'a> {
    pub(crate) fn new(info: AdmonitionMeta, content: &'a str, indent: usize) -> Self {
        let AdmonitionMeta {
            directive,
            title,
            additional_classnames,
            collapsible,
        } = info;
        Self {
            directive,
            title,
            content: Cow::Borrowed(content),
            additional_classnames,
            collapsible,
            indent,
        }
    }

    pub(crate) fn html_with_unique_ids(&self, id_counter: &mut HashMap<String, usize>) -> String {
        let anchor_id = unique_id_from_content(
            if !self.title.is_empty() {
                &self.title
            } else {
                ANCHOR_ID_DEFAULT
            },
            id_counter,
        );
        self.html(&anchor_id)
    }

    fn html(&self, anchor_id: &str) -> String {
        let mut additional_class = Cow::Borrowed(self.directive.classname());
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
{indent}<a class="admonition-anchor-link" href="#{ANCHOR_ID_PREFIX}-{anchor_id}"></a>
{indent}</{title_block}>
"##
            ))
        } else {
            Cow::Borrowed("")
        };

        if !self.additional_classnames.is_empty() {
            let mut buffer = additional_class.into_owned();
            for additional_classname in &self.additional_classnames {
                buffer.push(' ');
                buffer.push_str(additional_classname);
            }

            additional_class = Cow::Owned(buffer);
        }

        let admonition_block = if self.collapsible { "details" } else { "div" };
        // Notes on the HTML template:
        // - the additional whitespace around the content are deliberate
        //   In line with the commonmark spec, this allows the inner content to be
        //   rendered as markdown paragraphs.
        format!(
            r#"
{indent}<{admonition_block} id="{ANCHOR_ID_PREFIX}-{anchor_id}" class="admonition {additional_class}">
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

const ANCHOR_ID_PREFIX: &str = "admonition";
const ANCHOR_ID_DEFAULT: &str = "default";
