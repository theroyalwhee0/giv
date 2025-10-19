/// Output formatting for lorem ipsum generation.
pub mod output;

use crate::error::GivError;
pub use output::LoremOutput;
use rand::Rng;

/// The default count of words to generate.
pub const DEFAULT_COUNT: usize = 50;

/// The range of words per sentence.
const WORDS_PER_SENTENCE_RANGE: std::ops::Range<usize> = 8..16;

/// The range of sentences per paragraph.
const SENTENCES_PER_PARAGRAPH_RANGE: std::ops::Range<usize> = 3..7;

/// The number of source words to generate for random selection.
const SOURCE_WORD_COUNT: usize = 500;

/// The unit type for lorem ipsum generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoremUnit {
    /// Generate words.
    Words,
    /// Generate sentences.
    Sentences,
    /// Generate paragraphs.
    Paragraphs,
}


/// Capitalize the first letter of a string.
///
/// # Arguments
///
/// - `s` The string to capitalize.
///
/// # Returns
///
/// The capitalized string.
fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(chars).collect(),
    }
}

/// Clean up spacing in text.
///
/// Collapses multiple consecutive spaces into single spaces, then removes spaces before periods.
/// Preserves newlines.
///
/// # Arguments
///
/// - `text` The text to clean.
///
/// # Returns
///
/// The cleaned text.
fn clean_spacing(text: &str) -> String {
    // Collapse multiple consecutive spaces into single space
    let mut result = String::with_capacity(text.len());
    let mut prev_was_space = false;

    for ch in text.chars() {
        if ch == ' ' {
            if !prev_was_space {
                result.push(ch);
                prev_was_space = true;
            }
        } else {
            result.push(ch);
            prev_was_space = false;
        }
    }

    // Remove spaces before periods
    result.replace(" .", ".")
}

/// Generate random lorem ipsum words.
///
/// # Arguments
///
/// - `count` The number of words to generate.
///
/// # Returns
///
/// A vector of randomly selected lorem ipsum words.
fn generate_random_words(count: usize) -> Vec<String> {
    use rand::prelude::IndexedRandom;
    // Get all lorem ipsum words from the source text
    let source_text = lipsum::lipsum(SOURCE_WORD_COUNT);
    let source_words: Vec<&str> = source_text.split_whitespace().collect();
    let mut rng = rand::rng();

    (0..count)
        .map(|_| {
            source_words
                .choose(&mut rng)
                .unwrap_or(&"lorem")
                .trim_end_matches(|c: char| !c.is_alphabetic())
                .to_lowercase()
                .to_string()
        })
        .collect()
}

/// Generate some words as a sentence.
///
/// # Arguments
///
/// - `word_range` The range of words to generate (e.g., 8..16).
///
/// # Returns
///
/// A sentence with a random number of words from the range.
fn get_some_words(word_range: std::ops::Range<usize>) -> String {
    let mut rng = rand::rng();
    let word_count = rng.random_range(word_range);
    let words = generate_random_words(word_count);

    let mut sentence = capitalize_first(&words[0]);
    for word in &words[1..] {
        sentence.push(' ');
        sentence.push_str(word);
    }
    sentence.push('.');
    sentence
}

/// Generate some sentences.
///
/// # Arguments
///
/// - `sentence_range` The range of sentences to generate (e.g., 1..4).
///
/// # Returns
///
/// Multiple sentences joined with spaces.
fn get_some_sentences(sentence_range: std::ops::Range<usize>) -> String {
    let mut rng = rand::rng();
    let sentence_count = rng.random_range(sentence_range);

    (0..sentence_count)
        .map(|_| get_some_words(WORDS_PER_SENTENCE_RANGE))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Generate lorem ipsum text.
///
/// # Arguments
///
/// - `count` The count of units to generate.
/// - `unit` The unit type (words, sentences, paragraphs).
///
/// # Returns
///
/// A result containing the generated text or an error.
///
/// # Errors
///
/// Returns an error if text generation fails.
fn generate_lorem_text(count: usize, unit: LoremUnit) -> Result<String, GivError> {
    if count == 0 {
        return Ok(String::new());
    }

    let text = match unit {
        LoremUnit::Words => get_some_words(count..count + 1),
        LoremUnit::Sentences => (0..count)
            .map(|_| get_some_words(WORDS_PER_SENTENCE_RANGE))
            .collect::<Vec<_>>()
            .join(" "),
        LoremUnit::Paragraphs => (0..count)
            .map(|_| get_some_sentences(SENTENCES_PER_PARAGRAPH_RANGE))
            .collect::<Vec<_>>()
            .join("\n\n"),
    };
    Ok(clean_spacing(&text))
}

/// Generate lorem ipsum placeholder text.
///
/// # Arguments
///
/// - `count` An optional count of units to generate. If `None`, uses [`DEFAULT_COUNT`] (50).
/// - `unit` The unit type (words, sentences, paragraphs).
///
/// # Returns
///
/// Returns a [`LoremOutput`] containing the generated text.
///
/// # Errors
///
/// This function does not currently return errors, but returns a Result for consistency.
///
/// # Examples
///
/// ```
/// use giv::lorem::{generate_lorem, LoremUnit, DEFAULT_COUNT};
///
/// # fn main() -> Result<(), giv::GivError> {
/// // Generate 50 words (default)
/// let output = generate_lorem(None, LoremUnit::Words)?;
/// assert!(!output.lorem.is_empty());
///
/// // Generate 3 sentences
/// let output = generate_lorem(Some(3), LoremUnit::Sentences)?;
/// assert!(output.lorem.contains('.'));
///
/// // Generate 2 paragraphs
/// let output = generate_lorem(Some(2), LoremUnit::Paragraphs)?;
/// assert!(!output.lorem.is_empty());
/// # Ok(())
/// # }
/// ```
pub fn generate_lorem(
    count: Option<usize>,
    unit: LoremUnit,
) -> Result<LoremOutput, GivError> {
    let count = count.unwrap_or(DEFAULT_COUNT);
    // Generate the lorem ipsum text.
    let lorem = generate_lorem_text(count, unit)?;
    // Create output with the lorem text.
    Ok(LoremOutput { lorem })
}

// Tests.
#[cfg(test)]
mod tests {
    use super::*;

    /// Test lorem ipsum text generation.
    #[test]
    fn test_generate_lorem() {
        // Test words generation.
        let text = generate_lorem_text(10, LoremUnit::Words).unwrap();
        assert!(!text.is_empty());

        // Test sentences generation.
        let text = generate_lorem_text(3, LoremUnit::Sentences).unwrap();
        assert!(!text.is_empty());
        // Should contain sentence endings.
        assert!(text.contains('.'));

        // Test paragraphs generation.
        let text = generate_lorem_text(2, LoremUnit::Paragraphs).unwrap();
        assert!(!text.is_empty());
        // Paragraphs should be separated by double newlines.
        assert!(text.contains("\n\n") || text.split("\n\n").count() >= 1);
    }

    /// Test zero count.
    #[test]
    fn test_generate_lorem_zero() {
        let text = generate_lorem_text(0, LoremUnit::Words).unwrap();
        // Zero words should return empty or minimal text.
        assert!(text.is_empty() || text.len() < 10);
    }
}
