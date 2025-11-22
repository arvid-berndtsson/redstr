use crate::rng::SimpleRng;

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
