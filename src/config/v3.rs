use super::toml_wrangling::{
    format_invalid_directive, format_toml_parsing_error, UserInput, RX_DIRECTIVE,
};
use super::InstanceConfig;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct Wrapper<T> {
    config: T,
}

/// Transform our config string into valid toml
fn bare_inline_table_to_toml(pairs: &str) -> String {
    format!("config = {{ {pairs} }}")
}

fn user_input_from_config_string(config_string: &str) -> Result<UserInput, String> {
    match toml::from_str::<Wrapper<_>>(&bare_inline_table_to_toml(config_string)) {
        Ok(wrapper) => Ok(wrapper.config),
        Err(error) => Err(format_toml_parsing_error(error)),
    }
}

/// Parse and return the config assuming v3 format.
///
/// Note that if an error occurs, a parsed struct that can be returned to
/// show the error message will be returned.
///
/// The basic idea here is to accept the inside of an inline table, wrap it,
/// parse it, and then use the toml values.
pub(crate) fn from_config_string(config_string: &str) -> Result<InstanceConfig, String> {
    let config_string = config_string.trim();

    let config = match user_input_from_config_string(config_string) {
        Ok(config) => config,
        Err(error) => {
            // For ergonomic reasons, we allow users to specify the directive without
            // a key. So if parsing fails initially, take the first word,
            // use that as the directive, and reparse.
            let (directive, config_string) = match config_string.split_once(' ') {
                Some((directive, config_string)) => (directive.trim(), config_string.trim()),
                None => (config_string, ""),
            };

            if !RX_DIRECTIVE.is_match(directive) {
                return Err(format_invalid_directive(directive, error));
            }

            let mut config = user_input_from_config_string(config_string)?;
            config.r#type = Some(directive.to_owned());
            config
        }
    };

    let additional_classnames = config.classnames();
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
    fn test_from_config_string_v3() -> Result<(), ()> {
        fn check(config_string: &str, expected: InstanceConfig) -> Result<(), ()> {
            let actual = match from_config_string(config_string) {
                Ok(config) => config,
                Err(error) => {
                    panic!("Expected config '{config_string}' to be valid, got error:\n\n{error}")
                }
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
            r#"type="note", class="additional classname", title="Никита", collapsible=true"#,
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
            r#"info title="Information", collapsible=false"#,
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
            r#"info title="My Info", id="my-info-custom-id""#,
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
        // HTML with quotes inside content
        // Note that we use toml literal (single quoted) strings here
        check(
            r#"info title='My <span class="emphasis">Title</span>'"#,
            InstanceConfig {
                directive: "info".to_owned(),
                title: Some(r#"My <span class="emphasis">Title</span>"#.to_owned()),
                id: None,
                additional_classnames: Vec::new(),
                collapsible: None,
            },
        )?;

        Ok(())
    }

    #[test]
    fn test_from_config_string_invalid_directive() {
        assert_eq!(
            from_config_string(r#"oh!wow titlel=""#).unwrap_err(),
            r#"'oh!wow' is not a valid directive or TOML key-value pair.

TOML parsing error: TOML parse error at line 1, column 14
  |
1 | config = { oh!wow titlel=" }
  |              ^
expected `.`, `=`
"#
        );
    }

    #[test]
    fn test_from_config_string_invalid_toml_value() {
        assert_eq!(
            from_config_string(r#"note titlel=""#).unwrap_err(),
            r#"TOML parsing error: TOML parse error at line 1, column 22
  |
1 | config = { titlel=" }
  |                      ^
invalid basic string
"#
        );
    }
}
