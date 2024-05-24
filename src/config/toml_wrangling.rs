use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub(crate) struct UserInput {
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub class: Option<String>,
    #[serde(default)]
    pub collapsible: Option<bool>,
}

impl UserInput {
    pub fn classnames(&self) -> Vec<String> {
        self.class
            .as_ref()
            .map(|class| {
                class
                    .split(' ')
                    .filter(|classname| !classname.is_empty())
                    .map(|classname| classname.to_owned())
                    .collect()
            })
            .unwrap_or_default()
    }
}

pub(crate) static RX_DIRECTIVE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"^[A-Za-z0-9_-]+$"#).expect("directive regex"));

pub(crate) fn format_toml_parsing_error(error: impl Display) -> String {
    format!("TOML parsing error: {error}")
}

pub(crate) fn format_invalid_directive(directive: &str, original_error: impl Display) -> String {
    format!("'{directive}' is not a valid directive or TOML key-value pair.\n\n{original_error}")
}
