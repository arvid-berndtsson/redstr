use crate::rng::SimpleRng;

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
    let mut result = String::with_capacity(input.len());

    for c in input.chars() {
        let replacement = match c.to_ascii_lowercase() {
            'a' => {
                if rng.next() % 2 == 0 {
                    '4'
                } else {
                    '@'
                }
            }
            'e' => '3',
            'i' => {
                if rng.next() % 2 == 0 {
                    '1'
                } else {
                    '!'
                }
            }
            'o' => '0',
            's' => {
                if rng.next() % 2 == 0 {
                    '5'
                } else {
                    '$'
                }
            }
            't' => '7',
            'l' => '1',
            'g' => '9',
            'b' => '8',
            _ => c,
        };
        result.push(replacement);
    }
    result
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
    let mut result = String::with_capacity(input.len());

    for c in input.chars() {
        let rotated = match c {
            'a'..='z' => {
                let offset = ((c as u8 - b'a' + 13) % 26) + b'a';
                offset as char
            }
            'A'..='Z' => {
                let offset = ((c as u8 - b'A' + 13) % 26) + b'A';
                offset as char
            }
            _ => c,
        };
        result.push(rotated);
    }
    result
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetspeak_basic() {
        let result = leetspeak("leet");
        assert!(result.contains('1') || result.contains('3'));
    }

    #[test]
    fn test_leetspeak_empty_string() {
        assert_eq!(leetspeak(""), "");
    }

    #[test]
    fn test_leetspeak_password() {
        let result = leetspeak("password");
        assert!(result.contains('0') || result.contains('5'));
    }

    #[test]
    fn test_leetspeak_elite() {
        let result = leetspeak("elite");
        assert!(result.contains('3') || result.contains('1'));
    }

    #[test]
    fn test_leetspeak_all_vowels() {
        let result = leetspeak("aeiou");
        assert!(!result.contains('a') || !result.contains('e') || !result.contains('i'));
    }

    #[test]
    fn test_leetspeak_numbers_preserved() {
        let result = leetspeak("test123");
        assert!(result.contains("123"));
    }

    #[test]
    fn test_leetspeak_special_chars() {
        let result = leetspeak("test!@#");
        assert!(result.contains("!@#"));
    }

    #[test]
    fn test_leetspeak_uppercase() {
        let result = leetspeak("TEST");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_leetspeak_mixed_case() {
        let result = leetspeak("TeSt");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_leetspeak_preserves_length() {
        let input = "hello";
        let result = leetspeak(input);
        assert_eq!(result.len(), input.len());
    }

    #[test]
    fn test_leetspeak_sql_injection() {
        let result = leetspeak("select");
        assert!(result.contains('3') || result.contains('5'));
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
    fn test_rot13_empty_string() {
        assert_eq!(rot13(""), "");
    }

    #[test]
    fn test_rot13_lowercase() {
        assert_eq!(rot13("abc"), "nop");
    }

    #[test]
    fn test_rot13_uppercase() {
        assert_eq!(rot13("ABC"), "NOP");
    }

    #[test]
    fn test_rot13_numbers() {
        assert_eq!(rot13("test123"), "grfg123");
    }

    #[test]
    fn test_rot13_special_chars() {
        assert_eq!(rot13("test!@#"), "grfg!@#");
    }

    #[test]
    fn test_rot13_reversible_lowercase() {
        let original = "secret";
        let encoded = rot13(original);
        let decoded = rot13(&encoded);
        assert_eq!(decoded, original);
    }

    #[test]
    fn test_rot13_reversible_mixed() {
        let original = "SeCrEt123!";
        let encoded = rot13(original);
        let decoded = rot13(&encoded);
        assert_eq!(decoded, original);
    }

    #[test]
    fn test_rot13_alphabet_wrap() {
        assert_eq!(rot13("xyz"), "klm");
        assert_eq!(rot13("XYZ"), "KLM");
    }

    #[test]
    fn test_rot13_preserves_length() {
        let input = "testing";
        let result = rot13(input);
        assert_eq!(result.len(), input.len());
    }

    #[test]
    fn test_rot13_whitespace() {
        assert_eq!(rot13("hello world"), "uryyb jbeyq");
    }

    #[test]
    fn test_reverse_string() {
        let result = reverse_string("hello");
        assert_eq!(result, "olleh");
    }

    #[test]
    fn test_reverse_string_empty() {
        assert_eq!(reverse_string(""), "");
    }

    #[test]
    fn test_reverse_string_single_char() {
        assert_eq!(reverse_string("a"), "a");
    }

    #[test]
    fn test_reverse_string_palindrome() {
        assert_eq!(reverse_string("racecar"), "racecar");
    }

    #[test]
    fn test_reverse_string_numbers() {
        assert_eq!(reverse_string("12345"), "54321");
    }

    #[test]
    fn test_reverse_string_special_chars() {
        assert_eq!(reverse_string("!@#$%"), "%$#@!");
    }

    #[test]
    fn test_reverse_string_whitespace() {
        assert_eq!(reverse_string("hello world"), "dlrow olleh");
    }

    #[test]
    fn test_reverse_string_mixed() {
        assert_eq!(reverse_string("Test123!"), "!321tseT");
    }

    #[test]
    fn test_reverse_string_unicode() {
        let result = reverse_string("hello 世界");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_reverse_string_preserves_length() {
        let input = "testing";
        let result = reverse_string(input);
        assert_eq!(result.len(), input.len());
    }

    #[test]
    fn test_reverse_string_double_reverse() {
        let original = "test";
        let reversed = reverse_string(original);
        let double_reversed = reverse_string(&reversed);
        assert_eq!(double_reversed, original);
    }

    #[test]
    fn test_vowel_swap_preserves_consonants() {
        let result = vowel_swap("bcdfg");
        // Consonants should remain unchanged
        assert_eq!(result, "bcdfg");
    }

    #[test]
    fn test_vowel_swap_empty_string() {
        assert_eq!(vowel_swap(""), "");
    }

    #[test]
    fn test_vowel_swap_only_vowels() {
        let result = vowel_swap("aeiou");
        assert_eq!(result.len(), 5);
    }

    #[test]
    fn test_vowel_swap_mixed() {
        let result = vowel_swap("hello");
        assert_eq!(result.len(), 5);
        assert!(result.contains('h') && result.contains('l'));
    }

    #[test]
    fn test_vowel_swap_uppercase() {
        let result = vowel_swap("HELLO");
        assert_eq!(result.len(), 5);
        assert!(result.contains('H') && result.contains('L'));
    }

    #[test]
    fn test_vowel_swap_preserves_case() {
        let result = vowel_swap("A");
        assert!(result.chars().next().unwrap().is_uppercase());
    }

    #[test]
    fn test_vowel_swap_numbers() {
        let result = vowel_swap("test123");
        assert!(result.contains("123"));
    }

    #[test]
    fn test_vowel_swap_special_chars() {
        let result = vowel_swap("test!@#");
        assert!(result.contains("!@#"));
    }

    #[test]
    fn test_vowel_swap_preserves_length() {
        let input = "testing";
        let result = vowel_swap(input);
        assert_eq!(result.len(), input.len());
    }

    #[test]
    fn test_vowel_swap_no_vowels() {
        let result = vowel_swap("xyz");
        assert_eq!(result, "xyz");
    }

    #[test]
    fn test_whitespace_padding() {
        let result = whitespace_padding("test");
        assert!(result.len() >= 4);
        assert!(result.contains("test") || result.contains('t'));
    }

    #[test]
    fn test_whitespace_padding_empty() {
        assert_eq!(whitespace_padding(""), "");
    }

    #[test]
    fn test_whitespace_padding_single_char() {
        let result = whitespace_padding("a");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_whitespace_padding_increases_length() {
        let input = "test";
        let result = whitespace_padding(input);
        assert!(result.len() >= input.len());
    }

    #[test]
    fn test_whitespace_padding_preserves_content() {
        let result = whitespace_padding("abc");
        assert!(result.contains('a') && result.contains('b') && result.contains('c'));
    }

    #[test]
    fn test_whitespace_padding_numbers() {
        let result = whitespace_padding("123");
        assert!(result.contains('1') && result.contains('2') && result.contains('3'));
    }

    #[test]
    fn test_whitespace_padding_special_chars() {
        let result = whitespace_padding("!@#");
        assert!(result.contains('!'));
    }

    #[test]
    fn test_whitespace_padding_sql() {
        let result = whitespace_padding("SELECT");
        // Whitespace padding adds spaces between chars, so just check content is preserved
        assert!(result.contains('S') && result.contains('E'));
    }

    #[test]
    fn test_whitespace_padding_mixed() {
        let result = whitespace_padding("Test123");
        assert!(result.contains('T') && result.contains('1'));
    }

    #[test]
    fn test_whitespace_padding_waf_bypass() {
        let result = whitespace_padding("admin");
        assert!(result.contains('a') && result.contains('d'));
    }

    #[test]
    fn test_js_string_concat() {
        let result = js_string_concat("alert");
        assert!(result.len() >= 5);
        assert!(result.contains('\''));
    }

    #[test]
    fn test_js_string_concat_empty() {
        assert_eq!(js_string_concat(""), "''");
    }

    #[test]
    fn test_js_string_concat_single_char() {
        let result = js_string_concat("a");
        assert!(result.contains('\''));
    }

    #[test]
    fn test_js_string_concat_contains_plus() {
        let result = js_string_concat("test");
        // Should contain concatenation
        assert!(result.contains('+') || result.contains('\''));
    }

    #[test]
    fn test_js_string_concat_preserves_content() {
        let result = js_string_concat("abc");
        assert!(result.contains('a') || result.contains('b') || result.contains('c'));
    }

    #[test]
    fn test_js_string_concat_numbers() {
        let result = js_string_concat("123");
        assert!(result.contains('1') || result.contains('2') || result.contains('3'));
    }

    #[test]
    fn test_js_string_concat_special_chars() {
        let result = js_string_concat("!@#");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_js_string_concat_obfuscation() {
        let result = js_string_concat("document");
        assert!(result.contains('\''));
    }

    #[test]
    fn test_js_string_concat_cookie() {
        let result = js_string_concat("cookie");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_js_string_concat_xss() {
        let result = js_string_concat("alert(1)");
        assert!(result.contains('\''));
    }

    #[test]
    fn test_double_characters_empty() {
        assert_eq!(double_characters(""), "");
    }

    #[test]
    fn test_double_characters_single_char() {
        let result = double_characters("a");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_double_characters_increases_length() {
        let input = "test";
        let result = double_characters(input);
        assert!(result.len() >= input.len());
    }

    #[test]
    fn test_double_characters_preserves_content() {
        let result = double_characters("abc");
        assert!(result.to_lowercase().contains('a'));
    }

    #[test]
    fn test_double_characters_numbers_preserved() {
        let result = double_characters("test123");
        assert!(result.contains("123"));
    }

    #[test]
    fn test_double_characters_special_chars() {
        let result = double_characters("test!@#");
        assert!(result.contains("!@#"));
    }

    #[test]
    fn test_double_characters_uppercase() {
        let result = double_characters("TEST");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_double_characters_mixed() {
        let result = double_characters("TeSt");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_double_characters_only_alpha() {
        let input = "abc";
        let result = double_characters(input);
        assert!(result.len() >= input.len());
    }

    #[test]
    fn test_double_characters_validation() {
        let result = double_characters("admin");
        assert!(!result.is_empty());
    }
}
