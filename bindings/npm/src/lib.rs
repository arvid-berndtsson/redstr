#![deny(clippy::all)]

use napi_derive::napi;

// Case transformations
#[napi]
pub fn randomize_capitalization(input: String) -> String {
    redstr::randomize_capitalization(&input)
}

#[napi]
pub fn alternate_case(input: String) -> String {
    redstr::alternate_case(&input)
}

#[napi]
pub fn case_swap(input: String) -> String {
    redstr::case_swap(&input)
}

#[napi]
pub fn inverse_case(input: String) -> String {
    redstr::inverse_case(&input)
}

#[napi]
pub fn to_camel_case(input: String) -> String {
    redstr::to_camel_case(&input)
}

#[napi]
pub fn to_snake_case(input: String) -> String {
    redstr::to_snake_case(&input)
}

#[napi]
pub fn to_kebab_case(input: String) -> String {
    redstr::to_kebab_case(&input)
}

// Encoding transformations
#[napi]
pub fn base64_encode(input: String) -> String {
    redstr::base64_encode(&input)
}

#[napi]
pub fn url_encode(input: String) -> String {
    redstr::url_encode(&input)
}

#[napi]
pub fn hex_encode(input: String) -> String {
    redstr::hex_encode(&input)
}

#[napi]
pub fn hex_encode_mixed(input: String) -> String {
    redstr::hex_encode_mixed(&input)
}

#[napi]
pub fn html_entity_encode(input: String) -> String {
    redstr::html_entity_encode(&input)
}

#[napi]
pub fn mixed_encoding(input: String) -> String {
    redstr::mixed_encoding(&input)
}

// Unicode transformations
#[napi]
pub fn homoglyph_substitution(input: String) -> String {
    redstr::homoglyph_substitution(&input)
}

#[napi]
pub fn unicode_variations(input: String) -> String {
    redstr::unicode_variations(&input)
}

#[napi]
pub fn zalgo_text(input: String) -> String {
    redstr::zalgo_text(&input)
}

#[napi]
pub fn space_variants(input: String) -> String {
    redstr::space_variants(&input)
}

#[napi]
pub fn unicode_normalize_variants(input: String) -> String {
    redstr::unicode_normalize_variants(&input)
}

// Injection transformations
#[napi]
pub fn sql_comment_injection(input: String) -> String {
    redstr::sql_comment_injection(&input)
}

#[napi]
pub fn xss_tag_variations(input: String) -> String {
    redstr::xss_tag_variations(&input)
}

#[napi]
pub fn null_byte_injection(input: String) -> String {
    redstr::null_byte_injection(&input)
}

#[napi]
pub fn path_traversal(input: String) -> String {
    redstr::path_traversal(&input)
}

#[napi]
pub fn command_injection(input: String) -> String {
    redstr::command_injection(&input)
}

#[napi]
pub fn mongodb_injection(input: String) -> String {
    redstr::mongodb_injection(&input)
}

#[napi]
pub fn couchdb_injection(input: String) -> String {
    redstr::couchdb_injection(&input)
}

#[napi]
pub fn dynamodb_obfuscate(input: String) -> String {
    redstr::dynamodb_obfuscate(&input)
}

#[napi]
pub fn nosql_operator_injection(input: String) -> String {
    redstr::nosql_operator_injection(&input)
}

#[napi]
pub fn ssti_injection(input: String) -> String {
    redstr::ssti_injection(&input)
}

#[napi]
pub fn ssti_framework_variation(template: String, framework: String) -> String {
    redstr::ssti_framework_variation(&template, &framework)
}

#[napi]
pub fn ssti_syntax_obfuscate(input: String) -> String {
    redstr::ssti_syntax_obfuscate(&input)
}

// Obfuscation transformations
#[napi]
pub fn leetspeak(input: String) -> String {
    redstr::leetspeak(&input)
}

#[napi]
pub fn rot13(input: String) -> String {
    redstr::rot13(&input)
}

#[napi]
pub fn vowel_swap(input: String) -> String {
    redstr::vowel_swap(&input)
}

#[napi]
pub fn double_characters(input: String) -> String {
    redstr::double_characters(&input)
}

#[napi]
pub fn reverse_string(input: String) -> String {
    redstr::reverse_string(&input)
}

#[napi]
pub fn whitespace_padding(input: String) -> String {
    redstr::whitespace_padding(&input)
}

#[napi]
pub fn js_string_concat(input: String) -> String {
    redstr::js_string_concat(&input)
}

// Phishing transformations
#[napi]
pub fn domain_typosquat(input: String) -> String {
    redstr::domain_typosquat(&input)
}

#[napi]
pub fn advanced_domain_spoof(input: String) -> String {
    redstr::advanced_domain_spoof(&input)
}

#[napi]
pub fn email_obfuscation(input: String) -> String {
    redstr::email_obfuscation(&input)
}

#[napi]
pub fn url_shortening_pattern(input: String) -> String {
    redstr::url_shortening_pattern(&input)
}

// Bot detection transformations
#[napi]
pub fn random_user_agent() -> String {
    redstr::random_user_agent()
}

#[napi]
pub fn tls_fingerprint_variation(input: String) -> String {
    redstr::tls_fingerprint_variation(&input)
}

#[napi]
pub fn http2_header_order(input: String) -> String {
    redstr::http2_header_order(&input)
}

#[napi]
pub fn accept_language_variation(input: String) -> String {
    redstr::accept_language_variation(&input)
}

#[napi]
pub fn cloudflare_challenge_variation(input: String) -> String {
    redstr::cloudflare_challenge_variation(&input)
}

// Cloudflare transformations
#[napi]
pub fn cloudflare_turnstile_variation(input: String) -> String {
    redstr::cloudflare_turnstile_variation(&input)
}

#[napi]
pub fn cloudflare_challenge_response(input: String) -> String {
    redstr::cloudflare_challenge_response(&input)
}

#[napi]
pub fn tls_handshake_pattern(input: String) -> String {
    redstr::tls_handshake_pattern(&input)
}

#[napi]
pub fn canvas_fingerprint_variation(input: String) -> String {
    redstr::canvas_fingerprint_variation(&input)
}

#[napi]
pub fn webgl_fingerprint_obfuscate(input: String) -> String {
    redstr::webgl_fingerprint_obfuscate(&input)
}

#[napi]
pub fn font_fingerprint_consistency(input: String) -> String {
    redstr::font_fingerprint_consistency(&input)
}

// Web security transformations
#[napi]
pub fn http_header_variation(input: String) -> String {
    redstr::http_header_variation(&input)
}

#[napi]
pub fn api_endpoint_variation(input: String) -> String {
    redstr::api_endpoint_variation(&input)
}

#[napi]
pub fn graphql_obfuscate(input: String) -> String {
    redstr::graphql_obfuscate(&input)
}

#[napi]
pub fn session_token_variation(input: String) -> String {
    redstr::session_token_variation(&input)
}

#[napi]
pub fn graphql_variable_injection(input: String) -> String {
    redstr::graphql_variable_injection(&input)
}

#[napi]
pub fn graphql_introspection_bypass(input: String) -> String {
    redstr::graphql_introspection_bypass(&input)
}

#[napi]
pub fn jwt_header_manipulation(input: String) -> String {
    redstr::jwt_header_manipulation(&input)
}

#[napi]
pub fn jwt_payload_obfuscate(input: String) -> String {
    redstr::jwt_payload_obfuscate(&input)
}

#[napi]
pub fn jwt_algorithm_confusion(input: String) -> String {
    redstr::jwt_algorithm_confusion(&input)
}

#[napi]
pub fn jwt_signature_bypass(input: String) -> String {
    redstr::jwt_signature_bypass(&input)
}

// Shell transformations
#[napi]
pub fn powershell_obfuscate(input: String) -> String {
    redstr::powershell_obfuscate(&input)
}

#[napi]
pub fn bash_obfuscate(input: String) -> String {
    redstr::bash_obfuscate(&input)
}

#[napi]
pub fn env_var_obfuscate(input: String) -> String {
    redstr::env_var_obfuscate(&input)
}

#[napi]
pub fn file_path_obfuscate(input: String) -> String {
    redstr::file_path_obfuscate(&input)
}
