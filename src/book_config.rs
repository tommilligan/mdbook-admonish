use anyhow::{Context, Result};
use mdbook::preprocess::PreprocessorContext;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::types::{AdmonitionDefaults, BuiltinDirective, BuiltinDirectiveConfig};

/// Loads the plugin configuration from mdbook internals.
///
/// Roundtrips config to string, to avoid linking the plugin's internal version of toml
/// to the one publically exposed by the mdbook library.
pub(crate) fn admonish_config_from_context(ctx: &PreprocessorContext) -> Result<Config> {
    let table: String = toml::to_string(
        ctx.config
            .get_preprocessor("admonish")
            .context("No configuration for mdbook-admonish in book.toml")?,
    )
    .context("Could not serialize mdbook-admonish config. This is a bug in the toml library.")?;
    admonish_config_from_str(&table)
}

pub(crate) fn admonish_config_from_str(data: &str) -> Result<Config> {
    let readonly: ConfigReadonly =
        toml::from_str(data).context("Invalid mdbook-admonish configuration in book.toml")?;
    let config = readonly.into();
    log::debug!("Loaded admonish config: {:?}", config);
    Ok(config)
}

/// All valid input states including back-compatibility fields.
///
/// This struct deliberately does not implement Serialize as it never meant to
/// be written, only converted to Config.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Default)]
struct ConfigReadonly {
    #[serde(default)]
    pub on_failure: OnFailure,

    #[serde(default)]
    pub default: AdmonitionDefaults,

    #[serde(default)]
    pub renderer: HashMap<String, RendererConfig>,

    #[serde(default)]
    pub assets_version: Option<String>,

    #[serde(default)]
    pub custom: Vec<CustomDirectiveReadonly>,

    #[serde(default)]
    pub builtin: HashMap<BuiltinDirective, BuiltinDirectiveConfig>,

    #[serde(default)]
    pub directive: DirectiveConfig,
}

/// The canonical config format, without back-compatibility
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
    pub directive: DirectiveConfig,
}

impl From<ConfigReadonly> for Config {
    fn from(other: ConfigReadonly) -> Self {
        let ConfigReadonly {
            on_failure,
            default,
            renderer,
            assets_version,
            custom,
            builtin,
            mut directive,
        } = other;

        // Merge deprecated config fields into main config object
        directive.custom.extend(
            custom
                .into_iter()
                .map(|CustomDirectiveReadonly { directive, config }| (directive, config)),
        );
        directive.builtin.extend(builtin);

        Self {
            on_failure,
            default,
            renderer,
            assets_version,
            directive,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub(crate) struct DirectiveConfig {
    #[serde(default)]
    pub custom: HashMap<String, CustomDirective>,

    #[serde(default)]
    pub builtin: HashMap<BuiltinDirective, BuiltinDirectiveConfig>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub(crate) struct CustomDirective {
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

    /// Default collapsible value.
    #[serde(default)]
    pub collapsible: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub(crate) struct CustomDirectiveReadonly {
    /// The primary directive. Used for CSS classnames
    pub directive: String,

    /// Path to an SVG file, relative to the book root.
    #[serde(flatten)]
    config: CustomDirective,
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

    use crate::types::BuiltinDirective;

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
    fn merge_old_and_new_custom_directives() -> Result<()> {
        let serialized = r##"
[directive.custom.purple]
icon = "/tmp/test-directive.svg"
color = "#9B4F96"
aliases = ["test-directive-alias-0"]
title = "Purple"
collapsible = true

[[custom]]
directive = "blue"
icon = "/tmp/test-directive.svg"
color = "#0038A8"
aliases = []
title = "Blue"
        "##;
        let expected = Config {
            directive: DirectiveConfig {
                custom: HashMap::from([
                    (
                        "purple".to_owned(),
                        CustomDirective {
                            icon: PathBuf::from("/tmp/test-directive.svg"),
                            color: hex_color::HexColor::from((155, 79, 150)),
                            aliases: vec!["test-directive-alias-0".to_owned()],
                            title: Some("Purple".to_owned()),
                            collapsible: Some(true),
                        },
                    ),
                    (
                        "blue".to_owned(),
                        CustomDirective {
                            icon: PathBuf::from("/tmp/test-directive.svg"),
                            color: hex_color::HexColor::from((0, 56, 168)),
                            aliases: vec![],
                            title: Some("Blue".to_owned()),
                            collapsible: None,
                        },
                    ),
                ]),
                ..Default::default()
            },
            ..Default::default()
        };

        let actual = admonish_config_from_str(serialized)?;
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
            directive: DirectiveConfig {
                custom: HashMap::from([(
                    "test-directive".to_owned(),
                    CustomDirective {
                        icon: PathBuf::from("/tmp/test-directive.svg"),
                        color: hex_color::HexColor::from((155, 79, 150)),
                        aliases: vec!["test-directive-alias-0".to_owned()],
                        title: Some("test-directive-title".to_owned()),
                        collapsible: Some(true),
                    },
                )]),
                builtin: HashMap::from([(
                    BuiltinDirective::Warning,
                    BuiltinDirectiveConfig {
                        collapsible: Some(true),
                    },
                )]),
            },
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

[directive.custom.test-directive]
icon = "/tmp/test-directive.svg"
color = "#9B4F96"
aliases = ["test-directive-alias-0"]
title = "test-directive-title"
collapsible = true

[directive.builtin.warning]
collapsible = true
"##;

        let serialized = toml::to_string(&input)?;
        assert_eq!(serialized, expected);

        let actual = admonish_config_from_str(&serialized)?;
        assert_eq!(actual, input);
        Ok(())
    }
}
