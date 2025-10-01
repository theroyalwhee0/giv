/// Look up a special character by pattern.
///
/// # Arguments
///
/// - `pattern` The pattern to look up.
///
/// # Returns
///
/// An optional tuple of (character, name) if the pattern is found.
pub fn lookup_pattern(pattern: &str) -> Option<(&'static str, &'static str)> {
    // Match against known patterns.
    match pattern {
        // Fractions
        "1/4" => Some(("\u{00BC}", "vulgar fraction one quarter")),
        "1/2" => Some(("\u{00BD}", "vulgar fraction one half")),
        "3/4" => Some(("\u{00BE}", "vulgar fraction three quarters")),
        "1/3" => Some(("\u{2153}", "vulgar fraction one third")),
        "2/3" => Some(("\u{2154}", "vulgar fraction two thirds")),
        "1/8" => Some(("\u{215B}", "vulgar fraction one eighth")),
        "3/8" => Some(("\u{215C}", "vulgar fraction three eighths")),
        "5/8" => Some(("\u{215D}", "vulgar fraction five eighths")),
        "7/8" => Some(("\u{215E}", "vulgar fraction seven eighths")),

        // Symbols
        "(c)" | "(C)" => Some(("\u{00A9}", "copyright sign")),
        "(r)" | "(R)" => Some(("\u{00AE}", "registered sign")),
        "(tm)" | "(TM)" | "(t)" | "(T)" => Some(("\u{2122}", "trade mark sign")),
        "(p)" | "(P)" => Some(("\u{2117}", "sound recording copyright")),

        // Punctuation
        "..." => Some(("\u{2026}", "horizontal ellipsis")),
        "--" => Some(("\u{2014}", "em dash")),
        ":)" => Some(("\u{263A}", "white smiling face")),
        ":(" => Some(("\u{2639}", "white frowning face")),

        // Arrows
        "->" => Some(("\u{2192}", "rightwards arrow")),
        "<-" => Some(("\u{2190}", "leftwards arrow")),
        "=>" => Some(("\u{21D2}", "rightwards double arrow")),
        "<=" => Some(("\u{21D0}", "leftwards double arrow")),
        "<->" => Some(("\u{2194}", "left right arrow")),
        "<=>" => Some(("\u{21D4}", "left right double arrow")),

        // Currency
        "cent" => Some(("\u{00A2}", "cent sign")),
        "pound" => Some(("\u{00A3}", "pound sign")),
        "euro" => Some(("\u{20AC}", "euro sign")),
        "yen" => Some(("\u{00A5}", "yen sign")),

        // Math
        "degree" => Some(("\u{00B0}", "degree sign")),
        "+-" => Some(("\u{00B1}", "plus-minus sign")),
        "x" => Some(("\u{00D7}", "multiplication sign")),
        "divide" => Some(("\u{00F7}", "division sign")),
        "!=" => Some(("\u{2260}", "not equal to")),
        "lte" => Some(("\u{2264}", "less-than or equal to")),
        "gte" => Some(("\u{2265}", "greater-than or equal to")),
        "~=" => Some(("\u{2248}", "almost equal to")),
        "infinity" => Some(("\u{221E}", "infinity")),

        // Quotes
        "''" => Some(("\u{2019}", "right single quotation mark")),
        "\"\"" => Some(("\u{201D}", "right double quotation mark")),

        // Other
        _ => None,
    }
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
        assert_eq!(name, "vulgar fraction one quarter");
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
        assert!(lookup_pattern(":)").is_some());
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

    /// Test unknown pattern.
    #[test]
    fn test_unknown() {
        assert_eq!(lookup_pattern("unknown"), None);
    }
}
