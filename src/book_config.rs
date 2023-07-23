use anyhow::{Context, Result};
use mdbook::preprocess::PreprocessorContext;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::types::AdmonitionDefaults;

pub(crate) fn admonish_config_from_context(ctx: &PreprocessorContext) -> Result<Config> {
    let table: toml::Table = ctx
        .config
        .get_preprocessor("admonish")
        .context("No configuration for mdbook-admonish in book.toml")?
        .to_owned();
    table
        .try_into()
        .context("Invalid mdbook-admonish configuration in book.toml")
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub(crate) struct Config {
    #[serde(default)]
    pub assets_version: Option<String>,

    #[serde(default)]
    pub on_failure: OnFailure,

    #[serde(default)]
    pub default: AdmonitionDefaults,

    #[serde(default)]
    pub renderer: HashMap<String, RendererConfig>,
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
