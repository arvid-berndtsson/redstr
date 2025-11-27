//! Foreign Function Interface (FFI) for C/Go bindings
//!
//! This module provides C-compatible exports for all redstr transformations,
//! enabling usage from Go (via CGO) and other languages that support C FFI.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// Import all transformation functions
use crate::transformations::bot_detection::*;
use crate::transformations::case::*;
use crate::transformations::cloudflare::*;
use crate::transformations::encoding::*;
use crate::transformations::injection::*;
use crate::transformations::obfuscation::*;
use crate::transformations::phishing::*;
use crate::transformations::shell::*;
use crate::transformations::unicode::*;
use crate::transformations::web_security::*;

/// Helper function to convert C string to Rust string
///
/// # Safety
/// The caller must ensure that `ptr` is a valid null-terminated C string
unsafe fn c_str_to_rust(ptr: *const c_char) -> String {
    if ptr.is_null() {
        return String::new();
    }
    CStr::from_ptr(ptr).to_string_lossy().into_owned()
}

/// Helper function to convert Rust string to C string
///
/// Returns a pointer that must be freed by calling `redstr_free_string`
fn rust_str_to_c(s: String) -> *mut c_char {
    CString::new(s).unwrap_or_default().into_raw()
}

/// Free a string allocated by redstr
///
/// # Safety
/// The caller must ensure that `ptr` was allocated by a redstr function
/// and has not been freed before
/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        drop(CString::from_raw(ptr));
    }
}

// Case transformations

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_randomize_capitalization(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(randomize_capitalization(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_case_swap(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(case_swap(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_alternate_case(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(alternate_case(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_inverse_case(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(inverse_case(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_to_camel_case(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(to_camel_case(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_to_snake_case(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(to_snake_case(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_to_kebab_case(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(to_kebab_case(&rust_input))
}

// Encoding transformations

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_base64_encode(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(base64_encode(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_url_encode(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(url_encode(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_hex_encode(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(hex_encode(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_hex_encode_mixed(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(hex_encode_mixed(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_html_entity_encode(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(html_entity_encode(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_mixed_encoding(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(mixed_encoding(&rust_input))
}

// Unicode transformations

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_homoglyph_substitution(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(homoglyph_substitution(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_unicode_variations(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(unicode_variations(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_zalgo_text(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(zalgo_text(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_space_variants(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(space_variants(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_unicode_normalize_variants(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(unicode_normalize_variants(&rust_input))
}

// Injection transformations

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_sql_comment_injection(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(sql_comment_injection(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_xss_tag_variations(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(xss_tag_variations(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_null_byte_injection(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(null_byte_injection(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_path_traversal(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(path_traversal(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_command_injection(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(command_injection(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_mongodb_injection(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(mongodb_injection(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_couchdb_injection(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(couchdb_injection(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_dynamodb_obfuscate(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(dynamodb_obfuscate(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_nosql_operator_injection(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(nosql_operator_injection(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_ssti_injection(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(ssti_injection(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_ssti_framework_variation(
    template: *const c_char,
    framework: *const c_char,
) -> *mut c_char {
    let rust_template = c_str_to_rust(template);
    let rust_framework = c_str_to_rust(framework);
    rust_str_to_c(ssti_framework_variation(&rust_template, &rust_framework))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_ssti_syntax_obfuscate(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(ssti_syntax_obfuscate(&rust_input))
}

// Obfuscation transformations

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_leetspeak(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(leetspeak(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_rot13(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(rot13(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_vowel_swap(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(vowel_swap(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_double_characters(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(double_characters(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_reverse_string(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(reverse_string(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_whitespace_padding(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(whitespace_padding(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_js_string_concat(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(js_string_concat(&rust_input))
}

// Phishing transformations

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_domain_typosquat(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(domain_typosquat(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_advanced_domain_spoof(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(advanced_domain_spoof(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_email_obfuscation(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(email_obfuscation(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_url_shortening_pattern(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(url_shortening_pattern(&rust_input))
}

// Bot detection transformations

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_random_user_agent() -> *mut c_char {
    rust_str_to_c(random_user_agent())
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_tls_fingerprint_variation(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(tls_fingerprint_variation(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_accept_language_variation(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(accept_language_variation(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_cloudflare_challenge_variation(
    input: *const c_char,
) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(cloudflare_challenge_variation(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_http2_header_order(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(http2_header_order(&rust_input))
}

// Cloudflare transformations

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_cloudflare_turnstile_variation(
    input: *const c_char,
) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(cloudflare_turnstile_variation(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_cloudflare_challenge_response(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(cloudflare_challenge_response(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_tls_handshake_pattern(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(tls_handshake_pattern(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_canvas_fingerprint_variation(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(canvas_fingerprint_variation(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_webgl_fingerprint_obfuscate(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(webgl_fingerprint_obfuscate(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_font_fingerprint_consistency(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(font_fingerprint_consistency(&rust_input))
}

// Web security transformations

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_http_header_variation(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(http_header_variation(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_api_endpoint_variation(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(api_endpoint_variation(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_graphql_obfuscate(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(graphql_obfuscate(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_session_token_variation(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(session_token_variation(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_graphql_variable_injection(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(graphql_variable_injection(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_graphql_introspection_bypass(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(graphql_introspection_bypass(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_jwt_header_manipulation(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(jwt_header_manipulation(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_jwt_payload_obfuscate(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(jwt_payload_obfuscate(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_jwt_algorithm_confusion(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(jwt_algorithm_confusion(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_jwt_signature_bypass(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(jwt_signature_bypass(&rust_input))
}

// Shell transformations

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_powershell_obfuscate(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(powershell_obfuscate(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_bash_obfuscate(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(bash_obfuscate(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_env_var_obfuscate(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(env_var_obfuscate(&rust_input))
}

/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_file_path_obfuscate(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(file_path_obfuscate(&rust_input))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_ffi_leetspeak() {
        let input = CString::new("password").unwrap();
        let result = unsafe { redstr_leetspeak(input.as_ptr()) };
        assert!(!result.is_null());

        let result_str = unsafe { CStr::from_ptr(result).to_string_lossy().into_owned() };
        assert!(
            result_str.contains('4')
                || result_str.contains('@')
                || result_str.contains('5')
                || result_str.contains('0')
        );

        unsafe { redstr_free_string(result) };
    }

    #[test]
    fn test_ffi_base64_encode() {
        let input = CString::new("hello").unwrap();
        let result = unsafe { redstr_base64_encode(input.as_ptr()) };
        assert!(!result.is_null());

        let result_str = unsafe { CStr::from_ptr(result).to_string_lossy().into_owned() };
        assert_eq!(result_str, "aGVsbG8=");

        unsafe { redstr_free_string(result) };
    }

    #[test]
    fn test_ffi_null_handling() {
        let result = unsafe { redstr_leetspeak(std::ptr::null()) };
        assert!(!result.is_null());
        unsafe { redstr_free_string(result) };
    }
}
