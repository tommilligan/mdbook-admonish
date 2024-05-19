use anyhow::{Context, Result};
use mdbook::preprocess::PreprocessorContext;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::types::AdmonitionDefaults;

/// Loads the plugin configuration from mdbook internals.
///
/// Roundtrips config to string, to avoid linking the plugin's internal version of toml
/// to the one publically exposed by the mdbook library.
pub(crate) fn admonish_config_from_context(ctx: &PreprocessorContext) -> Result<Config> {
    let table: String = toml_mdbook::to_string(
        ctx.config
            .get_preprocessor("admonish")
            .context("No configuration for mdbook-admonish in book.toml")?,
    )?;
    admonish_config_from_str(&table)
}

pub(crate) fn admonish_config_from_str(data: &str) -> Result<Config> {
    toml::from_str(data).context("Invalid mdbook-admonish configuration in book.toml")
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub(crate) struct Config {
    #[serde(default)]
    pub on_failure: OnFailure,

    #[serde(default)]
    pub default: AdmonitionDefaults,

    #[serde(default)]
    pub renderer: HashMap<String, RendererConfig>,

    #[serde(default)]
    pub assets_version: Option<String>,

    #[serde(default)]
    pub custom: Vec<CustomDirective>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub(crate) struct CustomDirective {
    /// The primary directive. Used for CSS classnames
    pub directive: String,

    /// Path to an SVG file, relative to the book root.
    pub icon: PathBuf,

    /// Primary color for this directive.
    pub color: hex_color::HexColor,

    /// Alternative directives the user can specify
    #[serde(default)]
    pub aliases: Vec<String>,

    /// Title to use, human readable.
    #[serde(default)]
    pub title: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub(crate) struct RendererConfig {
    pub render_mode: Option<RenderMode>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum RenderMode {
    Preserve,
    Strip,
    Html,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum OnFailure {
    Bail,
    Continue,
}

impl Default for OnFailure {
    fn default() -> Self {
        Self::Continue
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn empty_config_okay() -> Result<()> {
        let actual = admonish_config_from_str("")?;
        let expected = Config::default();
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn css_id_prefix_kebab_case_allowed() -> Result<()> {
        let expected = Config {
            default: AdmonitionDefaults {
                css_id_prefix: Some("flam-".to_owned()),
                ..Default::default()
            },
            ..Default::default()
        };

        // Snake case okay
        let actual = admonish_config_from_str(r#"default = { css_id_prefix = "flam-" }"#)?;
        assert_eq!(actual, expected);

        // Kebab case back-compat okay
        let actual = admonish_config_from_str(r#"default = { css-id-prefix = "flam-" }"#)?;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn full_config_roundtrip() -> Result<()> {
        let input = Config {
            default: AdmonitionDefaults {
                css_id_prefix: Some("flam-".to_owned()),
                collapsible: true,
                title: Some("".to_owned()),
            },
            assets_version: Some("1.1.1".to_owned()),
            custom: vec![CustomDirective {
                directive: "test-directive".to_owned(),
                icon: PathBuf::from("/tmp/test-directive.svg"),
                color: hex_color::HexColor::from((155, 79, 150)),
                aliases: vec!["test-directive-alias-0".to_owned()],
                title: Some("test-directive-title".to_owned()),
            }],
            on_failure: OnFailure::Bail,
            renderer: HashMap::from([(
                "test-mode".to_owned(),
                RendererConfig {
                    render_mode: Some(RenderMode::Strip),
                },
            )]),
        };

        let expected = r##"on_failure = "bail"
assets_version = "1.1.1"

[default]
title = ""
collapsible = true
css_id_prefix = "flam-"

[renderer.test-mode]
render_mode = "strip"

[[custom]]
directive = "test-directive"
icon = "/tmp/test-directive.svg"
color = "#9B4F96"
aliases = ["test-directive-alias-0"]
title = "test-directive-title"
"##;

        let serialized = toml::to_string(&input)?;
        assert_eq!(serialized, expected);

        let actual = admonish_config_from_str(&serialized)?;
        assert_eq!(actual, input);
        Ok(())
    }
}
