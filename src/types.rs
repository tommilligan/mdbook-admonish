use serde::{Deserialize, Serialize};

/// Book wide defaults that may be provided by the user.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct AdmonitionDefaults {
    #[serde(default)]
    pub(crate) title: Option<String>,

    #[serde(default)]
    pub(crate) collapsible: bool,

    #[serde(default)]
    pub(crate) css_id_prefix: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RenderTextMode {
    Strip,
    Html,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CssId {
    /// id="my-id" in the admonishment
    ///
    /// used directly for the id field
    Verbatim(String),
    /// the prefix from default.css_id_prefix (or "admonish-" if not specified)
    ///
    /// will generate the rest of the id based on the title
    Prefix(String),
}
