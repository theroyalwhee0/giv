use rand::RngCore;

/// Generate a random integer in the range [min, max] (inclusive).
///
/// Uses rejection sampling to avoid modulo bias.
///
/// # Arguments
///
/// - `rng` The random number generator to use.
/// - `min` The minimum value (inclusive).
/// - `max` The maximum value (inclusive).
///
/// # Returns
///
/// A random integer in the specified range.
pub fn gen_range_int<R: RngCore>(rng: &mut R, min: u64, max: u64) -> u64 {
    debug_assert!(min <= max, "min must be less than or equal to max");
    let range = max - min + 1;

    // Calculate the maximum value we can accept to avoid modulo bias
    let max_acceptable = u64::MAX - (u64::MAX % range);

    // Rejection sampling: keep generating until we get a value in range
    loop {
        let random_u64 = rng.next_u64();
        if random_u64 < max_acceptable {
            return min + (random_u64 % range);
        }
        // If random_u64 >= max_acceptable, reject and try again
    }
}

/// Generate a random float in the range [min, max].
///
/// # Arguments
///
/// - `rng` The random number generator to use.
/// - `min` The minimum value (inclusive).
/// - `max` The maximum value (inclusive).
///
/// # Returns
///
/// A random float in the specified range.
pub fn gen_range_float<R: RngCore>(rng: &mut R, min: f64, max: f64) -> f64 {
    debug_assert!(min <= max, "min must be less than or equal to max");
    let random_u64 = rng.next_u64();
    // Convert u64 to f64 in range [0.0, 1.0)
    let normalized = (random_u64 as f64) / (u64::MAX as f64);
    // Scale to desired range
    min + normalized * (max - min)
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// Test integer range generation
    #[test]
    fn test_gen_range_int() {
        let mut rng = rand::rng();
        // Test that generated values are in range
        for _ in 0..100 {
            let value = gen_range_int(&mut rng, 1, 10);
            assert!(value >= 1 && value <= 10);
        }

        // Test single value range
        let value = gen_range_int(&mut rng, 5, 5);
        assert_eq!(value, 5);
    }

    /// Test float range generation
    #[test]
    fn test_gen_range_float() {
        let mut rng = rand::rng();
        // Test that generated values are in range
        for _ in 0..100 {
            let value = gen_range_float(&mut rng, 0.0, 1.0);
            assert!(value >= 0.0 && value <= 1.0);
        }

        for _ in 0..100 {
            let value = gen_range_float(&mut rng, 10.5, 20.5);
            assert!(value >= 10.5 && value <= 20.5);
        }
    }
}
