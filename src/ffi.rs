//! C FFI bindings for the redstr library
//!
//! This module provides C-compatible function exports that can be called from other languages
//! like C#, Python, etc. All functions follow the same pattern:
//! - Take a C string pointer as input
//! - Return a pointer to a newly allocated C string
//! - Caller must free the returned string using `redstr_free_string`
//!
//! # Safety
//! All FFI functions in this module are unsafe and require:
//! - Input pointers must be valid null-terminated C strings
//! - Returned pointers must be freed using `redstr_free_string`
//! - Pointers must not be used after being freed

#![allow(clippy::missing_safety_doc)]

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Free a string allocated by redstr
///
/// # Safety
/// This function must only be called with pointers returned by other redstr FFI functions.
/// The pointer must not be null and must not have been freed already.
#[no_mangle]
pub unsafe extern "C" fn redstr_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        drop(CString::from_raw(ptr));
    }
}

/// Helper function to convert C string to Rust string and call transformation
///
/// # Safety
/// The input pointer must be a valid null-terminated C string.
unsafe fn transform_string<F>(input: *const c_char, transform: F) -> *mut c_char
where
    F: FnOnce(&str) -> String,
{
    if input.is_null() {
        return std::ptr::null_mut();
    }

    let c_str = match CStr::from_ptr(input).to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let result = transform(c_str);

    match CString::new(result) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

// Case transformations
#[no_mangle]
pub unsafe extern "C" fn redstr_randomize_capitalization(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::randomize_capitalization)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_case_swap(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::case_swap)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_alternate_case(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::alternate_case)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_inverse_case(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::inverse_case)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_to_camel_case(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::to_camel_case)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_to_snake_case(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::to_snake_case)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_to_kebab_case(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::to_kebab_case)
}

// Encoding transformations
#[no_mangle]
pub unsafe extern "C" fn redstr_base64_encode(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::base64_encode)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_url_encode(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::url_encode)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_hex_encode(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::hex_encode)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_hex_encode_mixed(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::hex_encode_mixed)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_html_entity_encode(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::html_entity_encode)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_mixed_encoding(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::mixed_encoding)
}

// Unicode transformations
#[no_mangle]
pub unsafe extern "C" fn redstr_homoglyph_substitution(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::homoglyph_substitution)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_unicode_variations(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::unicode_variations)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_zalgo_text(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::zalgo_text)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_space_variants(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::space_variants)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_unicode_normalize_variants(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::unicode_normalize_variants)
}

// Injection transformations
#[no_mangle]
pub unsafe extern "C" fn redstr_sql_comment_injection(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::sql_comment_injection)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_xss_tag_variations(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::xss_tag_variations)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_command_injection(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::command_injection)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_path_traversal(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::path_traversal)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_null_byte_injection(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::null_byte_injection)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_mongodb_injection(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::mongodb_injection)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_couchdb_injection(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::couchdb_injection)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_dynamodb_obfuscate(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::dynamodb_obfuscate)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_nosql_operator_injection(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::nosql_operator_injection)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_ssti_injection(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::ssti_injection)
}

// Note: ssti_framework_variation takes two parameters (template and framework)
// and is not exposed in the simple FFI interface. Use the Rust API directly or
// create a more complex FFI wrapper if needed.

#[no_mangle]
pub unsafe extern "C" fn redstr_ssti_syntax_obfuscate(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::ssti_syntax_obfuscate)
}

// Obfuscation transformations
#[no_mangle]
pub unsafe extern "C" fn redstr_leetspeak(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::leetspeak)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_rot13(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::rot13)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_double_characters(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::double_characters)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_vowel_swap(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::vowel_swap)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_reverse_string(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::reverse_string)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_js_string_concat(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::js_string_concat)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_whitespace_padding(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::whitespace_padding)
}

// Phishing transformations
#[no_mangle]
pub unsafe extern "C" fn redstr_domain_typosquat(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::domain_typosquat)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_advanced_domain_spoof(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::advanced_domain_spoof)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_email_obfuscation(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::email_obfuscation)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_url_shortening_pattern(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::url_shortening_pattern)
}

// Bot detection transformations (these return strings)
#[no_mangle]
pub unsafe extern "C" fn redstr_random_user_agent() -> *mut c_char {
    let result = crate::random_user_agent();
    match CString::new(result) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn redstr_tls_fingerprint_variation(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::tls_fingerprint_variation)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_http2_header_order(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::http2_header_order)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_accept_language_variation(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::accept_language_variation)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_cloudflare_challenge_variation(
    input: *const c_char,
) -> *mut c_char {
    transform_string(input, crate::cloudflare_challenge_variation)
}

// Cloudflare transformations
#[no_mangle]
pub unsafe extern "C" fn redstr_cloudflare_turnstile_variation(
    input: *const c_char,
) -> *mut c_char {
    transform_string(input, crate::cloudflare_turnstile_variation)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_cloudflare_challenge_response(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::cloudflare_challenge_response)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_tls_handshake_pattern(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::tls_handshake_pattern)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_canvas_fingerprint_variation(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::canvas_fingerprint_variation)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_webgl_fingerprint_obfuscate(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::webgl_fingerprint_obfuscate)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_font_fingerprint_consistency(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::font_fingerprint_consistency)
}

// Web security transformations
#[no_mangle]
pub unsafe extern "C" fn redstr_http_header_variation(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::http_header_variation)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_api_endpoint_variation(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::api_endpoint_variation)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_graphql_obfuscate(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::graphql_obfuscate)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_session_token_variation(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::session_token_variation)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_graphql_variable_injection(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::graphql_variable_injection)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_graphql_introspection_bypass(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::graphql_introspection_bypass)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_jwt_header_manipulation(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::jwt_header_manipulation)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_jwt_payload_obfuscate(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::jwt_payload_obfuscate)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_jwt_algorithm_confusion(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::jwt_algorithm_confusion)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_jwt_signature_bypass(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::jwt_signature_bypass)
}

// Shell transformations
#[no_mangle]
pub unsafe extern "C" fn redstr_powershell_obfuscate(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::powershell_obfuscate)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_bash_obfuscate(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::bash_obfuscate)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_env_var_obfuscate(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::env_var_obfuscate)
}

#[no_mangle]
pub unsafe extern "C" fn redstr_file_path_obfuscate(input: *const c_char) -> *mut c_char {
    transform_string(input, crate::file_path_obfuscate)
}
