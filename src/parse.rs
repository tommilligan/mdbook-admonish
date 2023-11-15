use anyhow::{anyhow, Result};
use std::borrow::Cow;

pub use crate::preprocessor::Admonish;
use crate::{
    book_config::OnFailure,
    render::Admonition,
    resolve::AdmonitionMeta,
    types::{AdmonitionDefaults, CssId, Directive},
};

/// Given the content in the span of the code block, and the info string,
/// return `Some(Admonition)` if the code block is an admonition.
///
/// If there is an error parsing the admonition, either:
///
/// - Display a UI error message output in the book.
/// - If configured, break the build.
///
/// If the code block is not an admonition, return `None`.
pub(crate) fn parse_admonition<'a>(
    info_string: &'a str,
    admonition_defaults: &'a AdmonitionDefaults,
    content: &'a str,
    on_failure: OnFailure,
    indent: usize,
) -> Option<Result<Admonition<'a>>> {
    // We need to know fence details anyway for error messages
    let extracted = extract_admonish_body(content);

    let info = AdmonitionMeta::from_info_string(info_string, admonition_defaults)?;
    let info = match info {
        Ok(info) => info,
        Err(message) => {
            // Construct a fence capable of enclosing whatever we wrote for the
            // actual input block
            let fence = extracted.fence;
            let enclosing_fence: String = std::iter::repeat(fence.character)
                .take(fence.length + 1)
                .collect();
            return Some(match on_failure {
                OnFailure::Continue => {
                    log::warn!(
                        r#"Error processing admonition. To fail the build instead of continuing, set 'on_failure = "bail"'"#
                    );
                    Ok(Admonition {
                        directive: Directive::Bug,
                        title: "Error rendering admonishment".to_owned(),
                        css_id: CssId::Prefix("admonition-".to_owned()),
                        additional_classnames: Vec::new(),
                        collapsible: false,
                        content: Cow::Owned(format!(
                            r#"Failed with:

```log
{message}
```

Original markdown input:

{enclosing_fence}markdown
{content}
{enclosing_fence}
"#
                        )),
                        indent,
                    })
                }
                OnFailure::Bail => Err(anyhow!("Error processing admonition, bailing:\n{content}")),
            });
        }
    };

    Some(Ok(Admonition::new(
        info,
        extracted.body,
        // Note that this is a bit hacky - the fence information comes from the start
        // of the block, and includes the whole line.
        //
        // This is more likely to be what we want, as ending indentation is unrelated
        // according to the commonmark spec (ref https://spec.commonmark.org/0.12/#example-85)
        //
        // The main case we're worried about here is indenting enough to be inside list items,
        // and in this case the starting code fence must be indented enough to be considered
        // part of the list item.
        //
        // The hacky thing is that we're considering line indent in the document as a whole,
        // not relative to the context of some containing item. But I think that's what we
        // want for now, anyway.
        indent,
    )))
}

/// We can't trust the info string length to find the start of the body
/// it may change length if it contains HTML or character escapes.
///
/// So we scan for the first newline and use that.
/// If gods forbid it doesn't exist for some reason, just include the whole info string.
fn extract_admonish_body_start_index(content: &str) -> usize {
    let index = content
        .find('\n')
        // Start one character _after_ the newline
        .map(|index| index + 1);

    // If we can't get a valid index, include all content
    match index {
        // Couldn't find a newline
        None => 0,
        Some(index) => {
            // Index out of bound of content
            if index > (content.len() - 1) {
                0
            } else {
                index
            }
        }
    }
}

fn extract_admonish_body_end_index(content: &str) -> (usize, Fence) {
    let fence_character = content.chars().next_back().unwrap_or('`');
    let number_fence_characters = content
        .chars()
        .rev()
        .position(|c| c != fence_character)
        .unwrap_or_default();
    let fence = Fence::new(fence_character, number_fence_characters);

    let index = content.len() - fence.length;
    (index, fence)
}

#[derive(Debug, PartialEq)]
struct Fence {
    character: char,
    length: usize,
}

impl Fence {
    fn new(character: char, length: usize) -> Self {
        Self { character, length }
    }
}

#[derive(Debug, PartialEq)]
struct Extracted<'a> {
    body: &'a str,
    fence: Fence,
}

/// Given the whole text content of the code fence, extract the body.
///
/// This really feels like we should get the markdown parser to do it for us,
/// but it's not really clear a good way of doing that.
///
/// ref: https://spec.commonmark.org/0.30/#fenced-code-blocks
fn extract_admonish_body(content: &str) -> Extracted<'_> {
    let start_index = extract_admonish_body_start_index(content);
    let (end_index, fence) = extract_admonish_body_end_index(content);

    let admonish_content = &content[start_index..end_index];
    // The newline after a code block is technically optional, so we have to
    // trim it off dynamically.
    let body = admonish_content.trim_end();
    Extracted { body, fence }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_extract_start() {
        for (text, expected) in [
            ("```sane example\ncontent```", 16),
            ("~~~~~\nlonger fence", 6),
            // empty
            ("```\n```", 4),
            // bounds check, should not index outside of content
            ("```\n", 0),
        ] {
            let actual = extract_admonish_body_start_index(text);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_extract_end() {
        for (text, expected) in [
            ("\n```", (1, Fence::new('`', 3))),
            // different lengths
            ("\n``````", (1, Fence::new('`', 6))),
            ("\n~~~~", (1, Fence::new('~', 4))),
            // whitespace before fence end
            ("\n   ```", (4, Fence::new('`', 3))),
            ("content\n```", (8, Fence::new('`', 3))),
        ] {
            let actual = extract_admonish_body_end_index(text);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_extract() {
        fn content_fence(body: &'static str, character: char, length: usize) -> Extracted<'static> {
            Extracted {
                body,
                fence: Fence::new(character, length),
            }
        }
        for (text, expected) in [
            // empty
            ("```\n```", content_fence("", '`', 3)),
            // standard
            (
                "```admonish\ncontent\n```",
                content_fence("content", '`', 3),
            ),
            // whitespace
            (
                "```admonish  \n  content  \n  ```",
                content_fence("  content", '`', 3),
            ),
            // longer
            (
                "``````admonish\ncontent\n``````",
                content_fence("content", '`', 6),
            ),
            // unequal
            (
                "~~~admonish\ncontent\n~~~~~",
                // longer (end) fence returned
                content_fence("content", '~', 5),
            ),
        ] {
            let actual = extract_admonish_body(text);
            assert_eq!(actual, expected);
        }
    }
}
