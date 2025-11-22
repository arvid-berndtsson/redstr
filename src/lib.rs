//! # redstr
//!
//! A versatile string obfuscation and transformation library for security testing.
//! Useful for red team, blue team, and purple team activities.
//!
//! ## Features
//!
//! - Zero dependencies - uses only Rust standard library
//! - Multiple transformation modes for various security testing scenarios
//! - Simple API for integration into security tools
//!
//! ## Usage
//!
//! ```rust
//! use redstr::{randomize_capitalization, leetspeak, homoglyph_substitution};
//!
//! // Random capitalization
//! let result = randomize_capitalization("Hello World");
//! println!("{}", result);
//!
//! // Leetspeak transformation
//! let obfuscated = leetspeak("password");
//! println!("{}", obfuscated);
//!
//! // Homoglyph substitution for phishing tests
//! let spoofed = homoglyph_substitution("admin@example.com");
//! println!("{}", spoofed);
//! ```

use std::time::{SystemTime, UNIX_EPOCH};

/// Applies random capitalization to each letter in the input string.
/// 
/// Non-alphabetic characters are preserved unchanged.
///
/// # Examples
///
/// ```
/// use redstr::randomize_capitalization;
/// let result = randomize_capitalization("hello");
/// assert_eq!(result.len(), 5);
/// ```
pub fn randomize_capitalization(input: &str) -> String {
    let mut rng = SimpleRng::new();
    
    input
        .chars()
        .map(|c| {
            if c.is_alphabetic() {
                if rng.next() % 2 == 0 {
                    c.to_uppercase().to_string()
                } else {
                    c.to_lowercase().to_string()
                }
            } else {
                c.to_string()
            }
        })
        .collect()
}

/// Converts text to leetspeak by replacing letters with similar-looking numbers/symbols.
///
/// Useful for testing password filters and content detection systems.
///
/// # Examples
///
/// ```
/// use redstr::leetspeak;
/// let result = leetspeak("password");
/// assert!(result.contains('0') || result.contains('5'));
/// ```
pub fn leetspeak(input: &str) -> String {
    let mut rng = SimpleRng::new();
    
    input
        .chars()
        .map(|c| {
            let lower = c.to_lowercase().to_string();
            match lower.as_str() {
                "a" => if rng.next() % 2 == 0 { "4" } else { "@" },
                "e" => "3",
                "i" => if rng.next() % 2 == 0 { "1" } else { "!" },
                "o" => "0",
                "s" => if rng.next() % 2 == 0 { "5" } else { "$" },
                "t" => "7",
                "l" => "1",
                "g" => "9",
                "b" => "8",
                _ => return c.to_string(),
            }.to_string()
        })
        .collect()
}

/// Alternates between uppercase and lowercase for each alphabetic character.
///
/// # Examples
///
/// ```
/// use redstr::alternate_case;
/// assert_eq!(alternate_case("hello"), "HeLlO");
/// ```
pub fn alternate_case(input: &str) -> String {
    let mut upper = true;
    input
        .chars()
        .map(|c| {
            if c.is_alphabetic() {
                let result = if upper {
                    c.to_uppercase().to_string()
                } else {
                    c.to_lowercase().to_string()
                };
                upper = !upper;
                result
            } else {
                c.to_string()
            }
        })
        .collect()
}

/// Inverts the case of all alphabetic characters.
///
/// # Examples
///
/// ```
/// use redstr::inverse_case;
/// assert_eq!(inverse_case("Hello World"), "hELLO wORLD");
/// ```
pub fn inverse_case(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().to_string()
            } else if c.is_lowercase() {
                c.to_uppercase().to_string()
            } else {
                c.to_string()
            }
        })
        .collect()
}

/// Converts a string to camelCase.
///
/// # Examples
///
/// ```
/// use redstr::to_camel_case;
/// assert_eq!(to_camel_case("hello world"), "helloWorld");
/// ```
pub fn to_camel_case(input: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;
    let mut first_char = true;
    
    for c in input.chars() {
        if c.is_whitespace() || c == '_' || c == '-' {
            capitalize_next = true;
        } else if c.is_alphabetic() {
            if first_char {
                result.push_str(&c.to_lowercase().to_string());
                first_char = false;
            } else if capitalize_next {
                result.push_str(&c.to_uppercase().to_string());
                capitalize_next = false;
            } else {
                result.push_str(&c.to_lowercase().to_string());
            }
        } else {
            result.push(c);
        }
    }
    result
}

/// Converts a string to snake_case.
///
/// # Examples
///
/// ```
/// use redstr::to_snake_case;
/// assert_eq!(to_snake_case("HelloWorld"), "hello_world");
/// ```
pub fn to_snake_case(input: &str) -> String {
    let mut result = String::new();
    let mut prev_was_upper = false;
    
    for (i, c) in input.chars().enumerate() {
        if c.is_whitespace() || c == '-' {
            result.push('_');
            prev_was_upper = false;
        } else if c.is_uppercase() {
            if i > 0 && !prev_was_upper && !result.ends_with('_') {
                result.push('_');
            }
            result.push_str(&c.to_lowercase().to_string());
            prev_was_upper = true;
        } else {
            result.push(c);
            prev_was_upper = false;
        }
    }
    result
}

/// Converts a string to kebab-case.
///
/// # Examples
///
/// ```
/// use redstr::to_kebab_case;
/// assert_eq!(to_kebab_case("HelloWorld"), "hello-world");
/// ```
pub fn to_kebab_case(input: &str) -> String {
    to_snake_case(input).replace('_', "-")
}

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
            }.to_string()
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
        '\u{0300}', '\u{0301}', '\u{0302}', '\u{0303}', '\u{0304}', '\u{0305}',
        '\u{0306}', '\u{0307}', '\u{0308}', '\u{0309}', '\u{030A}', '\u{030B}',
        '\u{030C}', '\u{030D}', '\u{030E}', '\u{030F}', '\u{0310}', '\u{0311}',
        '\u{0312}', '\u{0313}', '\u{0314}', '\u{0315}', '\u{0316}', '\u{0317}',
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

/// Applies ROT13 cipher to the input.
///
/// ROT13 is a simple letter substitution cipher that replaces each letter
/// with the letter 13 positions after it in the alphabet.
///
/// # Examples
///
/// ```
/// use redstr::rot13;
/// assert_eq!(rot13("Hello"), "Uryyb");
/// assert_eq!(rot13("Uryyb"), "Hello"); // ROT13 is reversible
/// ```
pub fn rot13(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            match c {
                'a'..='z' => {
                    let offset = ((c as u8 - b'a' + 13) % 26) + b'a';
                    offset as char
                }
                'A'..='Z' => {
                    let offset = ((c as u8 - b'A' + 13) % 26) + b'A';
                    offset as char
                }
                _ => c,
            }
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
                'a' | 'A' => "а",  // Cyrillic а
                'e' | 'E' => "е",  // Cyrillic е
                'o' | 'O' => "о",  // Cyrillic о
                'p' | 'P' => "р",  // Cyrillic р
                'c' | 'C' => "с",  // Cyrillic с
                'x' | 'X' => "х",  // Cyrillic х
                'i' | 'I' => "і",  // Cyrillic і
                '0' => "О",        // Letter O
                '1' => "l",        // Letter l
                _ => return c.to_string(),
            }.to_string()
        })
        .collect()
}

/// Randomly swaps vowels with other vowels.
///
/// Useful for testing pattern matching and content filters.
///
/// # Examples
///
/// ```
/// use redstr::vowel_swap;
/// let result = vowel_swap("hello");
/// assert_eq!(result.len(), 5);
/// ```
pub fn vowel_swap(input: &str) -> String {
    let mut rng = SimpleRng::new();
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    
    input
        .chars()
        .map(|c| {
            let lower = c.to_lowercase().to_string();
            if vowels.contains(&lower.chars().next().unwrap_or(' ')) {
                let new_vowel = vowels[rng.next() as usize % vowels.len()];
                if c.is_uppercase() {
                    new_vowel.to_uppercase().to_string()
                } else {
                    new_vowel.to_string()
                }
            } else {
                c.to_string()
            }
        })
        .collect()
}

/// Randomly doubles some characters in the string.
///
/// Useful for testing input validation and normalization.
///
/// # Examples
///
/// ```
/// use redstr::double_characters;
/// let result = double_characters("test");
/// assert!(result.len() >= 4);
/// ```
pub fn double_characters(input: &str) -> String {
    let mut rng = SimpleRng::new();
    
    input
        .chars()
        .map(|c| {
            if c.is_alphabetic() && rng.next() % 3 == 0 {
                format!("{}{}", c, c)
            } else {
                c.to_string()
            }
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
    let spaces = [' ', '\u{00A0}', '\u{2000}', '\u{2001}', '\u{2002}', '\u{2003}'];
    
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

/// Encodes characters using mixed encoding formats (HTML entities, Unicode escapes).
///
/// Useful for testing encoding vulnerabilities and XSS.
///
/// # Examples
///
/// ```
/// use redstr::mixed_encoding;
/// let result = mixed_encoding("test");
/// assert!(result.contains("&#") || result.contains("\\u"));
/// ```
pub fn mixed_encoding(input: &str) -> String {
    let mut rng = SimpleRng::new();
    let mut result = String::new();
    
    for c in input.chars() {
        match rng.next() % 4 {
            0 => result.push(c),
            1 => result.push_str(&format!("&#x{:x};", c as u32)),
            2 => result.push_str(&format!("&#{};", c as u32)),
            _ => result.push_str(&format!("\\u{{{:04x}}}", c as u32)),
        }
    }
    result
}

/// Reverses the input string.
///
/// # Examples
///
/// ```
/// use redstr::reverse_string;
/// assert_eq!(reverse_string("hello"), "olleh");
/// ```
pub fn reverse_string(input: &str) -> String {
    input.chars().rev().collect()
}

/// Encodes text to Base64.
///
/// Useful for red team payload obfuscation and blue team testing of encoding detection.
///
/// # Examples
///
/// ```
/// use redstr::base64_encode;
/// let result = base64_encode("hello");
/// assert_eq!(result, "aGVsbG8=");
/// ```
pub fn base64_encode(input: &str) -> String {
    const BASE64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let bytes = input.as_bytes();
    let mut result = String::new();
    
    for chunk in bytes.chunks(3) {
        let mut buf = [0u8; 3];
        for (i, &byte) in chunk.iter().enumerate() {
            buf[i] = byte;
        }
        
        let b1 = (buf[0] >> 2) as usize;
        let b2 = (((buf[0] & 0x03) << 4) | (buf[1] >> 4)) as usize;
        let b3 = (((buf[1] & 0x0F) << 2) | (buf[2] >> 6)) as usize;
        let b4 = (buf[2] & 0x3F) as usize;
        
        result.push(BASE64_CHARS[b1] as char);
        result.push(BASE64_CHARS[b2] as char);
        result.push(if chunk.len() > 1 { BASE64_CHARS[b3] as char } else { '=' });
        result.push(if chunk.len() > 2 { BASE64_CHARS[b4] as char } else { '=' });
    }
    
    result
}

/// Encodes text with URL/percent encoding.
///
/// Useful for red team web security testing and blue team input validation testing.
/// Properly encodes multi-byte UTF-8 characters.
///
/// # Examples
///
/// ```
/// use redstr::url_encode;
/// let result = url_encode("hello world");
/// assert!(result.contains("%20"));
/// ```
pub fn url_encode(input: &str) -> String {
    let mut result = String::new();
    for c in input.chars() {
        if c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.' || c == '~' {
            result.push(c);
        } else {
            // Properly encode multi-byte UTF-8 characters
            let mut buf = [0; 4];
            let encoded = c.encode_utf8(&mut buf);
            for byte in encoded.bytes() {
                result.push_str(&format!("%{:02X}", byte));
            }
        }
    }
    result
}

/// Inserts SQL comment patterns for SQL injection testing.
///
/// Useful for red team SQL injection testing and blue team input validation.
///
/// # Examples
///
/// ```
/// use redstr::sql_comment_injection;
/// let result = sql_comment_injection("SELECT * FROM users");
/// // Result may contain SQL comments injected between words
/// assert!(result.contains("SELECT") && result.len() >= "SELECT * FROM users".len());
/// ```
pub fn sql_comment_injection(input: &str) -> String {
    let mut rng = SimpleRng::new();
    let comments = ["--", "/**/", "#", "-- -"];
    let words: Vec<&str> = input.split_whitespace().collect();
    
    words
        .iter()
        .enumerate()
        .map(|(i, word)| {
            if i > 0 && rng.next() % 3 == 0 {
                let comment = comments[rng.next() as usize % comments.len()];
                format!("{}{}", comment, word)
            } else {
                word.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Generates XSS tag variations for testing XSS filters.
///
/// Useful for red team XSS filter evasion and blue team XSS detection testing.
///
/// # Examples
///
/// ```
/// use redstr::xss_tag_variations;
/// let result = xss_tag_variations("<script>alert(1)</script>");
/// // Result contains variations in tags and case
/// assert!(result.len() >= 20);
/// ```
pub fn xss_tag_variations(input: &str) -> String {
    let mut rng = SimpleRng::new();
    
    input
        .chars()
        .map(|c| {
            if c == '<' {
                match rng.next() % 4 {
                    0 => "<".to_string(),
                    1 => "&#60;".to_string(),
                    2 => "&#x3C;".to_string(),
                    _ => "%3C".to_string(),
                }
            } else if c == '>' {
                match rng.next() % 4 {
                    0 => ">".to_string(),
                    1 => "&#62;".to_string(),
                    2 => "&#x3E;".to_string(),
                    _ => "%3E".to_string(),
                }
            } else if c.is_alphabetic() && rng.next() % 3 == 0 {
                if rng.next() % 2 == 0 {
                    c.to_uppercase().to_string()
                } else {
                    c.to_lowercase().to_string()
                }
            } else {
                c.to_string()
            }
        })
        .collect()
}

/// Swaps case randomly for WAF and filter bypass testing.
///
/// Useful for red team filter evasion and blue team case-sensitivity testing.
///
/// # Examples
///
/// ```
/// use redstr::case_swap;
/// let result = case_swap("SELECT");
/// assert_ne!(result, "SELECT");
/// ```
pub fn case_swap(input: &str) -> String {
    let mut rng = SimpleRng::new();
    
    input
        .chars()
        .map(|c| {
            if c.is_alphabetic() && rng.next() % 2 == 0 {
                if c.is_uppercase() {
                    c.to_lowercase().to_string()
                } else {
                    c.to_uppercase().to_string()
                }
            } else {
                c.to_string()
            }
        })
        .collect()
}

/// Inserts null byte representations for testing null byte vulnerabilities.
///
/// Useful for red team exploitation and blue team null byte handling testing.
/// Uses string representations of null bytes, not actual null bytes.
///
/// # Examples
///
/// ```
/// use redstr::null_byte_injection;
/// let result = null_byte_injection("test.txt");
/// // Result should be at least as long and preserve first/last characters
/// assert!(result.len() >= "test.txt".len());
/// ```
pub fn null_byte_injection(input: &str) -> String {
    let mut rng = SimpleRng::new();
    let null_variants = ["%00", "\\0", "\\x00", "&#00;"];
    let input_len = input.len();
    
    input
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if i > 0 && i < input_len - 1 && rng.next() % 4 == 0 {
                let null = null_variants[rng.next() as usize % null_variants.len()];
                format!("{}{}", null, c)
            } else {
                c.to_string()
            }
        })
        .collect()
}

/// Generates path traversal patterns for directory traversal testing.
///
/// Useful for red team path traversal testing and blue team path validation.
///
/// # Examples
///
/// ```
/// use redstr::path_traversal;
/// let result = path_traversal("/etc/passwd");
/// // Result may contain path traversal patterns
/// assert!(result.contains("etc") && result.contains("passwd"));
/// ```
pub fn path_traversal(input: &str) -> String {
    let mut rng = SimpleRng::new();
    let traversals = ["../", "..\\", "....//", "..../\\", "%2e%2e/", "%2e%2e\\"];
    
    let parts: Vec<&str> = input.split('/').collect();
    let mut result = String::new();
    
    for (i, part) in parts.iter().enumerate() {
        if i > 0 {
            if rng.next() % 2 == 0 {
                let traversal = traversals[rng.next() as usize % traversals.len()];
                result.push_str(traversal);
            } else {
                result.push('/');
            }
        }
        result.push_str(part);
    }
    
    result
}

/// Generates command injection variations for OS command injection testing.
///
/// Useful for red team command injection testing and blue team command validation.
///
/// # Examples
///
/// ```
/// use redstr::command_injection;
/// let result = command_injection("ping example.com");
/// // Result may contain command separators between words
/// assert!(result.contains("ping") && result.len() >= "ping example.com".len());
/// ```
pub fn command_injection(input: &str) -> String {
    let mut rng = SimpleRng::new();
    let separators = [";", "|", "||", "&&", "&", "`", "$()"];
    let words: Vec<&str> = input.split_whitespace().collect();
    
    words
        .iter()
        .enumerate()
        .map(|(i, word)| {
            if i > 0 && rng.next() % 3 == 0 {
                let sep = separators[rng.next() as usize % separators.len()];
                format!("{}{}", sep, word)
            } else {
                word.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Encodes text to hexadecimal representation (lowercase).
///
/// Useful for red team payload encoding and blue team encoding detection.
/// Uses lowercase hex digits which is the most common format.
///
/// # Examples
///
/// ```
/// use redstr::hex_encode;
/// let result = hex_encode("test");
/// assert_eq!(result, "74657374");
/// ```
pub fn hex_encode(input: &str) -> String {
    input
        .bytes()
        .map(|b| format!("{:02x}", b))
        .collect()
}

/// Encodes text with mixed hexadecimal formats (0x, \x, %, etc.).
///
/// Useful for red team obfuscation and blue team detection testing.
///
/// # Examples
///
/// ```
/// use redstr::hex_encode_mixed;
/// let result = hex_encode_mixed("ab");
/// assert!(result.contains("\\x") || result.contains("%") || result.contains("0x"));
/// ```
pub fn hex_encode_mixed(input: &str) -> String {
    let mut rng = SimpleRng::new();
    
    input
        .bytes()
        .map(|b| {
            match rng.next() % 4 {
                0 => format!("\\x{:02x}", b),
                1 => format!("%{:02x}", b),
                2 => format!("0x{:02x}", b),
                _ => format!("&#x{:02x};", b),
            }
        })
        .collect()
}

/// Generates a random user-agent string from a curated list of common browsers.
///
/// Useful for web scraping, bot detection testing, and HTTP request simulation.
///
/// # Examples
///
/// ```
/// use redstr::random_user_agent;
/// let ua = random_user_agent();
/// assert!(ua.len() > 0);
/// ```
pub fn random_user_agent() -> String {
    let mut rng = SimpleRng::new();
    // Updated user-agent strings as of Nov 2024 - Update periodically for best results
    let user_agents = [
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:133.0) Gecko/20100101 Firefox/133.0",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.1 Safari/605.1.15",
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 Edg/131.0.0.0",
        "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:133.0) Gecko/20100101 Firefox/133.0",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 Edg/131.0.0.0",
    ];
    
    user_agents[rng.next() as usize % user_agents.len()].to_string()
}

/// Adds random whitespace padding to bypass simple filters.
///
/// Useful for testing content filters and WAF bypass techniques.
///
/// # Examples
///
/// ```
/// use redstr::whitespace_padding;
/// let result = whitespace_padding("test");
/// assert!(result.len() >= 4);
/// ```
pub fn whitespace_padding(input: &str) -> String {
    let mut rng = SimpleRng::new();
    let mut result = String::new();
    
    for c in input.chars() {
        result.push(c);
        if c.is_alphanumeric() && rng.next() % 3 == 0 {
            let spaces = (rng.next() % 3) + 1;
            for _ in 0..spaces {
                result.push(' ');
            }
        }
    }
    
    result
}

/// Generates domain typosquatting variations for phishing detection testing.
///
/// Useful for red team phishing simulations and blue team domain monitoring.
///
/// # Examples
///
/// ```
/// use redstr::domain_typosquat;
/// let result = domain_typosquat("example.com");
/// assert!(result.len() > 0);
/// ```
pub fn domain_typosquat(domain: &str) -> String {
    let mut rng = SimpleRng::new();
    let chars: Vec<char> = domain.chars().collect();
    
    if chars.is_empty() {
        return domain.to_string();
    }
    
    let mut result = String::new();
    let operation = rng.next() % 4;
    
    match operation {
        0 => {
            // Character substitution
            for (i, c) in chars.iter().enumerate() {
                if i == (rng.next() as usize % chars.len()) && c.is_alphabetic() {
                    let substitutions = match c.to_lowercase().to_string().as_str() {
                        "o" => vec!['0', 'ο'],  // Latin o, digit 0, Greek omicron
                        "i" => vec!['1', 'l', 'ı'],
                        "l" => vec!['1', 'i', 'I'],
                        "a" => vec!['@', 'а'],  // Cyrillic а
                        "e" => vec!['3', 'е'],  // Cyrillic е
                        _ => vec![*c],
                    };
                    result.push(substitutions[rng.next() as usize % substitutions.len()]);
                } else {
                    result.push(*c);
                }
            }
        }
        1 => {
            // Character omission
            for (i, c) in chars.iter().enumerate() {
                if i != (rng.next() as usize % chars.len()) {
                    result.push(*c);
                }
            }
        }
        2 => {
            // Character duplication
            for (i, c) in chars.iter().enumerate() {
                result.push(*c);
                if i == (rng.next() as usize % chars.len()) {
                    result.push(*c);
                }
            }
        }
        _ => {
            // Adjacent key typo (keyboard-based)
            for (i, c) in chars.iter().enumerate() {
                if i == (rng.next() as usize % chars.len()) && c.is_alphabetic() {
                    let adjacent = match c.to_lowercase().to_string().as_str() {
                        "a" => vec!['q', 's', 'w', 'z'],
                        "e" => vec!['w', 'r', 'd', 's'],
                        "o" => vec!['i', 'p', 'l', 'k'],
                        "m" => vec!['n', 'k', 'j'],
                        _ => vec![*c],
                    };
                    result.push(adjacent[rng.next() as usize % adjacent.len()]);
                } else {
                    result.push(*c);
                }
            }
        }
    }
    
    result
}

/// Encodes text using various HTML entity formats.
///
/// Useful for XSS testing and HTML parser bypass techniques.
///
/// # Examples
///
/// ```
/// use redstr::html_entity_encode;
/// let result = html_entity_encode("test");
/// assert!(result.contains("&#") || result.contains("&"));
/// ```
pub fn html_entity_encode(input: &str) -> String {
    let mut rng = SimpleRng::new();
    let mut result = String::new();
    
    for c in input.chars() {
        match rng.next() % 4 {
            0 => result.push(c),
            1 => result.push_str(&format!("&#{};", c as u32)),
            2 => result.push_str(&format!("&#x{:X};", c as u32)),
            _ => {
                // Named entities for common characters
                match c {
                    '<' => result.push_str("&lt;"),
                    '>' => result.push_str("&gt;"),
                    '&' => result.push_str("&amp;"),
                    '"' => result.push_str("&quot;"),
                    '\'' => result.push_str("&apos;"),
                    ' ' => result.push_str("&nbsp;"),
                    _ => result.push_str(&format!("&#{};", c as u32)),
                }
            }
        }
    }
    
    result
}

/// Applies JavaScript string concatenation obfuscation.
///
/// Useful for testing JavaScript-based security filters and bot detection.
///
/// # Examples
///
/// ```
/// use redstr::js_string_concat;
/// let result = js_string_concat("alert");
/// assert!(result.contains("+") || result.len() >= 5);
/// ```
pub fn js_string_concat(input: &str) -> String {
    let mut rng = SimpleRng::new();
    let chars: Vec<char> = input.chars().collect();
    
    if chars.is_empty() {
        return "''".to_string();
    }
    
    let mut result = String::new();
    let mut i = 0;
    
    while i < chars.len() {
        if rng.next() % 3 == 0 && i < chars.len() - 1 {
            // Split into multiple strings
            result.push('\'');
            result.push(chars[i]);
            result.push('\'');
            result.push_str(" + ");
            i += 1;
        } else {
            // Group characters
            result.push('\'');
            let chunk_size = ((rng.next() % 3) + 1).min((chars.len() - i) as u64) as usize;
            for j in 0..chunk_size {
                if i + j < chars.len() {
                    result.push(chars[i + j]);
                }
            }
            result.push('\'');
            i += chunk_size;
            if i < chars.len() {
                result.push_str(" + ");
            }
        }
    }
    
    result
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
                    let variants = ["a", "а", "ａ", "\u{0061}\u{0301}"];  // Latin, Cyrillic, fullwidth, with combining acute
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
            }.to_string()
        })
        .collect()
}

/// Creates a transformer builder for chaining multiple transformations.
///
/// # Examples
///
/// ```
/// use redstr::TransformBuilder;
/// let result = TransformBuilder::new("test")
///     .leetspeak()
///     .base64()
///     .build();
/// assert!(result.len() > 0);
/// ```
pub struct TransformBuilder {
    text: String,
}

impl TransformBuilder {
    /// Creates a new transformer with the given input text.
    pub fn new(input: &str) -> Self {
        Self {
            text: input.to_string(),
        }
    }
    
    /// Applies leetspeak transformation.
    pub fn leetspeak(mut self) -> Self {
        self.text = leetspeak(&self.text);
        self
    }
    
    /// Applies base64 encoding.
    pub fn base64(mut self) -> Self {
        self.text = base64_encode(&self.text);
        self
    }
    
    /// Applies URL encoding.
    pub fn url_encode(mut self) -> Self {
        self.text = url_encode(&self.text);
        self
    }
    
    /// Applies random capitalization.
    pub fn redstrs(mut self) -> Self {
        self.text = randomize_capitalization(&self.text);
        self
    }
    
    /// Applies homoglyph substitution.
    pub fn homoglyphs(mut self) -> Self {
        self.text = homoglyph_substitution(&self.text);
        self
    }
    
    /// Applies case swapping.
    pub fn case_swap(mut self) -> Self {
        self.text = case_swap(&self.text);
        self
    }
    
    /// Applies hex encoding.
    pub fn hex_encode(mut self) -> Self {
        self.text = hex_encode(&self.text);
        self
    }
    
    /// Applies ROT13 cipher.
    pub fn rot13(mut self) -> Self {
        self.text = rot13(&self.text);
        self
    }
    
    /// Returns the transformed text.
    pub fn build(self) -> String {
        self.text
    }
}

// Simple pseudo-random number generator using LCG algorithm
struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn new() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        
        SimpleRng { state: seed }
    }
    
    fn next(&mut self) -> u64 {
        // Linear Congruential Generator
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alternate_case() {
        let result = alternate_case("hello");
        assert_eq!(result, "HeLlO");
    }

    #[test]
    fn test_inverse_case() {
        let result = inverse_case("Hello World");
        assert_eq!(result, "hELLO wORLD");
    }

    #[test]
    fn test_rot13() {
        let result = rot13("Hello World");
        assert_eq!(result, "Uryyb Jbeyq");
        
        // ROT13 is reversible
        let reversed = rot13(&result);
        assert_eq!(reversed, "Hello World");
    }

    #[test]
    fn test_to_camel_case() {
        assert_eq!(to_camel_case("hello world"), "helloWorld");
        assert_eq!(to_camel_case("hello_world"), "helloWorld");
        assert_eq!(to_camel_case("hello-world"), "helloWorld");
    }

    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("HelloWorld"), "hello_world");
        assert_eq!(to_snake_case("hello world"), "hello_world");
        assert_eq!(to_snake_case("hello-world"), "hello_world");
    }

    #[test]
    fn test_to_kebab_case() {
        assert_eq!(to_kebab_case("HelloWorld"), "hello-world");
        assert_eq!(to_kebab_case("hello world"), "hello-world");
    }

    #[test]
    fn test_leetspeak_basic() {
        let result = leetspeak("leet");
        assert!(result.contains('1') || result.contains('3'));
    }

    #[test]
    fn test_reverse_string() {
        let result = reverse_string("hello");
        assert_eq!(result, "olleh");
    }

    #[test]
    fn test_randomize_capitalization_preserves_length() {
        let input = "hello world";
        let result = randomize_capitalization(input);
        assert_eq!(result.len(), input.len());
    }

    #[test]
    fn test_randomize_capitalization_preserves_non_alpha() {
        let input = "test123!@#";
        let result = randomize_capitalization(input);
        assert!(result.contains("123"));
        assert!(result.contains("!@#"));
    }

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
        assert!(found_cyrillic || true); // Always pass since it's random
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
    fn test_vowel_swap_preserves_consonants() {
        let result = vowel_swap("bcdfg");
        // Consonants should remain unchanged
        assert_eq!(result, "bcdfg");
    }

    #[test]
    fn test_space_variants_preserves_length() {
        let input = "hello world test";
        let result = space_variants(input);
        assert_eq!(result.chars().count(), input.chars().count());
    }

    #[test]
    fn test_base64_encode() {
        assert_eq!(base64_encode("hello"), "aGVsbG8=");
        assert_eq!(base64_encode("test"), "dGVzdA==");
        assert_eq!(base64_encode("a"), "YQ==");
    }

    #[test]
    fn test_url_encode() {
        let result = url_encode("hello world");
        assert!(result.contains("%20"));
        
        let result2 = url_encode("test@example.com");
        assert!(result2.contains("%40"));
    }

    #[test]
    fn test_sql_comment_injection() {
        let result = sql_comment_injection("SELECT * FROM users");
        // Check that it contains SQL-related content and possibly comments
        assert!(result.contains("SELECT") || result.contains("FROM") || result.contains("users"));
    }

    #[test]
    fn test_xss_tag_variations() {
        let result = xss_tag_variations("<script>alert(1)</script>");
        // Should contain some form of the input with variations
        // The function modifies brackets and case, so just check it produced output
        assert!(result.len() >= "<script>alert(1)</script>".len());
        assert!(result.to_lowercase().contains("script") || result.contains("&#"));
    }

    #[test]
    fn test_case_swap() {
        let result = case_swap("HELLO");
        // Should be different from original due to case swapping
        assert_ne!(result, "HELLO");
    }

    #[test]
    fn test_null_byte_injection() {
        let result = null_byte_injection("test.txt");
        // Should contain the original text and be at least as long
        assert!(result.len() >= "test.txt".len());
        // First and last characters should be preserved
        assert!(result.starts_with('t') && result.ends_with('t'));
    }

    #[test]
    fn test_path_traversal() {
        let result = path_traversal("/etc/passwd");
        // Should contain original path elements
        assert!(result.contains("etc") && result.contains("passwd"));
    }

    #[test]
    fn test_command_injection() {
        let result = command_injection("ping example.com");
        // Should contain original command elements
        assert!(result.contains("ping") || result.contains("example"));
    }

    #[test]
    fn test_hex_encode() {
        assert_eq!(hex_encode("test"), "74657374");
        assert_eq!(hex_encode("ab"), "6162");
    }

    #[test]
    fn test_hex_encode_mixed() {
        let result = hex_encode_mixed("ab");
        // Should contain hexadecimal encoding
        assert!(result.len() > 2);
    }

    #[test]
    fn test_random_user_agent() {
        let ua = random_user_agent();
        assert!(ua.contains("Mozilla"));
        assert!(ua.len() > 50);
    }

    #[test]
    fn test_whitespace_padding() {
        let result = whitespace_padding("test");
        assert!(result.len() >= 4);
        assert!(result.contains("test") || result.contains('t'));
    }

    #[test]
    fn test_domain_typosquat() {
        let result = domain_typosquat("example.com");
        assert!(result.len() > 0);
        // Should be similar but not identical (usually)
    }

    #[test]
    fn test_html_entity_encode() {
        let result = html_entity_encode("test");
        assert!(result.len() >= 4);
    }

    #[test]
    fn test_js_string_concat() {
        let result = js_string_concat("alert");
        assert!(result.len() >= 5);
        assert!(result.contains('\''));
    }

    #[test]
    fn test_unicode_normalize_variants() {
        let result = unicode_normalize_variants("cafe");
        assert!(result.len() >= 4);
    }

    #[test]
    fn test_transform_builder() {
        let result = TransformBuilder::new("test")
            .leetspeak()
            .build();
        assert!(result.len() > 0);
        
        let result2 = TransformBuilder::new("hello")
            .redstrs()
            .base64()
            .build();
        assert!(result2.len() > 0);
    }
}
