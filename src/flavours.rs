use crate::color::Color;
use anyhow::anyhow;
use mdbook::errors::Result as MdbookResult;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;

pub(crate) fn is_valid_directive(directive: &str) -> bool {
    static REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r#"^[A-Za-z0-9_-]+$"#).expect("directive regex"));

    REGEX.is_match(directive)
}

// TODO do we need this? makes some code look nicer but
pub(crate) type FlavourMap = HashMap<String, Flavour>;

// test helper
#[cfg(test)]
pub(crate) fn default_flavour_map() -> FlavourMap {
    build_flavour_map(Vec::new()).unwrap()
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
    /// the flavour's specified title, otherwise the default of title-casing the directive
    pub(crate) fn title(&self) -> String {
        if let Some(title) = &self.title {
            title.to_string()
        } else {
            uppercase_first(&self.directive)
        }
    }

    pub(crate) fn class_name(&self) -> String {
        format!("admonish-{}", self.directive)
    }

    #[allow(unused)] // TODO remove
    pub(crate) fn css(&self) -> String {
        format!(
            r#"
:is(.admonition):is(.{class_name}) {{
    border-color: rgb({r}, {g}, {b});
}}

:is(.{class_name}) > :is(.admonition-title, summary.admonition-title) {{
    background-color: rgba({r}, {g}, {b}, 0.1);
}}
:is(.{class_name}) > :is(.admonition-title, summary.admonition-title)::before {{
    background-color: rgb({r}, {g}, {b});
    mask-image: url("{icon}");
    -webkit-mask-image: url("{icon}");
    mask-repeat: no-repeat;
    -webkit-mask-repeat: no-repeat;
    mask-size: contain;
    -webkit-mask-repeat: no-repeat;
}}
        "#,
            class_name = self.class_name(),
            r = self.color.red,
            g = self.color.green,
            b = self.color.blue,
            icon = self.icon,
        )
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

/// Makes sure there are no duplicates and all directives are valid,
/// then inserts the builtin flavours if they aren't overridden
pub(crate) fn build_flavour_map(custom_flavours: Vec<Flavour>) -> MdbookResult<FlavourMap> {
    let mut map = HashMap::with_capacity(custom_flavours.len() + BUILTIN_FLAVOURS.len());

    // validate and add all custom flavours
    for custom in custom_flavours {
        let directive = custom.directive.to_string();

        if !is_valid_directive(&directive) {
            // TODO fix this error message? (regex in errors is meh)
            return Err(anyhow!(
                "invalid directive (must match ^[A-Za-z0-9_-]+$): {directive}"
            ));
        }

        if map.contains_key(&directive) {
            return Err(anyhow!("duplicate custom directive: {directive}"));
        }

        map.insert(directive, custom);
    }

    // add all builtin flavours, skipping if already present
    for builtin in BUILTIN_FLAVOURS {
        if !map.contains_key(&*builtin.directive) {
            // the clone here is a no-op cuz all the cows are borrowed
            map.insert(builtin.directive.to_string(), builtin.clone());
        }
    }

    Ok(map)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uppercase_first() {
        assert_eq!(uppercase_first(""), "");
        assert_eq!(uppercase_first("a"), "A");
        assert_eq!(uppercase_first("note"), "Note");
        assert_eq!(uppercase_first("abstract"), "Abstract");
        // Unicode
        assert_eq!(uppercase_first("🦀"), "🦀");
    }

    #[test]
    fn valid_directives() {
        assert!(is_valid_directive("abcdefghijklmnopqrstuvwxyz"));
        assert!(is_valid_directive("ABCDEFGHIJKLMNOPQRSTUVWXYZ"));
        assert!(is_valid_directive("0123456789"));
        assert!(is_valid_directive("-_"));
        assert!(is_valid_directive("a"));
        assert!(is_valid_directive("note"));
        assert!(is_valid_directive("bug"));
        assert!(is_valid_directive("Frogs_001"));
        assert!(is_valid_directive("green-CATS_9"));
    }

    #[test]
    fn validate_builtin_directives() {
        for builtin in BUILTIN_FLAVOURS {
            let directive = &builtin.directive;

            assert!(
                is_valid_directive(directive),
                "invalid builtin directive: {directive}",
            );
        }
    }

    #[test]
    fn invalid_directives() {
        assert!(!is_valid_directive(""));
        assert!(!is_valid_directive(" "));
        assert!(!is_valid_directive("abc 123"));
        assert!(!is_valid_directive("meow 🐱")); // unicode
        assert!(!is_valid_directive("!"));
        assert!(!is_valid_directive("@"));
        assert!(!is_valid_directive("#"));
        assert!(!is_valid_directive("$"));
        assert!(!is_valid_directive("%"));
        assert!(!is_valid_directive("^"));
        assert!(!is_valid_directive("&"));
        assert!(!is_valid_directive("*"));
        assert!(!is_valid_directive("("));
        assert!(!is_valid_directive(")"));
        assert!(!is_valid_directive("+"));
        assert!(!is_valid_directive("="));
        assert!(!is_valid_directive("`"));
        assert!(!is_valid_directive("~"));
        assert!(!is_valid_directive("["));
        assert!(!is_valid_directive("{"));
        assert!(!is_valid_directive("]"));
        assert!(!is_valid_directive("}"));
        assert!(!is_valid_directive("\\"));
        assert!(!is_valid_directive("|"));
        assert!(!is_valid_directive(";"));
        assert!(!is_valid_directive(":"));
        assert!(!is_valid_directive("\'"));
        assert!(!is_valid_directive("\""));
        assert!(!is_valid_directive(","));
        assert!(!is_valid_directive("."));
        assert!(!is_valid_directive("<"));
        assert!(!is_valid_directive(">"));
        assert!(!is_valid_directive("/"));
        assert!(!is_valid_directive("?"));
    }
}