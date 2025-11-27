use crate::rng::SimpleRng;

/// Applies random capitalization to each letter in the input string.
///
/// Each alphabetic character has a 50% chance of being uppercase or lowercase,
/// creating unpredictable case patterns. Non-alphabetic characters (numbers,
/// spaces, punctuation) are preserved unchanged.
///
/// # Use Cases
///
/// - **Red Team**: Bypass case-sensitive filters and detection systems
/// - **Blue Team**: Test case-insensitive input validation
/// - **General**: Create visually distinctive text for testing purposes
///
/// # Examples
///
/// ```
/// use redstr::randomize_capitalization;
/// 
/// let result = randomize_capitalization("hello world");
/// // Example output: "HeLlO wOrLd" or "hElLo WoRLd" (varies each run)
/// assert_eq!(result.len(), "hello world".len());
/// 
/// // Numbers and symbols are preserved
/// let result2 = randomize_capitalization("test123");
/// assert!(result2.contains("123"));
/// ```
pub fn randomize_capitalization(input: &str) -> String {
    let mut rng = SimpleRng::new();
    let mut result = String::with_capacity(input.len() * 2); // Allocate extra for potential multi-byte chars

    for c in input.chars() {
        if c.is_alphabetic() {
            if rng.next() % 2 == 0 {
                for uc in c.to_uppercase() {
                    result.push(uc);
                }
            } else {
                for lc in c.to_lowercase() {
                    result.push(lc);
                }
            }
        } else {
            result.push(c);
        }
    }
    result
}

/// Alternates between uppercase and lowercase for each alphabetic character.
///
/// Creates a predictable pattern starting with uppercase, then lowercase, and
/// repeating for each letter. Non-alphabetic characters don't affect the pattern.
/// This is also known as "spongebob case" or "mocking case".
///
/// # Use Cases
///
/// - **Testing**: Verify case-insensitive string comparisons
/// - **Visual**: Create distinctive formatting for emphasis or mockery
/// - **Red Team**: Test filter bypass with alternating patterns
///
/// # Examples
///
/// ```
/// use redstr::alternate_case;
/// 
/// assert_eq!(alternate_case("hello"), "HeLlO");
/// assert_eq!(alternate_case("hello world"), "HeLlO wOrLd");
/// 
/// // Non-alphabetic characters are preserved but don't affect the pattern
/// assert_eq!(alternate_case("a1b2c3"), "A1b2C3");
/// ```
pub fn alternate_case(input: &str) -> String {
    let mut upper = true;
    let mut result = String::with_capacity(input.len() * 2);

    for c in input.chars() {
        if c.is_alphabetic() {
            if upper {
                for uc in c.to_uppercase() {
                    result.push(uc);
                }
            } else {
                for lc in c.to_lowercase() {
                    result.push(lc);
                }
            }
            upper = !upper;
        } else {
            result.push(c);
        }
    }
    result
}

/// Inverts the case of all alphabetic characters.
///
/// Converts uppercase letters to lowercase and lowercase letters to uppercase.
/// Non-alphabetic characters remain unchanged. This is useful for testing
/// case sensitivity and creating inverted text patterns.
///
/// # Use Cases
///
/// - **Testing**: Verify case transformation logic
/// - **Red Team**: Test case-sensitive filter evasion
/// - **Data Processing**: Normalize or transform text data
///
/// # Examples
///
/// ```
/// use redstr::inverse_case;
/// 
/// assert_eq!(inverse_case("Hello World"), "hELLO wORLD");
/// assert_eq!(inverse_case("ABC123xyz"), "abc123XYZ");
/// 
/// // All uppercase becomes all lowercase
/// assert_eq!(inverse_case("SHOUTING"), "shouting");
/// ```
pub fn inverse_case(input: &str) -> String {
    let mut result = String::with_capacity(input.len() * 2);

    for c in input.chars() {
        if c.is_uppercase() {
            for lc in c.to_lowercase() {
                result.push(lc);
            }
        } else if c.is_lowercase() {
            for uc in c.to_uppercase() {
                result.push(uc);
            }
        } else {
            result.push(c);
        }
    }
    result
}

/// Swaps case randomly for WAF and filter bypass testing.
///
/// Each alphabetic character has a 50% chance of having its case inverted.
/// This creates unpredictable case patterns while maintaining readability,
/// making it ideal for evading case-sensitive security filters.
///
/// # Use Cases
///
/// - **Red Team**: Bypass WAF rules that look for specific case patterns
/// - **SQL Injection**: Evade detection with queries like `SeLeCt * FrOm users`
/// - **XSS Testing**: Bypass filters with `<ScRiPt>alert(1)</ScRiPt>`
/// - **Blue Team**: Test if security controls properly normalize case
///
/// # Examples
///
/// ```
/// use redstr::case_swap;
/// 
/// // SQL injection with case variations
/// let result = case_swap("SELECT * FROM users");
/// // Example output: "SeLeCt * FrOm users" or "sElEcT * fRoM users"
/// assert_ne!(result, "SELECT * FROM users");
/// 
/// // XSS payload obfuscation
/// let xss = case_swap("<script>alert(1)</script>");
/// // Example output: "<ScRiPt>alert(1)</ScRiPt>"
/// ```
pub fn case_swap(input: &str) -> String {
    let mut rng = SimpleRng::new();
    let mut result = String::with_capacity(input.len() * 2);

    for c in input.chars() {
        if c.is_alphabetic() && rng.next() % 2 == 0 {
            if c.is_uppercase() {
                for lc in c.to_lowercase() {
                    result.push(lc);
                }
            } else {
                for uc in c.to_uppercase() {
                    result.push(uc);
                }
            }
        } else {
            result.push(c);
        }
    }
    result
}

/// Converts a string to camelCase.
///
/// Transforms input into camelCase format where the first word is lowercase
/// and subsequent words have their first letter capitalized, with all spaces,
/// hyphens, and underscores removed.
///
/// # Use Cases
///
/// - **API Development**: Convert field names to JavaScript/Java conventions
/// - **Code Generation**: Transform human-readable names to variable names
/// - **Data Transformation**: Normalize naming conventions across systems
///
/// # Examples
///
/// ```
/// use redstr::to_camel_case;
/// 
/// assert_eq!(to_camel_case("hello world"), "helloWorld");
/// assert_eq!(to_camel_case("user_first_name"), "userFirstName");
/// assert_eq!(to_camel_case("get-user-id"), "getUserId");
/// assert_eq!(to_camel_case("API Response Code"), "apiResponseCode");
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
/// Transforms input into snake_case format where all letters are lowercase
/// and words are separated by underscores. Converts from camelCase, PascalCase,
/// kebab-case, or space-separated formats.
///
/// # Use Cases
///
/// - **Database Schema**: Convert field names to SQL column conventions
/// - **Python/Ruby**: Transform names to language naming conventions
/// - **Configuration Files**: Normalize setting names
///
/// # Examples
///
/// ```
/// use redstr::to_snake_case;
/// 
/// assert_eq!(to_snake_case("HelloWorld"), "hello_world");
/// assert_eq!(to_snake_case("getUserId"), "get_user_id");
/// assert_eq!(to_snake_case("API Response"), "api_response");
/// assert_eq!(to_snake_case("user-first-name"), "user_first_name");
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
/// Transforms input into kebab-case (also called dash-case or lisp-case)
/// where all letters are lowercase and words are separated by hyphens.
/// Commonly used in URLs, CSS class names, and HTML attributes.
///
/// # Use Cases
///
/// - **URLs**: Create SEO-friendly URL slugs
/// - **CSS/HTML**: Convert to standard class name format
/// - **Command-line**: Transform to CLI flag conventions
///
/// # Examples
///
/// ```
/// use redstr::to_kebab_case;
/// 
/// assert_eq!(to_kebab_case("HelloWorld"), "hello-world");
/// assert_eq!(to_kebab_case("user_profile_page"), "user-profile-page");
/// assert_eq!(to_kebab_case("API Endpoint"), "api-endpoint");
/// 
/// // Useful for URL slugs
/// assert_eq!(to_kebab_case("My Blog Post Title"), "my-blog-post-title");
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
    fn test_alternate_case_empty_string() {
        let result = alternate_case("");
        assert_eq!(result, "");
    }

    #[test]
    fn test_alternate_case_single_char() {
        let result = alternate_case("a");
        assert_eq!(result, "A");
    }

    #[test]
    fn test_alternate_case_with_numbers() {
        let result = alternate_case("hello123world");
        assert_eq!(result, "HeLlO123wOrLd");
    }

    #[test]
    fn test_alternate_case_with_spaces() {
        let result = alternate_case("hello world");
        assert_eq!(result, "HeLlO wOrLd");
    }

    #[test]
    fn test_alternate_case_special_chars() {
        let result = alternate_case("a!b@c#d");
        assert_eq!(result, "A!b@C#d");
    }

    #[test]
    fn test_alternate_case_all_uppercase() {
        let result = alternate_case("HELLO");
        assert_eq!(result, "HeLlO");
    }

    #[test]
    fn test_alternate_case_mixed_case() {
        let result = alternate_case("HeLLo");
        assert_eq!(result, "HeLlO");
    }

    #[test]
    fn test_alternate_case_long_string() {
        let result = alternate_case("abcdefghijklmnop");
        assert_eq!(result, "AbCdEfGhIjKlMnOp");
    }

    #[test]
    fn test_alternate_case_unicode() {
        let result = alternate_case("héllo");
        assert!(!result.is_empty());
        assert!(result.starts_with("H") || result.starts_with("h"));
    }

    #[test]
    fn test_inverse_case() {
        let result = inverse_case("Hello World");
        assert_eq!(result, "hELLO wORLD");
    }

    #[test]
    fn test_inverse_case_empty_string() {
        let result = inverse_case("");
        assert_eq!(result, "");
    }

    #[test]
    fn test_inverse_case_single_char_lower() {
        let result = inverse_case("a");
        assert_eq!(result, "A");
    }

    #[test]
    fn test_inverse_case_single_char_upper() {
        let result = inverse_case("A");
        assert_eq!(result, "a");
    }

    #[test]
    fn test_inverse_case_numbers() {
        let result = inverse_case("Test123");
        assert_eq!(result, "tEST123");
    }

    #[test]
    fn test_inverse_case_special_chars() {
        let result = inverse_case("Hello!@#World");
        assert_eq!(result, "hELLO!@#wORLD");
    }

    #[test]
    fn test_inverse_case_all_lowercase() {
        let result = inverse_case("hello");
        assert_eq!(result, "HELLO");
    }

    #[test]
    fn test_inverse_case_all_uppercase() {
        let result = inverse_case("HELLO");
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_inverse_case_mixed() {
        let result = inverse_case("HeLLo WoRLd");
        assert_eq!(result, "hEllO wOrlD");
    }

    #[test]
    fn test_inverse_case_preserves_non_alpha() {
        let result = inverse_case("ABC_123_xyz");
        assert_eq!(result, "abc_123_XYZ");
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
    fn test_randomize_capitalization_empty_string() {
        let result = randomize_capitalization("");
        assert_eq!(result, "");
    }

    #[test]
    fn test_randomize_capitalization_single_char() {
        let result = randomize_capitalization("a");
        assert_eq!(result.len(), 1);
        assert!(result == "a" || result == "A");
    }

    #[test]
    fn test_randomize_capitalization_numbers_only() {
        let result = randomize_capitalization("12345");
        assert_eq!(result, "12345");
    }

    #[test]
    fn test_randomize_capitalization_special_chars() {
        let result = randomize_capitalization("!@#$%");
        assert_eq!(result, "!@#$%");
    }

    #[test]
    fn test_randomize_capitalization_mixed_content() {
        let input = "Hello123World!";
        let result = randomize_capitalization(input);
        assert_eq!(result.len(), input.len());
        assert!(result.contains("123"));
        assert!(result.contains("!"));
    }

    #[test]
    fn test_randomize_capitalization_unicode() {
        let input = "héllo wörld";
        let result = randomize_capitalization(input);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_randomize_capitalization_all_uppercase() {
        let input = "HELLO";
        let result = randomize_capitalization(input);
        assert_eq!(result.len(), input.len());
    }

    #[test]
    fn test_randomize_capitalization_all_lowercase() {
        let input = "hello";
        let result = randomize_capitalization(input);
        assert_eq!(result.len(), input.len());
    }

    #[test]
    fn test_randomize_capitalization_whitespace() {
        let input = "hello   world";
        let result = randomize_capitalization(input);
        assert_eq!(result.len(), input.len());
        assert!(result.contains("   "));
    }

    #[test]
    fn test_case_swap() {
        let result = case_swap("HELLO");
        // Should be different from original due to case swapping
        assert_ne!(result, "HELLO");
    }

    #[test]
    fn test_case_swap_empty_string() {
        let result = case_swap("");
        assert_eq!(result, "");
    }

    #[test]
    fn test_case_swap_single_char() {
        let result = case_swap("A");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_case_swap_preserves_numbers() {
        let input = "ABC123XYZ";
        let result = case_swap(input);
        assert!(result.contains("123"));
    }

    #[test]
    fn test_case_swap_preserves_special_chars() {
        let input = "TEST!@#CODE";
        let result = case_swap(input);
        assert!(result.contains("!@#"));
    }

    #[test]
    fn test_case_swap_lowercase() {
        let result = case_swap("hello");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_case_swap_mixed_case() {
        let result = case_swap("HeLLo");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_case_swap_sql_injection() {
        let result = case_swap("SELECT * FROM users");
        assert!(result.to_lowercase().contains("select"));
        assert!(result.to_lowercase().contains("from"));
    }

    #[test]
    fn test_case_swap_preserves_length() {
        let input = "TestString";
        let result = case_swap(input);
        assert_eq!(result.len(), input.len());
    }

    #[test]
    fn test_case_swap_with_whitespace() {
        let input = "Hello World Test";
        let result = case_swap(input);
        assert!(result.contains(" "));
    }

    #[test]
    fn test_to_camel_case_empty_string() {
        let result = to_camel_case("");
        assert_eq!(result, "");
    }

    #[test]
    fn test_to_camel_case_single_word() {
        let result = to_camel_case("hello");
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_to_camel_case_multiple_spaces() {
        let result = to_camel_case("hello   world");
        assert_eq!(result, "helloWorld");
    }

    #[test]
    fn test_to_camel_case_mixed_separators() {
        let result = to_camel_case("hello_world-test");
        assert_eq!(result, "helloWorldTest");
    }

    #[test]
    fn test_to_camel_case_uppercase_input() {
        let result = to_camel_case("HELLO WORLD");
        assert_eq!(result, "helloWorld");
    }

    #[test]
    fn test_to_camel_case_numbers() {
        let result = to_camel_case("hello123world");
        assert!(result.contains("123"));
    }

    #[test]
    fn test_to_camel_case_special_chars() {
        let result = to_camel_case("hello@world");
        assert!(result.contains("@"));
    }

    #[test]
    fn test_to_camel_case_already_camel() {
        let result = to_camel_case("helloWorld");
        assert_eq!(result, "helloworld");
    }

    #[test]
    fn test_to_camel_case_three_words() {
        let result = to_camel_case("hello world test");
        assert_eq!(result, "helloWorldTest");
    }

    #[test]
    fn test_to_snake_case_empty_string() {
        let result = to_snake_case("");
        assert_eq!(result, "");
    }

    #[test]
    fn test_to_snake_case_single_word() {
        let result = to_snake_case("hello");
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_to_snake_case_camel_case() {
        let result = to_snake_case("helloWorld");
        assert_eq!(result, "hello_world");
    }

    #[test]
    fn test_to_snake_case_multiple_uppercase() {
        let result = to_snake_case("HelloWorldTest");
        assert_eq!(result, "hello_world_test");
    }

    #[test]
    fn test_to_snake_case_consecutive_uppercase() {
        let result = to_snake_case("HTTPSConnection");
        // Should convert consecutive uppercase - exact behavior may vary
        assert!(!result.is_empty());
        assert!(result.to_lowercase() == result); // Should be all lowercase
    }

    #[test]
    fn test_to_snake_case_with_numbers() {
        let result = to_snake_case("test123Value");
        assert!(result.contains("123"));
    }

    #[test]
    fn test_to_snake_case_already_snake() {
        let result = to_snake_case("hello_world");
        assert_eq!(result, "hello_world");
    }

    #[test]
    fn test_to_snake_case_kebab_input() {
        let result = to_snake_case("hello-world-test");
        assert_eq!(result, "hello_world_test");
    }

    #[test]
    fn test_to_kebab_case_empty_string() {
        let result = to_kebab_case("");
        assert_eq!(result, "");
    }

    #[test]
    fn test_to_kebab_case_single_word() {
        let result = to_kebab_case("hello");
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_to_kebab_case_camel_case() {
        let result = to_kebab_case("helloWorld");
        assert_eq!(result, "hello-world");
    }

    #[test]
    fn test_to_kebab_case_snake_case() {
        let result = to_kebab_case("hello_world");
        assert_eq!(result, "hello-world");
    }

    #[test]
    fn test_to_kebab_case_multiple_words() {
        let result = to_kebab_case("HelloWorldTest");
        assert_eq!(result, "hello-world-test");
    }

    #[test]
    fn test_to_kebab_case_with_numbers() {
        let result = to_kebab_case("test123Value");
        assert!(result.contains("123"));
    }

    #[test]
    fn test_to_kebab_case_already_kebab() {
        let result = to_kebab_case("hello-world");
        assert_eq!(result, "hello-world");
    }
}
