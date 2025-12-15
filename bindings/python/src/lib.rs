//! # redstr-python
//!
//! Native Python bindings for redstr using PyO3.
//!
//! This crate provides zero-overhead access to all redstr transformation functions
//! from Python applications.
//!
//! ## Installation
//!
//! ```bash
//! pip install redstr
//! ```
//!
//! ## Usage
//!
//! ```python
//! from redstr import leetspeak, TransformBuilder
//!
//! # Direct function call
//! encoded = leetspeak('password')  # 'p@55w0rd'
//!
//! # Builder pattern
//! builder = TransformBuilder('<script>')
//! builder.case_swap()
//! builder.url_encode()
//! payload = builder.build()
//! ```

use pyo3::prelude::*;

// ============================================================================
// Case Transformations
// ============================================================================

/// Randomize the capitalization of each character.
#[pyfunction]
fn randomize_capitalization(input: &str) -> String {
    redstr::randomize_capitalization(input)
}

/// Swap the case of each character.
#[pyfunction]
fn case_swap(input: &str) -> String {
    redstr::case_swap(input)
}

/// Alternate case for each character.
#[pyfunction]
fn alternate_case(input: &str) -> String {
    redstr::alternate_case(input)
}

/// Inverse case transformation.
#[pyfunction]
fn inverse_case(input: &str) -> String {
    redstr::inverse_case(input)
}

/// Convert to camelCase.
#[pyfunction]
fn to_camel_case(input: &str) -> String {
    redstr::to_camel_case(input)
}

/// Convert to snake_case.
#[pyfunction]
fn to_snake_case(input: &str) -> String {
    redstr::to_snake_case(input)
}

/// Convert to kebab-case.
#[pyfunction]
fn to_kebab_case(input: &str) -> String {
    redstr::to_kebab_case(input)
}

// ============================================================================
// Encoding Transformations
// ============================================================================

/// Encode string to Base64.
#[pyfunction]
fn base64_encode(input: &str) -> String {
    redstr::base64_encode(input)
}

/// URL encode a string.
#[pyfunction]
fn url_encode(input: &str) -> String {
    redstr::url_encode(input)
}

/// Hex encode a string.
#[pyfunction]
fn hex_encode(input: &str) -> String {
    redstr::hex_encode(input)
}

/// Hex encode with mixed case.
#[pyfunction]
fn hex_encode_mixed(input: &str) -> String {
    redstr::hex_encode_mixed(input)
}

/// HTML entity encode a string.
#[pyfunction]
fn html_entity_encode(input: &str) -> String {
    redstr::html_entity_encode(input)
}

/// Apply mixed encoding.
#[pyfunction]
fn mixed_encoding(input: &str) -> String {
    redstr::mixed_encoding(input)
}

// ============================================================================
// Obfuscation Transformations
// ============================================================================

/// Apply leetspeak transformation.
#[pyfunction]
fn leetspeak(input: &str) -> String {
    redstr::leetspeak(input)
}

/// Apply ROT13 cipher.
#[pyfunction]
fn rot13(input: &str) -> String {
    redstr::rot13(input)
}

/// Reverse a string.
#[pyfunction]
fn reverse_string(input: &str) -> String {
    redstr::reverse_string(input)
}

/// Double each character in the string.
#[pyfunction]
fn double_characters(input: &str) -> String {
    redstr::double_characters(input)
}

/// Swap vowels in the string.
#[pyfunction]
fn vowel_swap(input: &str) -> String {
    redstr::vowel_swap(input)
}

/// Add whitespace padding.
#[pyfunction]
fn whitespace_padding(input: &str) -> String {
    redstr::whitespace_padding(input)
}

/// JavaScript string concatenation obfuscation.
#[pyfunction]
fn js_string_concat(input: &str) -> String {
    redstr::js_string_concat(input)
}

// ============================================================================
// Unicode Transformations
// ============================================================================

/// Apply homoglyph substitution.
#[pyfunction]
fn homoglyph_substitution(input: &str) -> String {
    redstr::homoglyph_substitution(input)
}

/// Apply zalgo text effect.
#[pyfunction]
fn zalgo_text(input: &str) -> String {
    redstr::zalgo_text(input)
}

/// Apply Unicode variations.
#[pyfunction]
fn unicode_variations(input: &str) -> String {
    redstr::unicode_variations(input)
}

/// Apply space variants.
#[pyfunction]
fn space_variants(input: &str) -> String {
    redstr::space_variants(input)
}

/// Apply Unicode normalization variants.
#[pyfunction]
fn unicode_normalize_variants(input: &str) -> String {
    redstr::unicode_normalize_variants(input)
}

// ============================================================================
// Phishing Transformations
// ============================================================================

/// Generate typosquatted domain.
#[pyfunction]
fn domain_typosquat(input: &str) -> String {
    redstr::domain_typosquat(input)
}

/// Obfuscate email address.
#[pyfunction]
fn email_obfuscation(input: &str) -> String {
    redstr::email_obfuscation(input)
}

/// Generate URL shortening pattern.
#[pyfunction]
fn url_shortening_pattern(input: &str) -> String {
    redstr::url_shortening_pattern(input)
}

/// Advanced domain spoofing.
#[pyfunction]
fn advanced_domain_spoof(input: &str) -> String {
    redstr::advanced_domain_spoof(input)
}

// ============================================================================
// Injection Transformations
// ============================================================================

/// Apply XSS tag variations.
#[pyfunction]
fn xss_tag_variations(input: &str) -> String {
    redstr::xss_tag_variations(input)
}

/// Apply SQL comment injection.
#[pyfunction]
fn sql_comment_injection(input: &str) -> String {
    redstr::sql_comment_injection(input)
}

/// Apply command injection patterns.
#[pyfunction]
fn command_injection(input: &str) -> String {
    redstr::command_injection(input)
}

/// Apply path traversal patterns.
#[pyfunction]
fn path_traversal(input: &str) -> String {
    redstr::path_traversal(input)
}

/// Apply null byte injection.
#[pyfunction]
fn null_byte_injection(input: &str) -> String {
    redstr::null_byte_injection(input)
}

/// Apply SSTI injection.
#[pyfunction]
fn ssti_injection(input: &str) -> String {
    redstr::ssti_injection(input)
}

/// Apply MongoDB injection.
#[pyfunction]
fn mongodb_injection(input: &str) -> String {
    redstr::mongodb_injection(input)
}

// ============================================================================
// Bot Detection Transformations
// ============================================================================

/// Generate a random user agent string.
#[pyfunction]
fn random_user_agent() -> String {
    redstr::random_user_agent()
}

/// Generate TLS fingerprint variation.
#[pyfunction]
fn tls_fingerprint_variation(input: &str) -> String {
    redstr::tls_fingerprint_variation(input)
}

/// Generate accept language variation.
#[pyfunction]
fn accept_language_variation(input: &str) -> String {
    redstr::accept_language_variation(input)
}

// ============================================================================
// Shell Transformations
// ============================================================================

/// Obfuscate PowerShell command.
#[pyfunction]
fn powershell_obfuscate(input: &str) -> String {
    redstr::powershell_obfuscate(input)
}

/// Obfuscate Bash command.
#[pyfunction]
fn bash_obfuscate(input: &str) -> String {
    redstr::bash_obfuscate(input)
}

/// Obfuscate environment variable.
#[pyfunction]
fn env_var_obfuscate(input: &str) -> String {
    redstr::env_var_obfuscate(input)
}

/// Obfuscate file path.
#[pyfunction]
fn file_path_obfuscate(input: &str) -> String {
    redstr::file_path_obfuscate(input)
}

// ============================================================================
// Web Security Transformations
// ============================================================================

/// GraphQL query obfuscation.
#[pyfunction]
fn graphql_obfuscate(input: &str) -> String {
    redstr::graphql_obfuscate(input)
}

/// JWT header manipulation.
#[pyfunction]
fn jwt_header_manipulation(input: &str) -> String {
    redstr::jwt_header_manipulation(input)
}

/// JWT payload obfuscation.
#[pyfunction]
fn jwt_payload_obfuscate(input: &str) -> String {
    redstr::jwt_payload_obfuscate(input)
}

// ============================================================================
// TransformBuilder
// ============================================================================

/// Builder for chaining multiple transformations.
///
/// Example:
///     builder = TransformBuilder('<script>')
///     builder.case_swap()
///     builder.url_encode()
///     result = builder.build()
#[pyclass]
struct TransformBuilder {
    text: String,
}

#[pymethods]
impl TransformBuilder {
    /// Create a new TransformBuilder with the given input.
    #[new]
    fn new(input: &str) -> Self {
        Self {
            text: input.to_string(),
        }
    }

    /// Apply leetspeak transformation.
    fn leetspeak(&mut self) {
        self.text = redstr::leetspeak(&self.text);
    }

    /// Apply Base64 encoding.
    fn base64(&mut self) {
        self.text = redstr::base64_encode(&self.text);
    }

    /// Apply URL encoding.
    fn url_encode(&mut self) {
        self.text = redstr::url_encode(&self.text);
    }

    /// Apply case swap.
    fn case_swap(&mut self) {
        self.text = redstr::case_swap(&self.text);
    }

    /// Apply ROT13.
    fn rot13(&mut self) {
        self.text = redstr::rot13(&self.text);
    }

    /// Apply hex encoding.
    fn hex_encode(&mut self) {
        self.text = redstr::hex_encode(&self.text);
    }

    /// Apply homoglyph substitution.
    fn homoglyphs(&mut self) {
        self.text = redstr::homoglyph_substitution(&self.text);
    }

    /// Reverse the string.
    fn reverse(&mut self) {
        self.text = redstr::reverse_string(&self.text);
    }

    /// Build and return the final transformed string.
    fn build(&self) -> String {
        self.text.clone()
    }
}

// ============================================================================
// Module Definition
// ============================================================================

/// Native string transformations for security testing.
#[pymodule]
fn _redstr(_py: Python, m: &PyModule) -> PyResult<()> {
    // Case transformations
    m.add_function(wrap_pyfunction!(randomize_capitalization, m)?)?;
    m.add_function(wrap_pyfunction!(case_swap, m)?)?;
    m.add_function(wrap_pyfunction!(alternate_case, m)?)?;
    m.add_function(wrap_pyfunction!(inverse_case, m)?)?;
    m.add_function(wrap_pyfunction!(to_camel_case, m)?)?;
    m.add_function(wrap_pyfunction!(to_snake_case, m)?)?;
    m.add_function(wrap_pyfunction!(to_kebab_case, m)?)?;

    // Encoding transformations
    m.add_function(wrap_pyfunction!(base64_encode, m)?)?;
    m.add_function(wrap_pyfunction!(url_encode, m)?)?;
    m.add_function(wrap_pyfunction!(hex_encode, m)?)?;
    m.add_function(wrap_pyfunction!(hex_encode_mixed, m)?)?;
    m.add_function(wrap_pyfunction!(html_entity_encode, m)?)?;
    m.add_function(wrap_pyfunction!(mixed_encoding, m)?)?;

    // Obfuscation transformations
    m.add_function(wrap_pyfunction!(leetspeak, m)?)?;
    m.add_function(wrap_pyfunction!(rot13, m)?)?;
    m.add_function(wrap_pyfunction!(reverse_string, m)?)?;
    m.add_function(wrap_pyfunction!(double_characters, m)?)?;
    m.add_function(wrap_pyfunction!(vowel_swap, m)?)?;
    m.add_function(wrap_pyfunction!(whitespace_padding, m)?)?;
    m.add_function(wrap_pyfunction!(js_string_concat, m)?)?;

    // Unicode transformations
    m.add_function(wrap_pyfunction!(homoglyph_substitution, m)?)?;
    m.add_function(wrap_pyfunction!(zalgo_text, m)?)?;
    m.add_function(wrap_pyfunction!(unicode_variations, m)?)?;
    m.add_function(wrap_pyfunction!(space_variants, m)?)?;
    m.add_function(wrap_pyfunction!(unicode_normalize_variants, m)?)?;

    // Phishing transformations
    m.add_function(wrap_pyfunction!(domain_typosquat, m)?)?;
    m.add_function(wrap_pyfunction!(email_obfuscation, m)?)?;
    m.add_function(wrap_pyfunction!(url_shortening_pattern, m)?)?;
    m.add_function(wrap_pyfunction!(advanced_domain_spoof, m)?)?;

    // Injection transformations
    m.add_function(wrap_pyfunction!(xss_tag_variations, m)?)?;
    m.add_function(wrap_pyfunction!(sql_comment_injection, m)?)?;
    m.add_function(wrap_pyfunction!(command_injection, m)?)?;
    m.add_function(wrap_pyfunction!(path_traversal, m)?)?;
    m.add_function(wrap_pyfunction!(null_byte_injection, m)?)?;
    m.add_function(wrap_pyfunction!(ssti_injection, m)?)?;
    m.add_function(wrap_pyfunction!(mongodb_injection, m)?)?;

    // Bot detection transformations
    m.add_function(wrap_pyfunction!(random_user_agent, m)?)?;
    m.add_function(wrap_pyfunction!(tls_fingerprint_variation, m)?)?;
    m.add_function(wrap_pyfunction!(accept_language_variation, m)?)?;

    // Shell transformations
    m.add_function(wrap_pyfunction!(powershell_obfuscate, m)?)?;
    m.add_function(wrap_pyfunction!(bash_obfuscate, m)?)?;
    m.add_function(wrap_pyfunction!(env_var_obfuscate, m)?)?;
    m.add_function(wrap_pyfunction!(file_path_obfuscate, m)?)?;

    // Web security transformations
    m.add_function(wrap_pyfunction!(graphql_obfuscate, m)?)?;
    m.add_function(wrap_pyfunction!(jwt_header_manipulation, m)?)?;
    m.add_function(wrap_pyfunction!(jwt_payload_obfuscate, m)?)?;

    // Classes
    m.add_class::<TransformBuilder>()?;

    Ok(())
}
