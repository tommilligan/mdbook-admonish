use super::AdmonitionInfoRaw;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct AdmonitionInfoConfig {
    #[serde(default)]
    r#type: Option<String>,
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    class: Option<String>,
    #[serde(default)]
    collapsible: bool,
}

/// Transform our config string into valid toml
fn bare_key_value_pairs_to_toml(pairs: &str) -> String {
    use regex::Captures;

    static RX_BARE_KEY_ASSIGNMENT: Lazy<Regex> = Lazy::new(|| {
        let bare_key = r#"[A-Za-z0-9_-]+"#;
        Regex::new(&format!("(?:{bare_key}) *=")).expect("bare key assignment regex")
    });

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
pub(crate) fn from_config_string(config_string: &str) -> Result<AdmonitionInfoRaw, String> {
    let config_toml = bare_key_value_pairs_to_toml(config_string);
    let config_toml = config_toml.trim();

    let config: AdmonitionInfoConfig = match toml::from_str(config_toml) {
        Ok(config) => config,
        Err(error) => {
            let original_error = Err(format!("TOML parsing error: {error}"));

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
                return original_error;
            }

            let mut config: AdmonitionInfoConfig = match toml::from_str(config_toml) {
                Ok(config) => config,
                Err(_) => return original_error,
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
    Ok(AdmonitionInfoRaw {
        directive: config.r#type.unwrap_or_default(),
        title: config.title,
        additional_classnames,
        collapsible: config.collapsible,
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_from_config_string_v2() {
        assert_eq!(
            from_config_string("").unwrap(),
            AdmonitionInfoRaw {
                directive: "".to_owned(),
                title: None,
                additional_classnames: Vec::new(),
                collapsible: false,
            }
        );
        assert_eq!(
            from_config_string(" ").unwrap(),
            AdmonitionInfoRaw {
                directive: "".to_owned(),
                title: None,
                additional_classnames: Vec::new(),
                collapsible: false,
            }
        );
        assert_eq!(
            from_config_string(r#"type="note" class="additional classname" title="Никита""#)
                .unwrap(),
            AdmonitionInfoRaw {
                directive: "note".to_owned(),
                title: Some("Никита".to_owned()),
                additional_classnames: vec!["additional".to_owned(), "classname".to_owned()],
                collapsible: false,
            }
        );
        // Specifying unknown keys is okay, as long as they're valid
        assert_eq!(
            from_config_string(r#"unkonwn="but valid toml""#).unwrap(),
            AdmonitionInfoRaw {
                directive: "".to_owned(),
                title: None,
                additional_classnames: Vec::new(),
                collapsible: false,
            }
        );
        // Just directive is fine
        assert_eq!(
            from_config_string(r#"info"#).unwrap(),
            AdmonitionInfoRaw {
                directive: "info".to_owned(),
                title: None,
                additional_classnames: Vec::new(),
                collapsible: false,
            }
        );
        // Directive plus toml config
        assert_eq!(
            from_config_string(r#"info title="Information""#).unwrap(),
            AdmonitionInfoRaw {
                directive: "info".to_owned(),
                title: Some("Information".to_owned()),
                additional_classnames: Vec::new(),
                collapsible: false,
            }
        );
        // Directive after toml config is an error
        assert!(from_config_string(r#"title="Information" info"#).is_err());
    }

    #[test]
    fn test_from_config_string_invalid_toml_value() {
        assert_eq!(
            from_config_string(r#"note titlel=""#).unwrap_err(),
            "TOML parsing error: expected an equals, found a newline at line 1 column 6".to_owned()
        );
    }
}
