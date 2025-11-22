// Note: Clippy may suggest using is_multiple_of() which is unstable in stable Rust.
// We use the % n == 0 pattern which is stable, idiomatic, and well-understood.
// This lint may not be available in all Rust versions, so we document the rationale here.

//! # redstr
//!
//! A versatile string obfuscation and transformation library for security testing.
//! Useful for red team, blue team, and purple team activities.
//!
//! ## Features
//!
//! - Zero dependencies - uses only Rust standard library
//! - Multiple transformation modes for various security testing scenarios
//! - Simple API for integration into security tools
//!
//! ## Usage
//!
//! ```rust
//! use redstr::{randomize_capitalization, leetspeak, homoglyph_substitution};
//!
//! // Random capitalization
//! let result = randomize_capitalization("Hello World");
//! println!("{}", result);
//!
//! // Leetspeak transformation
//! let obfuscated = leetspeak("password");
//! println!("{}", obfuscated);
//!
//! // Homoglyph substitution for phishing tests
//! let spoofed = homoglyph_substitution("admin@example.com");
//! println!("{}", spoofed);
//! ```

mod builder;
mod rng;
mod transformations;

// Re-export all public functions and types
pub use builder::TransformBuilder;

// Re-export case transformations
pub use transformations::case::{
    alternate_case, case_swap, inverse_case, randomize_capitalization, to_camel_case,
    to_kebab_case, to_snake_case,
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
    command_injection, null_byte_injection, path_traversal, sql_comment_injection,
    xss_tag_variations,
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

// Re-export web security transformations
pub use transformations::web_security::{
    api_endpoint_variation, graphql_obfuscate, http_header_variation, session_token_variation,
};

// Re-export shell transformations
pub use transformations::shell::{
    bash_obfuscate, env_var_obfuscate, file_path_obfuscate, powershell_obfuscate,
};
