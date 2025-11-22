use crate::rng::SimpleRng;
use crate::transformations::case::case_swap;

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

/// Generates HTTP/2 header order variations for Cloudflare bot detection evasion.
///
/// Cloudflare uses header order as a fingerprinting mechanism. This function
/// generates variations of common header combinations to evade detection.
///
/// Useful for red team bot detection evasion and blue team fingerprinting testing.
///
/// # Examples
///
/// ```
/// use redstr::http2_header_order;
/// let headers = "accept-language: en-US,en;q=0.9\naccept-encoding: gzip, deflate, br";
/// let result = http2_header_order(headers);
/// assert!(result.len() > 0);
/// ```
pub fn http2_header_order(input: &str) -> String {
    let mut rng = SimpleRng::new();
    let lines: Vec<&str> = input.lines().collect();

    if lines.is_empty() {
        return input.to_string();
    }

    // Common header orders that browsers use
    let header_orders = [
        vec![0, 1, 2, 3, 4], // Standard order
        vec![1, 0, 2, 3, 4], // Accept-encoding first
        vec![0, 2, 1, 3, 4], // Accept-language, user-agent, accept-encoding
        vec![2, 0, 1, 3, 4], // User-agent first
    ];

    let order = &header_orders[rng.next() as usize % header_orders.len()];
    let mut result = String::new();

    for (i, &idx) in order.iter().enumerate() {
        if idx < lines.len() {
            if i > 0 {
                result.push('\n');
            }
            result.push_str(lines[idx]);
        }
    }

    // Add any remaining headers not in the order
    for (i, line) in lines.iter().enumerate() {
        if !order.contains(&i) {
            result.push('\n');
            result.push_str(line);
        }
    }

    result
}

/// Generates TLS fingerprint variations for Cloudflare bot detection evasion.
///
/// Cloudflare analyzes TLS handshake characteristics. This function generates
/// variations of TLS-related strings that might be used in fingerprinting.
///
/// Useful for red team TLS fingerprinting evasion and blue team detection testing.
///
/// # Examples
///
/// ```
/// use redstr::tls_fingerprint_variation;
/// let cipher = "TLS_AES_256_GCM_SHA384";
/// let result = tls_fingerprint_variation(cipher);
/// assert!(result.len() > 0);
/// ```
pub fn tls_fingerprint_variation(input: &str) -> String {
    let mut rng = SimpleRng::new();

    // Add subtle variations that might affect fingerprinting
    input
        .chars()
        .map(|c| {
            match rng.next() % 10 {
                0..=7 => c.to_string(),
                8 => {
                    // Occasionally add a space or hyphen variation
                    if c == '_' && rng.next() % 2 == 0 {
                        "-".to_string()
                    } else {
                        c.to_string()
                    }
                }
                _ => {
                    // Case variation for some characters
                    if c.is_alphabetic() && rng.next() % 3 == 0 {
                        if c.is_uppercase() {
                            c.to_lowercase().to_string()
                        } else {
                            c.to_uppercase().to_string()
                        }
                    } else {
                        c.to_string()
                    }
                }
            }
        })
        .collect()
}

/// Generates Cloudflare challenge response variations.
///
/// Useful for testing Cloudflare challenge bypass techniques and bot detection evasion.
///
/// # Examples
///
/// ```
/// use redstr::cloudflare_challenge_variation;
/// let challenge = "cf_clearance=abc123";
/// let result = cloudflare_challenge_variation(challenge);
/// assert!(result.len() > 0);
/// ```
pub fn cloudflare_challenge_variation(input: &str) -> String {
    let mut rng = SimpleRng::new();

    // Add variations to challenge strings
    if input.contains("cf_clearance") || input.contains("__cf_bm") {
        // Vary the cookie format slightly
        input
            .chars()
            .map(|c| {
                if c == '=' && rng.next() % 3 == 0 {
                    " = ".to_string() // Add spaces around equals
                } else if c == '_' && rng.next() % 4 == 0 {
                    "-".to_string() // Replace underscore with hyphen
                } else {
                    c.to_string()
                }
            })
            .collect()
    } else {
        // For other challenge strings, apply case variations
        case_swap(input)
    }
}

/// Generates browser-like Accept-Language header variations.
///
/// Useful for Cloudflare bot detection evasion and browser fingerprinting testing.
///
/// # Examples
///
/// ```
/// use redstr::accept_language_variation;
/// let lang = "en-US,en;q=0.9";
/// let result = accept_language_variation(lang);
/// assert!(result.contains("en"));
/// ```
pub fn accept_language_variation(input: &str) -> String {
    let mut rng = SimpleRng::new();

    // Common language variations
    let variations = [
        "en-US,en;q=0.9",
        "en-US,en;q=0.9,fr;q=0.8",
        "en-US,en;q=0.9,de;q=0.8",
        "en-GB,en;q=0.9",
        "en-US,en;q=0.9,*;q=0.8",
    ];

    if rng.next() % 3 == 0 {
        variations[rng.next() as usize % variations.len()].to_string()
    } else {
        // Slight variation of input
        input
            .chars()
            .map(|c| {
                if c == ',' && rng.next() % 2 == 0 {
                    ", ".to_string() // Add space after comma
                } else {
                    c.to_string()
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_user_agent() {
        let ua = random_user_agent();
        assert!(ua.contains("Mozilla"));
        assert!(ua.len() > 50);
    }

    #[test]
    fn test_http2_header_order() {
        let headers = "accept-language: en-US\naccept-encoding: gzip\nuser-agent: test";
        let result = http2_header_order(headers);
        assert!(result.len() > 0);
        assert!(result.contains("accept"));
    }

    #[test]
    fn test_tls_fingerprint_variation() {
        let cipher = "TLS_AES_256_GCM_SHA384";
        let result = tls_fingerprint_variation(cipher);
        assert!(result.len() > 0);
        // Check case-insensitive since function can vary case
        assert!(result.to_uppercase().contains("TLS"));
    }

    #[test]
    fn test_cloudflare_challenge_variation() {
        let challenge = "cf_clearance=abc123";
        let result = cloudflare_challenge_variation(challenge);
        // Result should contain some form of the original challenge
        // (may be case-swapped or have variations, but should preserve key parts)
        assert!(
            result.to_lowercase().contains("cf_clearance")
                || result.to_lowercase().contains("cf-clearance")
        );
    }

    #[test]
    fn test_accept_language_variation() {
        let lang = "en-US,en;q=0.9";
        let result = accept_language_variation(lang);
        assert!(result.contains("en"));
    }
}
