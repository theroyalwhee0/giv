use serde::Serialize;

/// The result of a single random number generation.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum RngResult {
    /// Dice roll result
    #[serde(rename = "dice")]
    Dice {
        /// The dice notation used
        notation: String,
        /// The individual roll results
        rolls: Vec<u64>,
        /// The sum of all rolls
        value: u64,
    },
    /// Integer range result
    #[serde(rename = "range_int")]
    RangeInt {
        /// The range notation used
        notation: String,
        /// The generated value
        value: u64,
    },
    /// Float range result
    #[serde(rename = "range_float")]
    RangeFloat {
        /// The range notation used
        notation: String,
        /// The generated value
        value: f64,
        /// Number of decimal places
        precision: usize,
    },
}
