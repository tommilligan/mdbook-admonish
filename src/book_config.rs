use anyhow::{anyhow, Context, Result};
use mdbook::preprocess::PreprocessorContext;
use std::str::FromStr;

use crate::types::AdmonitionDefaults;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RenderMode {
    Preserve,
    Strip,
    Html,
}

impl FromStr for RenderMode {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, ()> {
        match string {
            "preserve" => Ok(Self::Preserve),
            "strip" => Ok(Self::Strip),
            "html" => Ok(Self::Html),
            _ => Err(()),
        }
    }
}

impl RenderMode {
    pub(crate) fn from_context(
        context: &PreprocessorContext,
        renderer: &str,
        default: Self,
    ) -> Result<Self> {
        let key = format!("preprocessor.admonish.renderer.{renderer}.render_mode");
        let value = context.config.get(&key);

        // If no key set, return default
        let value = if let Some(value) = value {
            value
        } else {
            return Ok(default);
        };

        // Othersise, parse value
        let value = value
            .as_str()
            .with_context(|| format!("Invalid value for {key}: {value:?}"))?;

        RenderMode::from_str(value).map_err(|_| anyhow!("Invalid value for {key}: {value}"))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OnFailure {
    Bail,
    Continue,
}

impl Default for OnFailure {
    fn default() -> Self {
        Self::Continue
    }
}

impl FromStr for OnFailure {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, ()> {
        match string {
            "bail" => Ok(Self::Bail),
            "continue" => Ok(Self::Continue),
            _ => Ok(Self::Continue),
        }
    }
}

impl OnFailure {
    pub(crate) fn from_context(context: &PreprocessorContext) -> Self {
        context
            .config
            .get("preprocessor.admonish.on_failure")
            .and_then(|value| value.as_str())
            .map(|value| OnFailure::from_str(value).unwrap_or_default())
            .unwrap_or_default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Renderer {
    Html,
    Test,
}

impl FromStr for Renderer {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, ()> {
        match string {
            "html" => Ok(Self::Html),
            "test" => Ok(Self::Test),
            _ => Err(()),
        }
    }
}

impl AdmonitionDefaults {
    pub(crate) fn from_context(ctx: &PreprocessorContext) -> Result<Self> {
        const KEY: &str = "preprocessor.admonish.default";
        let table = ctx.config.get(KEY);

        Ok(if let Some(table) = table {
            table
                .to_owned()
                .try_into()
                .with_context(|| "{KEY} could not be parsed from book.toml")?
        } else {
            Default::default()
        })
    }
}
