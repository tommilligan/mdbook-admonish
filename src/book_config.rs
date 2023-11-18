use crate::admonitions::CustomFlavours;
use crate::types::AdmonitionDefaults;
use anyhow::{Context, Result};
use mdbook::preprocess::PreprocessorContext;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    toml::from_str(&table).context("Invalid mdbook-admonish configuration in book.toml")
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

    // TODO is custom the right name?
    #[serde(default)]
    pub custom: CustomFlavours,
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
