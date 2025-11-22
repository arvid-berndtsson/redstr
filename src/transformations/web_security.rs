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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_header_variation() {
        let header = "application/json";
        let result = http_header_variation(header);
        assert!(result.len() > 0);
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
        assert!(result.len() > 0);
    }
}
