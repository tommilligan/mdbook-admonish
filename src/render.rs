use mdbook::utils::unique_id_from_content;
use std::borrow::Cow;
use std::collections::HashMap;

pub use crate::preprocessor::Admonish;
use crate::{
    resolve::AdmonitionMeta,
    types::{CssIdType, Directive},
};

impl Directive {
    fn classname(&self) -> &'static str {
        match self {
            Directive::Note => "admonish-note",
            Directive::Abstract => "admonish-abstract",
            Directive::Info => "admonish-info",
            Directive::Tip => "admonish-tip",
            Directive::Success => "admonish-success",
            Directive::Question => "admonish-question",
            Directive::Warning => "admonish-warning",
            Directive::Failure => "admonish-failure",
            Directive::Danger => "admonish-danger",
            Directive::Bug => "admonish-bug",
            Directive::Example => "admonish-example",
            Directive::Quote => "admonish-quote",
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct Admonition<'a> {
    pub(crate) directive: Directive,
    pub(crate) title: String,
    pub(crate) content: Cow<'a, str>,
    pub(crate) css_id: Option<CssIdType>,
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
        let css_id_type = self
            .css_id
            .clone()
            .unwrap_or_else(|| CssIdType::Prefix(ANCHOR_DEFAULT_ID_PREFIX.to_owned()));
        let anchor_id = match css_id_type {
            CssIdType::Verbatim(id) => id,
            CssIdType::Prefix(prefix) => {
                let id = unique_id_from_content(
                    if !self.title.is_empty() {
                        &self.title
                    } else {
                        ANCHOR_ID_DEFAULT
                    },
                    id_counter,
                );

                prefix + &id
            }
        };

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
{indent}<a class="admonition-anchor-link" href="#{anchor_id}"></a>
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

const ANCHOR_DEFAULT_ID_PREFIX: &str = "admonition-";
const ANCHOR_ID_DEFAULT: &str = "default";
