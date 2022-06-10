use crate::types::Directive;
use std::str::FromStr;

mod v1;
mod v2;

#[derive(Debug, PartialEq)]
pub(crate) struct AdmonitionInfoRaw {
    directive: String,
    title: Option<String>,
    additional_classnames: Vec<String>,
    collapsible: bool,
}

/// Extract the remaining info string, if this is an admonition block.
fn admonition_config_string(info_string: &str) -> Option<&str> {
    const ADMONISH_BLOCK_KEYWORD: &str = "admonish";

    // Get the rest of the info string if this is an admonition
    if info_string == ADMONISH_BLOCK_KEYWORD {
        return Some("");
    }

    match info_string.split_once(' ') {
        Some((keyword, rest)) if keyword == ADMONISH_BLOCK_KEYWORD => Some(rest),
        _ => None,
    }
}

impl AdmonitionInfoRaw {
    /// Returns:
    /// - `None` if this is not an `admonish` block.
    /// - `Some(AdmonitionInfoRaw)` if this is an `admonish` block
    pub fn from_info_string(info_string: &str) -> Option<Result<Self, String>> {
        let config_string = admonition_config_string(info_string)?;

        // If we succeed at parsing v2, return that. Otherwise hold onto the error
        let config_v2_error = match v2::from_config_string(config_string) {
            Ok(config) => return Some(Ok(config)),
            Err(config) => config,
        };

        Some(
            if let Ok(info_raw) = v1::from_config_string(config_string) {
                // If we succeed at parsing v1, return that.
                Ok(info_raw)
            } else {
                // Otherwise return our v2 error.
                Err(config_v2_error)
            },
        )
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct AdmonitionInfo {
    pub directive: Directive,
    pub title: Option<String>,
    pub additional_classnames: Vec<String>,
    pub collapsible: bool,
}

impl AdmonitionInfo {
    pub fn from_info_string(info_string: &str) -> Option<Result<Self, String>> {
        AdmonitionInfoRaw::from_info_string(info_string).map(|result| result.map(Into::into))
    }
}

impl From<AdmonitionInfoRaw> for AdmonitionInfo {
    fn from(other: AdmonitionInfoRaw) -> Self {
        let AdmonitionInfoRaw {
            directive: raw_directive,
            title,
            additional_classnames,
            collapsible,
        } = other;
        let (directive, title) = match (Directive::from_str(&raw_directive), title) {
            (Ok(directive), None) => (directive, ucfirst(&raw_directive)),
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
    fn test_from_info_string() {
        // Not admonition blocks
        assert_eq!(AdmonitionInfoRaw::from_info_string(""), None);
        assert_eq!(AdmonitionInfoRaw::from_info_string("adm"), None);
        // v1 syntax is supported back compatibly
        assert_eq!(
            AdmonitionInfoRaw::from_info_string("admonish note.additional-classname")
                .unwrap()
                .unwrap(),
            AdmonitionInfoRaw {
                directive: "note".to_owned(),
                title: None,
                additional_classnames: vec!["additional-classname".to_owned()],
                collapsible: false,
            }
        );
        // v2 syntax is supported
        assert_eq!(
            AdmonitionInfoRaw::from_info_string(r#"admonish title="Custom Title" type="question""#)
                .unwrap()
                .unwrap(),
            AdmonitionInfoRaw {
                directive: "question".to_owned(),
                title: Some("Custom Title".to_owned()),
                additional_classnames: Vec::new(),
                collapsible: false,
            }
        );
    }

    #[test]
    fn test_admonition_info_from_raw() {
        assert_eq!(
            AdmonitionInfo::from(AdmonitionInfoRaw {
                directive: " ".to_owned(),
                title: None,
                additional_classnames: Vec::new(),
                collapsible: false,
            }),
            AdmonitionInfo {
                directive: Directive::Note,
                title: Some("Note".to_owned()),
                additional_classnames: Vec::new(),
                collapsible: false,
            }
        );
    }
}
