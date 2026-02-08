//! # redstr-wasm
//!
//! WebAssembly bindings for redstr using wasm-bindgen.
//!
//! This crate provides browser-compatible access to all redstr transformation
//! functions for client-side JavaScript applications.
//!
//! ## Installation
//!
//! ```bash
//! npm install redstr-wasm
//! ```
//!
//! ## Usage
//!
//! ```javascript
//! import init, { leetspeak, base64_encode } from 'redstr-wasm';
//!
//! // Initialize WASM module
//! await init();
//!
//! // Use transformations
//! console.log(leetspeak('password')); // 'p@55w0rd'
//! ```

use wasm_bindgen::prelude::*;

// ============================================================================
// Case Transformations
// ============================================================================

/// Randomize the capitalization of each character.
#[wasm_bindgen]
pub fn randomize_capitalization(input: &str) -> String {
    redstr::randomize_capitalization(input)
}

/// Swap the case of each character.
#[wasm_bindgen]
pub fn case_swap(input: &str) -> String {
    redstr::case_swap(input)
}

/// Alternate case for each character.
#[wasm_bindgen]
pub fn alternate_case(input: &str) -> String {
    redstr::alternate_case(input)
}

/// Inverse case transformation.
#[wasm_bindgen]
pub fn inverse_case(input: &str) -> String {
    redstr::inverse_case(input)
}

/// Convert to camelCase.
#[wasm_bindgen]
pub fn to_camel_case(input: &str) -> String {
    redstr::to_camel_case(input)
}

/// Convert to snake_case.
#[wasm_bindgen]
pub fn to_snake_case(input: &str) -> String {
    redstr::to_snake_case(input)
}

/// Convert to kebab-case.
#[wasm_bindgen]
pub fn to_kebab_case(input: &str) -> String {
    redstr::to_kebab_case(input)
}

// ============================================================================
// Encoding Transformations
// ============================================================================

/// Encode string to Base64.
#[wasm_bindgen]
pub fn base64_encode(input: &str) -> String {
    redstr::base64_encode(input)
}

/// URL encode a string.
#[wasm_bindgen]
pub fn url_encode(input: &str) -> String {
    redstr::url_encode(input)
}

/// Hex encode a string.
#[wasm_bindgen]
pub fn hex_encode(input: &str) -> String {
    redstr::hex_encode(input)
}

/// Hex encode with mixed case.
#[wasm_bindgen]
pub fn hex_encode_mixed(input: &str) -> String {
    redstr::hex_encode_mixed(input)
}

/// HTML entity encode a string.
#[wasm_bindgen]
pub fn html_entity_encode(input: &str) -> String {
    redstr::html_entity_encode(input)
}

/// Apply mixed encoding.
#[wasm_bindgen]
pub fn mixed_encoding(input: &str) -> String {
    redstr::mixed_encoding(input)
}

// ============================================================================
// Obfuscation Transformations
// ============================================================================

/// Apply leetspeak transformation.
#[wasm_bindgen]
pub fn leetspeak(input: &str) -> String {
    redstr::leetspeak(input)
}

/// Apply ROT13 cipher.
#[wasm_bindgen]
pub fn rot13(input: &str) -> String {
    redstr::rot13(input)
}

/// Reverse a string.
#[wasm_bindgen]
pub fn reverse_string(input: &str) -> String {
    redstr::reverse_string(input)
}

/// Double each character in the string.
#[wasm_bindgen]
pub fn double_characters(input: &str) -> String {
    redstr::double_characters(input)
}

/// Swap vowels in the string.
#[wasm_bindgen]
pub fn vowel_swap(input: &str) -> String {
    redstr::vowel_swap(input)
}

/// Add whitespace padding.
#[wasm_bindgen]
pub fn whitespace_padding(input: &str) -> String {
    redstr::whitespace_padding(input)
}

/// JavaScript string concatenation obfuscation.
#[wasm_bindgen]
pub fn js_string_concat(input: &str) -> String {
    redstr::js_string_concat(input)
}

// ============================================================================
// Unicode Transformations
// ============================================================================

/// Apply homoglyph substitution.
#[wasm_bindgen]
pub fn homoglyph_substitution(input: &str) -> String {
    redstr::homoglyph_substitution(input)
}

/// Apply zalgo text effect.
#[wasm_bindgen]
pub fn zalgo_text(input: &str) -> String {
    redstr::zalgo_text(input)
}

/// Apply Unicode variations.
#[wasm_bindgen]
pub fn unicode_variations(input: &str) -> String {
    redstr::unicode_variations(input)
}

/// Apply space variants.
#[wasm_bindgen]
pub fn space_variants(input: &str) -> String {
    redstr::space_variants(input)
}

/// Apply Unicode normalization variants.
#[wasm_bindgen]
pub fn unicode_normalize_variants(input: &str) -> String {
    redstr::unicode_normalize_variants(input)
}

// ============================================================================
// Phishing Transformations
// ============================================================================

/// Generate typosquatted domain.
#[wasm_bindgen]
pub fn domain_typosquat(input: &str) -> String {
    redstr::domain_typosquat(input)
}

/// Obfuscate email address.
#[wasm_bindgen]
pub fn email_obfuscation(input: &str) -> String {
    redstr::email_obfuscation(input)
}

/// Generate URL shortening pattern.
#[wasm_bindgen]
pub fn url_shortening_pattern(input: &str) -> String {
    redstr::url_shortening_pattern(input)
}

/// Advanced domain spoofing.
#[wasm_bindgen]
pub fn advanced_domain_spoof(input: &str) -> String {
    redstr::advanced_domain_spoof(input)
}

// ============================================================================
// Injection Transformations
// ============================================================================

/// Apply XSS tag variations.
#[wasm_bindgen]
pub fn xss_tag_variations(input: &str) -> String {
    redstr::xss_tag_variations(input)
}

/// Apply SQL comment injection.
#[wasm_bindgen]
pub fn sql_comment_injection(input: &str) -> String {
    redstr::sql_comment_injection(input)
}

/// Apply command injection patterns.
#[wasm_bindgen]
pub fn command_injection(input: &str) -> String {
    redstr::command_injection(input)
}

/// Apply path traversal patterns.
#[wasm_bindgen]
pub fn path_traversal(input: &str) -> String {
    redstr::path_traversal(input)
}

/// Apply null byte injection.
#[wasm_bindgen]
pub fn null_byte_injection(input: &str) -> String {
    redstr::null_byte_injection(input)
}

/// Apply SSTI injection.
#[wasm_bindgen]
pub fn ssti_injection(input: &str) -> String {
    redstr::ssti_injection(input)
}

/// Apply MongoDB injection.
#[wasm_bindgen]
pub fn mongodb_injection(input: &str) -> String {
    redstr::mongodb_injection(input)
}

// ============================================================================
// Bot Detection Transformations
// ============================================================================

/// Generate a random user agent string.
#[wasm_bindgen]
pub fn random_user_agent() -> String {
    redstr::random_user_agent()
}

/// Generate TLS fingerprint variation.
#[wasm_bindgen]
pub fn tls_fingerprint_variation(input: &str) -> String {
    redstr::tls_fingerprint_variation(input)
}

/// Generate accept language variation.
#[wasm_bindgen]
pub fn accept_language_variation(input: &str) -> String {
    redstr::accept_language_variation(input)
}

// ============================================================================
// Shell Transformations
// ============================================================================

/// Obfuscate PowerShell command.
#[wasm_bindgen]
pub fn powershell_obfuscate(input: &str) -> String {
    redstr::powershell_obfuscate(input)
}

/// Obfuscate Bash command.
#[wasm_bindgen]
pub fn bash_obfuscate(input: &str) -> String {
    redstr::bash_obfuscate(input)
}

/// Obfuscate environment variable.
#[wasm_bindgen]
pub fn env_var_obfuscate(input: &str) -> String {
    redstr::env_var_obfuscate(input)
}

/// Obfuscate file path.
#[wasm_bindgen]
pub fn file_path_obfuscate(input: &str) -> String {
    redstr::file_path_obfuscate(input)
}

// ============================================================================
// Web Security Transformations
// ============================================================================

/// GraphQL query obfuscation.
#[wasm_bindgen]
pub fn graphql_obfuscate(input: &str) -> String {
    redstr::graphql_obfuscate(input)
}

/// JWT header manipulation.
#[wasm_bindgen]
pub fn jwt_header_manipulation(input: &str) -> String {
    redstr::jwt_header_manipulation(input)
}

/// JWT payload obfuscation.
#[wasm_bindgen]
pub fn jwt_payload_obfuscate(input: &str) -> String {
    redstr::jwt_payload_obfuscate(input)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_leetspeak() {
        let result = leetspeak("password");
        assert!(!result.is_empty());
    }

    #[wasm_bindgen_test]
    fn test_base64_encode() {
        let result = base64_encode("hello");
        assert_eq!(result, "aGVsbG8=");
    }

    #[wasm_bindgen_test]
    fn test_rot13() {
        let result = rot13("hello");
        assert_eq!(result, "uryyb");
    }

    #[wasm_bindgen_test]
    fn test_random_user_agent() {
        let result = random_user_agent();
        assert!(result.contains("Mozilla") || result.contains("Chrome"));
    }
}
