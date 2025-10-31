/// Output formatting for character/emoji conversion.
pub mod output;
/// Character pattern mappings.
pub mod patterns;

use crate::error::GivError;
pub use output::{CharResult, CharsOutput};

/// Convert a single input to a character or emoji.
///
/// This function uses the `penmanship` crate for Unicode character lookup,
/// which supports multiple input formats:
/// - Patterns: `"..."`, `"alpha"`, `"(c)"`, `"em"` (for em-dash), `"->"`, etc.
/// - HTML entities: `"&nbsp;"`, `"&lt;"`, `"&copy;"`, etc.
/// - Emoji shortcodes: `":smile:"`, `":heart:"`, `":thumbsup:"`, etc.
///
/// # Arguments
///
/// - `input` The input pattern, HTML entity, or emoji shortcode.
///
/// # Returns
///
/// A `CharResult` with the conversion result.
///
/// # Errors
///
/// Returns an error if the pattern, entity, or shortcode is not recognized.
pub fn convert_input(input: &str) -> Result<CharResult, GivError> {
    // Use penmanship for all lookups (handles patterns, HTML entities, and emoji).
    if let Some((character, name)) = patterns::lookup_pattern(input) {
        // Determine the type based on the input format.
        let result_type = if input.starts_with(':') && input.ends_with(':') {
            "emoji"
        } else if input.starts_with('&') && input.ends_with(';') {
            "html"
        } else {
            "pattern"
        };

        return Ok(CharResult {
            input: input.to_string(),
            output: character.to_string(),
            result_type: result_type.to_string(),
            name: Some(name.to_string()),
        });
    }

    // If no match, return an error.
    Err(GivError::UnknownCharacterPattern(input.to_string()))
}

// Tests.
#[cfg(test)]
mod tests {
    use super::*;

    /// Test emoji conversion via penmanship.
    #[test]
    fn test_emoji_conversion() {
        let result = convert_input(":smile:").unwrap();
        assert_eq!(result.output, "😄");
        assert_eq!(result.result_type, "emoji");
    }

    /// Test HTML entity conversion via penmanship.
    #[test]
    fn test_html_entity_conversion() {
        let result = convert_input("&nbsp;").unwrap();
        assert_eq!(result.output, "\u{00A0}");
        assert_eq!(result.result_type, "html");
    }

    /// Test pattern conversion.
    #[test]
    fn test_pattern_conversion() {
        let result = convert_input("1/4").unwrap();
        assert_eq!(result.output, "¼");
        assert_eq!(result.result_type, "pattern");
    }

    /// Test copyright symbol.
    #[test]
    fn test_copyright() {
        let result = convert_input("(c)").unwrap();
        assert_eq!(result.output, "©");
        assert_eq!(result.result_type, "pattern");
    }

    /// Test em-dash via "em" alias (issue #56).
    #[test]
    fn test_em_dash_alias() {
        let result = convert_input("em").unwrap();
        assert_eq!(result.output, "—");
        assert_eq!(result.result_type, "pattern");
    }

    /// Test arrow patterns (issue #56).
    #[test]
    fn test_arrow_patterns() {
        let result = convert_input("->").unwrap();
        assert_eq!(result.output, "→");
        assert_eq!(result.result_type, "pattern");
    }

    /// Test unknown pattern returns error.
    #[test]
    fn test_unknown_pattern_error() {
        let result = convert_input("unknown");
        assert!(result.is_err());
        match result {
            Err(GivError::UnknownCharacterPattern(s)) => {
                assert_eq!(s, "unknown");
            }
            _ => panic!("Expected UnknownCharacterPattern error"),
        }
    }
}
