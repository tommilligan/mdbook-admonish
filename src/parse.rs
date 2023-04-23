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

fn extract_admonish_body_end_index(content: &str) -> usize {
    let number_fence_characters = content
        .chars()
        .rev()
        .position(|c| !(c == '`' || c == '~'))
        .unwrap_or_default();

    content.len() - number_fence_characters
}

/// Given the whole text content of the code fence, extract the body.
///
/// This really feels like we should get the markdown parser to do it for us,
/// but it's not really clear a good way of doing that.
///
/// ref: https://spec.commonmark.org/0.30/#fenced-code-blocks
pub(crate) fn extract_admonish_body(content: &str) -> &str {
    let start_index = extract_admonish_body_start_index(content);
    let end_index = extract_admonish_body_end_index(content);

    let admonish_content = &content[start_index..end_index];
    // The newline after a code block is technically optional, so we have to
    // trim it off dynamically.
    admonish_content.trim_end()
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
            ("\n```", 1),
            // different lengths
            ("\n``````", 1),
            ("\n~~~~", 1),
            // whitespace before fence end
            ("\n   ```", 4),
            ("content\n```", 8),
        ] {
            let actual = extract_admonish_body_end_index(text);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_extract() {
        for (text, expected) in [
            // standard
            ("```admonish\ncontent\n```", "content"),
            // whitespace
            ("```admonish  \n  content  \n  ```", "  content"),
            // longer
            ("`````admonish\ncontent\n`````", "content"),
            // unequal
            ("~~~admonish\ncontent\n~~~~~", "content"),
        ] {
            let actual = extract_admonish_body(text);
            assert_eq!(actual, expected);
        }
    }
}
