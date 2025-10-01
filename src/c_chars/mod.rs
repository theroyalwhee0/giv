/// Character pattern mappings.
mod patterns;
/// Output formatting for character/emoji conversion.
mod output;

use crate::{app::AppContext, error::GivError};
use output::{CharResult, CharsOutput};

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
fn convert_input(input: &str) -> Result<CharResult, GivError> {
    // First, try emoji lookup (strip colons if present).
    let emoji_shortcode = input.trim_matches(':');
    if let Some(emoji) = emojis::get_by_shortcode(emoji_shortcode) {
        return Ok(CharResult {
            input: input.to_string(),
            output: emoji.as_str().to_string(),
            result_type: "emoji".to_string(),
            name: Some(emoji.name().to_string()),
        });
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

/// The 'chars' command handler.
///
/// # Arguments
///
/// - `inputs` The list of patterns or shortcodes to convert.
/// - `ctx` The command context.
///
/// # Returns
///
/// A result indicating success or failure.
///
/// # Errors
///
/// Returns an error if any pattern or shortcode is not recognized.
pub fn chars_command(inputs: Vec<String>, ctx: &mut AppContext) -> Result<(), GivError> {
    // Convert all inputs, collecting into a Result.
    let results: Result<Vec<CharResult>, GivError> = inputs
        .iter()
        .map(|input| convert_input(input))
        .collect();

    // If any conversion failed, return the error.
    let results = results?;

    // Create output.
    let output = CharsOutput::new(results);

    // Output the results.
    ctx.output().output(&output);

    // Success.
    Ok(())
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

    /// Test emoji without colons.
    #[test]
    fn test_emoji_no_colons() {
        let result = convert_input("thumbsup").unwrap();
        assert_eq!(result.output, "👍");
        assert_eq!(result.result_type, "emoji");
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
