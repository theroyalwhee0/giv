#![cfg(feature = "key")]

use crate::{cli::CommandOptions, error::GivError, output::outputln};
use rand::RngCore;

// The default size of the key.
const DEFAULT_KEY_SIZE: usize = 36;

// The alphabet used for generating the key.
const KEY_ALPHABET: &str = concat!(
    "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
    "abcdefghijklmnopqrstuvwxyz",
    "0123456789",
);

// The maximum value for the RNG byte to prevent bias.
const U8_MAX: usize = u8::MAX as usize;
const RNG_RANGE: usize = U8_MAX - (U8_MAX % KEY_ALPHABET.len());

// The prefix for the generated key.
const KEY_PREFIX: &str = "key_";

/// Generate a random prefixless key of the specified size.
///
/// # Arguments
///
/// - `size` The size of the key to generate.
///
/// # Returns
///
/// A result containing the generated key or an error.
fn get_key(size: usize) -> Result<String, GivError> {
    debug_assert!(
        KEY_ALPHABET.len() <= U8_MAX,
        "The key alphabet is too large for the RNG byte range."
    );
    // Get the random number generator.
    let mut rng = rand::rng();
    // Allocate the result string with the specified size.
    let mut result = String::with_capacity(size);
    // Keep generating until we have enough characters...
    while result.len() < size {
        let rand_u64 = rng.next_u64();
        // For each byte in the u64...
        for idx in 0..u64::BITS / u8::BITS {
            // Get the byte.
            let byte = ((rand_u64 >> (idx * u8::BITS)) & (u8::MAX as u64)) as u8;
            let byte = byte as usize;
            // Check if the byte is in range.
            if byte >= RNG_RANGE {
                // Skip bytes that are out of range to prevent bias.
                continue;
            }
            // Map the byte to an index in the KEY_ALPHABET.
            let idx = byte % KEY_ALPHABET.len();
            let char = KEY_ALPHABET.chars().nth(idx).unwrap();
            result.push(char);
            // If done, break out of the loop.
            if result.len() >= size {
                break;
            }
        }
    }
    debug_assert!(
        result.len() == size,
        "Generated key length does not match requested size."
    );
    Ok(result)
}

/// The 'key' command handler.
///
/// # Arguments
///
/// - `size` An optional size for the key.
///
/// # Returns
///
/// A result indicating success or failure.
pub fn key_command(size: Option<usize>, options: CommandOptions) -> Result<(), GivError> {
    let size = size.unwrap_or(DEFAULT_KEY_SIZE);
    // Generate the key with the specified size.
    let key = get_key(size)?;
    // Print the generated key with the prefix.
    outputln(options, format!("{}{}", KEY_PREFIX, key));
    // Success.
    Ok(())
}

// Tests.
#[cfg(test)]
mod tests {
    use super::*;

    /// Test key generation at various sizes.
    #[test]
    fn test_get_key() {
        // The sizes to test.
        let sizes = [0, 1, 10, 36, 100, 1000];
        // For each size...
        for size in sizes {
            // Generate the key.
            let key = get_key(size).unwrap();
            assert_eq!(key.len(), size);
            // Verify that chars are in alphabet.
            assert!(key.chars().all(|ch| KEY_ALPHABET.contains(ch)));
        }
    }
}
