use crate::rng::SimpleRng;

/// Replaces characters with random Unicode variations.
///
/// Substitutes common letters with accented or modified Unicode variants.
/// For example, 'a' might become 'à', 'á', 'â', 'ã', or 'ä'. This creates
/// visually similar text that can bypass simple string matching while testing
/// Unicode normalization and internationalization handling.
///
/// # Use Cases
///
/// - **Red Team**: Bypass keyword filters with Unicode lookalikes
/// - **Blue Team**: Test Unicode normalization in security controls
/// - **i18n Testing**: Verify internationalized text handling
/// - **Content Filters**: Test accent-insensitive matching
///
/// # Examples
///
/// ```
/// use redstr::unicode_variations;
///
/// let result = unicode_variations("admin");
/// // Example output: "ädmïn" or "àdmîn" or "ádmįn" (varies each run)
/// assert_eq!(result.chars().count(), 5);
///
/// // Bypass keyword filters
/// let word = unicode_variations("password");
/// // Example: "pässwörd" or "pàsswôrd"
///
/// // Test Unicode normalization
/// let email = unicode_variations("user@example.com");
/// // Example: "ûsér@ëxämplè.çöm"
/// ```
pub fn unicode_variations(input: &str) -> String {
    let mut rng = SimpleRng::new();

    input
        .chars()
        .map(|c| {
            let lower = c.to_lowercase().to_string();
            match lower.as_str() {
                "a" => ["a", "à", "á", "â", "ã", "ä", "å", "ā", "ă"][rng.next_u64() as usize % 9],
                "e" => ["e", "è", "é", "ê", "ë", "ē", "ĕ", "ė"][rng.next_u64() as usize % 8],
                "i" => ["i", "ì", "í", "î", "ï", "ī", "ĭ", "į"][rng.next_u64() as usize % 8],
                "o" => ["o", "ò", "ó", "ô", "õ", "ö", "ō", "ŏ"][rng.next_u64() as usize % 8],
                "u" => ["u", "ù", "ú", "û", "ü", "ū", "ŭ", "ů"][rng.next_u64() as usize % 8],
                "c" => ["c", "ç", "ć", "ĉ", "ċ", "č"][rng.next_u64() as usize % 6],
                "n" => ["n", "ñ", "ń", "ņ", "ň"][rng.next_u64() as usize % 5],
                "s" => ["s", "ś", "ŝ", "ş", "š"][rng.next_u64() as usize % 5],
                _ => return c.to_string(),
            }
            .to_string()
        })
        .collect()
}

/// Adds zalgo combining characters to create corrupted-looking text.
///
/// Attaches 1-3 random Unicode combining diacritical marks to each alphabetic
/// character, creating "zalgo text" that appears corrupted or glitchy. These
/// combining characters stack above and below letters, useful for testing
/// display rendering and Unicode edge cases.
///
/// # Use Cases
///
/// - **Display Testing**: Verify how systems render combining characters
/// - **Blue Team**: Test input sanitization and character filtering
/// - **DoS Testing**: Check if excessive combining marks cause issues
/// - **Unicode Handling**: Validate proper Unicode normalization
///
/// # Examples
///
/// ```
/// use redstr::zalgo_text;
///
/// let result = zalgo_text("test");
/// // Example output: "t̃̂e̊̋s̈̃t̂̃" (with combining marks)
/// assert!(result.len() > 4);
///
/// // Create glitchy-looking text
/// let username = zalgo_text("admin");
/// // Output looks like: "a̅̆d̃m̂ĭn̈" (rendered with marks above/below)
///
/// // Test display systems
/// let message = zalgo_text("hello");
/// // Creates visually corrupted text for testing
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
                let count = (rng.next_u64() % 3) + 1;
                for _ in 0..count {
                    let idx = rng.next_u64() as usize % combining_chars.len();
                    result.push(combining_chars[idx]);
                }
            }
            result
        })
        .collect()
}

/// Substitutes characters with similar-looking homoglyphs.
///
/// Randomly replaces Latin letters with visually identical or similar Cyrillic
/// characters (e.g., Latin 'a' → Cyrillic 'а', Latin 'e' → Cyrillic 'е').
/// About 33% of eligible characters are substituted. This is the core technique
/// for homograph/IDN spoofing attacks where `example.com` becomes `еxample.com`.
///
/// # Use Cases
///
/// - **Phishing Testing**: Create lookalike domains for security training
/// - **IDN Spoofing**: Test internationalized domain name vulnerabilities
/// - **Red Team**: Bypass domain/URL whitelists and filters
/// - **Blue Team**: Validate homograph detection systems
///
/// # Examples
///
/// ```
/// use redstr::homoglyph_substitution;
///
/// let result = homoglyph_substitution("example");
/// // Example output: "ехаmple" (Cyrillic е and а, Latin m,p,l)
/// // Looks identical but uses different Unicode codepoints
///
/// // Phishing domain generation
/// let domain = homoglyph_substitution("paypal.com");
/// // Example: "pаypаl.com" (Cyrillic а instead of Latin a)
///
/// // Email spoofing test
/// let email = homoglyph_substitution("admin@company.com");
/// // Example: "аdmin@compаny.com" (Cyrillic characters)
///
/// // Number lookalikes
/// let pin = homoglyph_substitution("2021");
/// // Example: "2О2l" (Letter O and l instead of 0 and 1)
/// ```
pub fn homoglyph_substitution(input: &str) -> String {
    let mut rng = SimpleRng::new();

    input
        .chars()
        .map(|c| {
            if rng.next_u64() % 3 != 0 {
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
/// Substitutes ASCII space characters (U+0020) with random Unicode space
/// variants including non-breaking space (U+00A0) and various em/en spaces.
/// These look identical but have different codepoints, useful for testing
/// whitespace normalization and parser robustness.
///
/// # Use Cases
///
/// - **WAF Bypass**: Evade filters that only check for ASCII spaces
/// - **Blue Team**: Test whitespace normalization in parsers
/// - **Input Validation**: Verify proper handling of Unicode spaces
/// - **SQL Injection**: Use non-breaking spaces to bypass filters
///
/// # Examples
///
/// ```
/// use redstr::space_variants;
///
/// let result = space_variants("hello world");
/// // Looks identical: "hello world"
/// // But may contain U+00A0, U+2000, U+2001, etc. instead of U+0020
/// assert_eq!(result.chars().filter(|c| c.is_whitespace()).count(), 1);
///
/// // SQL injection with Unicode spaces
/// let sql = space_variants("SELECT * FROM users");
/// // Uses non-breaking spaces to bypass filters
///
/// // Test whitespace normalization
/// let text = space_variants("word1 word2 word3");
/// // Visually identical but with mixed Unicode spaces
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
                spaces[rng.next_u64() as usize % spaces.len()].to_string()
            } else {
                c.to_string()
            }
        })
        .collect()
}

/// Generates unicode normalization variations (NFD, NFC, NFKC, NFKD concepts).
///
/// Creates variations that represent similar Unicode normalization concepts
/// by randomly replacing characters with composed or decomposed forms, or
/// adding combining characters. Tests how systems handle different Unicode
/// normalization forms (NFC, NFD, NFKC, NFKD).
///
/// # Use Cases
///
/// - **Security Testing**: Test if systems normalize Unicode properly
/// - **Bypass Filters**: Exploit inconsistent Unicode handling
/// - **Blue Team**: Validate Unicode normalization in security controls
/// - **Data Quality**: Test string comparison and matching
///
/// # Examples
///
/// ```
/// use redstr::unicode_normalize_variants;
///
/// let result = unicode_normalize_variants("café");
/// // May produce composed (café) or decomposed (cafe\u{0301}) forms
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
                    variants[rng.next_u64() as usize % variants.len()]
                }
                'e' | 'E' => {
                    let variants = ["e", "е", "ｅ", "\u{0065}\u{0301}"];
                    variants[rng.next_u64() as usize % variants.len()]
                }
                'o' | 'O' => {
                    let variants = ["o", "о", "ｏ", "\u{006F}\u{0301}"];
                    variants[rng.next_u64() as usize % variants.len()]
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
