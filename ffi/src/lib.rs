//! # redstr-ffi
//!
//! C-compatible FFI bindings for redstr.
//!
//! This crate provides a C-compatible interface to all redstr transformation functions,
//! enabling integration with any language that supports calling C functions (Python, C#, Ruby, etc.).
//!
//! ## Memory Management
//!
//! All strings returned by redstr functions are heap-allocated and must be freed using
//! `redstr_free_string()` to avoid memory leaks.
//!
//! ## Example (C)
//!
//! ```c
//! #include "redstr.h"
//!
//! int main() {
//!     char* result = redstr_leetspeak("password");
//!     printf("%s\n", result);  // "p@55w0rd"
//!     redstr_free_string(result);
//!     return 0;
//! }
//! ```

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// ============================================================================
// Memory Management
// ============================================================================

/// Free a string that was allocated by a redstr function.
///
/// # Safety
///
/// The pointer must have been returned by a redstr function and must not have
/// been freed already. Passing a null pointer is safe (no-op).
#[no_mangle]
pub unsafe extern "C" fn redstr_free_string(s: *mut c_char) {
    if !s.is_null() {
        drop(CString::from_raw(s));
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Convert a C string to a Rust &str, returning None if invalid
unsafe fn c_str_to_str<'a>(s: *const c_char) -> Option<&'a str> {
    if s.is_null() {
        return None;
    }
    CStr::from_ptr(s).to_str().ok()
}

/// Convert a Rust String to a C string, returning null on failure
fn string_to_c_char(s: String) -> *mut c_char {
    match CString::new(s) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

// ============================================================================
// Case Transformations
// ============================================================================

/// Randomize the capitalization of each character.
///
/// # Safety
///
/// `input` must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn redstr_randomize_capitalization(input: *const c_char) -> *mut c_char {
    let input_str = match c_str_to_str(input) {
        Some(s) => s,
        None => return std::ptr::null_mut(),
    };
    string_to_c_char(redstr::randomize_capitalization(input_str))
}

/// Swap the case of each character.
///
/// # Safety
///
/// `input` must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn redstr_case_swap(input: *const c_char) -> *mut c_char {
    let input_str = match c_str_to_str(input) {
        Some(s) => s,
        None => return std::ptr::null_mut(),
    };
    string_to_c_char(redstr::case_swap(input_str))
}

/// Alternate case for each character.
///
/// # Safety
///
/// `input` must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn redstr_alternate_case(input: *const c_char) -> *mut c_char {
    let input_str = match c_str_to_str(input) {
        Some(s) => s,
        None => return std::ptr::null_mut(),
    };
    string_to_c_char(redstr::alternate_case(input_str))
}

/// Inverse case transformation.
///
/// # Safety
///
/// `input` must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn redstr_inverse_case(input: *const c_char) -> *mut c_char {
    let input_str = match c_str_to_str(input) {
        Some(s) => s,
        None => return std::ptr::null_mut(),
    };
    string_to_c_char(redstr::inverse_case(input_str))
}

// ============================================================================
// Encoding Transformations
// ============================================================================

/// Encode string to Base64.
///
/// # Safety
///
/// `input` must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn redstr_base64_encode(input: *const c_char) -> *mut c_char {
    let input_str = match c_str_to_str(input) {
        Some(s) => s,
        None => return std::ptr::null_mut(),
    };
    string_to_c_char(redstr::base64_encode(input_str))
}

/// URL encode a string.
///
/// # Safety
///
/// `input` must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn redstr_url_encode(input: *const c_char) -> *mut c_char {
    let input_str = match c_str_to_str(input) {
        Some(s) => s,
        None => return std::ptr::null_mut(),
    };
    string_to_c_char(redstr::url_encode(input_str))
}

/// Hex encode a string.
///
/// # Safety
///
/// `input` must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn redstr_hex_encode(input: *const c_char) -> *mut c_char {
    let input_str = match c_str_to_str(input) {
        Some(s) => s,
        None => return std::ptr::null_mut(),
    };
    string_to_c_char(redstr::hex_encode(input_str))
}

/// HTML entity encode a string.
///
/// # Safety
///
/// `input` must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn redstr_html_entity_encode(input: *const c_char) -> *mut c_char {
    let input_str = match c_str_to_str(input) {
        Some(s) => s,
        None => return std::ptr::null_mut(),
    };
    string_to_c_char(redstr::html_entity_encode(input_str))
}

// ============================================================================
// Obfuscation Transformations
// ============================================================================

/// Apply leetspeak transformation.
///
/// # Safety
///
/// `input` must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn redstr_leetspeak(input: *const c_char) -> *mut c_char {
    let input_str = match c_str_to_str(input) {
        Some(s) => s,
        None => return std::ptr::null_mut(),
    };
    string_to_c_char(redstr::leetspeak(input_str))
}

/// Apply ROT13 cipher.
///
/// # Safety
///
/// `input` must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn redstr_rot13(input: *const c_char) -> *mut c_char {
    let input_str = match c_str_to_str(input) {
        Some(s) => s,
        None => return std::ptr::null_mut(),
    };
    string_to_c_char(redstr::rot13(input_str))
}

/// Reverse a string.
///
/// # Safety
///
/// `input` must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn redstr_reverse_string(input: *const c_char) -> *mut c_char {
    let input_str = match c_str_to_str(input) {
        Some(s) => s,
        None => return std::ptr::null_mut(),
    };
    string_to_c_char(redstr::reverse_string(input_str))
}

/// Double each character in the string.
///
/// # Safety
///
/// `input` must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn redstr_double_characters(input: *const c_char) -> *mut c_char {
    let input_str = match c_str_to_str(input) {
        Some(s) => s,
        None => return std::ptr::null_mut(),
    };
    string_to_c_char(redstr::double_characters(input_str))
}

// ============================================================================
// Unicode Transformations
// ============================================================================

/// Apply homoglyph substitution.
///
/// # Safety
///
/// `input` must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn redstr_homoglyph_substitution(input: *const c_char) -> *mut c_char {
    let input_str = match c_str_to_str(input) {
        Some(s) => s,
        None => return std::ptr::null_mut(),
    };
    string_to_c_char(redstr::homoglyph_substitution(input_str))
}

/// Apply zalgo text effect.
///
/// # Safety
///
/// `input` must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn redstr_zalgo_text(input: *const c_char) -> *mut c_char {
    let input_str = match c_str_to_str(input) {
        Some(s) => s,
        None => return std::ptr::null_mut(),
    };
    string_to_c_char(redstr::zalgo_text(input_str))
}

// ============================================================================
// Phishing Transformations
// ============================================================================

/// Generate typosquatted domain.
///
/// # Safety
///
/// `input` must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn redstr_domain_typosquat(input: *const c_char) -> *mut c_char {
    let input_str = match c_str_to_str(input) {
        Some(s) => s,
        None => return std::ptr::null_mut(),
    };
    string_to_c_char(redstr::domain_typosquat(input_str))
}

/// Obfuscate email address.
///
/// # Safety
///
/// `input` must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn redstr_email_obfuscation(input: *const c_char) -> *mut c_char {
    let input_str = match c_str_to_str(input) {
        Some(s) => s,
        None => return std::ptr::null_mut(),
    };
    string_to_c_char(redstr::email_obfuscation(input_str))
}

// ============================================================================
// Injection Transformations
// ============================================================================

/// Apply XSS tag variations.
///
/// # Safety
///
/// `input` must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn redstr_xss_tag_variations(input: *const c_char) -> *mut c_char {
    let input_str = match c_str_to_str(input) {
        Some(s) => s,
        None => return std::ptr::null_mut(),
    };
    string_to_c_char(redstr::xss_tag_variations(input_str))
}

/// Apply SQL comment injection.
///
/// # Safety
///
/// `input` must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn redstr_sql_comment_injection(input: *const c_char) -> *mut c_char {
    let input_str = match c_str_to_str(input) {
        Some(s) => s,
        None => return std::ptr::null_mut(),
    };
    string_to_c_char(redstr::sql_comment_injection(input_str))
}

/// Apply command injection patterns.
///
/// # Safety
///
/// `input` must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn redstr_command_injection(input: *const c_char) -> *mut c_char {
    let input_str = match c_str_to_str(input) {
        Some(s) => s,
        None => return std::ptr::null_mut(),
    };
    string_to_c_char(redstr::command_injection(input_str))
}

/// Apply path traversal patterns.
///
/// # Safety
///
/// `input` must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn redstr_path_traversal(input: *const c_char) -> *mut c_char {
    let input_str = match c_str_to_str(input) {
        Some(s) => s,
        None => return std::ptr::null_mut(),
    };
    string_to_c_char(redstr::path_traversal(input_str))
}

// ============================================================================
// Bot Detection Transformations
// ============================================================================

/// Generate a random user agent string.
///
/// # Safety
///
/// This function is always safe to call.
#[no_mangle]
pub extern "C" fn redstr_random_user_agent() -> *mut c_char {
    string_to_c_char(redstr::random_user_agent())
}

// ============================================================================
// Shell Transformations
// ============================================================================

/// Obfuscate PowerShell command.
///
/// # Safety
///
/// `input` must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn redstr_powershell_obfuscate(input: *const c_char) -> *mut c_char {
    let input_str = match c_str_to_str(input) {
        Some(s) => s,
        None => return std::ptr::null_mut(),
    };
    string_to_c_char(redstr::powershell_obfuscate(input_str))
}

/// Obfuscate Bash command.
///
/// # Safety
///
/// `input` must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn redstr_bash_obfuscate(input: *const c_char) -> *mut c_char {
    let input_str = match c_str_to_str(input) {
        Some(s) => s,
        None => return std::ptr::null_mut(),
    };
    string_to_c_char(redstr::bash_obfuscate(input_str))
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_leetspeak_ffi() {
        unsafe {
            let input = CString::new("password").unwrap();
            let result = redstr_leetspeak(input.as_ptr());
            assert!(!result.is_null());
            let result_str = CStr::from_ptr(result).to_str().unwrap();
            assert!(!result_str.is_empty());
            redstr_free_string(result);
        }
    }

    #[test]
    fn test_base64_encode_ffi() {
        unsafe {
            let input = CString::new("hello").unwrap();
            let result = redstr_base64_encode(input.as_ptr());
            assert!(!result.is_null());
            let result_str = CStr::from_ptr(result).to_str().unwrap();
            assert_eq!(result_str, "aGVsbG8=");
            redstr_free_string(result);
        }
    }

    #[test]
    fn test_null_input() {
        unsafe {
            let result = redstr_leetspeak(std::ptr::null());
            assert!(result.is_null());
        }
    }

    #[test]
    fn test_free_null() {
        unsafe {
            // Should not crash
            redstr_free_string(std::ptr::null_mut());
        }
    }

    #[test]
    fn test_random_user_agent_ffi() {
        let result = redstr_random_user_agent();
        assert!(!result.is_null());
        unsafe {
            let result_str = CStr::from_ptr(result).to_str().unwrap();
            assert!(result_str.contains("Mozilla") || result_str.contains("Chrome"));
            redstr_free_string(result);
        }
    }
}
