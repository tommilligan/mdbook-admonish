use crate::config::InstanceConfig;
use crate::types::{AdmonitionDefaults, Directive};
use std::str::FromStr;

/// All information required to render an admonition.
///
/// i.e. all configured options have been resolved at this point.
#[derive(Debug, PartialEq)]
pub(crate) struct AdmonitionMeta {
    pub directive: Directive,
    pub title: String,
    pub additional_classnames: Vec<String>,
    pub collapsible: bool,
}

impl AdmonitionMeta {
    pub fn from_info_string(
        info_string: &str,
        defaults: &AdmonitionDefaults,
    ) -> Option<Result<Self, String>> {
        InstanceConfig::from_info_string(info_string)
            .map(|raw| raw.map(|raw| Self::resolve(raw, defaults)))
    }

    /// Combine the per-admonition configuration with global defaults (and
    /// other logic) to resolve the values needed for rendering.
    fn resolve(raw: InstanceConfig, defaults: &AdmonitionDefaults) -> Self {
        let InstanceConfig {
            directive: raw_directive,
            title,
            additional_classnames,
            collapsible,
        } = raw;

        // Use values from block, else load default value
        let title = title.or_else(|| defaults.title.clone());
        let collapsible = collapsible.unwrap_or(defaults.collapsible);

        // Load the directive (and title, if one still not given)
        let (directive, title) = match (Directive::from_str(&raw_directive), title) {
            (Ok(directive), None) => (directive, format_directive_title(&raw_directive)),
            (Err(_), None) => (Directive::Note, "Note".to_owned()),
            (Ok(directive), Some(title)) => (directive, title),
            (Err(_), Some(title)) => (Directive::Note, title),
        };

        Self {
            directive,
            title,
            additional_classnames,
            collapsible,
        }
    }
}

/// Format the title of an admonition directive
///
/// We special case a few words to make them look nicer (e.g. "tldr" -> "TL;DR" and "faq" -> "FAQ").
fn format_directive_title(input: &str) -> String {
    match input {
        "tldr" => "TL;DR".to_owned(),
        "faq" => "FAQ".to_owned(),
        _ => uppercase_first(input),
    }
}

/// Make the first letter of `input` upppercase.
///
/// source: https://stackoverflow.com/a/38406885
fn uppercase_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_format_directive_title() {
        assert_eq!(format_directive_title(""), "");
        assert_eq!(format_directive_title("a"), "A");
        assert_eq!(format_directive_title("tldr"), "TL;DR");
        assert_eq!(format_directive_title("faq"), "FAQ");
        assert_eq!(format_directive_title("note"), "Note");
        assert_eq!(format_directive_title("abstract"), "Abstract");
        // Unicode
        assert_eq!(format_directive_title("🦀"), "🦀");
    }

    #[test]
    fn test_admonition_info_from_raw() {
        assert_eq!(
            AdmonitionMeta::resolve(
                InstanceConfig {
                    directive: " ".to_owned(),
                    title: None,
                    additional_classnames: Vec::new(),
                    collapsible: None,
                },
                &Default::default()
            ),
            AdmonitionMeta {
                directive: Directive::Note,
                title: "Note".to_owned(),
                additional_classnames: Vec::new(),
                collapsible: false,
            }
        );
    }

    #[test]
    fn test_admonition_info_from_raw_with_defaults() {
        assert_eq!(
            AdmonitionMeta::resolve(
                InstanceConfig {
                    directive: " ".to_owned(),
                    title: None,
                    additional_classnames: Vec::new(),
                    collapsible: None,
                },
                &AdmonitionDefaults {
                    title: Some("Important!!!".to_owned()),
                    collapsible: true,
                },
            ),
            AdmonitionMeta {
                directive: Directive::Note,
                title: "Important!!!".to_owned(),
                additional_classnames: Vec::new(),
                collapsible: true,
            }
        );
    }
}
