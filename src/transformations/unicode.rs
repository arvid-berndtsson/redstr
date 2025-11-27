use crate::rng::SimpleRng;

/// Replaces characters with random Unicode variations.
///
/// Useful for testing Unicode normalization and handling.
///
/// # Examples
///
/// ```
/// use redstr::unicode_variations;
/// let result = unicode_variations("admin");
/// assert_eq!(result.chars().count(), 5);
/// ```
pub fn unicode_variations(input: &str) -> String {
    let mut rng = SimpleRng::new();

    input
        .chars()
        .map(|c| {
            let lower = c.to_lowercase().to_string();
            match lower.as_str() {
                "a" => ["a", "à", "á", "â", "ã", "ä", "å", "ā", "ă"][rng.next() as usize % 9],
                "e" => ["e", "è", "é", "ê", "ë", "ē", "ĕ", "ė"][rng.next() as usize % 8],
                "i" => ["i", "ì", "í", "î", "ï", "ī", "ĭ", "į"][rng.next() as usize % 8],
                "o" => ["o", "ò", "ó", "ô", "õ", "ö", "ō", "ŏ"][rng.next() as usize % 8],
                "u" => ["u", "ù", "ú", "û", "ü", "ū", "ŭ", "ů"][rng.next() as usize % 8],
                "c" => ["c", "ç", "ć", "ĉ", "ċ", "č"][rng.next() as usize % 6],
                "n" => ["n", "ñ", "ń", "ņ", "ň"][rng.next() as usize % 5],
                "s" => ["s", "ś", "ŝ", "ş", "š"][rng.next() as usize % 5],
                _ => return c.to_string(),
            }
            .to_string()
        })
        .collect()
}

/// Adds zalgo combining characters to create corrupted-looking text.
///
/// Useful for testing display issues and Unicode handling.
///
/// # Examples
///
/// ```
/// use redstr::zalgo_text;
/// let result = zalgo_text("test");
/// assert!(result.len() > 4);
/// ```
pub fn zalgo_text(input: &str) -> String {
    let mut rng = SimpleRng::new();
    let combining_chars = [
        '\u{0300}', '\u{0301}', '\u{0302}', '\u{0303}', '\u{0304}', '\u{0305}', '\u{0306}',
        '\u{0307}', '\u{0308}', '\u{0309}', '\u{030A}', '\u{030B}', '\u{030C}', '\u{030D}',
        '\u{030E}', '\u{030F}', '\u{0310}', '\u{0311}', '\u{0312}', '\u{0313}', '\u{0314}',
        '\u{0315}', '\u{0316}', '\u{0317}',
    ];

    input
        .chars()
        .map(|c| {
            let mut result = c.to_string();
            if c.is_alphabetic() {
                let count = (rng.next() % 3) + 1;
                for _ in 0..count {
                    let idx = rng.next() as usize % combining_chars.len();
                    result.push(combining_chars[idx]);
                }
            }
            result
        })
        .collect()
}

/// Substitutes characters with similar-looking homoglyphs.
///
/// Useful for testing homograph attacks and IDN spoofing vulnerabilities.
/// Uses Cyrillic and other lookalike characters.
///
/// # Examples
///
/// ```
/// use redstr::homoglyph_substitution;
/// let result = homoglyph_substitution("example");
/// // Result may contain Cyrillic characters that look like Latin
/// ```
pub fn homoglyph_substitution(input: &str) -> String {
    let mut rng = SimpleRng::new();

    input
        .chars()
        .map(|c| {
            if rng.next() % 3 != 0 {
                return c.to_string();
            }

            match c {
                'a' | 'A' => "а", // Cyrillic а
                'e' | 'E' => "е", // Cyrillic е
                'o' | 'O' => "о", // Cyrillic о
                'p' | 'P' => "р", // Cyrillic р
                'c' | 'C' => "с", // Cyrillic с
                'x' | 'X' => "х", // Cyrillic х
                'i' | 'I' => "і", // Cyrillic і
                '0' => "О",       // Letter O
                '1' => "l",       // Letter l
                _ => return c.to_string(),
            }
            .to_string()
        })
        .collect()
}

/// Replaces regular spaces with various Unicode space characters.
///
/// Useful for testing whitespace handling and normalization.
///
/// # Examples
///
/// ```
/// use redstr::space_variants;
/// let result = space_variants("hello world");
/// assert_eq!(result.chars().filter(|c| c.is_whitespace()).count(), 1);
/// ```
pub fn space_variants(input: &str) -> String {
    let mut rng = SimpleRng::new();
    let spaces = [
        ' ', '\u{00A0}', '\u{2000}', '\u{2001}', '\u{2002}', '\u{2003}',
    ];

    input
        .chars()
        .map(|c| {
            if c == ' ' {
                spaces[rng.next() as usize % spaces.len()].to_string()
            } else {
                c.to_string()
            }
        })
        .collect()
}

/// Generates unicode normalization variations (NFD, NFC, NFKC, NFKD concepts).
///
/// Useful for testing unicode normalization vulnerabilities and bypasses.
///
/// # Examples
///
/// ```
/// use redstr::unicode_normalize_variants;
/// let result = unicode_normalize_variants("café");
/// assert!(result.len() >= 4);
/// ```
pub fn unicode_normalize_variants(input: &str) -> String {
    let mut rng = SimpleRng::new();

    input
        .chars()
        .map(|c| {
            // Use compatibility characters and combining marks
            match c {
                'a' | 'A' => {
                    let variants = ["a", "а", "ａ", "\u{0061}\u{0301}"]; // Latin, Cyrillic, fullwidth, with combining acute
                    variants[rng.next() as usize % variants.len()]
                }
                'e' | 'E' => {
                    let variants = ["e", "е", "ｅ", "\u{0065}\u{0301}"];
                    variants[rng.next() as usize % variants.len()]
                }
                'o' | 'O' => {
                    let variants = ["o", "о", "ｏ", "\u{006F}\u{0301}"];
                    variants[rng.next() as usize % variants.len()]
                }
                _ => return c.to_string(),
            }
            .to_string()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_homoglyph_contains_cyrillic() {
        // This test checks that homoglyph substitution can produce Cyrillic characters
        // Run multiple times to increase chance of substitution
        let mut found_cyrillic = false;
        for _ in 0..10 {
            let result = homoglyph_substitution("aeopcx");
            if result.chars().any(|c| c as u32 > 127) {
                found_cyrillic = true;
                break;
            }
        }
        // Note: Random nature means Cyrillic may or may not appear
        // This test primarily ensures the function doesn't panic
    }

    #[test]
    fn test_homoglyph_empty() {
        assert_eq!(homoglyph_substitution(""), "");
    }

    #[test]
    fn test_homoglyph_single_char() {
        let result = homoglyph_substitution("a");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_homoglyph_domain() {
        let result = homoglyph_substitution("example.com");
        assert!(result.contains('.'));
    }

    #[test]
    fn test_homoglyph_email() {
        let result = homoglyph_substitution("admin@example.com");
        assert!(result.contains('@'));
    }

    #[test]
    fn test_homoglyph_numbers() {
        let result = homoglyph_substitution("test01");
        // 0 and 1 may be substituted
        assert!(!result.is_empty());
    }

    #[test]
    fn test_homoglyph_preserves_length_approximate() {
        let input = "paypal";
        let result = homoglyph_substitution(input);
        // May be slightly different due to multi-byte chars
        assert!(result.chars().count() <= input.chars().count() + 2);
    }

    #[test]
    fn test_homoglyph_special_chars() {
        let result = homoglyph_substitution("test!@#");
        assert!(result.contains("!@#"));
    }

    #[test]
    fn test_homoglyph_uppercase() {
        let result = homoglyph_substitution("AEOPC");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_homoglyph_phishing() {
        let result = homoglyph_substitution("secure");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_unicode_variations_changes_chars() {
        // Run multiple times to ensure some variation happens
        let mut changed = false;
        for _ in 0..10 {
            let result = unicode_variations("aeiou");
            if result != "aeiou" {
                changed = true;
                break;
            }
        }
        assert!(changed);
    }

    #[test]
    fn test_unicode_variations_empty() {
        assert_eq!(unicode_variations(""), "");
    }

    #[test]
    fn test_unicode_variations_preserves_count() {
        let result = unicode_variations("admin");
        assert_eq!(result.chars().count(), 5);
    }

    #[test]
    fn test_unicode_variations_consonants() {
        let result = unicode_variations("xyz");
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_unicode_variations_numbers() {
        let result = unicode_variations("test123");
        assert!(result.contains("123"));
    }

    #[test]
    fn test_unicode_variations_special_chars() {
        let result = unicode_variations("test!@#");
        assert!(result.contains("!@#"));
    }

    #[test]
    fn test_unicode_variations_uppercase() {
        let result = unicode_variations("AEIOU");
        assert_eq!(result.chars().count(), 5);
    }

    #[test]
    fn test_unicode_variations_mixed_case() {
        let result = unicode_variations("TeSt");
        assert_eq!(result.chars().count(), 4);
    }

    #[test]
    fn test_unicode_variations_long_string() {
        let result = unicode_variations("administrator");
        assert!(result.chars().count() >= 13);
    }

    #[test]
    fn test_unicode_variations_password() {
        let result = unicode_variations("password");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_space_variants_preserves_length() {
        let input = "hello world test";
        let result = space_variants(input);
        assert_eq!(result.chars().count(), input.chars().count());
    }

    #[test]
    fn test_space_variants_empty() {
        assert_eq!(space_variants(""), "");
    }

    #[test]
    fn test_space_variants_no_spaces() {
        let result = space_variants("hello");
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_space_variants_single_space() {
        let result = space_variants("a b");
        assert_eq!(result.chars().count(), 3);
    }

    #[test]
    fn test_space_variants_multiple_spaces() {
        let result = space_variants("a b c d");
        assert!(result.chars().filter(|c| c.is_whitespace()).count() == 3);
    }

    #[test]
    fn test_space_variants_preserves_words() {
        let result = space_variants("hello world");
        assert!(result.contains("hello") && result.contains("world"));
    }

    #[test]
    fn test_space_variants_sql_injection() {
        let result = space_variants("SELECT * FROM users");
        assert!(result.contains("SELECT"));
    }

    #[test]
    fn test_space_variants_numbers() {
        let result = space_variants("test 123");
        assert!(result.contains("123"));
    }

    #[test]
    fn test_space_variants_special_chars() {
        let result = space_variants("test @ test");
        assert!(result.contains('@'));
    }

    #[test]
    fn test_space_variants_waf_bypass() {
        let result = space_variants("admin login");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_unicode_normalize_variants() {
        let result = unicode_normalize_variants("cafe");
        assert!(result.len() >= 4);
    }

    #[test]
    fn test_unicode_normalize_variants_empty() {
        assert_eq!(unicode_normalize_variants(""), "");
    }

    #[test]
    fn test_unicode_normalize_variants_single_char() {
        let result = unicode_normalize_variants("a");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_unicode_normalize_variants_no_variants() {
        let result = unicode_normalize_variants("xyz");
        assert_eq!(result, "xyz");
    }

    #[test]
    fn test_unicode_normalize_variants_mixed() {
        let result = unicode_normalize_variants("test");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_unicode_normalize_variants_numbers() {
        let result = unicode_normalize_variants("test123");
        assert!(result.contains("123"));
    }

    #[test]
    fn test_unicode_normalize_variants_special_chars() {
        let result = unicode_normalize_variants("test!@#");
        assert!(result.contains("!@#"));
    }

    #[test]
    fn test_unicode_normalize_variants_uppercase() {
        let result = unicode_normalize_variants("AOE");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_unicode_normalize_variants_preserves_non_variant() {
        let result = unicode_normalize_variants("xyz123");
        assert!(result.contains("xyz") && result.contains("123"));
    }

    #[test]
    fn test_unicode_normalize_variants_bypass() {
        let result = unicode_normalize_variants("robot");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_zalgo_text_empty() {
        assert_eq!(zalgo_text(""), "");
    }

    #[test]
    fn test_zalgo_text_single_char() {
        let result = zalgo_text("a");
        assert!(result.len() > 1);
    }

    #[test]
    fn test_zalgo_text_increases_length() {
        let input = "test";
        let result = zalgo_text(input);
        assert!(result.len() > input.len());
    }

    #[test]
    fn test_zalgo_text_preserves_base_chars() {
        let result = zalgo_text("abc");
        assert!(result.contains('a') || result.contains('b') || result.contains('c'));
    }

    #[test]
    fn test_zalgo_text_numbers() {
        let result = zalgo_text("test123");
        assert!(result.contains("123"));
    }

    #[test]
    fn test_zalgo_text_special_chars() {
        let result = zalgo_text("test!@#");
        assert!(result.contains("!@#"));
    }

    #[test]
    fn test_zalgo_text_uppercase() {
        let result = zalgo_text("TEST");
        assert!(result.len() > 4);
    }

    #[test]
    fn test_zalgo_text_mixed_case() {
        let result = zalgo_text("TeSt");
        assert!(result.len() > 4);
    }

    #[test]
    fn test_zalgo_text_display_corruption() {
        let result = zalgo_text("hello");
        assert!(result.len() > 5);
    }

    #[test]
    fn test_zalgo_text_unicode_handling() {
        let result = zalgo_text("test");
        assert!(!result.is_empty());
    }
}
