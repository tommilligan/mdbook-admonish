use crate::config::InstanceConfig;
use crate::types::{BuiltinDirective, CssId, CustomDirective, CustomDirectiveMap, Overrides};
use std::fmt;
use std::str::FromStr;

/// All information required to render an admonition.
///
/// i.e. all configured options have been resolved at this point.
#[derive(Debug, PartialEq)]
pub(crate) struct AdmonitionMeta {
    pub directive: String,
    pub title: String,
    pub css_id: CssId,
    pub additional_classnames: Vec<String>,
    pub collapsible: bool,
}

/// Wrapper type to hold any value directive configuration.
enum Directive {
    Builtin(BuiltinDirective),
    Custom(CustomDirective),
}

impl fmt::Display for Directive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Builtin(builtin) => builtin.fmt(f),
            Self::Custom(custom) => f.write_str(&custom.directive),
        }
    }
}

impl Directive {
    fn from_str(custom_directive_map: &CustomDirectiveMap, string: &str) -> Result<Self, ()> {
        if let Ok(builtin) = BuiltinDirective::from_str(string) {
            return Ok(Self::Builtin(builtin));
        }

        if let Some(config) = custom_directive_map.get(string) {
            return Ok(Self::Custom(config.clone()));
        }

        Err(())
    }

    fn title(self, raw_directive: &str) -> String {
        match self {
            Directive::Builtin(_) => format_builtin_directive_title(raw_directive),
            Directive::Custom(custom) => custom
                .title
                .clone()
                .unwrap_or_else(|| uppercase_first(raw_directive)),
        }
    }
}

impl AdmonitionMeta {
    pub fn from_info_string(
        info_string: &str,
        overrides: &Overrides,
    ) -> Option<Result<Self, String>> {
        InstanceConfig::from_info_string(info_string)
            .map(|raw| raw.map(|raw| Self::resolve(raw, overrides)))
    }

    /// Combine the per-admonition configuration with global defaults (and
    /// other logic) to resolve the values needed for rendering.
    fn resolve(raw: InstanceConfig, overrides: &Overrides) -> Self {
        let InstanceConfig {
            directive: raw_directive,
            title,
            id,
            additional_classnames,
            collapsible,
        } = raw;

        // Use values from block, else load default value
        let title = title.or_else(|| overrides.book.title.clone());

        let directive = Directive::from_str(&overrides.custom, &raw_directive);

        let collapsible = match directive {
            // If the directive is a builin one, use collapsible from block, else use default
            // value of the builtin directive, else use global default value
            Ok(Directive::Builtin(directive)) => collapsible.unwrap_or(
                overrides
                    .builtin
                    .get(&directive)
                    .and_then(|config| config.collapsible)
                    .unwrap_or(overrides.book.collapsible),
            ),
            // If the directive is a custom one, use collapsible from block, else use default
            // value of the custom directive, else use global default value
            Ok(Directive::Custom(ref custom_dir)) => {
                collapsible.unwrap_or(custom_dir.collapsible.unwrap_or(overrides.book.collapsible))
            }
            Err(_) => collapsible.unwrap_or(overrides.book.collapsible),
        };

        // Load the directive (and title, if one still not given)
        let (directive, title) = match (directive, title) {
            (Ok(directive), None) => (directive.to_string(), directive.title(&raw_directive)),
            (Err(_), None) => (BuiltinDirective::Note.to_string(), "Note".to_owned()),
            (Ok(directive), Some(title)) => (directive.to_string(), title),
            (Err(_), Some(title)) => (BuiltinDirective::Note.to_string(), title),
        };

        let css_id = if let Some(verbatim) = id {
            CssId::Verbatim(verbatim)
        } else {
            const DEFAULT_CSS_ID_PREFIX: &str = "admonition-";
            CssId::Prefix(
                overrides
                    .book
                    .css_id_prefix
                    .clone()
                    .unwrap_or_else(|| DEFAULT_CSS_ID_PREFIX.to_owned()),
            )
        };

        Self {
            directive,
            title,
            css_id,
            additional_classnames,
            collapsible,
        }
    }
}

/// Format the title of an admonition directive
///
/// We special case a few words to make them look nicer (e.g. "tldr" -> "TL;DR" and "faq" -> "FAQ").
fn format_builtin_directive_title(input: &str) -> String {
    match input {
        "tldr" => "TL;DR".to_owned(),
        "faq" => "FAQ".to_owned(),
        _ => uppercase_first(input),
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

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::types::{AdmonitionDefaults, BuiltinDirectiveConfig};

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_format_builtin_directive_title() {
        assert_eq!(format_builtin_directive_title(""), "");
        assert_eq!(format_builtin_directive_title("a"), "A");
        assert_eq!(format_builtin_directive_title("tldr"), "TL;DR");
        assert_eq!(format_builtin_directive_title("faq"), "FAQ");
        assert_eq!(format_builtin_directive_title("note"), "Note");
        assert_eq!(format_builtin_directive_title("abstract"), "Abstract");
        // Unicode
        assert_eq!(format_builtin_directive_title("🦀"), "🦀");
    }

    #[test]
    fn test_admonition_info_from_raw() {
        assert_eq!(
            AdmonitionMeta::resolve(
                InstanceConfig {
                    directive: " ".to_owned(),
                    title: None,
                    id: None,
                    additional_classnames: Vec::new(),
                    collapsible: None,
                },
                &Overrides::default(),
            ),
            AdmonitionMeta {
                directive: "note".to_owned(),
                title: "Note".to_owned(),
                css_id: CssId::Prefix("admonition-".to_owned()),
                additional_classnames: Vec::new(),
                collapsible: false,
            }
        );
    }

    #[test]
    fn test_admonition_info_from_raw_with_defaults() {
        assert_eq!(
            AdmonitionMeta::resolve(
                InstanceConfig {
                    directive: " ".to_owned(),
                    title: None,
                    id: None,
                    additional_classnames: Vec::new(),
                    collapsible: None,
                },
                &Overrides {
                    book: AdmonitionDefaults {
                        title: Some("Important!!!".to_owned()),
                        css_id_prefix: Some("custom-prefix-".to_owned()),
                        collapsible: true,
                    },
                    ..Default::default()
                }
            ),
            AdmonitionMeta {
                directive: "note".to_owned(),
                title: "Important!!!".to_owned(),
                css_id: CssId::Prefix("custom-prefix-".to_owned()),
                additional_classnames: Vec::new(),
                collapsible: true,
            }
        );
    }

    #[test]
    fn test_admonition_info_from_raw_with_defaults_and_custom_id() {
        assert_eq!(
            AdmonitionMeta::resolve(
                InstanceConfig {
                    directive: " ".to_owned(),
                    title: None,
                    id: Some("my-custom-id".to_owned()),
                    additional_classnames: Vec::new(),
                    collapsible: None,
                },
                &Overrides {
                    book: AdmonitionDefaults {
                        title: Some("Important!!!".to_owned()),
                        css_id_prefix: Some("ignored-custom-prefix-".to_owned()),
                        collapsible: true,
                    },
                    ..Default::default()
                }
            ),
            AdmonitionMeta {
                directive: "note".to_owned(),
                title: "Important!!!".to_owned(),
                css_id: CssId::Verbatim("my-custom-id".to_owned()),
                additional_classnames: Vec::new(),
                collapsible: true,
            }
        );
    }

    #[test]
    fn test_admonition_info_from_raw_with_custom_directive() {
        assert_eq!(
            AdmonitionMeta::resolve(
                InstanceConfig {
                    directive: "frog".to_owned(),
                    title: None,
                    id: None,
                    additional_classnames: Vec::new(),
                    collapsible: None,
                },
                &Overrides {
                    custom: [CustomDirective {
                        directive: "frog".to_owned(),
                        aliases: Vec::new(),
                        title: None,
                        collapsible: None,
                    }]
                    .into_iter()
                    .collect(),
                    ..Default::default()
                }
            ),
            AdmonitionMeta {
                directive: "frog".to_owned(),
                title: "Frog".to_owned(),
                css_id: CssId::Prefix("admonition-".to_owned()),
                additional_classnames: Vec::new(),
                collapsible: false,
            }
        );
    }

    #[test]
    fn test_admonition_info_from_raw_with_custom_directive_and_title() {
        assert_eq!(
            AdmonitionMeta::resolve(
                InstanceConfig {
                    directive: "frog".to_owned(),
                    title: None,
                    id: None,
                    additional_classnames: Vec::new(),
                    collapsible: None,
                },
                &Overrides {
                    custom: [CustomDirective {
                        directive: "frog".to_owned(),
                        aliases: Vec::new(),
                        title: Some("🏳️‍🌈".to_owned()),
                        collapsible: None,
                    }]
                    .into_iter()
                    .collect(),
                    ..Default::default()
                }
            ),
            AdmonitionMeta {
                directive: "frog".to_owned(),
                title: "🏳️‍🌈".to_owned(),
                css_id: CssId::Prefix("admonition-".to_owned()),
                additional_classnames: Vec::new(),
                collapsible: false,
            }
        );
    }

    #[test]
    fn test_admonition_info_from_raw_with_custom_directive_alias() {
        assert_eq!(
            AdmonitionMeta::resolve(
                InstanceConfig {
                    directive: "toad".to_owned(),
                    title: Some("Still a frog".to_owned()),
                    id: None,
                    additional_classnames: Vec::new(),
                    collapsible: None,
                },
                &Overrides {
                    custom: [CustomDirective {
                        directive: "frog".to_owned(),
                        aliases: vec!["newt".to_owned(), "toad".to_owned()],
                        title: Some("🏳️‍🌈".to_owned()),
                        collapsible: None,
                    }]
                    .into_iter()
                    .collect(),
                    ..Default::default()
                }
            ),
            AdmonitionMeta {
                directive: "frog".to_owned(),
                title: "Still a frog".to_owned(),
                css_id: CssId::Prefix("admonition-".to_owned()),
                additional_classnames: Vec::new(),
                collapsible: false,
            }
        );
    }

    #[test]
    fn test_admonition_info_from_raw_with_collapsible_custom_directive() {
        assert_eq!(
            AdmonitionMeta::resolve(
                InstanceConfig {
                    directive: "frog".to_owned(),
                    title: None,
                    id: None,
                    additional_classnames: Vec::new(),
                    collapsible: None,
                },
                &Overrides {
                    custom: [CustomDirective {
                        directive: "frog".to_owned(),
                        aliases: Vec::new(),
                        title: None,
                        collapsible: Some(true),
                    }]
                    .into_iter()
                    .collect(),
                    ..Default::default()
                }
            ),
            AdmonitionMeta {
                directive: "frog".to_owned(),
                title: "Frog".to_owned(),
                css_id: CssId::Prefix("admonition-".to_owned()),
                additional_classnames: Vec::new(),
                collapsible: true,
            }
        );
    }

    #[test]
    fn test_admonition_info_from_raw_with_collapsible_builtin_directive() {
        assert_eq!(
            AdmonitionMeta::resolve(
                InstanceConfig {
                    directive: "abstract".to_owned(),
                    title: None,
                    id: None,
                    additional_classnames: Vec::new(),
                    collapsible: None,
                },
                &Overrides {
                    book: AdmonitionDefaults {
                        title: None,
                        css_id_prefix: None,
                        collapsible: false,
                    },
                    builtin: HashMap::from([(
                        BuiltinDirective::Abstract,
                        BuiltinDirectiveConfig {
                            collapsible: Some(true),
                        }
                    )]),
                    ..Default::default()
                }
            ),
            AdmonitionMeta {
                directive: "abstract".to_owned(),
                title: "Abstract".to_owned(),
                css_id: CssId::Prefix("admonition-".to_owned()),
                additional_classnames: Vec::new(),
                collapsible: true,
            }
        );
    }

    #[test]
    fn test_admonition_info_from_raw_with_non_collapsible_builtin_directive() {
        assert_eq!(
            AdmonitionMeta::resolve(
                InstanceConfig {
                    directive: "abstract".to_owned(),
                    title: None,
                    id: None,
                    additional_classnames: Vec::new(),
                    collapsible: None,
                },
                &Overrides {
                    book: AdmonitionDefaults {
                        title: None,
                        css_id_prefix: None,
                        collapsible: true,
                    },
                    builtin: HashMap::from([(
                        BuiltinDirective::Abstract,
                        BuiltinDirectiveConfig {
                            collapsible: Some(false),
                        }
                    )]),
                    ..Default::default()
                }
            ),
            AdmonitionMeta {
                directive: "abstract".to_owned(),
                title: "Abstract".to_owned(),
                css_id: CssId::Prefix("admonition-".to_owned()),
                additional_classnames: Vec::new(),
                collapsible: false,
            }
        );
    }
}
