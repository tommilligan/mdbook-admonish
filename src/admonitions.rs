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
pub(crate) struct AdmonitionKind {
    // TODO should we name this directive instead?
    pub(crate) id: Cow<'static, str>,
    #[serde(default)]
    pub(crate) title: Option<Cow<'static, str>>,
    pub(crate) icon: Cow<'static, str>,
    pub(crate) color: Color,
}

impl AdmonitionKind {
    pub(crate) fn title(&self) -> String {
        if let Some(title) = &self.title {
            title.clone().into_owned()
        } else {
            uppercase_first(&self.id)
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
pub(crate) struct AdmonitionKinds {
    pub(crate) custom: Vec<AdmonitionKind>,
}

impl AdmonitionKinds {
    // TODO validate custom input
    // - valid directives
    // - no duplicate directives
    #[allow(unused)]
    pub(crate) fn validate(&self) {
        todo!()
    }

    /// finds an admonition kind with the specified directive
    ///
    /// will fall back on the builtin/default directives if a custom one isn't found
    pub(crate) fn get(&self, directive: &str) -> Option<&AdmonitionKind> {
        self.custom
            .iter()
            .chain(DEFAULT_ADMONITIONS)
            .find(|kind| kind.id == directive)
    }
}

// defaults are always included but any custom ones with the same id will override the built in one

// TODO macro for this?
pub(crate) const DEFAULT_ADMONITIONS: &[AdmonitionKind] = &[
    AdmonitionKind {
        id: Cow::Borrowed("note"),
        title: None,
        icon: Cow::Borrowed("data:image/svg+xml;charset=utf-8,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'><path d='M20.71 7.04c.39-.39.39-1.04 0-1.41l-2.34-2.34c-.37-.39-1.02-.39-1.41 0l-1.84 1.83 3.75 3.75M3 17.25V21h3.75L17.81 9.93l-3.75-3.75L3 17.25z'/></svg>"),
        color: Color::hex(0x448aff),
    },
    AdmonitionKind {
        id: Cow::Borrowed("tldr"),
        title: Some(Cow::Borrowed("TL;DR")),
        icon: Cow::Borrowed("data:image/svg+xml;charset=utf-8,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'><path d='M17 9H7V7h10m0 6H7v-2h10m-3 6H7v-2h7M12 3a1 1 0 0 1 1 1 1 1 0 0 1-1 1 1 1 0 0 1-1-1 1 1 0 0 1 1-1m7 0h-4.18C14.4 1.84 13.3 1 12 1c-1.3 0-2.4.84-2.82 2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V5a2 2 0 0 0-2-2z'/></svg>"),
        color: Color::hex(0x00b0ff),
    },
    // TODO add the rest
];
