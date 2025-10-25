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

/// The number of words from the classic lorem ipsum to use as opening.
const CLASSIC_OPENING_WORDS: usize = 5;

/// Parse the classic lorem ipsum text into words.
///
/// # Returns
///
/// A vector of words from the classic lorem ipsum paragraph.
fn get_classic_words() -> Vec<&'static str> {
    lipsum::LOREM_IPSUM
        .split_whitespace()
        .map(|word| word.trim_end_matches(|c: char| !c.is_alphabetic()))
        .collect()
}

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

/// Generate lorem ipsum words, optionally starting with classic opening.
///
/// # Arguments
///
/// - `count` The number of words to generate.
/// - `start_with_classic` If true, starts with "Lorem ipsum dolor sit amet" before random words.
///
/// # Returns
///
/// A vector of lorem ipsum words.
fn generate_words(count: usize, start_with_classic: bool) -> Vec<String> {
    use rand::prelude::IndexedRandom;

    let mut words = Vec::with_capacity(count);

    // If we should start with classic opening and have room for it
    if start_with_classic && count > 0 {
        let classic_words = get_classic_words();
        let classic_count = CLASSIC_OPENING_WORDS.min(count);

        // Add classic opening words (lowercased except first)
        for (i, word) in classic_words.iter().take(classic_count).enumerate() {
            if i == 0 {
                // Keep "Lorem" capitalized
                words.push(word.to_string());
            } else {
                words.push(word.to_lowercase());
            }
        }

        // If we need more words, generate them randomly
        if count > classic_count {
            let remaining = count - classic_count;
            let source_text = lipsum::lipsum(SOURCE_WORD_COUNT);
            let source_words: Vec<&str> = source_text.split_whitespace().collect();
            let mut rng = rand::rng();

            for _ in 0..remaining {
                let word = source_words
                    .choose(&mut rng)
                    .unwrap_or(&"lorem")
                    .trim_end_matches(|c: char| !c.is_alphabetic())
                    .to_lowercase();
                words.push(word.to_string());
            }
        }
    } else {
        // Generate all random words
        let source_text = lipsum::lipsum(SOURCE_WORD_COUNT);
        let source_words: Vec<&str> = source_text.split_whitespace().collect();
        let mut rng = rand::rng();

        for _ in 0..count {
            let word = source_words
                .choose(&mut rng)
                .unwrap_or(&"lorem")
                .trim_end_matches(|c: char| !c.is_alphabetic())
                .to_lowercase();
            words.push(word.to_string());
        }
    }

    words
}

/// Generate some words as a sentence.
///
/// # Arguments
///
/// - `word_range` The range of words to generate (e.g., 8..16).
/// - `start_with_classic` If true, starts with classic lorem ipsum opening.
///
/// # Returns
///
/// A sentence with a random number of words from the range.
fn get_some_words(word_range: std::ops::Range<usize>, start_with_classic: bool) -> String {
    let mut rng = rand::rng();
    let word_count = rng.random_range(word_range);
    let words = generate_words(word_count, start_with_classic);

    if words.is_empty() {
        return String::from(".");
    }

    let mut sentence = if start_with_classic {
        // First word is already capitalized ("Lorem")
        words[0].clone()
    } else {
        capitalize_first(&words[0])
    };

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
/// - `start_with_classic` If true, starts first sentence with classic lorem ipsum opening.
///
/// # Returns
///
/// Multiple sentences joined with spaces.
fn get_some_sentences(sentence_range: std::ops::Range<usize>, start_with_classic: bool) -> String {
    let mut rng = rand::rng();
    let sentence_count = rng.random_range(sentence_range);

    (0..sentence_count)
        .enumerate()
        .map(|(i, _)| {
            // Only the first sentence starts with classic opening
            let use_classic = start_with_classic && i == 0;
            get_some_words(WORDS_PER_SENTENCE_RANGE, use_classic)
        })
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
        LoremUnit::Words => {
            // Generate words starting with classic opening
            let words = generate_words(count, true);
            let mut result = words.join(" ");
            result.push('.');
            result
        }
        LoremUnit::Sentences => {
            (0..count)
                .enumerate()
                .map(|(i, _)| {
                    // First sentence starts with classic opening
                    get_some_words(WORDS_PER_SENTENCE_RANGE, i == 0)
                })
                .collect::<Vec<_>>()
                .join(" ")
        }
        LoremUnit::Paragraphs => {
            (0..count)
                .enumerate()
                .map(|(i, _)| {
                    if i == 0 {
                        // First paragraph is the complete classic lorem ipsum
                        lipsum::LOREM_IPSUM.trim().to_string()
                    } else {
                        // Subsequent paragraphs are random
                        get_some_sentences(SENTENCES_PER_PARAGRAPH_RANGE, false)
                    }
                })
                .collect::<Vec<_>>()
                .join("\n\n")
        }
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
pub fn generate_lorem(count: Option<usize>, unit: LoremUnit) -> Result<LoremOutput, GivError> {
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

    /// Test that words mode starts with classic lorem ipsum opening.
    #[test]
    fn test_words_classic_opening() {
        // Test with exactly 5 words
        let text = generate_lorem_text(5, LoremUnit::Words).unwrap();
        assert_eq!(text, "Lorem ipsum dolor sit amet.");

        // Test with fewer than 5 words
        let text = generate_lorem_text(3, LoremUnit::Words).unwrap();
        assert_eq!(text, "Lorem ipsum dolor.");

        // Test with more than 5 words - should start with classic opening
        let text = generate_lorem_text(10, LoremUnit::Words).unwrap();
        assert!(text.starts_with("Lorem ipsum dolor sit amet"));
    }

    /// Test that sentences mode starts with classic lorem ipsum opening.
    #[test]
    fn test_sentences_classic_opening() {
        // First sentence should start with "Lorem ipsum dolor sit amet"
        let text = generate_lorem_text(1, LoremUnit::Sentences).unwrap();
        assert!(text.starts_with("Lorem ipsum dolor sit amet"));

        // Multiple sentences - first should have classic opening
        let text = generate_lorem_text(3, LoremUnit::Sentences).unwrap();
        assert!(text.starts_with("Lorem ipsum dolor sit amet"));
    }

    /// Test that paragraphs mode starts with full classic lorem ipsum.
    #[test]
    fn test_paragraphs_classic_opening() {
        // First paragraph should be the full classic lorem ipsum
        let text = generate_lorem_text(1, LoremUnit::Paragraphs).unwrap();
        let classic = lipsum::LOREM_IPSUM.trim();
        assert_eq!(text, classic);

        // Multiple paragraphs - first should be classic, followed by double newline
        let text = generate_lorem_text(2, LoremUnit::Paragraphs).unwrap();
        assert!(text.starts_with(classic));
        assert!(text.contains("\n\n"));
    }

    /// Test helper functions for classic lorem ipsum.
    #[test]
    fn test_get_classic_words() {
        let words = get_classic_words();
        assert!(!words.is_empty());
        assert_eq!(words[0], "Lorem");
        assert_eq!(words[1], "ipsum");
        assert_eq!(words[2], "dolor");
        assert_eq!(words[3], "sit");
        assert_eq!(words[4], "amet");
    }

    /// Test generate_words with and without classic opening.
    #[test]
    fn test_generate_words() {
        // With classic opening
        let words = generate_words(5, true);
        assert_eq!(words.len(), 5);
        assert_eq!(words[0], "Lorem");
        assert_eq!(words[1], "ipsum");
        assert_eq!(words[2], "dolor");
        assert_eq!(words[3], "sit");
        assert_eq!(words[4], "amet");

        // Without classic opening - should be all lowercase random
        let words = generate_words(5, false);
        assert_eq!(words.len(), 5);
        // Should not start with "Lorem"
        assert_ne!(words[0], "Lorem");
    }
}
