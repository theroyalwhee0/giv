#[cfg(feature = "json")]
use serde::Serialize;

/// The result of a single random number generation.
#[cfg_attr(feature = "json", derive(Serialize))]
#[cfg_attr(feature = "json", serde(tag = "type"))]
#[derive(Debug, Clone)]
pub enum RngResult {
    /// Dice roll result
    #[cfg_attr(feature = "json", serde(rename = "dice"))]
    Dice {
        /// The dice notation used
        notation: String,
        /// The sum of all rolls plus modifier
        value: i64,
        /// The modifier applied (can be negative)
        modifier: i64,
        /// The individual roll results
        source: Vec<u64>,
    },
    /// Integer range result
    #[cfg_attr(feature = "json", serde(rename = "range_int"))]
    RangeInt {
        /// The range notation used
        notation: String,
        /// The generated value
        value: u64,
    },
    /// Float range result
    #[cfg_attr(feature = "json", serde(rename = "range_float"))]
    RangeFloat {
        /// The range notation used
        notation: String,
        /// The formatted value (rounded to precision)
        value: String,
        /// Number of decimal places
        precision: usize,
        /// The full-precision source values
        source: Vec<f64>,
    },
}
