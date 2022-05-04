use super::AdmonitionInfoRaw;
use once_cell::sync::Lazy;
use regex::Regex;

pub(crate) fn from_config_string(config_string: &str) -> Result<AdmonitionInfoRaw, String> {
    let config_string = config_string.trim();

    static RX_CONFIG_STRING_V1: Lazy<Regex> = Lazy::new(|| {
        let directive = r#"[a-z]+"#;
        let css_classname = r#"-?[_a-zA-Z]+[_a-zA-Z0-9-]*"#;
        let title = r#"".*""#;
        Regex::new(&format!(
            "^({directive})?(\\.({css_classname})?)*( {title})?$"
        ))
        .expect("config string v1 regex")
    });

    // Check if this is a valid looking v1 directive
    if !RX_CONFIG_STRING_V1.is_match(config_string) {
        return Err("Invalid configuration string".to_owned());
    }

    // If we're just given the directive, handle that
    let (directive, title) = config_string
        .split_once(' ')
        .map(|(directive, title)| (directive, Some(title)))
        .unwrap_or_else(|| (config_string, None));

    // The title is expected to be a quoted JSON string
    // If parsing fails, output the error message as the title for the user to correct
    let title = title
        .map(|title| {
            serde_json::from_str::<String>(title)
                .map_err(|error| format!("Error parsing JSON string: {error}"))
        })
        .transpose()?;

    // If the directive contains additional classes, parse them out
    const CLASSNAME_SEPARATOR: char = '.';
    let (directive, additional_classnames) = match directive.split_once(CLASSNAME_SEPARATOR) {
        None => (directive, Vec::new()),
        Some((directive, additional_classnames)) => (
            directive,
            additional_classnames
                .split(CLASSNAME_SEPARATOR)
                .filter(|classname| !classname.is_empty())
                .map(|classname| classname.to_owned())
                .collect(),
        ),
    };

    Ok(AdmonitionInfoRaw {
        directive: directive.to_owned(),
        title,
        additional_classnames,
        collapsible: false,
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_from_config_string() {
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
            from_config_string("unknown").unwrap(),
            AdmonitionInfoRaw {
                directive: "unknown".to_owned(),
                title: None,
                additional_classnames: Vec::new(),
                collapsible: false,
            }
        );
        assert_eq!(
            from_config_string("note").unwrap(),
            AdmonitionInfoRaw {
                directive: "note".to_owned(),
                title: None,
                additional_classnames: Vec::new(),
                collapsible: false,
            }
        );
        assert_eq!(
            from_config_string("note.additional-classname").unwrap(),
            AdmonitionInfoRaw {
                directive: "note".to_owned(),
                title: None,
                additional_classnames: vec!["additional-classname".to_owned()],
                collapsible: false,
            }
        );
    }

    #[test]
    fn test_from_config_string_invalid_title_json() {
        // Test invalid JSON title
        assert_eq!(
            from_config_string(r#"note "\""#).unwrap_err(),
            "Error parsing JSON string: EOF while parsing a string at line 1 column 3".to_owned()
        );
    }

    #[test]
    fn test_from_config_string_v2_format() {
        assert_eq!(
            from_config_string(r#"note title="Custom""#).unwrap_err(),
            "Invalid configuration string".to_owned()
        );
    }
}
