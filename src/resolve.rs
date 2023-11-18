use crate::config::InstanceConfig;
use crate::flavours::FlavourMap;
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
        flavours: &FlavourMap,
    ) -> Option<Result<Self, String>> {
        InstanceConfig::from_info_string(info_string)
            .map(|raw| raw.and_then(|raw| Self::resolve(raw, defaults, flavours)))
    }

    /// Combine the per-admonition configuration with global defaults (and
    /// other logic) to resolve the values needed for rendering.
    fn resolve(
        raw: InstanceConfig,
        defaults: &AdmonitionDefaults,
        flavours: &FlavourMap,
    ) -> Result<Self, String> {
        let InstanceConfig {
            directive: raw_directive,
            title,
            id,
            additional_classnames,
            collapsible,
        } = raw;

        // default directive if not specified is note
        let directive = if raw_directive.trim().is_empty() {
            "note".to_owned()
        } else {
            raw_directive
        };

        let Some(flavour) = flavours.get(&directive) else {
            return Err(format!("unknown directive: {directive}"));
        };

        // Use values from block, else load default value
        let title = title.or_else(|| defaults.title.clone());
        let collapsible = collapsible.unwrap_or(defaults.collapsible);

        // if no provided or global default title, use the flavour's title
        let title = title.unwrap_or_else(|| flavour.title());

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

#[cfg(test)]
mod test {
    use super::*;
    use crate::flavours::default_flavour_map;
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
                &Default::default(),
                &default_flavour_map(),
            ),
            Ok(AdmonitionMeta {
                directive: "note".to_owned(),
                title: "Note".to_owned(),
                css_id: CssId::Prefix("admonition-".to_owned()),
                additional_classnames: Vec::new(),
                collapsible: false,
            })
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
                &default_flavour_map(),
            ),
            Ok(AdmonitionMeta {
                directive: "note".to_owned(),
                title: "Important!!!".to_owned(),
                css_id: CssId::Prefix("custom-prefix-".to_owned()),
                additional_classnames: Vec::new(),
                collapsible: true,
            })
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
                &default_flavour_map()
            ),
            Ok(AdmonitionMeta {
                directive: "note".to_owned(),
                title: "Important!!!".to_owned(),
                css_id: CssId::Verbatim("my-custom-id".to_owned()),
                additional_classnames: Vec::new(),
                collapsible: true,
            })
        );
    }
}
