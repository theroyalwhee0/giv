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
fn convert_input(input: &str) -> CharResult {
    // First, try emoji lookup (strip colons if present).
    let emoji_shortcode = input.trim_matches(':');
    if let Some(emoji) = emojis::get_by_shortcode(emoji_shortcode) {
        return CharResult {
            input: input.to_string(),
            output: emoji.as_str().to_string(),
            result_type: "emoji".to_string(),
            name: Some(emoji.name().to_string()),
        };
    }

    // Next, try pattern lookup.
    if let Some((character, name)) = patterns::lookup_pattern(input) {
        return CharResult {
            input: input.to_string(),
            output: character.to_string(),
            result_type: "pattern".to_string(),
            name: Some(name.to_string()),
        };
    }

    // If no match, return the input as passthrough.
    CharResult {
        input: input.to_string(),
        output: input.to_string(),
        result_type: "passthrough".to_string(),
        name: None,
    }
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
pub fn chars_command(inputs: Vec<String>, ctx: &mut AppContext) -> Result<(), GivError> {
    // Convert all inputs.
    let results: Vec<CharResult> = inputs.iter().map(|input| convert_input(input)).collect();

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
        let result = convert_input(":smile:");
        assert_eq!(result.output, "😄");
        assert_eq!(result.result_type, "emoji");
    }

    /// Test emoji without colons.
    #[test]
    fn test_emoji_no_colons() {
        let result = convert_input("thumbsup");
        assert_eq!(result.output, "👍");
        assert_eq!(result.result_type, "emoji");
    }

    /// Test pattern conversion.
    #[test]
    fn test_pattern_conversion() {
        let result = convert_input("1/4");
        assert_eq!(result.output, "¼");
        assert_eq!(result.result_type, "pattern");
    }

    /// Test copyright symbol.
    #[test]
    fn test_copyright() {
        let result = convert_input("(c)");
        assert_eq!(result.output, "©");
        assert_eq!(result.result_type, "pattern");
    }

    /// Test passthrough.
    #[test]
    fn test_passthrough() {
        let result = convert_input("unknown");
        assert_eq!(result.output, "unknown");
        assert_eq!(result.result_type, "passthrough");
    }
}
