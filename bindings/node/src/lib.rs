//! # redstr-node
//!
//! Native Node.js bindings for redstr using napi-rs.
//!
//! This crate provides zero-overhead access to all redstr transformation functions
//! from Node.js and TypeScript applications.
//!
//! ## Installation
//!
//! ```bash
//! npm install redstr
//! ```
//!
//! ## Usage
//!
//! ```javascript
//! const { leetspeak, TransformBuilder } = require('redstr');
//!
//! // Direct function call
//! const encoded = leetspeak('password'); // 'p@55w0rd'
//!
//! // Builder pattern
//! const payload = new TransformBuilder('<script>')
//!     .caseSwap()
//!     .urlEncode()
//!     .build();
//! ```

use napi_derive::napi;

// ============================================================================
// Case Transformations
// ============================================================================

/// Randomize the capitalization of each character.
#[napi]
pub fn randomize_capitalization(input: String) -> String {
    redstr::randomize_capitalization(&input)
}

/// Swap the case of each character.
#[napi]
pub fn case_swap(input: String) -> String {
    redstr::case_swap(&input)
}

/// Alternate case for each character.
#[napi]
pub fn alternate_case(input: String) -> String {
    redstr::alternate_case(&input)
}

/// Inverse case transformation.
#[napi]
pub fn inverse_case(input: String) -> String {
    redstr::inverse_case(&input)
}

/// Convert to camelCase.
#[napi]
pub fn to_camel_case(input: String) -> String {
    redstr::to_camel_case(&input)
}

/// Convert to snake_case.
#[napi]
pub fn to_snake_case(input: String) -> String {
    redstr::to_snake_case(&input)
}

/// Convert to kebab-case.
#[napi]
pub fn to_kebab_case(input: String) -> String {
    redstr::to_kebab_case(&input)
}

// ============================================================================
// Encoding Transformations
// ============================================================================

/// Encode string to Base64.
#[napi]
pub fn base64_encode(input: String) -> String {
    redstr::base64_encode(&input)
}

/// URL encode a string.
#[napi]
pub fn url_encode(input: String) -> String {
    redstr::url_encode(&input)
}

/// Hex encode a string.
#[napi]
pub fn hex_encode(input: String) -> String {
    redstr::hex_encode(&input)
}

/// Hex encode with mixed case.
#[napi]
pub fn hex_encode_mixed(input: String) -> String {
    redstr::hex_encode_mixed(&input)
}

/// HTML entity encode a string.
#[napi]
pub fn html_entity_encode(input: String) -> String {
    redstr::html_entity_encode(&input)
}

/// Apply mixed encoding.
#[napi]
pub fn mixed_encoding(input: String) -> String {
    redstr::mixed_encoding(&input)
}

// ============================================================================
// Obfuscation Transformations
// ============================================================================

/// Apply leetspeak transformation.
#[napi]
pub fn leetspeak(input: String) -> String {
    redstr::leetspeak(&input)
}

/// Apply ROT13 cipher.
#[napi]
pub fn rot13(input: String) -> String {
    redstr::rot13(&input)
}

/// Reverse a string.
#[napi]
pub fn reverse_string(input: String) -> String {
    redstr::reverse_string(&input)
}

/// Double each character in the string.
#[napi]
pub fn double_characters(input: String) -> String {
    redstr::double_characters(&input)
}

/// Swap vowels in the string.
#[napi]
pub fn vowel_swap(input: String) -> String {
    redstr::vowel_swap(&input)
}

/// Add whitespace padding.
#[napi]
pub fn whitespace_padding(input: String) -> String {
    redstr::whitespace_padding(&input)
}

/// JavaScript string concatenation obfuscation.
#[napi]
pub fn js_string_concat(input: String) -> String {
    redstr::js_string_concat(&input)
}

// ============================================================================
// Unicode Transformations
// ============================================================================

/// Apply homoglyph substitution.
#[napi]
pub fn homoglyph_substitution(input: String) -> String {
    redstr::homoglyph_substitution(&input)
}

/// Apply zalgo text effect.
#[napi]
pub fn zalgo_text(input: String) -> String {
    redstr::zalgo_text(&input)
}

/// Apply Unicode variations.
#[napi]
pub fn unicode_variations(input: String) -> String {
    redstr::unicode_variations(&input)
}

/// Apply space variants.
#[napi]
pub fn space_variants(input: String) -> String {
    redstr::space_variants(&input)
}

/// Apply Unicode normalization variants.
#[napi]
pub fn unicode_normalize_variants(input: String) -> String {
    redstr::unicode_normalize_variants(&input)
}

// ============================================================================
// Phishing Transformations
// ============================================================================

/// Generate typosquatted domain.
#[napi]
pub fn domain_typosquat(input: String) -> String {
    redstr::domain_typosquat(&input)
}

/// Obfuscate email address.
#[napi]
pub fn email_obfuscation(input: String) -> String {
    redstr::email_obfuscation(&input)
}

/// Generate URL shortening pattern.
#[napi]
pub fn url_shortening_pattern(input: String) -> String {
    redstr::url_shortening_pattern(&input)
}

/// Advanced domain spoofing.
#[napi]
pub fn advanced_domain_spoof(input: String) -> String {
    redstr::advanced_domain_spoof(&input)
}

// ============================================================================
// Injection Transformations
// ============================================================================

/// Apply XSS tag variations.
#[napi]
pub fn xss_tag_variations(input: String) -> String {
    redstr::xss_tag_variations(&input)
}

/// Apply SQL comment injection.
#[napi]
pub fn sql_comment_injection(input: String) -> String {
    redstr::sql_comment_injection(&input)
}

/// Apply command injection patterns.
#[napi]
pub fn command_injection(input: String) -> String {
    redstr::command_injection(&input)
}

/// Apply path traversal patterns.
#[napi]
pub fn path_traversal(input: String) -> String {
    redstr::path_traversal(&input)
}

/// Apply null byte injection.
#[napi]
pub fn null_byte_injection(input: String) -> String {
    redstr::null_byte_injection(&input)
}

/// Apply SSTI injection.
#[napi]
pub fn ssti_injection(input: String) -> String {
    redstr::ssti_injection(&input)
}

/// Apply MongoDB injection.
#[napi]
pub fn mongodb_injection(input: String) -> String {
    redstr::mongodb_injection(&input)
}

// ============================================================================
// Bot Detection Transformations
// ============================================================================

/// Generate a random user agent string.
#[napi]
pub fn random_user_agent() -> String {
    redstr::random_user_agent()
}

/// Generate TLS fingerprint variation.
#[napi]
pub fn tls_fingerprint_variation(input: String) -> String {
    redstr::tls_fingerprint_variation(&input)
}

/// Generate accept language variation.
#[napi]
pub fn accept_language_variation(input: String) -> String {
    redstr::accept_language_variation(&input)
}

// ============================================================================
// Shell Transformations
// ============================================================================

/// Obfuscate PowerShell command.
#[napi]
pub fn powershell_obfuscate(input: String) -> String {
    redstr::powershell_obfuscate(&input)
}

/// Obfuscate Bash command.
#[napi]
pub fn bash_obfuscate(input: String) -> String {
    redstr::bash_obfuscate(&input)
}

/// Obfuscate environment variable.
#[napi]
pub fn env_var_obfuscate(input: String) -> String {
    redstr::env_var_obfuscate(&input)
}

/// Obfuscate file path.
#[napi]
pub fn file_path_obfuscate(input: String) -> String {
    redstr::file_path_obfuscate(&input)
}

// ============================================================================
// Web Security Transformations
// ============================================================================

/// GraphQL query obfuscation.
#[napi]
pub fn graphql_obfuscate(input: String) -> String {
    redstr::graphql_obfuscate(&input)
}

/// JWT header manipulation.
#[napi]
pub fn jwt_header_manipulation(input: String) -> String {
    redstr::jwt_header_manipulation(&input)
}

/// JWT payload obfuscation.
#[napi]
pub fn jwt_payload_obfuscate(input: String) -> String {
    redstr::jwt_payload_obfuscate(&input)
}

// ============================================================================
// TransformBuilder
// ============================================================================

/// Builder for chaining multiple transformations.
#[napi]
#[derive(Clone)]
pub struct TransformBuilder {
    text: String,
}

#[napi]
impl TransformBuilder {
    /// Create a new TransformBuilder with the given input.
    #[napi(constructor)]
    pub fn new(input: String) -> Self {
        Self { text: input }
    }

    /// Apply leetspeak transformation.
    #[napi]
    pub fn leetspeak(&mut self) -> Self {
        self.text = redstr::leetspeak(&self.text);
        self.clone()
    }

    /// Apply Base64 encoding.
    #[napi]
    pub fn base64(&mut self) -> Self {
        self.text = redstr::base64_encode(&self.text);
        self.clone()
    }

    /// Apply URL encoding.
    #[napi]
    pub fn url_encode(&mut self) -> Self {
        self.text = redstr::url_encode(&self.text);
        self.clone()
    }

    /// Apply case swap.
    #[napi]
    pub fn case_swap(&mut self) -> Self {
        self.text = redstr::case_swap(&self.text);
        self.clone()
    }

    /// Apply ROT13.
    #[napi]
    pub fn rot13(&mut self) -> Self {
        self.text = redstr::rot13(&self.text);
        self.clone()
    }

    /// Apply hex encoding.
    #[napi]
    pub fn hex_encode(&mut self) -> Self {
        self.text = redstr::hex_encode(&self.text);
        self.clone()
    }

    /// Apply homoglyph substitution.
    #[napi]
    pub fn homoglyphs(&mut self) -> Self {
        self.text = redstr::homoglyph_substitution(&self.text);
        self.clone()
    }

    /// Reverse the string.
    #[napi]
    pub fn reverse(&mut self) -> Self {
        self.text = redstr::reverse_string(&self.text);
        self.clone()
    }

    /// Build and return the final transformed string.
    #[napi]
    pub fn build(&self) -> String {
        self.text.clone()
    }
}
