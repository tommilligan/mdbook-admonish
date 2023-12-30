//! This module is responsible for generating custom CSS for new admonition variants.
//!
//! It has unit tests to ensure the output matches that of the compile_assets CSS.

use anyhow::{anyhow, Context, Result};
use hex_color::{Case, HexColor};
use once_cell::sync::Lazy;
use regex::Regex;
use std::fs;
use std::path::Path;

static RX_COLLAPSE_NEWLINES: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"[\r\n]+\s*").expect("invalid whitespace regex"));

/// Do some simple things to make the svg input probably a valid data url
/// Based on this gist: https://gist.github.com/jennyknuth/222825e315d45a738ed9d6e04c7a88d0
fn svg_to_data_url(svg: &str) -> String {
    const XMLNS: &str = r#"http://www.w3.org/2000/svg"#;
    //
    let mut svg = RX_COLLAPSE_NEWLINES.replace_all(svg, "").to_string();
    if !svg.contains(XMLNS) {
        log::warn!("Your SVG file does not contain '<svg xmlns=\"{XMLNS}\"', it will likely fail to render.");
    }

    svg = svg
        .replace('"', "'")
        .replace('%', "%25")
        .replace('#', "%23")
        .replace('{', "%7B")
        .replace('}', "%7D");
    format!("url(\"data:image/svg+xml;charset=utf-8,{}\")", svg)
}

/// Given a valid set of inputs, generate the relevant CSS.
///
/// It is up to the caller to validate inputs.
fn directive_css(name: &str, svg_data: &str, tint: HexColor) -> String {
    let data_url = svg_to_data_url(svg_data);
    let tint_faint = format!("rgba({}, {}, {}, {})", tint.r, tint.g, tint.b, 0.1);
    let tint = tint.display_rgb().with_case(Case::Lower);
    format!(
        ":root {{
  --md-admonition-icon--admonish-{name}: {data_url};
}}

:is(.admonition):is(.admonish-{name}) {{
  border-color: {tint};
}}

:is(.admonish-{name}) > :is(.admonition-title, summary.admonition-title) {{
  background-color: {tint_faint};
}}
:is(.admonish-{name}) > :is(.admonition-title, summary.admonition-title)::before {{
  background-color: {tint};
  mask-image: var(--md-admonition-icon--admonish-{name});
  -webkit-mask-image: var(--md-admonition-icon--admonish-{name});
  mask-repeat: no-repeat;
  -webkit-mask-repeat: no-repeat;
  mask-size: contain;
  -webkit-mask-repeat: no-repeat;
}}
",
        name = name,
        data_url = data_url,
        tint = tint,
        tint_faint = tint_faint
    )
}

#[doc(hidden)]
pub fn css_from_config(book_dir: &Path, config: &str) -> Result<String> {
    let config = crate::book_config::admonish_config_from_str(config)?;
    let custom_directives = config.custom;

    if custom_directives.is_empty() {
        return Err(anyhow!("No custom directives provided"));
    }

    log::info!("Loaded {} custom directives", custom_directives.len());
    let mut css = String::new();
    for directive in custom_directives.iter() {
        let svg = fs::read_to_string(book_dir.join(&directive.icon))
            .with_context(|| format!("can't read icon file '{}'", directive.icon.display()))?;
        css.push_str(&directive_css(&directive.directive, &svg, directive.color));
    }
    Ok(css)
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    const GENERATED_CSS: &str = include_str!("./test_data/mdbook-admonish-custom-expected.css");
    const NOTE_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox='0 0 24 24'>
  <path d='M20.71 7.04c.39-.39.39-1.04 0-1.41l-2.34-2.34c-.37-.39-1.02-.39-1.41 0l-1.84 1.83 3.75 3.75M3 17.25V21h3.75L17.81 9.93l-3.75-3.75L3 17.25z'/>
</svg>
"#;

    // Verify the generated CSS here against a sample from the compile_assets output.
    //
    // The ensures that any new custom CSS will be in line with official styles.
    #[test]
    fn verify_against_generated_css() {
        let actual = directive_css("note", NOTE_SVG, HexColor::parse("#448aff").unwrap());
        assert_eq!(
            GENERATED_CSS, actual,
            "Rust generated CSS is out of step with SCSS generated CSS"
        )
    }
}
