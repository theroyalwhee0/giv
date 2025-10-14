//! Build information module.
//!
//! Provides access to build-time information captured by build.rs,
//! including build timestamp and profile.

/// Get the build timestamp in seconds.
///
/// This is set by build.rs during compilation from SOURCE_DATE_EPOCH.
///
/// # Returns
///
/// The build timestamp in seconds since UNIX epoch.
///
/// # Panics
///
/// Panics if SOURCE_DATE_EPOCH is not set at compile time or cannot be parsed as a u64.
#[must_use]
pub fn timestamp() -> u64 {
    env!("SOURCE_DATE_EPOCH").parse::<u64>().unwrap()
}

/// Get the build profile.
///
/// Returns the profile used to build the binary (either "debug" or "release").
///
/// # Returns
///
/// The build profile as a string.
#[must_use]
pub fn profile() -> &'static str {
    env!("EXPECT_PROFILE")
}

/// Get the build version.
///
/// # Returns
///
/// The build version from Cargo.toml.
#[must_use]
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Get the build date in ISO 8601 format (YYYY-MM-DD).
///
/// # Returns
///
/// The build date as an ISO 8601 formatted string.
#[must_use]
pub fn date_iso() -> &'static str {
    env!("BUILD_DATE_ISO")
}

/// Get the build datetime in ISO 8601 format (YYYY-MM-DDTHH:MM:SSZ).
///
/// # Returns
///
/// The build datetime as an ISO 8601 formatted string in UTC.
#[must_use]
pub fn datetime_iso() -> &'static str {
    env!("BUILD_DATETIME_ISO")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that timestamp returns a valid Unix timestamp.
    ///
    /// # Panics
    ///
    /// Panics if SOURCE_DATE_EPOCH is not set at compile time.
    #[test]
    fn test_timestamp() {
        let ts = timestamp();
        // Should be a reasonable timestamp (after 2020-01-01 and before 2100-01-01)
        assert!(ts > 1577836800, "Timestamp should be after 2020-01-01");
        assert!(ts < 4102444800, "Timestamp should be before 2100-01-01");
    }

    /// Test that profile returns a valid profile string.
    ///
    /// # Panics
    ///
    /// Panics if EXPECT_PROFILE is not set at compile time.
    #[test]
    fn test_profile() {
        let profile = profile();
        assert!(
            profile == "debug" || profile == "release",
            "Profile should be either 'debug' or 'release', got: {profile}"
        );
    }

    /// Test that version returns a non-empty string.
    ///
    /// # Panics
    ///
    /// Panics if CARGO_PKG_VERSION is not set at compile time.
    #[test]
    fn test_version() {
        let ver = version();
        assert!(!ver.is_empty(), "Version should not be empty");
        // Should follow semver pattern (basic check)
        assert!(
            ver.contains('.'),
            "Version should contain at least one dot: {ver}"
        );
    }

    /// Test that timestamp is consistent across multiple calls.
    ///
    /// # Panics
    ///
    /// Panics if SOURCE_DATE_EPOCH is not set at compile time.
    #[test]
    fn test_timestamp_consistency() {
        let ts1 = timestamp();
        let ts2 = timestamp();
        assert_eq!(ts1, ts2, "Timestamp should be consistent across calls");
    }

    /// Test that profile is consistent across multiple calls.
    ///
    /// # Panics
    ///
    /// # Panics if EXPECT_PROFILE is not set at compile time.
    #[test]
    fn test_profile_consistency() {
        let p1 = profile();
        let p2 = profile();
        assert_eq!(p1, p2, "Profile should be consistent across calls");
    }

    /// Test that date_iso returns a valid ISO 8601 date.
    ///
    /// # Panics
    ///
    /// Panics if BUILD_DATE_ISO is not set at compile time.
    #[test]
    fn test_date_iso() {
        let date = date_iso();
        // Should match YYYY-MM-DD format (basic check)
        assert_eq!(date.len(), 10, "ISO date should be 10 characters");
        assert!(date.contains('-'), "ISO date should contain dashes");
        // Verify it starts with a reasonable year
        let year: u32 = date[0..4].parse().expect("Year should be numeric");
        assert!((2020..=2100).contains(&year), "Year should be reasonable");
    }

    /// Test that datetime_iso returns a valid ISO 8601 datetime.
    ///
    /// # Panics
    ///
    /// Panics if BUILD_DATETIME_ISO is not set at compile time.
    #[test]
    fn test_datetime_iso() {
        let datetime = datetime_iso();
        // Should match YYYY-MM-DDTHH:MM:SSZ format (basic check)
        assert!(datetime.len() >= 19, "ISO datetime should be at least 19 characters");
        assert!(datetime.contains('T'), "ISO datetime should contain 'T'");
        assert!(datetime.ends_with('Z'), "ISO datetime should end with 'Z'");
    }
}
