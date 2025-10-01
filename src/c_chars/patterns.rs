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
        "1/4" => Some(("\u{00BC}", "fraction one quarter")),
        "1/2" => Some(("\u{00BD}", "fraction one half")),
        "3/4" => Some(("\u{00BE}", "fraction three quarters")),
        "1/3" => Some(("\u{2153}", "fraction one third")),
        "2/3" => Some(("\u{2154}", "fraction two thirds")),
        "1/8" => Some(("\u{215B}", "fraction one eighth")),
        "3/8" => Some(("\u{215C}", "fraction three eighths")),
        "5/8" => Some(("\u{215D}", "fraction five eighths")),
        "7/8" => Some(("\u{215E}", "fraction seven eighths")),

        // Symbols
        "(c)" | "(C)" => Some(("\u{00A9}", "copyright sign")),
        "(r)" | "(R)" => Some(("\u{00AE}", "registered sign")),
        "(tm)" | "(TM)" | "(t)" | "(T)" => Some(("\u{2122}", "trade mark sign")),
        "(p)" | "(P)" => Some(("\u{2117}", "sound recording copyright")),

        // Punctuation
        "..." => Some(("\u{2026}", "horizontal ellipsis")),
        "--" => Some(("\u{2014}", "em dash")),

        // Arrows
        "->" => Some(("\u{2192}", "rightwards arrow")),
        "<-" => Some(("\u{2190}", "leftwards arrow")),
        "=>" => Some(("\u{21D2}", "rightwards double arrow")),
        "<=" => Some(("\u{21D0}", "leftwards double arrow")),
        "<->" => Some(("\u{2194}", "left right arrow")),
        "<=>" => Some(("\u{21D4}", "left right double arrow")),
        "^^" | "up" => Some(("\u{2191}", "upwards arrow")),
        "vv" | "down" => Some(("\u{2193}", "downwards arrow")),

        // Currency
        "cent" => Some(("\u{00A2}", "cent sign")),
        "pound" => Some(("\u{00A3}", "pound sign")),
        "euro" => Some(("\u{20AC}", "euro sign")),
        "yen" => Some(("\u{00A5}", "yen sign")),
        "rupee" => Some(("\u{20B9}", "rupee sign")),
        "won" => Some(("\u{20A9}", "won sign")),
        "bitcoin" | "btc" => Some(("\u{20BF}", "bitcoin sign")),

        // Math
        "deg" | "degree" => Some(("\u{00B0}", "degree sign")),
        "+-" => Some(("\u{00B1}", "plus-minus sign")),
        "*" | "x" => Some(("\u{00D7}", "multiplication sign")),
        "div" | "divide" => Some(("\u{00F7}", "division sign")),
        "ne" | "!=" => Some(("\u{2260}", "not equal to")),
        "lte" => Some(("\u{2264}", "less-than or equal to")),
        "gte" => Some(("\u{2265}", "greater-than or equal to")),
        "~=" => Some(("\u{2248}", "almost equal to")),
        "inf" | "infinity" => Some(("\u{221E}", "infinity")),
        "sqrt" => Some(("\u{221A}", "square root")),
        "sum" => Some(("\u{2211}", "n-ary summation")),
        "prod" | "product" => Some(("\u{220F}", "n-ary product")),
        "int" => Some(("\u{222B}", "integral")),
        "partial" => Some(("\u{2202}", "partial differential")),
        "nabla" => Some(("\u{2207}", "nabla")),
        "in" => Some(("\u{2208}", "element of")),
        "notin" => Some(("\u{2209}", "not an element of")),
        "subset" => Some(("\u{2282}", "subset of")),
        "superset" => Some(("\u{2283}", "superset of")),
        "union" => Some(("\u{222A}", "union")),
        "intersect" => Some(("\u{2229}", "intersection")),
        "forall" => Some(("\u{2200}", "for all")),
        "exists" => Some(("\u{2203}", "there exists")),
        "emptyset" => Some(("\u{2205}", "empty set")),
        "propto" => Some(("\u{221D}", "proportional to")),

        // Greek letters (lowercase)
        "alpha" => Some(("\u{03B1}", "greek small letter alpha")),
        "beta" => Some(("\u{03B2}", "greek small letter beta")),
        "gamma" => Some(("\u{03B3}", "greek small letter gamma")),
        "delta" => Some(("\u{03B4}", "greek small letter delta")),
        "epsilon" => Some(("\u{03B5}", "greek small letter epsilon")),
        "zeta" => Some(("\u{03B6}", "greek small letter zeta")),
        "eta" => Some(("\u{03B7}", "greek small letter eta")),
        "theta" => Some(("\u{03B8}", "greek small letter theta")),
        "iota" => Some(("\u{03B9}", "greek small letter iota")),
        "kappa" => Some(("\u{03BA}", "greek small letter kappa")),
        "lambda" | "lamda" => Some(("\u{03BB}", "greek small letter lambda")),
        "mu" => Some(("\u{03BC}", "greek small letter mu")),
        "nu" => Some(("\u{03BD}", "greek small letter nu")),
        "xi" => Some(("\u{03BE}", "greek small letter xi")),
        "omicron" => Some(("\u{03BF}", "greek small letter omicron")),
        "pi" => Some(("\u{03C0}", "greek small letter pi")),
        "rho" => Some(("\u{03C1}", "greek small letter rho")),
        "sigma" => Some(("\u{03C3}", "greek small letter sigma")),
        "tau" => Some(("\u{03C4}", "greek small letter tau")),
        "upsilon" => Some(("\u{03C5}", "greek small letter upsilon")),
        "phi" => Some(("\u{03C6}", "greek small letter phi")),
        "chi" => Some(("\u{03C7}", "greek small letter chi")),
        "psi" => Some(("\u{03C8}", "greek small letter psi")),
        "omega" => Some(("\u{03C9}", "greek small letter omega")),

        // Greek letters (uppercase - commonly used)
        "Alpha" => Some(("\u{0391}", "greek capital letter alpha")),
        "Beta" => Some(("\u{0392}", "greek capital letter beta")),
        "Gamma" => Some(("\u{0393}", "greek capital letter gamma")),
        "Delta" => Some(("\u{0394}", "greek capital letter delta")),
        "Theta" => Some(("\u{0398}", "greek capital letter theta")),
        "Lambda" | "Lamda" => Some(("\u{039B}", "greek capital letter lambda")),
        "Pi" => Some(("\u{03A0}", "greek capital letter pi")),
        "Sigma" => Some(("\u{03A3}", "greek capital letter sigma")),
        "Phi" => Some(("\u{03A6}", "greek capital letter phi")),
        "Psi" => Some(("\u{03A8}", "greek capital letter psi")),
        "Omega" => Some(("\u{03A9}", "greek capital letter omega")),

        // Punctuation and symbols
        "section" | "sect" => Some(("\u{00A7}", "section sign")),
        "para" | "paragraph" => Some(("\u{00B6}", "pilcrow sign")),
        "dag" | "dagger" => Some(("\u{2020}", "dagger")),
        "ddag" => Some(("\u{2021}", "double dagger")),
        "bullet" => Some(("\u{2022}", "bullet")),
        "middot" => Some(("\u{00B7}", "middle dot")),

        // Superscripts
        "^0" => Some(("\u{2070}", "superscript zero")),
        "^1" => Some(("\u{00B9}", "superscript one")),
        "^2" => Some(("\u{00B2}", "superscript two")),
        "^3" => Some(("\u{00B3}", "superscript three")),
        "^4" => Some(("\u{2074}", "superscript four")),
        "^5" => Some(("\u{2075}", "superscript five")),
        "^6" => Some(("\u{2076}", "superscript six")),
        "^7" => Some(("\u{2077}", "superscript seven")),
        "^8" => Some(("\u{2078}", "superscript eight")),
        "^9" => Some(("\u{2079}", "superscript nine")),
        "^n" => Some(("\u{207F}", "superscript latin small letter n")),

        // Subscripts
        "_0" => Some(("\u{2080}", "subscript zero")),
        "_1" => Some(("\u{2081}", "subscript one")),
        "_2" => Some(("\u{2082}", "subscript two")),
        "_3" => Some(("\u{2083}", "subscript three")),
        "_4" => Some(("\u{2084}", "subscript four")),
        "_5" => Some(("\u{2085}", "subscript five")),
        "_6" => Some(("\u{2086}", "subscript six")),
        "_7" => Some(("\u{2087}", "subscript seven")),
        "_8" => Some(("\u{2088}", "subscript eight")),
        "_9" => Some(("\u{2089}", "subscript nine")),

        // Miscellaneous
        "star" => Some(("\u{2605}", "black star")),

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
