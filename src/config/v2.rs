use super::InstanceConfig;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct UserInput {
    #[serde(default)]
    r#type: Option<String>,
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    class: Option<String>,
    #[serde(default)]
    collapsible: Option<bool>,
}

/// Transform our config string into valid toml
fn bare_key_value_pairs_to_toml(pairs: &str) -> String {
    use regex::Captures;

    static RX_BARE_KEY_ASSIGNMENT: Lazy<Regex> =
        Lazy::new(|| Regex::new(r#"(?:[A-Za-z0-9_-]+) *="#).expect("bare key assignment regex"));

    fn prefix_with_newline(captures: &Captures) -> String {
        format!(
            "\n{}",
            captures
                .get(0)
                .expect("capture to have group zero")
                .as_str()
        )
    }

    RX_BARE_KEY_ASSIGNMENT
        .replace_all(pairs, prefix_with_newline)
        .into_owned()
}

/// Parse and return the config assuming v2 format.
///
/// Note that if an error occurs, a parsed struct that can be returned to
/// show the error message will be returned.
pub(crate) fn from_config_string(config_string: &str) -> Result<InstanceConfig, String> {
    let config_toml = bare_key_value_pairs_to_toml(config_string);
    let config_toml = config_toml.trim();

    let config: UserInput = match toml::from_str(config_toml) {
        Ok(config) => config,
        Err(error) => {
            let original_error = format!("TOML parsing error: {error}");

            // For ergonomic reasons, we allow users to specify the directive without
            // a key. So if parsing fails initially, take the first line,
            // use that as the directive, and reparse.
            let (directive, config_toml) = match config_toml.split_once('\n') {
                Some((directive, config_toml)) => (directive.trim(), config_toml),
                None => (config_toml, ""),
            };

            static RX_DIRECTIVE: Lazy<Regex> =
                Lazy::new(|| Regex::new(r#"^[A-Za-z0-9_-]+$"#).expect("directive regex"));

            if !RX_DIRECTIVE.is_match(directive) {
                return Err(format!("'{directive}' is not a valid directive or TOML key-value pair.\n\n{original_error}"));
            }

            let mut config: UserInput = match toml::from_str(config_toml) {
                Ok(config) => config,
                Err(error) => {
                    return Err(format!("TOML parsing error: {error}"));
                }
            };
            config.r#type = Some(directive.to_owned());
            config
        }
    };
    let additional_classnames = config
        .class
        .map(|class| {
            class
                .split(' ')
                .filter(|classname| !classname.is_empty())
                .map(|classname| classname.to_owned())
                .collect()
        })
        .unwrap_or_default();
    Ok(InstanceConfig {
        directive: config.r#type.unwrap_or_default(),
        title: config.title,
        id: config.id,
        additional_classnames,
        collapsible: config.collapsible,
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_from_config_string_v2() -> Result<(), ()> {
        fn check(config_string: &str, expected: InstanceConfig) -> Result<(), ()> {
            let actual = match from_config_string(config_string) {
                Ok(config) => config,
                Err(error) => panic!("Expected config to be valid, got error:\n\n{}", error),
            };
            assert_eq!(actual, expected);
            Ok(())
        }

        check(
            "",
            InstanceConfig {
                directive: "".to_owned(),
                title: None,
                id: None,
                additional_classnames: Vec::new(),
                collapsible: None,
            },
        )?;
        check(
            " ",
            InstanceConfig {
                directive: "".to_owned(),
                title: None,
                id: None,
                additional_classnames: Vec::new(),
                collapsible: None,
            },
        )?;
        check(
            r#"type="note" class="additional classname" title="Никита" collapsible=true"#,
            InstanceConfig {
                directive: "note".to_owned(),
                title: Some("Никита".to_owned()),
                id: None,
                additional_classnames: vec!["additional".to_owned(), "classname".to_owned()],
                collapsible: Some(true),
            },
        )?;
        // Specifying unknown keys is okay, as long as they're valid
        check(
            r#"unkonwn="but valid toml""#,
            InstanceConfig {
                directive: "".to_owned(),
                title: None,
                id: None,
                additional_classnames: Vec::new(),
                collapsible: None,
            },
        )?;
        // Just directive is fine
        check(
            r#"info"#,
            InstanceConfig {
                directive: "info".to_owned(),
                title: None,
                id: None,
                additional_classnames: Vec::new(),
                collapsible: None,
            },
        )?;
        // Directive plus toml config
        check(
            r#"info title="Information" collapsible=false"#,
            InstanceConfig {
                directive: "info".to_owned(),
                title: Some("Information".to_owned()),
                id: None,
                additional_classnames: Vec::new(),
                collapsible: Some(false),
            },
        )?;
        // Test custom id
        check(
            r#"info title="My Info" id="my-info-custom-id""#,
            InstanceConfig {
                directive: "info".to_owned(),
                title: Some("My Info".to_owned()),
                id: Some("my-info-custom-id".to_owned()),
                additional_classnames: Vec::new(),
                collapsible: None,
            },
        )?;
        // Directive after toml config is an error
        assert!(from_config_string(r#"title="Information" info"#).is_err());
        Ok(())
    }

    #[test]
    fn test_from_config_string_invalid_directive() {
        assert_eq!(
            from_config_string(r#"oh!wow titlel=""#).unwrap_err(),
            r#"'oh!wow' is not a valid directive or TOML key-value pair.

TOML parsing error: TOML parse error at line 1, column 3
  |
1 | oh!wow 
  |   ^
expected `.`, `=`
"#
        );
    }

    #[test]
    fn test_from_config_string_invalid_toml_value() {
        assert_eq!(
            from_config_string(r#"note titlel=""#).unwrap_err(),
            r#"TOML parsing error: TOML parse error at line 1, column 9
  |
1 | titlel="
  |         ^
invalid basic string
"#
        );
    }
}
