use crate::admonitions::AdmonitionKinds;
use crate::config::InstanceConfig;
use crate::types::{AdmonitionDefaults, CssId};

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

impl AdmonitionMeta {
    pub fn from_info_string(
        info_string: &str,
        defaults: &AdmonitionDefaults,
        kinds: &AdmonitionKinds,
    ) -> Option<Result<Self, String>> {
        InstanceConfig::from_info_string(info_string)
            .map(|raw| raw.and_then(|raw| Self::resolve(raw, defaults, kinds)))
    }

    /// Combine the per-admonition configuration with global defaults (and
    /// other logic) to resolve the values needed for rendering.
    fn resolve(
        raw: InstanceConfig,
        defaults: &AdmonitionDefaults,
        kinds: &AdmonitionKinds,
    ) -> Result<Self, String> {
        let InstanceConfig {
            directive: raw_directive,
            title,
            id,
            additional_classnames,
            collapsible,
        } = raw;

        // empty directives default to notes
        let directive = if raw_directive.is_empty() {
            "note".to_owned()
        } else {
            raw_directive
        };

        let Some(kind) = kinds.get(&directive) else {
            return Err(format!("unknown directive: {directive}"));
        };

        // Use values from block, else load default value
        let title = title.or_else(|| defaults.title.clone());
        let collapsible = collapsible.unwrap_or(defaults.collapsible);

        // if no provided or global default title, use the kind's title
        let title = title.unwrap_or_else(|| kind.title());

        let css_id = if let Some(verbatim) = id {
            CssId::Verbatim(verbatim)
        } else {
            const DEFAULT_CSS_ID_PREFIX: &str = "admonition-";
            CssId::Prefix(
                defaults
                    .css_id_prefix
                    .clone()
                    .unwrap_or_else(|| DEFAULT_CSS_ID_PREFIX.to_owned()),
            )
        };

        Ok(Self {
            directive,
            title,
            css_id,
            additional_classnames,
            collapsible,
        })
    }
}

// TODO fix tests :|
#[cfg(FALSE)]
// #[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

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
                &Default::default()
            ),
            AdmonitionMeta {
                directive: Directive::Note,
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
                &AdmonitionDefaults {
                    title: Some("Important!!!".to_owned()),
                    css_id_prefix: Some("custom-prefix-".to_owned()),
                    collapsible: true,
                },
            ),
            AdmonitionMeta {
                directive: Directive::Note,
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
                &AdmonitionDefaults {
                    title: Some("Important!!!".to_owned()),
                    css_id_prefix: Some("ignored-custom-prefix-".to_owned()),
                    collapsible: true,
                },
            ),
            AdmonitionMeta {
                directive: Directive::Note,
                title: "Important!!!".to_owned(),
                css_id: CssId::Verbatim("my-custom-id".to_owned()),
                additional_classnames: Vec::new(),
                collapsible: true,
            }
        );
    }
}
