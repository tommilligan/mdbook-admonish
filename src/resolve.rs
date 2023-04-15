use crate::config::AdmonitionInfoRaw;
use crate::types::{AdmonitionDefaults, Directive};
use std::str::FromStr;

/// All information required to render an admonition.
///
/// i.e. all configured options have been resolved at this point.
#[derive(Debug, PartialEq)]
pub(crate) struct AdmonitionInfo {
    pub directive: Directive,
    pub title: String,
    pub additional_classnames: Vec<String>,
    pub collapsible: bool,
}

impl AdmonitionInfo {
    pub fn from_info_string(
        info_string: &str,
        defaults: &AdmonitionDefaults,
    ) -> Option<Result<Self, String>> {
        AdmonitionInfoRaw::from_info_string(info_string)
            .map(|raw| raw.map(|raw| Self::resolve(raw, defaults)))
    }

    /// Combine the per-admonition configuration with global defaults (and
    /// other logic) to resolve the values needed for rendering.
    fn resolve(raw: AdmonitionInfoRaw, defaults: &AdmonitionDefaults) -> Self {
        let AdmonitionInfoRaw {
            directive: raw_directive,
            title,
            additional_classnames,
            collapsible,
        } = raw;

        // Use values from block, else load default value
        let title = title.or_else(|| defaults.title.clone());
        let collapsible = collapsible.or(defaults.collapsible).unwrap_or_default();

        // Load the directive (and title, if one still not given)
        let (directive, title) = match (Directive::from_str(&raw_directive), title) {
            (Ok(directive), None) => (directive, ucfirst(&raw_directive)),
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

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_admonition_info_from_raw() {
        assert_eq!(
            AdmonitionInfo::resolve(
                AdmonitionInfoRaw {
                    directive: " ".to_owned(),
                    title: None,
                    additional_classnames: Vec::new(),
                    collapsible: None,
                },
                &Default::default()
            ),
            AdmonitionInfo {
                directive: Directive::Note,
                title: "Note".to_owned(),
                additional_classnames: Vec::new(),
                collapsible: false,
            }
        );
    }
}
