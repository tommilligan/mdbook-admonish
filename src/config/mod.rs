mod toml_wrangling;
mod v1;
mod v2;
mod v3;

/// Configuration as described by the instance of an admonition in markdown.
///
/// This structure represents the configuration the user must provide in each
/// instance.
#[derive(Debug, PartialEq, Default)]
pub(crate) struct InstanceConfig {
    pub(crate) directive: String,
    pub(crate) title: Option<String>,
    pub(crate) id: Option<String>,
    pub(crate) additional_classnames: Vec<String>,
    pub(crate) collapsible: Option<bool>,
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

impl InstanceConfig {
    /// Returns:
    /// - `None` if this is not an `admonish` block.
    /// - `Some(InstanceConfig)` if this is an `admonish` block
    pub fn from_info_string(info_string: &str) -> Option<Result<Self, String>> {
        let config_string = admonition_config_string(info_string)?;
        Some(Self::from_admonish_config_string(config_string))
    }

    /// Parse an info string that is known to be for `admonish`.
    fn from_admonish_config_string(config_string: &str) -> Result<Self, String> {
        // If we succeed at parsing v3, return that. Otherwise hold onto the error
        let config_v3_error = match v3::from_config_string(config_string) {
            Ok(config) => return Ok(config),
            Err(error) => error,
        };

        // If we succeed at parsing v2, return that
        if let Ok(config) = v2::from_config_string(config_string) {
            return Ok(config);
        };

        // If we succeed at parsing v1, return that.
        if let Ok(config) = v1::from_config_string(config_string) {
            return Ok(config);
        }

        // Otherwise return our v3 error.
        Err(config_v3_error)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_from_info_string() {
        // Not admonition blocks
        assert_eq!(InstanceConfig::from_info_string(""), None);
        assert_eq!(InstanceConfig::from_info_string("adm"), None);
        // v1 syntax is supported back compatibly
        assert_eq!(
            InstanceConfig::from_info_string("admonish note.additional-classname")
                .unwrap()
                .unwrap(),
            InstanceConfig {
                directive: "note".to_owned(),
                title: None,
                id: None,
                additional_classnames: vec!["additional-classname".to_owned()],
                collapsible: None,
            }
        );
        // v2 syntax is supported
        assert_eq!(
            InstanceConfig::from_info_string(
                r#"admonish title="Custom Title" type="question" id="my-id""#
            )
            .unwrap()
            .unwrap(),
            InstanceConfig {
                directive: "question".to_owned(),
                title: Some("Custom Title".to_owned()),
                id: Some("my-id".to_owned()),
                additional_classnames: Vec::new(),
                collapsible: None,
            }
        );
        // v3 syntax is supported
        assert_eq!(
            InstanceConfig::from_info_string(
                r#"admonish title="Custom Title", type="question", id="my-id""#
            )
            .unwrap()
            .unwrap(),
            InstanceConfig {
                directive: "question".to_owned(),
                title: Some("Custom Title".to_owned()),
                id: Some("my-id".to_owned()),
                additional_classnames: Vec::new(),
                collapsible: None,
            }
        );
    }
}
