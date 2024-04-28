use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

/// Book wide defaults that may be provided by the user.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub(crate) struct AdmonitionDefaults {
    #[serde(default)]
    pub(crate) title: Option<String>,

    #[serde(default)]
    pub(crate) collapsible: bool,

    #[serde(default)]
    // For backwards compatibility, we support this field with kebab-case style
    // naming, even though this was introduced in error.
    #[serde(alias = "css-id-prefix")]
    pub(crate) css_id_prefix: Option<String>,
}

/// First class supported directives by the crate.
///
/// These are guaranteed to have valid CSS/icons available.
///
/// Custom directives can also be added via the book.toml config.
#[derive(Debug, PartialEq, Clone, Copy, Eq, Deserialize, Serialize, Hash)]
#[serde(rename_all = "lowercase")]
pub(crate) enum BuiltinDirective {
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

impl FromStr for BuiltinDirective {
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

impl fmt::Display for BuiltinDirective {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Note => "note",
            Self::Abstract => "abstract",
            Self::Info => "info",
            Self::Tip => "tip",
            Self::Success => "success",
            Self::Question => "question",
            Self::Warning => "warning",
            Self::Failure => "failure",
            Self::Danger => "danger",
            Self::Bug => "bug",
            Self::Example => "example",
            Self::Quote => "quote",
        };
        f.write_str(value)
    }
}

/// The subset of information we care about during plugin runtime for custom directives.
///
/// This drops information only needed during CSS generation.
#[derive(Debug, Clone)]
pub(crate) struct CustomDirective {
    pub directive: String,
    pub aliases: Vec<String>,
    pub title: Option<String>,
    pub collapsible: Option<bool>,
}

impl From<(String, crate::book_config::CustomDirective)> for CustomDirective {
    fn from((directive, config): (String, crate::book_config::CustomDirective)) -> Self {
        let crate::book_config::CustomDirective {
            aliases,
            title,
            collapsible,
            ..
        } = config;
        Self {
            directive,
            aliases,
            title,
            collapsible,
        }
    }
}

/// A map from the user given directive to underlying config.
///
/// The terminology is a bit mixed here - this map allows any input-directive,
/// and returns the output-directive config.
///
/// i.e. this is the step alias mapping happens at
#[derive(Debug, Clone, Default)]
pub(crate) struct CustomDirectiveMap {
    inner: HashMap<String, CustomDirective>,
}

impl CustomDirectiveMap {
    pub fn get(&self, key: &str) -> Option<&CustomDirective> {
        self.inner.get(key)
    }
}

impl FromIterator<CustomDirective> for CustomDirectiveMap {
    fn from_iter<I: IntoIterator<Item = CustomDirective>>(iter: I) -> Self {
        let mut inner = HashMap::default();
        for config in iter.into_iter() {
            inner
                .entry(config.directive.clone())
                .or_insert(config.clone());

            for alias in config.aliases.iter() {
                inner.entry(alias.clone()).or_insert(config.clone());
            }
        }

        Self { inner }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub(crate) struct BuiltinDirectiveConfig {
    /// Default collapsible value.
    #[serde(default)]
    pub collapsible: Option<bool>,
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

#[derive(Debug, Clone, Default)]
pub(crate) struct Overrides {
    pub book: AdmonitionDefaults,
    pub builtin: HashMap<BuiltinDirective, BuiltinDirectiveConfig>,
    pub custom: CustomDirectiveMap,
}
