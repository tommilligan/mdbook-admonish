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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
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
