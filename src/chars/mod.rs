/// Character pattern mappings.
pub mod patterns;
/// Output formatting for character/emoji conversion.
pub mod output;

use crate::error::GivError;
pub use output::{CharResult, CharsOutput};


/// Convert a single input to a character or emoji.
///
/// # Arguments
///
/// - `input` The input pattern or shortcode.
///
/// # Returns
///
/// A `CharResult` with the conversion result.
///
/// # Errors
///
/// Returns an error if the pattern or shortcode is not recognized.
pub fn convert_input(input: &str) -> Result<CharResult, GivError> {
    // First, try emoji lookup (requires colons).
    if input.starts_with(':') && input.ends_with(':') && input.len() > 2 {
        let emoji_shortcode = &input[1..input.len() - 1];
        if let Some(emoji) = emojis::get_by_shortcode(emoji_shortcode) {
            return Ok(CharResult {
                input: input.to_string(),
                output: emoji.as_str().to_string(),
                result_type: "emoji".to_string(),
                name: Some(emoji.name().to_string()),
            });
        }
    }

    // Next, try pattern lookup.
    if let Some((character, name)) = patterns::lookup_pattern(input) {
        return Ok(CharResult {
            input: input.to_string(),
            output: character.to_string(),
            result_type: "pattern".to_string(),
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

    /// Test emoji conversion.
    #[test]
    fn test_emoji_conversion() {
        let result = convert_input(":smile:").unwrap();
        assert_eq!(result.output, "😄");
        assert_eq!(result.result_type, "emoji");
    }

    /// Test emoji without colons should fail.
    #[test]
    fn test_emoji_requires_colons() {
        let result = convert_input("thumbsup");
        assert!(result.is_err());
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
