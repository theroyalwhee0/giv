use crate::error::GivError;
use rand::RngCore;

use super::generator::{gen_range_float, gen_range_int};
use super::result::RngResult;
use super::spec::RngSpec;

/// Execute a single RNG specification.
///
/// # Arguments
///
/// - `rng` The random number generator to use.
/// - `spec` The specification to execute.
///
/// # Returns
///
/// A result containing the `RngResult` or an error.
pub fn execute_spec<R: RngCore>(rng: &mut R, spec: &RngSpec) -> Result<RngResult, GivError> {
    match spec {
        RngSpec::Dice { count, sides } => {
            let rolls: Vec<u64> = (0..*count).map(|_| gen_range_int(rng, 1, *sides)).collect();
            let value: u64 = rolls.iter().sum();
            let notation = if *count == 1 {
                format!("d{sides}")
            } else {
                format!("{count}d{sides}")
            };
            Ok(RngResult::Dice {
                notation,
                rolls,
                value,
            })
        }
        RngSpec::RangeInt { start, end } => {
            let value = gen_range_int(rng, *start, *end);
            let notation = format!("{start}..{end}");
            Ok(RngResult::RangeInt { notation, value })
        }
        RngSpec::RangeFloat {
            start,
            end,
            precision,
        } => {
            let value = gen_range_float(rng, *start, *end);
            // Format notation with the precision to preserve decimal places
            let notation = format!("{start:.precision$}..{end:.precision$}");
            Ok(RngResult::RangeFloat {
                notation,
                value,
                precision: *precision,
            })
        }
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// Test spec execution
    #[test]
    fn test_execute_spec() {
        let mut rng = rand::rng();

        // Test dice execution
        let spec = RngSpec::Dice { count: 3, sides: 6 };
        let result = execute_spec(&mut rng, &spec).unwrap();
        match result {
            RngResult::Dice {
                notation,
                rolls,
                value,
            } => {
                assert_eq!(notation, "3d6");
                assert_eq!(rolls.len(), 3);
                let expected_sum: u64 = rolls.iter().sum();
                assert_eq!(value, expected_sum);
                for roll in rolls {
                    assert!(roll >= 1 && roll <= 6);
                }
                assert!(value >= 3 && value <= 18);
            }
            _ => panic!("Expected Dice result"),
        }

        // Test integer range execution
        let spec = RngSpec::RangeInt { start: 1, end: 100 };
        let result = execute_spec(&mut rng, &spec).unwrap();
        match result {
            RngResult::RangeInt { notation, value } => {
                assert_eq!(notation, "1..100");
                assert!(value >= 1 && value <= 100);
            }
            _ => panic!("Expected RangeInt result"),
        }

        // Test float range execution
        let spec = RngSpec::RangeFloat {
            start: 0.0,
            end: 1.0,
            precision: 3,
        };
        let result = execute_spec(&mut rng, &spec).unwrap();
        match result {
            RngResult::RangeFloat {
                notation,
                value,
                precision,
            } => {
                assert_eq!(notation, "0.000..1.000");
                assert_eq!(precision, 3);
                assert!(value >= 0.0 && value <= 1.0);
            }
            _ => panic!("Expected RangeFloat result"),
        }
    }
}
