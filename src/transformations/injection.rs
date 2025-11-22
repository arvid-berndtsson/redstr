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

/// Generates MongoDB injection patterns for NoSQL injection testing.
///
/// Useful for red team NoSQL injection testing and blue team input validation.
///
/// # Examples
///
/// ```
/// use redstr::mongodb_injection;
/// let query = r#"{"username": "admin", "password": "secret"}"#;
/// let result = mongodb_injection(query);
/// assert!(result.len() > 0);
/// ```
pub fn mongodb_injection(query: &str) -> String {
    let mut rng = SimpleRng::new();
    let mut result = query.to_string();

    // Common MongoDB injection patterns
    let operators = ["$ne", "$gt", "$lt", "$gte", "$lte", "$in", "$nin", "$regex"];

    // Inject MongoDB operators
    if result.contains("password") || result.contains("username") {
        match rng.next() % 4 {
            0 => {
                // Not equal operator
                result = result.replace(r#""password": "#, r#""password": {"$ne": "#);
                if result.contains(r#"{"$ne": "#) && !result.contains("}") {
                    result.push('}');
                }
            }
            1 => {
                // Regex operator
                result = result.replace(r#""password": "#, r#""password": {"$regex": "#);
                if result.contains(r#"{"$regex": "#) && !result.contains("}") {
                    result.push('}');
                }
            }
            2 => {
                // Greater than operator
                result = result.replace(r#""username": "#, r#""username": {"$gt": "#);
                if result.contains(r#"{"$gt": "#) && !result.contains("}") {
                    result.push('}');
                }
            }
            _ => {
                // In operator
                result = result.replace(r#""username": "#, r#""username": {"$in": ["#);
                if result.contains(r#"{"$in": ["#) && !result.contains("]") {
                    result.push(']');
                }
            }
        }
    } else {
        // Generic operator injection
        let op = operators[rng.next() as usize % operators.len()];
        if result.contains(':') {
            let replacement = format!(": {{\"{}\": ", op);
            result = result.replace(":", &replacement);
            if !result.contains('}') {
                result.push('}');
            }
        }
    }

    result
}

/// Generates CouchDB injection patterns for NoSQL injection testing.
///
/// Useful for red team NoSQL injection testing and blue team input validation.
///
/// # Examples
///
/// ```
/// use redstr::couchdb_injection;
/// let query = r#"{"selector": {"name": "admin"}}"#;
/// let result = couchdb_injection(query);
/// assert!(result.len() > 0);
/// ```
pub fn couchdb_injection(query: &str) -> String {
    let mut rng = SimpleRng::new();
    let mut result = query.to_string();

    // Common CouchDB injection patterns
    match rng.next() % 3 {
        0 => {
            // Inject $or operator
            if result.contains("selector") {
                result = result.replace(r#""selector": {"#, r#""selector": {"$or": [{"#);
                if result.contains(r#"{"$or": [{"#) && !result.contains("]") {
                    result = result.replace("}", "}]}");
                }
            }
        }
        1 => {
            // Inject $regex operator
            if result.contains(":") {
                result = result.replace(":", r#": {"$regex": "#);
                if result.contains(r#"{"$regex": "#) && !result.contains("}") {
                    result.push('}');
                }
            }
        }
        _ => {
            // Inject $exists operator
            if result.contains(":") {
                result = result.replace(":", r#": {"$exists": true, "#);
                if result.contains(r#"{"$exists": true, "#) && !result.contains("}") {
                    result.push('}');
                }
            }
        }
    }

    result
}

/// Generates DynamoDB query obfuscation patterns for NoSQL injection testing.
///
/// Useful for red team NoSQL injection testing and blue team input validation.
///
/// # Examples
///
/// ```
/// use redstr::dynamodb_obfuscate;
/// let query = r#"{"Key": {"id": {"S": "123"}}}"#;
/// let result = dynamodb_obfuscate(query);
/// assert!(result.len() > 0);
/// ```
pub fn dynamodb_obfuscate(query: &str) -> String {
    let mut rng = SimpleRng::new();
    let mut result = query.to_string();

    // Common DynamoDB obfuscation patterns
    match rng.next() % 3 {
        0 => {
            // Change attribute type
            result = result.replace(r#"{"S": "#, r#"{"N": "#);
            result = result.replace(r#"{"N": "#, r#"{"S": "#);
        }
        1 => {
            // Add extra attributes
            if result.contains("Key") {
                result = result.replace(r#""Key": {"#, r#""Key": {"extra": {"S": "value"}, "#);
            }
        }
        _ => {
            // Modify comparison operators
            result = result.replace("=", "!=");
            result = result.replace("!=", "=");
        }
    }

    result
}

/// Generates NoSQL operator injection patterns for NoSQL injection testing.
///
/// Useful for red team NoSQL injection testing and blue team input validation.
///
/// # Examples
///
/// ```
/// use redstr::nosql_operator_injection;
/// let query = r#"{"username": "admin"}"#;
/// let result = nosql_operator_injection(query);
/// assert!(result.len() > 0);
/// ```
pub fn nosql_operator_injection(query: &str) -> String {
    let mut rng = SimpleRng::new();
    let mut result = query.to_string();

    // Common NoSQL operators
    let operators = [
        ("$ne", "not equal"),
        ("$gt", "greater than"),
        ("$lt", "less than"),
        ("$gte", "greater than or equal"),
        ("$lte", "less than or equal"),
        ("$in", "in array"),
        ("$nin", "not in array"),
        ("$regex", "regex match"),
        ("$exists", "field exists"),
        ("$or", "logical or"),
        ("$and", "logical and"),
        ("$nor", "logical nor"),
    ];

    // Inject operator based on query structure
    if result.contains(':') {
        let (op_name, _) = operators[rng.next() as usize % operators.len()];
        match rng.next() % 3 {
            0 => {
                // Replace value with operator
                if let Some(pos) = result.find(':') {
                    let (before, after) = result.split_at(pos + 1);
                    let after_trimmed = after.trim_start();
                    if after_trimmed.starts_with('"') {
                        result = format!(
                            r#"{} {{"{}": {}}}"#,
                            before.trim_end(),
                            op_name,
                            after_trimmed
                        );
                    }
                }
            }
            1 => {
                // Add operator to existing field
                let replacement = format!(": {{\"{}\": ", op_name);
                result = result.replace(":", &replacement);
                if !result.contains("}") {
                    result.push('}');
                }
            }
            _ => {
                // Wrap in $or
                result = format!(r#"{{"$or": [{}]}}"#, result);
            }
        }
    }

    result
}

/// Generates Server-Side Template Injection (SSTI) patterns for template injection testing.
///
/// Useful for red team SSTI testing and blue team template validation.
///
/// # Examples
///
/// ```
/// use redstr::ssti_injection;
/// let template = "Hello {{ name }}";
/// let result = ssti_injection(template);
/// assert!(result.len() > 0);
/// ```
pub fn ssti_injection(template: &str) -> String {
    let mut rng = SimpleRng::new();
    let mut result = template.to_string();

    // Common SSTI patterns for various template engines
    let jinja2_patterns = [
        "{{ config }}",
        "{{ self.__dict__ }}",
        "{{ ''.__class__.__mro__[2].__subclasses__() }}",
        "{{ request }}",
    ];

    let freemarker_patterns = ["${.vars}", "${.data_model}", "${.main}", "${.namespace}"];

    let velocity_patterns = ["$class", "$class.inspect", "$class.type"];

    // Detect template engine and inject appropriate pattern
    if result.contains("{{") || result.contains("}}") {
        // Jinja2/Handlebars style
        let pattern = jinja2_patterns[rng.next() as usize % jinja2_patterns.len()];
        if result.contains("{{") {
            result = result.replace("{{", &format!("{}{{", pattern));
        } else {
            result = format!("{}{}", result, pattern);
        }
    } else if result.contains("${") {
        // Freemarker/Velocity style
        let pattern = if rng.next() % 2 == 0 {
            freemarker_patterns[rng.next() as usize % freemarker_patterns.len()]
        } else {
            velocity_patterns[rng.next() as usize % velocity_patterns.len()]
        };
        if result.contains("${") {
            result = result.replace("${", &format!("{}${{", pattern));
        } else {
            result = format!("{}{}", result, pattern);
        }
    } else {
        // Generic injection
        let generic_patterns = ["{{7*7}}", "${7*7}", "#{7*7}", "${@print(md5(31337))}"];
        let pattern = generic_patterns[rng.next() as usize % generic_patterns.len()];
        result = format!("{}{}", result, pattern);
    }

    result
}

/// Generates framework-specific SSTI variations for template injection testing.
///
/// Useful for red team SSTI testing and blue team template validation.
///
/// # Examples
///
/// ```
/// use redstr::ssti_framework_variation;
/// let template = "Hello {{ name }}";
/// let result = ssti_framework_variation(template, "jinja2");
/// assert!(result.len() > 0);
/// ```
pub fn ssti_framework_variation(template: &str, framework: &str) -> String {
    let mut rng = SimpleRng::new();
    let mut result = template.to_string();

    match framework.to_lowercase().as_str() {
        "jinja2" | "jinja" => {
            let patterns = [
                "{{ config.items() }}",
                "{{ request.application.__globals__ }}",
                "{{ ''.__class__.__mro__[1].__subclasses__() }}",
            ];
            let pattern = patterns[rng.next() as usize % patterns.len()];
            result = format!("{}{}", result, pattern);
        }
        "freemarker" => {
            let patterns = ["${.vars}", "${.data_model}", "${.main}"];
            let pattern = patterns[rng.next() as usize % patterns.len()];
            result = format!("{}{}", result, pattern);
        }
        "velocity" => {
            let patterns = ["$class.inspect", "$class.type", "$class.forName"];
            let pattern = patterns[rng.next() as usize % patterns.len()];
            result = format!("{}{}", result, pattern);
        }
        "twig" => {
            let patterns = ["{{ _self }}", "{{ _self.env }}", "{{ dump(_self) }}"];
            let pattern = patterns[rng.next() as usize % patterns.len()];
            result = format!("{}{}", result, pattern);
        }
        _ => {
            // Generic pattern
            result = format!("{}{{7*7}}", result);
        }
    }

    result
}

/// Generates template syntax obfuscation for SSTI testing.
///
/// Useful for red team SSTI obfuscation and blue team template parsing validation.
///
/// # Examples
///
/// ```
/// use redstr::ssti_syntax_obfuscate;
/// let template = "{{ name }}";
/// let result = ssti_syntax_obfuscate(template);
/// assert!(result.len() > 0);
/// ```
pub fn ssti_syntax_obfuscate(template: &str) -> String {
    let mut rng = SimpleRng::new();
    let mut result = template.to_string();

    // Obfuscate template syntax
    match rng.next() % 4 {
        0 => {
            // Add whitespace variations
            result = result.replace("{{", "{ {");
            result = result.replace("}}", "} }");
        }
        1 => {
            // Use different delimiters
            result = result.replace("{{", "${");
            result = result.replace("}}", "}");
        }
        2 => {
            // Add comments
            result = result.replace("{{", "{# comment #}{{");
        }
        _ => {
            // Case variation in keywords
            result = result.replace("name", "Name");
            result = result.replace("Name", "NAME");
        }
    }

    result
}

#[cfg(test)]
mod nosql_ssti_tests {
    use super::*;

    #[test]
    fn test_mongodb_injection() {
        let query = r#"{"username": "admin", "password": "secret"}"#;
        let result = mongodb_injection(query);
        assert!(!result.is_empty());
        assert!(result.contains("username") || result.contains("password"));
    }

    #[test]
    fn test_couchdb_injection() {
        let query = r#"{"selector": {"name": "admin"}}"#;
        let result = couchdb_injection(query);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_dynamodb_obfuscate() {
        let query = r#"{"Key": {"id": {"S": "123"}}}"#;
        let result = dynamodb_obfuscate(query);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_nosql_operator_injection() {
        let query = r#"{"username": "admin"}"#;
        let result = nosql_operator_injection(query);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_ssti_injection() {
        let template = "Hello {{ name }}";
        let result = ssti_injection(template);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_ssti_framework_variation() {
        let template = "Hello {{ name }}";
        let result = ssti_framework_variation(template, "jinja2");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_ssti_syntax_obfuscate() {
        let template = "{{ name }}";
        let result = ssti_syntax_obfuscate(template);
        assert!(!result.is_empty());
    }
}
