
use crate::rng::SimpleRng;

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
    fn test_case_swap() {
        let result = case_swap("HELLO");
        // Should be different from original due to case swapping
        assert_ne!(result, "HELLO");
    }
}
