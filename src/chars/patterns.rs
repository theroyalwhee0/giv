/// Look up a special character by pattern.
///
/// This function uses the `penmanship` crate for Unicode character lookup,
/// which provides comprehensive support for punctuation, math symbols, Greek letters,
/// fractions, currency, and more. It includes helpful aliases like `"em"` for em-dash.
///
/// # Arguments
///
/// - `pattern` The pattern to look up.
///
/// # Returns
///
/// An optional tuple of (character, name) if the pattern is found.
pub fn lookup_pattern(pattern: &str) -> Option<(&'static str, &'static str)> {
    penmanship::lookup(pattern)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test fraction lookups.
    #[test]
    fn test_fractions() {
        assert!(lookup_pattern("1/4").is_some());
        assert!(lookup_pattern("1/2").is_some());
        assert!(lookup_pattern("3/4").is_some());
        let (char, name) = lookup_pattern("1/4").unwrap();
        assert_eq!(char, "\u{00BC}");
        assert_eq!(name, "fraction one quarter");
    }

    /// Test symbol lookups.
    #[test]
    fn test_symbols() {
        assert!(lookup_pattern("(c)").is_some());
        assert!(lookup_pattern("(C)").is_some());
        assert!(lookup_pattern("(r)").is_some());
        assert!(lookup_pattern("(tm)").is_some());
        assert!(lookup_pattern("(t)").is_some());
        let (char, name) = lookup_pattern("(c)").unwrap();
        assert_eq!(char, "\u{00A9}");
        assert_eq!(name, "copyright sign");
    }

    /// Test punctuation lookups.
    #[test]
    fn test_punctuation() {
        assert!(lookup_pattern("...").is_some());
        assert!(lookup_pattern("--").is_some());
        let (char, name) = lookup_pattern("...").unwrap();
        assert_eq!(char, "\u{2026}");
        assert_eq!(name, "horizontal ellipsis");
    }

    /// Test arrow lookups.
    #[test]
    fn test_arrows() {
        assert!(lookup_pattern("->").is_some());
        assert!(lookup_pattern("<-").is_some());
        assert!(lookup_pattern("=>").is_some());
        let (char, name) = lookup_pattern("->").unwrap();
        assert_eq!(char, "\u{2192}");
        assert_eq!(name, "rightwards arrow");
    }

    /// Test Greek letter lookups.
    #[test]
    fn test_greek_letters() {
        assert!(lookup_pattern("alpha").is_some());
        assert!(lookup_pattern("beta").is_some());
        assert!(lookup_pattern("lambda").is_some());
        assert!(lookup_pattern("lamda").is_some());
        assert!(lookup_pattern("Lambda").is_some());
        assert!(lookup_pattern("Omega").is_some());
        let (char, name) = lookup_pattern("lambda").unwrap();
        assert_eq!(char, "\u{03BB}");
        assert_eq!(name, "greek small letter lambda");
    }

    /// Test unknown pattern.
    #[test]
    fn test_unknown() {
        assert_eq!(lookup_pattern("unknown"), None);
    }
}
