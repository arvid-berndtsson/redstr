use crate::rng::SimpleRng;
use crate::transformations::case::randomize_capitalization;
use crate::transformations::obfuscation::leetspeak;
use crate::transformations::unicode::homoglyph_substitution;

/// Generates domain typosquatting variations for phishing detection testing.
///
/// Useful for red team phishing simulations and blue team domain monitoring.
///
/// # Examples
///
/// ```
/// use redstr::domain_typosquat;
/// let result = domain_typosquat("example.com");
/// assert!(result.len() > 0);
/// ```
pub fn domain_typosquat(domain: &str) -> String {
    let mut rng = SimpleRng::new();
    let chars: Vec<char> = domain.chars().collect();

    if chars.is_empty() {
        return domain.to_string();
    }

    let mut result = String::new();
    let operation = rng.next() % 4;

    match operation {
        0 => {
            // Character substitution
            for (i, c) in chars.iter().enumerate() {
                if i == (rng.next() as usize % chars.len()) && c.is_alphabetic() {
                    let substitutions = match c.to_lowercase().to_string().as_str() {
                        "o" => vec!['0', 'ο'], // Latin o, digit 0, Greek omicron
                        "i" => vec!['1', 'l', 'ı'],
                        "l" => vec!['1', 'i', 'I'],
                        "a" => vec!['@', 'а'], // Cyrillic а
                        "e" => vec!['3', 'е'], // Cyrillic е
                        _ => vec![*c],
                    };
                    result.push(substitutions[rng.next() as usize % substitutions.len()]);
                } else {
                    result.push(*c);
                }
            }
        }
        1 => {
            // Character omission
            for (i, c) in chars.iter().enumerate() {
                if i != (rng.next() as usize % chars.len()) {
                    result.push(*c);
                }
            }
        }
        2 => {
            // Character duplication
            for (i, c) in chars.iter().enumerate() {
                result.push(*c);
                if i == (rng.next() as usize % chars.len()) {
                    result.push(*c);
                }
            }
        }
        _ => {
            // Adjacent key typo (keyboard-based)
            for (i, c) in chars.iter().enumerate() {
                if i == (rng.next() as usize % chars.len()) && c.is_alphabetic() {
                    let adjacent = match c.to_lowercase().to_string().as_str() {
                        "a" => vec!['q', 's', 'w', 'z'],
                        "e" => vec!['w', 'r', 'd', 's'],
                        "o" => vec!['i', 'p', 'l', 'k'],
                        "m" => vec!['n', 'k', 'j'],
                        _ => vec![*c],
                    };
                    result.push(adjacent[rng.next() as usize % adjacent.len()]);
                } else {
                    result.push(*c);
                }
            }
        }
    }

    result
}

/// Generates advanced domain typosquatting with multiple techniques.
///
/// Enhanced version for EvilJinx and phishing frameworks. Combines multiple
/// spoofing techniques including homoglyphs, typos, and TLD variations.
///
/// # Examples
///
/// ```
/// use redstr::advanced_domain_spoof;
/// let domain = "paypal.com";
/// let result = advanced_domain_spoof(domain);
/// assert!(result.len() > 0);
/// ```
pub fn advanced_domain_spoof(domain: &str) -> String {
    let mut rng = SimpleRng::new();

    // Split domain and TLD
    let parts: Vec<&str> = domain.split('.').collect();
    if parts.len() < 2 {
        return domain.to_string();
    }

    let domain_part = parts[0];
    let tld = parts[1..].join(".");

    let mut result = String::new();
    let operation = rng.next() % 5;

    match operation {
        0 => {
            // Homoglyph substitution
            result = homoglyph_substitution(domain_part);
        }
        1 => {
            // Character insertion
            let chars: Vec<char> = domain_part.chars().collect();
            for (i, c) in chars.iter().enumerate() {
                result.push(*c);
                if i == (rng.next() as usize % chars.len().max(1)) && chars.len() > 1 {
                    // Insert similar character
                    let insertions = match c.to_lowercase().to_string().as_str() {
                        "a" => vec!['a', '4'],
                        "e" => vec!['e', '3'],
                        "i" => vec!['i', '1'],
                        "o" => vec!['o', '0'],
                        _ => vec![*c],
                    };
                    result.push(insertions[rng.next() as usize % insertions.len()]);
                }
            }
        }
        2 => {
            // Adjacent key typos
            result = domain_typosquat(domain_part);
        }
        3 => {
            // TLD variation (common phishing technique)
            let tld_variants = match tld.as_str() {
                "com" => vec!["co", "cm", "om"],
                "net" => vec!["ne", "et"],
                "org" => vec!["or", "og"],
                _ => vec![],
            };
            if !tld_variants.is_empty() {
                result = domain_part.to_string();
                result.push('.');
                result.push_str(tld_variants[rng.next() as usize % tld_variants.len()]);
                return result;
            } else {
                result = domain_part.to_string();
            }
        }
        _ => {
            // Character omission
            let chars: Vec<char> = domain_part.chars().collect();
            let omit_idx = if chars.len() > 1 {
                rng.next() as usize % chars.len()
            } else {
                chars.len()
            };
            for (i, c) in chars.iter().enumerate() {
                if i != omit_idx {
                    result.push(*c);
                }
            }
        }
    }

    result.push('.');
    result.push_str(&tld);
    result
}

/// Obfuscates email addresses for phishing and social engineering testing.
///
/// Useful for EvilJinx and phishing framework integration. Applies various
/// obfuscation techniques to email addresses.
///
/// # Examples
///
/// ```
/// use redstr::email_obfuscation;
/// let email = "admin@example.com";
/// let result = email_obfuscation(email);
/// assert!(result.contains("@"));
/// ```
pub fn email_obfuscation(email: &str) -> String {
    let mut rng = SimpleRng::new();

    if !email.contains('@') {
        return email.to_string();
    }

    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return email.to_string();
    }

    let local = parts[0];
    let domain = parts[1];

    // Obfuscate local part
    let mut result = match rng.next() % 3 {
        0 => {
            // Homoglyph substitution
            homoglyph_substitution(local)
        }
        1 => {
            // Leetspeak
            leetspeak(local)
        }
        _ => {
            // Case variation
            randomize_capitalization(local)
        }
    };

    result.push('@');

    // Obfuscate domain
    match rng.next() % 2 {
        0 => {
            result.push_str(&advanced_domain_spoof(domain));
        }
        _ => {
            result.push_str(&homoglyph_substitution(domain));
        }
    }

    result
}

/// Generates URL shortening patterns for phishing campaigns.
///
/// Useful for EvilJinx and phishing frameworks to generate shortened URL patterns
/// that might bypass detection.
///
/// # Examples
///
/// ```
/// use redstr::url_shortening_pattern;
/// let url = "https://example.com/login";
/// let result = url_shortening_pattern(url);
/// assert!(result.len() > 0);
/// ```
pub fn url_shortening_pattern(_url: &str) -> String {
    let mut rng = SimpleRng::new();

    // Common URL shortening services patterns
    let shorteners = ["bit.ly", "tinyurl.com", "goo.gl", "t.co", "ow.ly"];
    let shortener = shorteners[rng.next() as usize % shorteners.len()];

    // Generate a short code-like pattern
    let code_chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
        .chars()
        .collect();
    let mut code = String::new();
    for _ in 0..((rng.next() % 5) + 5) {
        code.push(code_chars[rng.next() as usize % code_chars.len()]);
    }

    format!("https://{}/{}", shortener, code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_typosquat() {
        let result = domain_typosquat("example.com");
        assert!(result.len() > 0);
        // Should be similar but not identical (usually)
    }

    #[test]
    fn test_advanced_domain_spoof() {
        let domain = "paypal.com";
        let result = advanced_domain_spoof(domain);
        assert!(result.len() > 0);
        assert!(result.contains("."));
    }

    #[test]
    fn test_email_obfuscation() {
        let email = "admin@example.com";
        let result = email_obfuscation(email);
        assert!(result.contains("@"));
        // Domain might be obfuscated, but should still be a valid email format
        let parts: Vec<&str> = result.split('@').collect();
        assert_eq!(parts.len(), 2);
        assert!(!parts[1].is_empty());
    }

    #[test]
    fn test_url_shortening_pattern() {
        let result = url_shortening_pattern("https://example.com");
        assert!(result.contains("https://"));
        assert!(result.len() > 10);
    }
}
