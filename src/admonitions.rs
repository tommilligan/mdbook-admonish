use crate::color::Color;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

pub(crate) fn is_valid_directive(directive: &str) -> bool {
    static REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r#"^[A-Za-z0-9_-]+$"#).expect("directive regex"));

    REGEX.is_match(directive)
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct Flavour {
    pub(crate) directive: Cow<'static, str>,
    #[serde(default)]
    pub(crate) title: Option<Cow<'static, str>>,
    pub(crate) icon: Cow<'static, str>,
    pub(crate) color: Color,
}

impl Flavour {
    /// the flavour's specified title, or the default of title-casing the directive
    pub(crate) fn title(&self) -> String {
        if let Some(title) = &self.title {
            title.clone().into_owned()
        } else {
            uppercase_first(&self.directive)
        }
    }
}

/// Make the first letter of `input` uppercase.
///
/// source: https://stackoverflow.com/a/38406885
fn uppercase_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

#[test]
fn test_uppercase_first() {
    assert_eq!(uppercase_first(""), "");
    assert_eq!(uppercase_first("a"), "A");
    assert_eq!(uppercase_first("note"), "Note");
    assert_eq!(uppercase_first("abstract"), "Abstract");
    // Unicode
    assert_eq!(uppercase_first("🦀"), "🦀");
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
#[serde(transparent)]
pub(crate) struct CustomFlavours {
    pub(crate) custom: Vec<Flavour>,
}

impl CustomFlavours {
    // TODO validate custom input
    // - valid directives
    // - no duplicate directives
    #[allow(unused)]
    pub(crate) fn validate(&self) {
        todo!()
    }

    /// tries to finds an admonition kind with the specified directive
    ///
    /// will fall back on the builtin/default directives if a custom one isn't found
    pub(crate) fn get_or_builtin(&self, directive: &str) -> Option<&Flavour> {
        self.custom
            .iter()
            .chain(BUILTIN_FLAVOURS)
            .find(|kind| kind.directive == directive)
    }
}

macro_rules! flavours {
    ($([$($directive:literal $(: $title:literal)?),+] $icon_path:literal $color:literal),+ $(,)?) => {
        &[$($(
        Flavour {
            directive: Cow::Borrowed($directive),
            title: flavours!(@title $($title)?),
            icon: Cow::Borrowed(concat!("data:image/svg+xml;charset=utf-8,", include_str!(concat!("assets/", $icon_path)))),
            color: Color::hex($color),
        },
        )+)+]
    };

    (@title) => {
        None
    };
    (@title $title:literal) => {
        Some(Cow::Borrowed($title))
    };
}

// svg files live in src/assets
// TODO each flavour on its own line or combine similar?
pub(crate) const BUILTIN_FLAVOURS: &[Flavour] = flavours! {
    ["note"]                                 "pencil.svg"               0x448aff, // blue-a200
    ["abstract", "summary", "tldr": "TL;DR"] "clipboard-text.svg"       0x00b0ff, // light-blue-a400
    ["info", "todo": "TODO"]                 "information.svg"          0x00b8d4, // cyan-a700
    ["tip", "hint", "important"]             "fire.svg"                 0x00bfa5, // teal-a700
    ["success", "check", "done"]             "check-bold.svg"           0x00c853, // green-a700
    ["question", "help", "faq": "FAQ"]       "help-circle.svg"          0x64dd17, // light-green-a700
    ["warning", "caution", "attention"]      "alert.svg"                0xff9100, // orange-a400
    ["failure", "fail", "missing"]           "close-thick.svg"          0xff5252, // red-a200
    ["danger", "error"]                      "lightning-bold.svg"       0xff1744, // red-a400
    ["bug"]                                  "bug.svg"                  0xf50057, // pink-a400
    ["example"]                              "format-list-numbered.svg" 0x7c4dff, // deep-purple-a200
    ["quote", "cite"]                        "format-quote-close.svg"   0x9e9e9e, // grey-base
};
