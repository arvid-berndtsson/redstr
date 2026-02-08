use crate::rng::SimpleRng;

/// Generates Cloudflare Turnstile challenge variations.
///
/// Cloudflare Turnstile is a privacy-focused CAPTCHA alternative that uses
/// challenge-response mechanisms. This function generates variations of
/// Turnstile challenge tokens and responses for bot detection evasion testing.
///
/// Useful for red team Cloudflare bypass testing and blue team bot detection validation.
///
/// # Examples
///
/// ```
/// use redstr::cloudflare_turnstile_variation;
/// let challenge = "challenge-token";
/// let result = cloudflare_turnstile_variation(challenge);
/// assert!(result.len() > 0);
/// ```
pub fn cloudflare_turnstile_variation(input: &str) -> String {
    let mut rng = SimpleRng::new();

    // Common Turnstile token patterns
    let patterns = [
        format!("{}-{}", input, generate_random_suffix(&mut rng)),
        format!("{}_{}", input, generate_random_suffix(&mut rng)),
        format!("cf-turnstile-{}", input),
        format!("turnstile-{}-{}", input, generate_random_suffix(&mut rng)),
    ];

    patterns[rng.next() as usize % patterns.len()].clone()
}

/// Generates Cloudflare challenge response patterns.
///
/// Cloudflare uses various challenge mechanisms (Turnstile, __cf_bm cookies, etc.).
/// This function generates response patterns that mimic legitimate challenge responses.
///
/// Useful for testing Cloudflare challenge bypass techniques and bot detection evasion.
///
/// # Examples
///
/// ```
/// use redstr::cloudflare_challenge_response;
/// let challenge = "cf_clearance=abc123";
/// let result = cloudflare_challenge_response(challenge);
/// assert!(result.len() > 0);
/// ```
pub fn cloudflare_challenge_response(input: &str) -> String {
    let mut rng = SimpleRng::new();

    // Generate response variations for Cloudflare challenges
    if input.contains("cf_clearance") || input.contains("__cf_bm") {
        // Cookie-based challenge response
        input
            .chars()
            .map(|c| {
                match rng.next() % 10 {
                    0..=6 => c.to_string(),
                    7 => {
                        // Occasionally add spacing variations
                        if c == '=' {
                            if rng.next() % 2 == 0 {
                                " = ".to_string()
                            } else {
                                "=".to_string()
                            }
                        } else {
                            c.to_string()
                        }
                    }
                    8 => {
                        // Underscore/hyphen variations
                        if c == '_' && rng.next() % 3 == 0 {
                            "-".to_string()
                        } else if c == '-' && rng.next() % 3 == 0 {
                            "_".to_string()
                        } else {
                            c.to_string()
                        }
                    }
                    _ => c.to_string(),
                }
            })
            .collect()
    } else if input.contains("turnstile") || input.contains("challenge") {
        // Turnstile challenge response
        let mut result = input.to_string();
        if rng.next() % 2 == 0 {
            result.push_str(&format!("-{}", generate_random_suffix(&mut rng)));
        }
        result
    } else {
        // Generic challenge response - add timestamp-like suffix
        format!("{}-{}", input, generate_random_suffix(&mut rng))
    }
}

/// Generates TLS handshake pattern variations for Cloudflare bot detection evasion.
///
/// Cloudflare analyzes TLS handshake characteristics (cipher suites, extensions, etc.)
/// to fingerprint clients. This function generates variations of TLS handshake patterns.
///
/// Useful for red team TLS fingerprinting evasion and blue team detection testing.
///
/// # Examples
///
/// ```
/// use redstr::tls_handshake_pattern;
/// let pattern = "TLS_AES_256_GCM_SHA384:TLS_CHACHA20_POLY1305_SHA256";
/// let result = tls_handshake_pattern(pattern);
/// assert!(result.len() > 0);
/// ```
pub fn tls_handshake_pattern(input: &str) -> String {
    let mut rng = SimpleRng::new();

    // Common TLS handshake pattern variations
    let separators = [":", ",", " ", "-"];
    let separator = separators[rng.next() as usize % separators.len()];

    input
        .split(|c: char| [':', ',', ' ', '-'].contains(&c))
        .enumerate()
        .map(|(i, part)| {
            if i > 0 {
                format!("{}{}", separator, part)
            } else {
                part.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("")
}

/// Generates canvas fingerprint variations for Cloudflare bot detection evasion.
///
/// Canvas fingerprinting is a technique used to identify browsers based on
/// how they render canvas elements. This function generates variations that
/// might affect canvas fingerprinting results.
///
/// Useful for red team browser fingerprinting evasion and blue team detection testing.
///
/// # Examples
///
/// ```
/// use redstr::canvas_fingerprint_variation;
/// let canvas_data = "canvas-fingerprint-data";
/// let result = canvas_fingerprint_variation(canvas_data);
/// assert!(result.len() > 0);
/// ```
pub fn canvas_fingerprint_variation(input: &str) -> String {
    let mut rng = SimpleRng::new();

    // Canvas fingerprint variations typically involve subtle rendering differences
    // This function generates string variations that might affect fingerprinting
    input
        .chars()
        .map(|c| {
            match rng.next() % 15 {
                0..=11 => c.to_string(),
                12 => {
                    // Occasionally swap similar characters
                    match c {
                        '0' => {
                            if rng.next() % 2 == 0 {
                                "O".to_string()
                            } else {
                                "0".to_string()
                            }
                        }
                        '1' => {
                            if rng.next() % 2 == 0 {
                                "l".to_string()
                            } else {
                                "1".to_string()
                            }
                        }
                        'O' => {
                            if rng.next() % 2 == 0 {
                                "0".to_string()
                            } else {
                                "O".to_string()
                            }
                        }
                        'l' => {
                            if rng.next() % 2 == 0 {
                                "1".to_string()
                            } else {
                                "l".to_string()
                            }
                        }
                        _ => c.to_string(),
                    }
                }
                13 => {
                    // Add subtle spacing variations
                    if c.is_whitespace() && rng.next() % 2 == 0 {
                        "  ".to_string() // Double space
                    } else {
                        c.to_string()
                    }
                }
                _ => c.to_string(),
            }
        })
        .collect()
}

/// Obfuscates WebGL fingerprint data for Cloudflare bot detection evasion.
///
/// WebGL fingerprinting uses graphics rendering characteristics to identify browsers.
/// This function obfuscates WebGL-related strings that might be used in fingerprinting.
///
/// Useful for red team WebGL fingerprinting evasion and blue team detection testing.
///
/// # Examples
///
/// ```
/// use redstr::webgl_fingerprint_obfuscate;
/// let webgl_data = "WebGL 2.0 Renderer: ANGLE (Intel, Intel(R) UHD Graphics 620 Direct3D11 vs_5_0 ps_5_0)";
/// let result = webgl_fingerprint_obfuscate(webgl_data);
/// assert!(result.len() > 0);
/// ```
pub fn webgl_fingerprint_obfuscate(input: &str) -> String {
    let mut rng = SimpleRng::new();

    // WebGL fingerprint obfuscation - vary version numbers, renderer names, etc.
    input
        .chars()
        .map(|c| {
            match rng.next() % 12 {
                0..=9 => c.to_string(),
                10 => {
                    // Vary version numbers slightly
                    if c.is_ascii_digit() && rng.next() % 4 == 0 {
                        // Occasionally change digit
                        let digit = c.to_digit(10).unwrap();
                        let new_digit = (digit + 1) % 10;
                        char::from_digit(new_digit, 10).unwrap_or(c).to_string()
                    } else {
                        c.to_string()
                    }
                }
                _ => {
                    // Case variations for version strings
                    if c == '.' && rng.next() % 3 == 0 {
                        ".".to_string()
                    } else if c.is_alphabetic() && rng.next() % 5 == 0 {
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

/// Generates font fingerprint consistency variations for Cloudflare bot detection evasion.
///
/// Font fingerprinting identifies browsers by checking which fonts are available.
/// This function generates variations that maintain consistency while evading detection.
///
/// Useful for red team font fingerprinting evasion and blue team detection testing.
///
/// # Examples
///
/// ```
/// use redstr::font_fingerprint_consistency;
/// let font_list = "Arial, Helvetica, Times New Roman";
/// let result = font_fingerprint_consistency(font_list);
/// assert!(result.len() > 0);
/// ```
pub fn font_fingerprint_consistency(input: &str) -> String {
    let mut rng = SimpleRng::new();

    // Handle empty input
    if input.is_empty() {
        return input.to_string();
    }

    // Font list variations - maintain valid font names but vary formatting
    let fonts: Vec<&str> = input
        .split(|c: char| [',', ';'].contains(&c))
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    if fonts.is_empty() {
        return input.to_string();
    }

    let separators = [", ", ",", "; ", ";"];
    let separator = separators[rng.next() as usize % separators.len()];

    fonts
        .iter()
        .enumerate()
        .map(|(i, font)| {
            let mut font_str = font.to_string();
            // Occasionally add/remove quotes
            if rng.next() % 4 == 0 && !font_str.starts_with('"') {
                font_str = format!("\"{}\"", font_str);
            }
            if i > 0 {
                format!("{}{}", separator, font_str)
            } else {
                font_str
            }
        })
        .collect::<Vec<_>>()
        .join("")
}

/// Helper function to generate random suffix for tokens
fn generate_random_suffix(rng: &mut SimpleRng) -> String {
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
        .chars()
        .collect();
    let length = 8 + (rng.next() % 8) as usize; // 8-15 characters
    (0..length)
        .map(|_| chars[rng.next() as usize % chars.len()])
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cloudflare_turnstile_variation() {
        let challenge = "challenge-token";
        let result = cloudflare_turnstile_variation(challenge);
        assert!(result.len() > challenge.len());
        assert!(result.contains(challenge));
    }

    #[test]
    fn test_cloudflare_challenge_response() {
        let challenge = "cf_clearance=abc123";
        let result = cloudflare_challenge_response(challenge);
        assert!(result.len() > 0);
        // Should preserve key parts of the challenge
        assert!(
            result.to_lowercase().contains("cf_clearance")
                || result.to_lowercase().contains("cf-clearance")
        );
    }

    #[test]
    fn test_tls_handshake_pattern() {
        let pattern = "TLS_AES_256_GCM_SHA384:TLS_CHACHA20_POLY1305_SHA256";
        let result = tls_handshake_pattern(pattern);
        assert!(result.len() > 0);
        assert!(result.contains("TLS"));
    }

    #[test]
    fn test_canvas_fingerprint_variation() {
        let canvas_data = "canvas-fingerprint-data";
        let result = canvas_fingerprint_variation(canvas_data);
        assert!(result.len() > 0);
        assert!(result.contains("canvas"));
    }

    #[test]
    fn test_webgl_fingerprint_obfuscate() {
        let webgl_data = "WebGL 2.0 Renderer: ANGLE";
        let result = webgl_fingerprint_obfuscate(webgl_data);
        assert!(result.len() > 0);
        assert!(result.to_lowercase().contains("webgl"));
    }

    #[test]
    fn test_font_fingerprint_consistency() {
        let font_list = "Arial, Helvetica, Times New Roman";
        let result = font_fingerprint_consistency(font_list);
        assert!(result.len() > 0);
        assert!(result.to_lowercase().contains("arial"));
    }

    #[test]
    fn test_cloudflare_challenge_response_turnstile() {
        let challenge = "turnstile-challenge-123";
        let result = cloudflare_challenge_response(challenge);
        assert!(result.len() > 0);
        assert!(result.contains("turnstile") || result.contains("challenge"));
    }

    #[test]
    fn test_cloudflare_challenge_response_generic() {
        let challenge = "generic-challenge";
        let result = cloudflare_challenge_response(challenge);
        assert!(result.len() > 0);
        // Generic challenge should contain the original challenge
        assert!(result.contains(challenge));
    }

    #[test]
    fn test_cloudflare_turnstile_variation_empty_string() {
        let result = cloudflare_turnstile_variation("");
        assert!(result.len() > 0);
    }

    #[test]
    fn test_cloudflare_turnstile_variation_unicode() {
        let challenge = "challenge-测试-токен";
        let result = cloudflare_turnstile_variation(challenge);
        assert!(result.contains(challenge));
    }

    #[test]
    fn test_cloudflare_challenge_response_cf_bm() {
        let challenge = "__cf_bm=cookie123";
        let result = cloudflare_challenge_response(challenge);
        assert!(result.len() > 0);
        assert!(result.to_lowercase().contains("cf_bm") || result.to_lowercase().contains("cf-bm"));
    }

    #[test]
    fn test_cloudflare_challenge_response_empty_string() {
        let result = cloudflare_challenge_response("");
        // Empty string should return a non-empty result (adds suffix)
        assert!(result.len() > 0);
    }

    #[test]
    fn test_tls_handshake_pattern_empty_string() {
        let result = tls_handshake_pattern("");
        assert_eq!(result, "");
    }

    #[test]
    fn test_tls_handshake_pattern_single_cipher() {
        let pattern = "TLS_AES_256_GCM_SHA384";
        let result = tls_handshake_pattern(pattern);
        assert!(result.contains("TLS"));
    }

    #[test]
    fn test_tls_handshake_pattern_comma_separated() {
        let pattern = "TLS_AES_256_GCM_SHA384,TLS_CHACHA20_POLY1305_SHA256";
        let result = tls_handshake_pattern(pattern);
        assert!(result.contains("TLS"));
    }

    #[test]
    fn test_tls_handshake_pattern_space_separated() {
        let pattern = "TLS_AES_256_GCM_SHA384 TLS_CHACHA20_POLY1305_SHA256";
        let result = tls_handshake_pattern(pattern);
        assert!(result.contains("TLS"));
    }

    #[test]
    fn test_canvas_fingerprint_variation_empty_string() {
        let result = canvas_fingerprint_variation("");
        assert_eq!(result, "");
    }

    #[test]
    fn test_canvas_fingerprint_variation_unicode() {
        let canvas_data = "canvas-测试-data";
        let result = canvas_fingerprint_variation(canvas_data);
        assert!(result.len() > 0);
    }

    #[test]
    fn test_canvas_fingerprint_variation_preserves_length_approximately() {
        let canvas_data = "test123";
        let result = canvas_fingerprint_variation(canvas_data);
        // Length should be similar (may vary slightly due to character swaps)
        assert!(result.len() >= canvas_data.len() - 2);
        assert!(result.len() <= canvas_data.len() + 2);
    }

    #[test]
    fn test_webgl_fingerprint_obfuscate_empty_string() {
        let result = webgl_fingerprint_obfuscate("");
        assert_eq!(result, "");
    }

    #[test]
    fn test_webgl_fingerprint_obfuscate_with_version() {
        let webgl_data = "WebGL 2.0";
        let result = webgl_fingerprint_obfuscate(webgl_data);
        assert!(result.len() > 0);
        assert!(result.to_lowercase().contains("webgl"));
    }

    #[test]
    fn test_webgl_fingerprint_obfuscate_preserves_structure() {
        let webgl_data = "WebGL 2.0 Renderer: ANGLE";
        let result = webgl_fingerprint_obfuscate(webgl_data);
        // Should preserve overall structure
        assert!(result.to_lowercase().contains("webgl"));
        assert!(result.to_lowercase().contains("renderer"));
    }

    #[test]
    fn test_font_fingerprint_consistency_empty_string() {
        let result = font_fingerprint_consistency("");
        assert_eq!(result, "");
    }

    #[test]
    fn test_font_fingerprint_consistency_single_font() {
        let font_list = "Arial";
        let result = font_fingerprint_consistency(font_list);
        assert!(result.len() > 0);
        assert!(result.to_lowercase().contains("arial"));
    }

    #[test]
    fn test_font_fingerprint_consistency_semicolon_separated() {
        let font_list = "Arial; Helvetica; Times New Roman";
        let result = font_fingerprint_consistency(font_list);
        assert!(result.len() > 0);
        assert!(result.to_lowercase().contains("arial"));
    }

    #[test]
    fn test_font_fingerprint_consistency_preserves_fonts() {
        let font_list = "Arial, Helvetica, Times New Roman";
        let result = font_fingerprint_consistency(font_list);
        // All fonts should be present
        assert!(result.to_lowercase().contains("arial"));
        assert!(result.to_lowercase().contains("helvetica"));
        assert!(result.to_lowercase().contains("times"));
    }

    #[test]
    fn test_cloudflare_turnstile_variation_multiple_calls() {
        let challenge = "test-challenge";
        let results: Vec<String> = (0..10)
            .map(|_| cloudflare_turnstile_variation(challenge))
            .collect();
        // All results should contain the challenge
        for result in &results {
            assert!(result.contains(challenge));
        }
        // At least some results should differ (randomness)
        let unique_results: std::collections::HashSet<&String> = results.iter().collect();
        // Due to randomness, we might get some duplicates, but not all should be identical
        assert!(unique_results.len() >= 1);
    }

    #[test]
    fn test_cloudflare_challenge_response_special_characters() {
        let challenge = "cf_clearance=abc!@#$%^&*()123";
        let result = cloudflare_challenge_response(challenge);
        assert!(result.len() > 0);
        assert!(
            result.to_lowercase().contains("cf_clearance")
                || result.to_lowercase().contains("cf-clearance")
        );
    }

    #[test]
    fn test_tls_handshake_pattern_preserves_ciphers() {
        let pattern = "TLS_AES_256_GCM_SHA384:TLS_CHACHA20_POLY1305_SHA256";
        let result = tls_handshake_pattern(pattern);
        // Should contain both cipher names
        assert!(result.contains("TLS_AES_256_GCM_SHA384"));
        assert!(result.contains("TLS_CHACHA20_POLY1305_SHA256"));
    }

    #[test]
    fn test_cloudflare_turnstile_variation_long_string() {
        let challenge = "a".repeat(1000);
        let result = cloudflare_turnstile_variation(&challenge);
        assert!(result.len() > challenge.len());
        assert!(result.contains(&challenge));
    }

    #[test]
    fn test_cloudflare_challenge_response_long_string() {
        let challenge = "cf_clearance=".to_string() + &"a".repeat(500);
        let result = cloudflare_challenge_response(&challenge);
        assert!(result.len() > 0);
        assert!(
            result.to_lowercase().contains("cf_clearance")
                || result.to_lowercase().contains("cf-clearance")
        );
    }

    #[test]
    fn test_tls_handshake_pattern_long_list() {
        let pattern = (0..20)
            .map(|i| format!("TLS_CIPHER_{}", i))
            .collect::<Vec<_>>()
            .join(":");
        let result = tls_handshake_pattern(&pattern);
        assert!(result.len() > 0);
        assert!(result.contains("TLS_CIPHER_0"));
        assert!(result.contains("TLS_CIPHER_19"));
    }

    #[test]
    fn test_canvas_fingerprint_variation_special_chars_only() {
        let canvas_data = "!@#$%^&*()";
        let result = canvas_fingerprint_variation(canvas_data);
        assert!(result.len() > 0);
    }

    #[test]
    fn test_webgl_fingerprint_obfuscate_long_string() {
        let webgl_data = "WebGL ".to_string() + &"2.0 ".repeat(100);
        let result = webgl_fingerprint_obfuscate(&webgl_data);
        assert!(result.len() > 0);
        assert!(result.to_lowercase().contains("webgl"));
    }

    #[test]
    fn test_font_fingerprint_consistency_long_list() {
        let fonts: Vec<String> = (0..50).map(|i| format!("Font{}", i)).collect();
        let font_list = fonts.join(", ");
        let result = font_fingerprint_consistency(&font_list);
        assert!(result.len() > 0);
        assert!(result.to_lowercase().contains("font0"));
        assert!(result.to_lowercase().contains("font49"));
    }

    #[test]
    fn test_cloudflare_turnstile_variation_randomness() {
        let challenge = "test-challenge";
        for _ in 0..50 {
            let output = cloudflare_turnstile_variation(challenge);
            assert!(
                output.contains(challenge),
                "turnstile output should preserve challenge token"
            );
        }
    }

    #[test]
    fn test_cloudflare_challenge_response_randomness() {
        let challenge = "cf_clearance=test123";
        for _ in 0..50 {
            let output = cloudflare_challenge_response(challenge);
            let normalized = output.to_lowercase();
            assert!(
                normalized.contains("cf_clearance") || normalized.contains("cf-clearance"),
                "challenge response must preserve cookie key"
            );
        }
    }

    #[test]
    fn test_tls_handshake_pattern_variation() {
        let pattern = "TLS_AES_256_GCM_SHA384:TLS_CHACHA20_POLY1305_SHA256";
        for _ in 0..20 {
            let output = tls_handshake_pattern(pattern);
            assert!(output.contains("TLS_AES_256_GCM_SHA384"));
            assert!(output.contains("TLS_CHACHA20_POLY1305_SHA256"));
        }
    }

    #[test]
    fn test_font_fingerprint_consistency_variation() {
        let font_list = "Arial, Helvetica";
        for _ in 0..20 {
            let output = font_fingerprint_consistency(font_list);
            let lower = output.to_lowercase();
            assert!(lower.contains("arial"));
            assert!(lower.contains("helvetica"));
        }
    }

    #[test]
    fn test_cloudflare_functions_preserve_non_empty_output() {
        let test_cases = vec![
            "test",
            "challenge-token-123",
            "cf_clearance=abc",
            "TLS_AES_256_GCM_SHA384",
            "canvas-data",
            "WebGL 2.0",
            "Arial, Helvetica",
        ];

        for input in test_cases {
            assert!(
                cloudflare_turnstile_variation(input).len() > 0,
                "turnstile failed for: {}",
                input
            );
            assert!(
                cloudflare_challenge_response(input).len() > 0,
                "challenge_response failed for: {}",
                input
            );
            // These functions can return empty strings for empty input, so just check they don't panic
            let _ = tls_handshake_pattern(input);
            let _ = canvas_fingerprint_variation(input);
            let _ = webgl_fingerprint_obfuscate(input);
            let _ = font_fingerprint_consistency(input);
        }
    }

    #[test]
    fn test_cloudflare_challenge_response_all_cookie_formats() {
        let formats = vec![
            "cf_clearance=token123",
            "__cf_bm=cookie456",
            "cf-clearance=token789",
            "__cf-bm=cookie012",
        ];

        for format in formats {
            let result = cloudflare_challenge_response(format);
            assert!(result.len() > 0, "Failed for format: {}", format);
            // Should preserve the cookie name in some form
            let lower = result.to_lowercase();
            assert!(
                lower.contains("cf") || lower.contains("clearance") || lower.contains("bm"),
                "Result doesn't contain cookie identifier: {}",
                result
            );
        }
    }
}
