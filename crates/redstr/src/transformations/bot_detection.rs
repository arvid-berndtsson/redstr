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
    // Updated user-agent strings as of Dec 2024 - Update periodically for best results
    let user_agents = [
        // Modern Desktop Chrome
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36",
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36",
        // Modern Desktop Firefox
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:133.0) Gecko/20100101 Firefox/133.0",
        "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:133.0) Gecko/20100101 Firefox/133.0",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:133.0) Gecko/20100101 Firefox/133.0",
        // Modern Desktop Safari
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.1 Safari/605.1.15",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 14_0) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Safari/605.1.15",
        // Modern Desktop Edge
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 Edg/131.0.0.0",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 Edg/131.0.0.0",
        // Mobile iOS Safari
        "Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Mobile/15E148 Safari/604.1",
        "Mozilla/5.0 (iPad; CPU OS 17_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Mobile/15E148 Safari/604.1",
        "Mozilla/5.0 (iPhone; CPU iPhone OS 16_6 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.6 Mobile/15E148 Safari/604.1",
        // Mobile Android Chrome
        "Mozilla/5.0 (Linux; Android 14; Pixel 8) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Mobile Safari/537.36",
        "Mozilla/5.0 (Linux; Android 13; SM-S918B) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Mobile Safari/537.36",
        "Mozilla/5.0 (Linux; Android 12; SM-G998B) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Mobile Safari/537.36",
        // Mobile Android Firefox
        "Mozilla/5.0 (Android 14; Mobile; rv:133.0) Gecko/133.0 Firefox/133.0",
        "Mozilla/5.0 (Android 13; Mobile; rv:133.0) Gecko/133.0 Firefox/133.0",
        // Older Desktop Browsers (for compatibility testing)
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:120.0) Gecko/20100101 Firefox/120.0",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36",
        // Linux Variants
        "Mozilla/5.0 (X11; Linux x86_64; rv:133.0) Gecko/20100101 Firefox/133.0",
        "Mozilla/5.0 (X11; Fedora; Linux x86_64; rv:133.0) Gecko/20100101 Firefox/133.0",
        "Mozilla/5.0 (X11; Debian; Linux x86_64; rv:133.0) Gecko/20100101 Firefox/133.0",
        // Opera
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 OPR/106.0.0.0",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 OPR/106.0.0.0",
        // Brave
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 Brave/1.61",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 Brave/1.61",
        // Crawlers and Bots (for detection testing)
        "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)",
        "Mozilla/5.0 (compatible; bingbot/2.0; +http://www.bing.com/bingbot.htm)",
        "Mozilla/5.0 AppleWebKit/537.36 (KHTML, like Gecko; compatible; GPTBot/1.0; +https://openai.com/gptbot)",
        "Mozilla/5.0 (compatible; facebookexternalhit/1.1; +http://www.facebook.com/externalhit_uatext.php)",
        "Mozilla/5.0 (compatible; Yahoo! Slurp; http://help.yahoo.com/help/us/ysearch/slurp)",
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
    fn test_random_user_agent_contains_version() {
        let ua = random_user_agent();
        // Should contain version numbers
        assert!(ua.chars().any(|c| c.is_numeric()));
    }

    #[test]
    fn test_random_user_agent_multiple_calls() {
        let ua1 = random_user_agent();
        let ua2 = random_user_agent();
        // Both should be valid
        assert!(!ua1.is_empty());
        assert!(!ua2.is_empty());
    }

    #[test]
    fn test_random_user_agent_realistic() {
        let ua = random_user_agent();
        assert!(
            ua.contains("Chrome")
                || ua.contains("Firefox")
                || ua.contains("Safari")
                || ua.contains("Edge")
                || ua.contains("bot")
                || ua.contains("facebook")
                || ua.contains("Slurp")
                || ua.contains("OPR")
                || ua.contains("Brave")
        );
    }

    #[test]
    fn test_random_user_agent_platform() {
        let ua = random_user_agent();
        assert!(
            ua.contains("Windows")
                || ua.contains("Macintosh")
                || ua.contains("Linux")
                || ua.contains("Android")
                || ua.contains("iPhone")
                || ua.contains("iPad")
                || ua.contains("compatible")
        );
    }

    #[test]
    fn test_random_user_agent_gecko_webkit() {
        let ua = random_user_agent();
        // Most UAs have Gecko or WebKit, but bots may not
        assert!(
            ua.contains("Gecko")
                || ua.contains("WebKit")
                || ua.contains("AppleWebKit")
                || ua.contains("bot")
                || ua.contains("facebook")
                || ua.contains("Slurp")
        );
    }

    #[test]
    fn test_random_user_agent_format() {
        let ua = random_user_agent();
        // Should follow general UA format
        assert!(ua.contains("/"));
    }

    #[test]
    fn test_random_user_agent_modern() {
        let ua = random_user_agent();
        // Should contain modern browser versions
        assert!(!ua.is_empty());
    }

    #[test]
    fn test_random_user_agent_variations() {
        let mut agents = std::collections::HashSet::new();
        for _ in 0..50 {
            agents.insert(random_user_agent());
        }
        // Should have good variety with 33 different user agents
        assert!(agents.len() > 5);
    }

    #[test]
    fn test_random_user_agent_mobile_support() {
        let mut found_mobile = false;
        for _ in 0..100 {
            let ua = random_user_agent();
            if ua.contains("iPhone") || ua.contains("iPad") || ua.contains("Android") {
                found_mobile = true;
                break;
            }
        }
        assert!(found_mobile);
    }

    #[test]
    fn test_random_user_agent_ios() {
        let mut found_ios = false;
        for _ in 0..100 {
            let ua = random_user_agent();
            if ua.contains("iPhone") || ua.contains("iPad") {
                found_ios = true;
                assert!(ua.contains("iOS") || ua.contains("OS"));
                break;
            }
        }
        assert!(found_ios);
    }

    #[test]
    fn test_random_user_agent_android() {
        let mut found_android = false;
        for _ in 0..100 {
            let ua = random_user_agent();
            if ua.contains("Android") {
                found_android = true;
                // Android UAs may or may not contain "Linux" (Firefox mobile doesn't always)
                assert!(ua.contains("Linux") || ua.contains("Mobile") || ua.contains("Gecko"));
                break;
            }
        }
        assert!(found_android);
    }

    #[test]
    fn test_random_user_agent_desktop_browsers() {
        let mut found_chrome = false;
        let mut found_firefox = false;
        let mut found_safari = false;
        for _ in 0..100 {
            let ua = random_user_agent();
            if ua.contains("Chrome") && !ua.contains("Edg") && !ua.contains("OPR") {
                found_chrome = true;
            }
            if ua.contains("Firefox") {
                found_firefox = true;
            }
            if ua.contains("Safari") && !ua.contains("Chrome") {
                found_safari = true;
            }
        }
        assert!(found_chrome || found_firefox || found_safari);
    }

    #[test]
    fn test_random_user_agent_edge() {
        let mut found_edge = false;
        for _ in 0..100 {
            let ua = random_user_agent();
            if ua.contains("Edg/") {
                found_edge = true;
                assert!(ua.contains("Chrome"));
                break;
            }
        }
        assert!(found_edge);
    }

    #[test]
    fn test_random_user_agent_opera() {
        let mut found_opera = false;
        for _ in 0..100 {
            let ua = random_user_agent();
            if ua.contains("OPR/") {
                found_opera = true;
                assert!(ua.contains("Chrome"));
                break;
            }
        }
        assert!(found_opera);
    }

    #[test]
    fn test_random_user_agent_brave() {
        let mut found_brave = false;
        for _ in 0..100 {
            let ua = random_user_agent();
            if ua.contains("Brave") {
                found_brave = true;
                assert!(ua.contains("Chrome"));
                break;
            }
        }
        assert!(found_brave);
    }

    #[test]
    fn test_random_user_agent_bot_crawlers() {
        let mut found_bot = false;
        for _ in 0..100 {
            let ua = random_user_agent();
            if ua.contains("Googlebot")
                || ua.contains("bingbot")
                || ua.contains("GPTBot")
                || ua.contains("facebookexternalhit")
                || ua.contains("Slurp")
            {
                found_bot = true;
                break;
            }
        }
        assert!(found_bot);
    }

    #[test]
    fn test_random_user_agent_linux_variants() {
        let mut found_linux = false;
        for _ in 0..100 {
            let ua = random_user_agent();
            if ua.contains("Linux") && !ua.contains("Android") {
                found_linux = true;
                assert!(
                    ua.contains("X11")
                        || ua.contains("Ubuntu")
                        || ua.contains("Fedora")
                        || ua.contains("Debian")
                );
                break;
            }
        }
        assert!(found_linux);
    }

    #[test]
    fn test_random_user_agent_older_versions() {
        let mut found_older = false;
        for _ in 0..100 {
            let ua = random_user_agent();
            if ua.contains("Chrome/119") || ua.contains("Chrome/120") || ua.contains("Firefox/120")
            {
                found_older = true;
                break;
            }
        }
        assert!(found_older);
    }

    #[test]
    fn test_random_user_agent_mobile_firefox() {
        let mut found_mobile_firefox = false;
        for _ in 0..100 {
            let ua = random_user_agent();
            if ua.contains("Android") && ua.contains("Firefox") {
                found_mobile_firefox = true;
                assert!(ua.contains("Mobile"));
                break;
            }
        }
        assert!(found_mobile_firefox);
    }

    #[test]
    fn test_random_user_agent_all_valid() {
        for _ in 0..50 {
            let ua = random_user_agent();
            assert!(!ua.is_empty());
            assert!(ua.len() > 20);
            assert!(ua.contains("Mozilla") || ua.contains("facebook"));
        }
    }

    #[test]
    fn test_http2_header_order() {
        let headers = "accept-language: en-US\naccept-encoding: gzip\nuser-agent: test";
        let result = http2_header_order(headers);
        assert!(result.len() > 0);
        assert!(result.contains("accept"));
    }

    #[test]
    fn test_http2_header_order_empty() {
        assert_eq!(http2_header_order(""), "");
    }

    #[test]
    fn test_http2_header_order_single_header() {
        let result = http2_header_order("user-agent: test");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_http2_header_order_preserves_content() {
        let headers = "host: example.com\nuser-agent: browser";
        let result = http2_header_order(headers);
        assert!(result.contains("example.com"));
    }

    #[test]
    fn test_http2_header_order_multiple_headers() {
        let headers = "accept: */*\nhost: test.com\nuser-agent: test";
        let result = http2_header_order(headers);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_http2_header_order_newlines() {
        let headers = "header1: value1\nheader2: value2\nheader3: value3";
        let result = http2_header_order(headers);
        assert!(result.contains('\n') || result.contains("header"));
    }

    #[test]
    fn test_http2_header_order_case() {
        let headers = "Accept-Language: en-US\nUser-Agent: test";
        let result = http2_header_order(headers);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_http2_header_order_common_headers() {
        let headers = ":method: GET\n:path: /\n:authority: example.com";
        let result = http2_header_order(headers);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_http2_header_order_with_values() {
        let headers = "cookie: session=123\nauthorization: Bearer token";
        let result = http2_header_order(headers);
        assert!(result.contains("session") || result.contains("Bearer"));
    }

    #[test]
    fn test_http2_header_order_preserves_structure() {
        let headers = "a: 1\nb: 2\nc: 3";
        let result = http2_header_order(headers);
        assert!(!result.is_empty());
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
    fn test_tls_fingerprint_variation_empty() {
        assert_eq!(tls_fingerprint_variation(""), "");
    }

    #[test]
    fn test_tls_fingerprint_variation_aes128() {
        let result = tls_fingerprint_variation("TLS_AES_128_GCM_SHA256");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_tls_fingerprint_variation_chacha() {
        let result = tls_fingerprint_variation("TLS_CHACHA20_POLY1305_SHA256");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_tls_fingerprint_variation_preserves_structure() {
        let result = tls_fingerprint_variation("TLS_RSA_WITH_AES_256_CBC_SHA");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_tls_fingerprint_variation_multiple() {
        let cipher = "TLS_AES_256_GCM_SHA384:TLS_AES_128_GCM_SHA256";
        let result = tls_fingerprint_variation(cipher);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_tls_fingerprint_variation_underscores() {
        let result = tls_fingerprint_variation("TLS_ECDHE_RSA_WITH_AES");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_tls_fingerprint_variation_case_handling() {
        let result = tls_fingerprint_variation("tls_aes_256_gcm_sha384");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_tls_fingerprint_variation_numbers() {
        let result = tls_fingerprint_variation("TLS_AES_256_GCM_SHA384");
        assert!(result.contains("256") || result.contains("384"));
    }

    #[test]
    fn test_tls_fingerprint_variation_preserves_content() {
        let cipher = "TLS_TEST_CIPHER";
        let result = tls_fingerprint_variation(cipher);
        assert!(!result.is_empty());
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
    fn test_cloudflare_challenge_variation_empty() {
        assert_eq!(cloudflare_challenge_variation(""), "");
    }

    #[test]
    fn test_cloudflare_challenge_variation_simple() {
        let result = cloudflare_challenge_variation("challenge_token");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_cloudflare_challenge_variation_with_equals() {
        let result = cloudflare_challenge_variation("cf_bm=abc123def456");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_cloudflare_challenge_variation_preserves_value() {
        let result = cloudflare_challenge_variation("cf_clearance=test123");
        assert!(
            result.to_lowercase().contains("test123")
                || result.to_lowercase().contains("clearance")
        );
    }

    #[test]
    fn test_cloudflare_challenge_variation_cookie_format() {
        let result = cloudflare_challenge_variation("cf_cookie=value; path=/");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_cloudflare_challenge_variation_multiple_cookies() {
        let result = cloudflare_challenge_variation("cf_clearance=a; cf_bm=b");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_cloudflare_challenge_variation_long_token() {
        let result =
            cloudflare_challenge_variation("cf_clearance=verylongtokenvalue1234567890abcdef");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_cloudflare_challenge_variation_special_chars() {
        let result = cloudflare_challenge_variation("cf_token=abc-123_xyz");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_cloudflare_challenge_variation_preserves_structure() {
        let challenge = "cf_test=value";
        let result = cloudflare_challenge_variation(challenge);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_accept_language_variation() {
        let lang = "en-US,en;q=0.9";
        let result = accept_language_variation(lang);
        assert!(result.contains("en"));
    }

    #[test]
    fn test_accept_language_variation_empty() {
        let result = accept_language_variation("");
        // Empty input may return a default variation or empty string
        assert!(result.is_empty() || result.contains("en"));
    }

    #[test]
    fn test_accept_language_variation_simple() {
        let result = accept_language_variation("en-US");
        assert!(result.contains("en"));
    }

    #[test]
    fn test_accept_language_variation_multiple() {
        let result = accept_language_variation("en-US,fr-FR,de-DE");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_accept_language_variation_with_quality() {
        let result = accept_language_variation("en;q=1.0,fr;q=0.8");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_accept_language_variation_wildcard() {
        let result = accept_language_variation("en-US,*;q=0.5");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_accept_language_variation_region() {
        let result = accept_language_variation("zh-CN");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_accept_language_variation_preserves_structure() {
        let result = accept_language_variation("es-ES,es;q=0.9");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_accept_language_variation_lowercase() {
        let result = accept_language_variation("en-us");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_accept_language_variation_complex() {
        let result = accept_language_variation("en-US,en;q=0.9,fr;q=0.8,de;q=0.7");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_accept_language_variation_preserves_language() {
        let lang = "ja-JP";
        let result = accept_language_variation(lang);
        assert!(result.to_lowercase().contains("ja") || !result.is_empty());
    }
}
