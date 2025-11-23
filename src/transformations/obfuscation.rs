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
    fn test_rot13() {
        let result = rot13("Hello World");
        assert_eq!(result, "Uryyb Jbeyq");

        // ROT13 is reversible
        let reversed = rot13(&result);
        assert_eq!(reversed, "Hello World");
    }

    #[test]
    fn test_reverse_string() {
        let result = reverse_string("hello");
        assert_eq!(result, "olleh");
    }

    #[test]
    fn test_vowel_swap_preserves_consonants() {
        let result = vowel_swap("bcdfg");
        // Consonants should remain unchanged
        assert_eq!(result, "bcdfg");
    }

    #[test]
    fn test_whitespace_padding() {
        let result = whitespace_padding("test");
        assert!(result.len() >= 4);
        assert!(result.contains("test") || result.contains('t'));
    }

    #[test]
    fn test_js_string_concat() {
        let result = js_string_concat("alert");
        assert!(result.len() >= 5);
        assert!(result.contains('\''));
    }
}
