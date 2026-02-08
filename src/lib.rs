// Suppress manual_is_multiple_of lint - is_multiple_of() is unstable in stable Rust.
// We use the % n == 0 pattern which is stable, idiomatic, and well-understood.
// Note: This lint exists in newer clippy versions. The allow(unknown_lints) allows
// referencing it even if it doesn't exist in the current clippy version.
#![allow(unknown_lints)]
#![allow(clippy::manual_is_multiple_of)]

//! # redstr
//!
//! A comprehensive string obfuscation and transformation library for security testing,
//! penetration testing, and red/blue/purple team operations.
//!
//! ## Overview
//!
//! `redstr` provides 60+ transformation functions organized into categories:
//! - **Case Transformations**: Modify capitalization patterns for filter bypass
//! - **Encoding**: Base64, URL encoding, hex encoding with various formats
//! - **Unicode**: Homoglyphs, zalgo text, Unicode variations for IDN spoofing
//! - **Injection Testing**: SQL, XSS, command injection, path traversal patterns
//! - **Obfuscation**: Leetspeak, ROT13, character doubling, JavaScript obfuscation
//! - **Phishing**: Domain typosquatting, email obfuscation, URL patterns
//! - **Bot Detection**: User-agent strings, TLS fingerprinting, header variations
//! - **Web Security**: JWT manipulation, GraphQL obfuscation, API endpoint testing
//! - **Shell**: PowerShell and Bash command obfuscation
//!
//! ## Features
//!
//! - **Zero Dependencies**: Uses only Rust standard library (optional serde support)
//! - **Security-Focused**: Designed specifically for offensive and defensive security
//! - **Production-Ready**: Well-tested with comprehensive test coverage
//! - **Builder Pattern**: Chain multiple transformations with `TransformBuilder`
//!
//! ## Quick Start
//!
//! ```rust
//! use redstr::{randomize_capitalization, leetspeak, homoglyph_substitution};
//!
//! // Random capitalization for filter bypass
//! let result = randomize_capitalization("Hello World");
//! // Output: "HeLlO wOrLd" (varies each run)
//!
//! // Leetspeak for content filter evasion
//! let obfuscated = leetspeak("password");
//! // Output: "p@55w0rd" or "p4$$w0rd"
//!
//! // Homoglyph substitution for phishing tests
//! let spoofed = homoglyph_substitution("admin@example.com");
//! // Output: "аdmіn@еxаmple.com" (Cyrillic characters)
//! ```
//!
//! ## Use Cases by Team
//!
//! ### Red Team / Offensive Security
//! - WAF bypass with case variations and encoding
//! - XSS payload obfuscation
//! - SQL injection with comment insertion
//! - Phishing domain generation
//! - Command injection testing
//!
//! ### Blue Team / Defensive Security
//! - Test security control effectiveness
//! - Validate input sanitization
//! - Test filter and detection systems
//! - Verify Unicode handling
//!
//! ### Purple Team / Security Testing
//! - Collaborative red/blue exercises
//! - Security tool validation
//! - Baseline security testing
//!
//! ## Transformation Categories
//!
//! See individual function documentation for detailed use cases and examples.

mod builder;
mod rng;
mod transformations;

// Re-export all public functions and types
pub use builder::TransformBuilder;

// Re-export case transformations
pub use transformations::case::{
    alternate_case, case_swap, case_swap_with_seed, inverse_case, randomize_capitalization,
    randomize_capitalization_with_seed, to_camel_case, to_kebab_case, to_snake_case,
};

// Re-export encoding transformations
pub use transformations::encoding::{
    base64_encode, hex_encode, hex_encode_mixed, html_entity_encode, mixed_encoding, url_encode,
};

// Re-export unicode transformations
pub use transformations::unicode::{
    homoglyph_substitution, space_variants, unicode_normalize_variants, unicode_variations,
    zalgo_text,
};

// Re-export injection transformations
pub use transformations::injection::{
    command_injection, couchdb_injection, dynamodb_obfuscate, mongodb_injection,
    nosql_operator_injection, null_byte_injection, path_traversal, sql_comment_injection,
    ssti_framework_variation, ssti_injection, ssti_syntax_obfuscate, xss_tag_variations,
};

// Re-export obfuscation transformations
pub use transformations::obfuscation::{
    double_characters, js_string_concat, leetspeak, reverse_string, rot13, vowel_swap,
    whitespace_padding,
};

// Re-export phishing transformations
pub use transformations::phishing::{
    advanced_domain_spoof, domain_typosquat, email_obfuscation, url_shortening_pattern,
};

// Re-export bot detection transformations
pub use transformations::bot_detection::{
    accept_language_variation, cloudflare_challenge_variation, http2_header_order,
    random_user_agent, tls_fingerprint_variation,
};

// Re-export cloudflare transformations
pub use transformations::cloudflare::{
    canvas_fingerprint_variation, cloudflare_challenge_response, cloudflare_turnstile_variation,
    font_fingerprint_consistency, tls_handshake_pattern, webgl_fingerprint_obfuscate,
};

// Re-export web security transformations
pub use transformations::web_security::{
    api_endpoint_variation, graphql_introspection_bypass, graphql_obfuscate,
    graphql_variable_injection, html_form_action_variation, html_form_field_obfuscate,
    html_input_attribute_variation, html_input_type_variation, html_input_value_obfuscate,
    http_header_variation, jwt_algorithm_confusion, jwt_header_manipulation, jwt_payload_obfuscate,
    jwt_signature_bypass, session_token_variation,
};

// Re-export shell transformations
pub use transformations::shell::{
    bash_obfuscate, env_var_obfuscate, file_path_obfuscate, powershell_obfuscate,
};

// Re-export RNG so users can provide deterministic seeds where needed.
pub use rng::SimpleRng;
