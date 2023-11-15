use serde::{Deserialize, Serialize};
use std::str::FromStr;

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

#[derive(Debug, PartialEq)]
pub(crate) enum Directive {
    Note,
    Abstract,
    Info,
    Tip,
    Success,
    Question,
    Warning,
    Failure,
    Danger,
    Bug,
    Example,
    Quote,
}

impl FromStr for Directive {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, ()> {
        match string {
            "note" => Ok(Self::Note),
            "abstract" | "summary" | "tldr" => Ok(Self::Abstract),
            "info" | "todo" => Ok(Self::Info),
            "tip" | "hint" | "important" => Ok(Self::Tip),
            "success" | "check" | "done" => Ok(Self::Success),
            "question" | "help" | "faq" => Ok(Self::Question),
            "warning" | "caution" | "attention" => Ok(Self::Warning),
            "failure" | "fail" | "missing" => Ok(Self::Failure),
            "danger" | "error" => Ok(Self::Danger),
            "bug" => Ok(Self::Bug),
            "example" => Ok(Self::Example),
            "quote" | "cite" => Ok(Self::Quote),
            _ => Err(()),
        }
    }
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
    /// the prefix from default.css_id_prefix
    ///
    /// will generate the rest of the id based on the title
    Prefix(String),
}
