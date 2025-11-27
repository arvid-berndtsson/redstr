//! Python bindings for redstr using PyO3
//!
//! This module provides Python-friendly wrappers for all redstr transformations.

use pyo3::prelude::*;
use pyo3::types::PyModule;

// Import all transformations from the library
use crate::{
    // Bot detection transformations
    accept_language_variation,
    // Phishing transformations
    advanced_domain_spoof,
    // Case transformations
    alternate_case,
    // Web security transformations
    api_endpoint_variation,
    // Encoding transformations
    base64_encode,
    // Shell transformations
    bash_obfuscate,
    // Cloudflare transformations
    canvas_fingerprint_variation,
    case_swap,
    cloudflare_challenge_response,
    cloudflare_challenge_variation,
    cloudflare_turnstile_variation,
    // Injection transformations
    command_injection,
    couchdb_injection,
    domain_typosquat,
    // Obfuscation transformations
    double_characters,
    dynamodb_obfuscate,
    email_obfuscation,
    env_var_obfuscate,
    file_path_obfuscate,
    font_fingerprint_consistency,
    graphql_introspection_bypass,
    graphql_obfuscate,
    graphql_variable_injection,
    hex_encode,
    hex_encode_mixed,
    // Unicode transformations
    homoglyph_substitution,
    html_entity_encode,
    http2_header_order,
    http_header_variation,
    inverse_case,
    js_string_concat,
    jwt_algorithm_confusion,
    jwt_header_manipulation,
    jwt_payload_obfuscate,
    jwt_signature_bypass,
    leetspeak,
    mixed_encoding,
    mongodb_injection,
    nosql_operator_injection,
    null_byte_injection,
    path_traversal,
    powershell_obfuscate,
    random_user_agent,
    randomize_capitalization,
    reverse_string,
    rot13,
    session_token_variation,
    space_variants,
    sql_comment_injection,
    ssti_framework_variation,
    ssti_injection,
    ssti_syntax_obfuscate,
    tls_fingerprint_variation,
    tls_handshake_pattern,
    to_camel_case,
    to_kebab_case,
    to_snake_case,
    unicode_normalize_variants,
    unicode_variations,
    url_encode,
    url_shortening_pattern,
    vowel_swap,
    webgl_fingerprint_obfuscate,
    whitespace_padding,
    xss_tag_variations,
    zalgo_text,
};

// ============================================================================
// Case Transformations
// ============================================================================

/// Alternates between lowercase and uppercase for each character.
///
/// Args:
///     text (str): Input text to transform
///
/// Returns:
///     str: Text with alternating case
///
/// Example:
///     >>> import redstr
///     >>> redstr.alternate_case("hello")
///     'hElLo'
#[pyfunction]
fn py_alternate_case(text: &str) -> String {
    alternate_case(text)
}

/// Swaps the case of each character (uppercase becomes lowercase and vice versa).
///
/// Args:
///     text (str): Input text to transform
///
/// Returns:
///     str: Text with swapped case
///
/// Example:
///     >>> import redstr
///     >>> redstr.case_swap("Hello World")
///     'hELLO wORLD'
#[pyfunction]
fn py_case_swap(text: &str) -> String {
    case_swap(text)
}

/// Inverts the case (uppercase to lowercase, vice versa).
///
/// Args:
///     text (str): Input text to transform
///
/// Returns:
///     str: Text with inverted case
///
/// Example:
///     >>> import redstr
///     >>> redstr.inverse_case("Hello")
///     'hELLO'
#[pyfunction]
fn py_inverse_case(text: &str) -> String {
    inverse_case(text)
}

/// Randomly capitalizes characters in the input string.
///
/// Args:
///     text (str): Input text to transform
///
/// Returns:
///     str: Text with random capitalization
///
/// Example:
///     >>> import redstr
///     >>> redstr.randomize_capitalization("hello world")
///     'HeLlO WoRLd'  # Output varies
#[pyfunction]
fn py_randomize_capitalization(text: &str) -> String {
    randomize_capitalization(text)
}

/// Converts text to camelCase.
///
/// Args:
///     text (str): Input text to transform
///
/// Returns:
///     str: Text in camelCase
///
/// Example:
///     >>> import redstr
///     >>> redstr.to_camel_case("hello world")
///     'helloWorld'
#[pyfunction]
fn py_to_camel_case(text: &str) -> String {
    to_camel_case(text)
}

/// Converts text to kebab-case.
///
/// Args:
///     text (str): Input text to transform
///
/// Returns:
///     str: Text in kebab-case
///
/// Example:
///     >>> import redstr
///     >>> redstr.to_kebab_case("Hello World")
///     'hello-world'
#[pyfunction]
fn py_to_kebab_case(text: &str) -> String {
    to_kebab_case(text)
}

/// Converts text to snake_case.
///
/// Args:
///     text (str): Input text to transform
///
/// Returns:
///     str: Text in snake_case
///
/// Example:
///     >>> import redstr
///     >>> redstr.to_snake_case("Hello World")
///     'hello_world'
#[pyfunction]
fn py_to_snake_case(text: &str) -> String {
    to_snake_case(text)
}

// ============================================================================
// Encoding Transformations
// ============================================================================

/// Encodes the input string in Base64.
///
/// Args:
///     text (str): Input text to encode
///
/// Returns:
///     str: Base64 encoded text
///
/// Example:
///     >>> import redstr
///     >>> redstr.base64_encode("hello")
///     'aGVsbG8='
#[pyfunction]
fn py_base64_encode(text: &str) -> String {
    base64_encode(text)
}

/// Encodes the input string in hexadecimal format.
///
/// Args:
///     text (str): Input text to encode
///
/// Returns:
///     str: Hexadecimal encoded text
///
/// Example:
///     >>> import redstr
///     >>> redstr.hex_encode("hello")
///     '68656c6c6f'
#[pyfunction]
fn py_hex_encode(text: &str) -> String {
    hex_encode(text)
}

/// Encodes the input string with mixed hex formats (0x prefix, \\x prefix).
///
/// Args:
///     text (str): Input text to encode
///
/// Returns:
///     str: Mixed hex encoded text
///
/// Example:
///     >>> import redstr
///     >>> redstr.hex_encode_mixed("hi")
///     '0x68\\x69'  # Format varies
#[pyfunction]
fn py_hex_encode_mixed(text: &str) -> String {
    hex_encode_mixed(text)
}

/// Encodes the input string using HTML entities.
///
/// Args:
///     text (str): Input text to encode
///
/// Returns:
///     str: HTML entity encoded text
///
/// Example:
///     >>> import redstr
///     >>> redstr.html_entity_encode("<script>")
///     '&lt;script&gt;'
#[pyfunction]
fn py_html_entity_encode(text: &str) -> String {
    html_entity_encode(text)
}

/// Applies mixed encoding (base64, hex, URL) to the input.
///
/// Args:
///     text (str): Input text to encode
///
/// Returns:
///     str: Mixed encoded text
///
/// Example:
///     >>> import redstr
///     >>> redstr.mixed_encoding("hello")
///     'aG%45%6C%6Co'  # Output varies
#[pyfunction]
fn py_mixed_encoding(text: &str) -> String {
    mixed_encoding(text)
}

/// URL encodes the input string.
///
/// Args:
///     text (str): Input text to encode
///
/// Returns:
///     str: URL encoded text
///
/// Example:
///     >>> import redstr
///     >>> redstr.url_encode("hello world")
///     'hello%20world'
#[pyfunction]
fn py_url_encode(text: &str) -> String {
    url_encode(text)
}

// ============================================================================
// Unicode Transformations
// ============================================================================

/// Substitutes characters with visually similar Unicode homoglyphs.
///
/// Args:
///     text (str): Input text to transform
///
/// Returns:
///     str: Text with homoglyph substitutions
///
/// Example:
///     >>> import redstr
///     >>> redstr.homoglyph_substitution("admin")
///     'аdmіn'  # Uses Cyrillic characters
#[pyfunction]
fn py_homoglyph_substitution(text: &str) -> String {
    homoglyph_substitution(text)
}

/// Replaces spaces with Unicode space variants.
///
/// Args:
///     text (str): Input text to transform
///
/// Returns:
///     str: Text with space variants
///
/// Example:
///     >>> import redstr
///     >>> redstr.space_variants("hello world")
///     'hello world'  # Uses different Unicode spaces
#[pyfunction]
fn py_space_variants(text: &str) -> String {
    space_variants(text)
}

/// Applies Unicode normalization variants (NFC, NFD, NFKC, NFKD).
///
/// Args:
///     text (str): Input text to transform
///
/// Returns:
///     str: Text with Unicode normalization
///
/// Example:
///     >>> import redstr
///     >>> redstr.unicode_normalize_variants("café")
///     'café'  # Normalization form varies
#[pyfunction]
fn py_unicode_normalize_variants(text: &str) -> String {
    unicode_normalize_variants(text)
}

/// Substitutes characters with Unicode variations.
///
/// Args:
///     text (str): Input text to transform
///
/// Returns:
///     str: Text with Unicode variations
///
/// Example:
///     >>> import redstr
///     >>> redstr.unicode_variations("hello")
///     'ℎℯ𝓁𝓁ℴ'  # Uses Unicode variations
#[pyfunction]
fn py_unicode_variations(text: &str) -> String {
    unicode_variations(text)
}

/// Adds zalgo text combining diacritical marks.
///
/// Args:
///     text (str): Input text to transform
///
/// Returns:
///     str: Text with zalgo effect
///
/// Example:
///     >>> import redstr
///     >>> redstr.zalgo_text("hello")
///     'h̷̢͇e̸̛͜l̵͙̀l̶̰̂ȯ̴̡'  # Diacritics vary
#[pyfunction]
fn py_zalgo_text(text: &str) -> String {
    zalgo_text(text)
}

// ============================================================================
// Injection Transformations
// ============================================================================

/// Generates command injection test payloads.
///
/// Args:
///     text (str): Input text to transform
///
/// Returns:
///     str: Command injection payload
///
/// Example:
///     >>> import redstr
///     >>> redstr.command_injection("cat /etc/passwd")
///     'cat /etc/passwd; echo vulnerable'
#[pyfunction]
fn py_command_injection(text: &str) -> String {
    command_injection(text)
}

/// Generates CouchDB injection test payloads.
///
/// Args:
///     text (str): Input text to transform
///
/// Returns:
///     str: CouchDB injection payload
///
/// Example:
///     >>> import redstr
///     >>> redstr.couchdb_injection("admin")
///     '{"selector":{"user":"admin"}}'
#[pyfunction]
fn py_couchdb_injection(text: &str) -> String {
    couchdb_injection(text)
}

/// Generates DynamoDB obfuscation test payloads.
///
/// Args:
///     text (str): Input text to transform
///
/// Returns:
///     str: DynamoDB obfuscated payload
///
/// Example:
///     >>> import redstr
///     >>> redstr.dynamodb_obfuscate("user")
///     '{"user": {"S": "user"}}'
#[pyfunction]
fn py_dynamodb_obfuscate(text: &str) -> String {
    dynamodb_obfuscate(text)
}

/// Generates MongoDB injection test payloads.
///
/// Args:
///     text (str): Input text to transform
///
/// Returns:
///     str: MongoDB injection payload
///
/// Example:
///     >>> import redstr
///     >>> redstr.mongodb_injection("admin")
///     '{"$ne": "admin"}'
#[pyfunction]
fn py_mongodb_injection(text: &str) -> String {
    mongodb_injection(text)
}

/// Generates NoSQL operator injection test payloads.
///
/// Args:
///     text (str): Input text to transform
///
/// Returns:
///     str: NoSQL operator injection payload
///
/// Example:
///     >>> import redstr
///     >>> redstr.nosql_operator_injection("value")
///     '{"$gt": "value"}'
#[pyfunction]
fn py_nosql_operator_injection(text: &str) -> String {
    nosql_operator_injection(text)
}

/// Inserts null bytes for injection testing.
///
/// Args:
///     text (str): Input text to transform
///
/// Returns:
///     str: Text with null bytes
///
/// Example:
///     >>> import redstr
///     >>> redstr.null_byte_injection("file.txt")
///     'file.txt\\x00.jpg'
#[pyfunction]
fn py_null_byte_injection(text: &str) -> String {
    null_byte_injection(text)
}

/// Generates path traversal test payloads.
///
/// Args:
///     text (str): Input text to transform
///
/// Returns:
///     str: Path traversal payload
///
/// Example:
///     >>> import redstr
///     >>> redstr.path_traversal("etc/passwd")
///     '../../../etc/passwd'
#[pyfunction]
fn py_path_traversal(text: &str) -> String {
    path_traversal(text)
}

/// Inserts SQL comments for injection testing.
///
/// Args:
///     text (str): Input text to transform
///
/// Returns:
///     str: SQL with comment injection
///
/// Example:
///     >>> import redstr
///     >>> redstr.sql_comment_injection("SELECT * FROM users")
///     'SELECT/**/\\*/**/FROM/**/users'
#[pyfunction]
fn py_sql_comment_injection(text: &str) -> String {
    sql_comment_injection(text)
}

/// Generates SSTI framework-specific variations.
///
/// Args:
///     template (str): Template to transform
///     framework (str): Framework name (e.g., "jinja2", "django")
///
/// Returns:
///     str: SSTI framework variation
///
/// Example:
///     >>> import redstr
///     >>> redstr.ssti_framework_variation("{{7*7}}", "jinja2")
///     '{{7*7}}'  # Framework-specific syntax
#[pyfunction]
fn py_ssti_framework_variation(template: &str, framework: &str) -> String {
    ssti_framework_variation(template, framework)
}

/// Generates SSTI (Server-Side Template Injection) test payloads.
///
/// Args:
///     text (str): Input text to transform
///
/// Returns:
///     str: SSTI payload
///
/// Example:
///     >>> import redstr
///     >>> redstr.ssti_injection("name")
///     '{{name}}'
#[pyfunction]
fn py_ssti_injection(text: &str) -> String {
    ssti_injection(text)
}

/// Obfuscates SSTI syntax.
///
/// Args:
///     text (str): Input text to transform
///
/// Returns:
///     str: Obfuscated SSTI syntax
///
/// Example:
///     >>> import redstr
///     >>> redstr.ssti_syntax_obfuscate("{{7*7}}")
///     '{%7B7*7%7D'
#[pyfunction]
fn py_ssti_syntax_obfuscate(text: &str) -> String {
    ssti_syntax_obfuscate(text)
}

/// Generates XSS tag variations for testing.
///
/// Args:
///     text (str): Input text to transform
///
/// Returns:
///     str: XSS tag variation
///
/// Example:
///     >>> import redstr
///     >>> redstr.xss_tag_variations("alert(1)")
///     '<script>alert(1)</script>'
#[pyfunction]
fn py_xss_tag_variations(text: &str) -> String {
    xss_tag_variations(text)
}

// ============================================================================
// Obfuscation Transformations
// ============================================================================

/// Doubles each character in the string.
///
/// Args:
///     text (str): Input text to transform
///
/// Returns:
///     str: Text with doubled characters
///
/// Example:
///     >>> import redstr
///     >>> redstr.double_characters("hello")
///     'hheelllloo'
#[pyfunction]
fn py_double_characters(text: &str) -> String {
    double_characters(text)
}

/// Obfuscates JavaScript strings using concatenation.
///
/// Args:
///     text (str): Input text to transform
///
/// Returns:
///     str: JavaScript string concatenation
///
/// Example:
///     >>> import redstr
///     >>> redstr.js_string_concat("alert")
///     '"al"+"er"+"t"'
#[pyfunction]
fn py_js_string_concat(text: &str) -> String {
    js_string_concat(text)
}

/// Converts text to leetspeak (1337 speak).
///
/// Args:
///     text (str): Input text to transform
///
/// Returns:
///     str: Leetspeak text
///
/// Example:
///     >>> import redstr
///     >>> redstr.leetspeak("elite")
///     '3l1t3'
#[pyfunction]
fn py_leetspeak(text: &str) -> String {
    leetspeak(text)
}

/// Reverses the input string.
///
/// Args:
///     text (str): Input text to reverse
///
/// Returns:
///     str: Reversed text
///
/// Example:
///     >>> import redstr
///     >>> redstr.reverse_string("hello")
///     'olleh'
#[pyfunction]
fn py_reverse_string(text: &str) -> String {
    reverse_string(text)
}

/// Applies ROT13 cipher to the input.
///
/// Args:
///     text (str): Input text to transform
///
/// Returns:
///     str: ROT13 encoded text
///
/// Example:
///     >>> import redstr
///     >>> redstr.rot13("hello")
///     'uryyb'
#[pyfunction]
fn py_rot13(text: &str) -> String {
    rot13(text)
}

/// Swaps vowels with other vowels randomly.
///
/// Args:
///     text (str): Input text to transform
///
/// Returns:
///     str: Text with swapped vowels
///
/// Example:
///     >>> import redstr
///     >>> redstr.vowel_swap("hello")
///     'hillu'  # Vowels vary
#[pyfunction]
fn py_vowel_swap(text: &str) -> String {
    vowel_swap(text)
}

/// Adds random whitespace padding to the input.
///
/// Args:
///     text (str): Input text to transform
///
/// Returns:
///     str: Text with whitespace padding
///
/// Example:
///     >>> import redstr
///     >>> redstr.whitespace_padding("hello")
///     '  hello  '  # Padding varies
#[pyfunction]
fn py_whitespace_padding(text: &str) -> String {
    whitespace_padding(text)
}

// ============================================================================
// Phishing Transformations
// ============================================================================

/// Generates advanced domain spoofing variations.
///
/// Args:
///     text (str): Domain to spoof
///
/// Returns:
///     str: Spoofed domain
///
/// Example:
///     >>> import redstr
///     >>> redstr.advanced_domain_spoof("paypal.com")
///     'рaypal.com'  # Uses homoglyphs
#[pyfunction]
fn py_advanced_domain_spoof(text: &str) -> String {
    advanced_domain_spoof(text)
}

/// Generates typosquatting domain variations.
///
/// Args:
///     text (str): Domain to typosquat
///
/// Returns:
///     str: Typosquatted domain
///
/// Example:
///     >>> import redstr
///     >>> redstr.domain_typosquat("google.com")
///     'gooogle.com'
#[pyfunction]
fn py_domain_typosquat(text: &str) -> String {
    domain_typosquat(text)
}

/// Obfuscates email addresses.
///
/// Args:
///     text (str): Email to obfuscate
///
/// Returns:
///     str: Obfuscated email
///
/// Example:
///     >>> import redstr
///     >>> redstr.email_obfuscation("admin@example.com")
///     'admin[at]example[dot]com'
#[pyfunction]
fn py_email_obfuscation(text: &str) -> String {
    email_obfuscation(text)
}

/// Generates URL shortening patterns.
///
/// Args:
///     text (str): URL to shorten pattern
///
/// Returns:
///     str: URL shortening pattern
///
/// Example:
///     >>> import redstr
///     >>> redstr.url_shortening_pattern("https://example.com")
///     'https://bit.ly/abc123'  # Pattern varies
#[pyfunction]
fn py_url_shortening_pattern(text: &str) -> String {
    url_shortening_pattern(text)
}

// ============================================================================
// Bot Detection Transformations
// ============================================================================

/// Generates Accept-Language header variations.
///
/// Args:
///     text (str): Input language
///
/// Returns:
///     str: Accept-Language header
///
/// Example:
///     >>> import redstr
///     >>> redstr.accept_language_variation("en")
///     'en-US,en;q=0.9'
#[pyfunction]
fn py_accept_language_variation(text: &str) -> String {
    accept_language_variation(text)
}

/// Generates Cloudflare challenge response variations.
///
/// Args:
///     text (str): Input challenge
///
/// Returns:
///     str: Challenge response
///
/// Example:
///     >>> import redstr
///     >>> redstr.cloudflare_challenge_variation("challenge")
///     'cf_clearance=...'
#[pyfunction]
fn py_cloudflare_challenge_variation(text: &str) -> String {
    cloudflare_challenge_variation(text)
}

/// Generates HTTP/2 header order variations.
///
/// Args:
///     text (str): Input headers
///
/// Returns:
///     str: Reordered headers
///
/// Example:
///     >>> import redstr
///     >>> redstr.http2_header_order("User-Agent: ...")
///     ':method: GET\\n:path: /\\nuser-agent: ...'
#[pyfunction]
fn py_http2_header_order(text: &str) -> String {
    http2_header_order(text)
}

/// Generates random user agent strings.
///
/// Returns:
///     str: Random user agent
///
/// Example:
///     >>> import redstr
///     >>> redstr.random_user_agent()
///     'Mozilla/5.0 (Windows NT 10.0; Win64; x64)...'
#[pyfunction]
fn py_random_user_agent() -> String {
    random_user_agent()
}

/// Generates TLS fingerprint variations.
///
/// Args:
///     text (str): Input fingerprint
///
/// Returns:
///     str: TLS fingerprint variation
///
/// Example:
///     >>> import redstr
///     >>> redstr.tls_fingerprint_variation("771,4865-4866-4867")
///     '772,4865-4867-4866'  # Variation
#[pyfunction]
fn py_tls_fingerprint_variation(text: &str) -> String {
    tls_fingerprint_variation(text)
}

// ============================================================================
// Cloudflare Transformations
// ============================================================================

/// Generates canvas fingerprint variations.
///
/// Args:
///     text (str): Input data
///
/// Returns:
///     str: Canvas fingerprint
///
/// Example:
///     >>> import redstr
///     >>> redstr.canvas_fingerprint_variation("data")
///     'canvas_fp_...'
#[pyfunction]
fn py_canvas_fingerprint_variation(text: &str) -> String {
    canvas_fingerprint_variation(text)
}

/// Generates Cloudflare challenge responses.
///
/// Args:
///     text (str): Challenge data
///
/// Returns:
///     str: Challenge response
///
/// Example:
///     >>> import redstr
///     >>> redstr.cloudflare_challenge_response("challenge")
///     'response_data'
#[pyfunction]
fn py_cloudflare_challenge_response(text: &str) -> String {
    cloudflare_challenge_response(text)
}

/// Generates Cloudflare Turnstile variations.
///
/// Args:
///     text (str): Turnstile data
///
/// Returns:
///     str: Turnstile variation
///
/// Example:
///     >>> import redstr
///     >>> redstr.cloudflare_turnstile_variation("data")
///     'turnstile_token_...'
#[pyfunction]
fn py_cloudflare_turnstile_variation(text: &str) -> String {
    cloudflare_turnstile_variation(text)
}

/// Ensures font fingerprint consistency.
///
/// Args:
///     text (str): Font data
///
/// Returns:
///     str: Consistent font fingerprint
///
/// Example:
///     >>> import redstr
///     >>> redstr.font_fingerprint_consistency("fonts")
///     'font_fp_...'
#[pyfunction]
fn py_font_fingerprint_consistency(text: &str) -> String {
    font_fingerprint_consistency(text)
}

/// Generates TLS handshake patterns.
///
/// Args:
///     text (str): Handshake data
///
/// Returns:
///     str: TLS handshake pattern
///
/// Example:
///     >>> import redstr
///     >>> redstr.tls_handshake_pattern("handshake")
///     'tls_pattern_...'
#[pyfunction]
fn py_tls_handshake_pattern(text: &str) -> String {
    tls_handshake_pattern(text)
}

/// Obfuscates WebGL fingerprints.
///
/// Args:
///     text (str): WebGL data
///
/// Returns:
///     str: Obfuscated WebGL fingerprint
///
/// Example:
///     >>> import redstr
///     >>> redstr.webgl_fingerprint_obfuscate("webgl")
///     'webgl_fp_...'
#[pyfunction]
fn py_webgl_fingerprint_obfuscate(text: &str) -> String {
    webgl_fingerprint_obfuscate(text)
}

// ============================================================================
// Shell Transformations
// ============================================================================

/// Obfuscates Bash commands.
///
/// Args:
///     text (str): Bash command to obfuscate
///
/// Returns:
///     str: Obfuscated Bash command
///
/// Example:
///     >>> import redstr
///     >>> redstr.bash_obfuscate("cat file.txt")
///     'c\\at f\\ile.txt'
#[pyfunction]
fn py_bash_obfuscate(text: &str) -> String {
    bash_obfuscate(text)
}

/// Obfuscates environment variables.
///
/// Args:
///     text (str): Environment variable to obfuscate
///
/// Returns:
///     str: Obfuscated environment variable
///
/// Example:
///     >>> import redstr
///     >>> redstr.env_var_obfuscate("PATH")
///     '$PATH'
#[pyfunction]
fn py_env_var_obfuscate(text: &str) -> String {
    env_var_obfuscate(text)
}

/// Obfuscates file paths.
///
/// Args:
///     text (str): File path to obfuscate
///
/// Returns:
///     str: Obfuscated file path
///
/// Example:
///     >>> import redstr
///     >>> redstr.file_path_obfuscate("/etc/passwd")
///     '/e\\tc/p\\asswd'
#[pyfunction]
fn py_file_path_obfuscate(text: &str) -> String {
    file_path_obfuscate(text)
}

/// Obfuscates PowerShell commands.
///
/// Args:
///     text (str): PowerShell command to obfuscate
///
/// Returns:
///     str: Obfuscated PowerShell command
///
/// Example:
///     >>> import redstr
///     >>> redstr.powershell_obfuscate("Get-Process")
///     '&('Get'+'-Process')'
#[pyfunction]
fn py_powershell_obfuscate(text: &str) -> String {
    powershell_obfuscate(text)
}

// ============================================================================
// Web Security Transformations
// ============================================================================

/// Generates API endpoint variations.
///
/// Args:
///     text (str): API endpoint
///
/// Returns:
///     str: Endpoint variation
///
/// Example:
///     >>> import redstr
///     >>> redstr.api_endpoint_variation("/api/users")
///     '/api/users/'  # Variation
#[pyfunction]
fn py_api_endpoint_variation(text: &str) -> String {
    api_endpoint_variation(text)
}

/// Generates GraphQL introspection bypass payloads.
///
/// Args:
///     text (str): GraphQL query
///
/// Returns:
///     str: Introspection bypass
///
/// Example:
///     >>> import redstr
///     >>> redstr.graphql_introspection_bypass("query")
///     'query{__schema{types{name}}}'
#[pyfunction]
fn py_graphql_introspection_bypass(text: &str) -> String {
    graphql_introspection_bypass(text)
}

/// Obfuscates GraphQL queries.
///
/// Args:
///     text (str): GraphQL query to obfuscate
///
/// Returns:
///     str: Obfuscated GraphQL query
///
/// Example:
///     >>> import redstr
///     >>> redstr.graphql_obfuscate("query { user { name } }")
///     'query{user{name}}'
#[pyfunction]
fn py_graphql_obfuscate(text: &str) -> String {
    graphql_obfuscate(text)
}

/// Generates GraphQL variable injection payloads.
///
/// Args:
///     text (str): GraphQL variable
///
/// Returns:
///     str: Variable injection
///
/// Example:
///     >>> import redstr
///     >>> redstr.graphql_variable_injection("id")
///     '{"id": 1}'
#[pyfunction]
fn py_graphql_variable_injection(text: &str) -> String {
    graphql_variable_injection(text)
}

/// Generates HTTP header variations.
///
/// Args:
///     text (str): HTTP header
///
/// Returns:
///     str: Header variation
///
/// Example:
///     >>> import redstr
///     >>> redstr.http_header_variation("User-Agent")
///     'user-agent'
#[pyfunction]
fn py_http_header_variation(text: &str) -> String {
    http_header_variation(text)
}

/// Generates JWT algorithm confusion attacks.
///
/// Args:
///     text (str): JWT token
///
/// Returns:
///     str: Token with algorithm confusion
///
/// Example:
///     >>> import redstr
///     >>> redstr.jwt_algorithm_confusion("eyJ...")
///     'eyJ...'  # Algorithm modified
#[pyfunction]
fn py_jwt_algorithm_confusion(text: &str) -> String {
    jwt_algorithm_confusion(text)
}

/// Manipulates JWT headers.
///
/// Args:
///     text (str): JWT token
///
/// Returns:
///     str: Token with manipulated header
///
/// Example:
///     >>> import redstr
///     >>> redstr.jwt_header_manipulation("eyJ...")
///     'eyJ...'  # Header modified
#[pyfunction]
fn py_jwt_header_manipulation(text: &str) -> String {
    jwt_header_manipulation(text)
}

/// Obfuscates JWT payload.
///
/// Args:
///     text (str): JWT token
///
/// Returns:
///     str: Token with obfuscated payload
///
/// Example:
///     >>> import redstr
///     >>> redstr.jwt_payload_obfuscate("eyJ...")
///     'eyJ...'  # Payload modified
#[pyfunction]
fn py_jwt_payload_obfuscate(text: &str) -> String {
    jwt_payload_obfuscate(text)
}

/// Generates JWT signature bypass attempts.
///
/// Args:
///     text (str): JWT token
///
/// Returns:
///     str: Token with signature bypass
///
/// Example:
///     >>> import redstr
///     >>> redstr.jwt_signature_bypass("eyJ...")
///     'eyJ...'  # Signature removed/modified
#[pyfunction]
fn py_jwt_signature_bypass(text: &str) -> String {
    jwt_signature_bypass(text)
}

/// Generates session token variations.
///
/// Args:
///     text (str): Session token
///
/// Returns:
///     str: Token variation
///
/// Example:
///     >>> import redstr
///     >>> redstr.session_token_variation("abc123")
///     'abc123def'  # Variation
#[pyfunction]
fn py_session_token_variation(text: &str) -> String {
    session_token_variation(text)
}

// ============================================================================
// TransformBuilder Class
// ============================================================================

/// Python wrapper for TransformBuilder.
///
/// Allows chaining multiple transformations fluently.
///
/// Example:
///     >>> import redstr
///     >>> builder = redstr.TransformBuilder("hello world")
///     >>> result = builder.leetspeak().base64().build()
///     >>> print(result)
#[pyclass]
struct PyTransformBuilder {
    text: String,
}

#[pymethods]
impl PyTransformBuilder {
    /// Creates a new TransformBuilder with the given text.
    ///
    /// Args:
    ///     text (str): Initial text to transform
    ///
    /// Returns:
    ///     TransformBuilder: New builder instance
    #[new]
    fn new(text: String) -> Self {
        PyTransformBuilder { text }
    }

    /// Applies alternate case transformation.
    fn alternate_case(mut slf: PyRefMut<Self>) -> PyRefMut<Self> {
        slf.text = alternate_case(&slf.text);
        slf
    }

    /// Applies case swap transformation.
    fn case_swap(mut slf: PyRefMut<Self>) -> PyRefMut<Self> {
        slf.text = case_swap(&slf.text);
        slf
    }

    /// Applies inverse case transformation.
    fn inverse_case(mut slf: PyRefMut<Self>) -> PyRefMut<Self> {
        slf.text = inverse_case(&slf.text);
        slf
    }

    /// Applies random capitalization transformation.
    fn randomize_capitalization(mut slf: PyRefMut<Self>) -> PyRefMut<Self> {
        slf.text = randomize_capitalization(&slf.text);
        slf
    }

    /// Applies camelCase transformation.
    fn camel_case(mut slf: PyRefMut<Self>) -> PyRefMut<Self> {
        slf.text = to_camel_case(&slf.text);
        slf
    }

    /// Applies kebab-case transformation.
    fn kebab_case(mut slf: PyRefMut<Self>) -> PyRefMut<Self> {
        slf.text = to_kebab_case(&slf.text);
        slf
    }

    /// Applies snake_case transformation.
    fn snake_case(mut slf: PyRefMut<Self>) -> PyRefMut<Self> {
        slf.text = to_snake_case(&slf.text);
        slf
    }

    /// Applies Base64 encoding.
    fn base64(mut slf: PyRefMut<Self>) -> PyRefMut<Self> {
        slf.text = base64_encode(&slf.text);
        slf
    }

    /// Applies hex encoding.
    fn hex(mut slf: PyRefMut<Self>) -> PyRefMut<Self> {
        slf.text = hex_encode(&slf.text);
        slf
    }

    /// Applies URL encoding.
    fn url(mut slf: PyRefMut<Self>) -> PyRefMut<Self> {
        slf.text = url_encode(&slf.text);
        slf
    }

    /// Applies leetspeak transformation.
    fn leetspeak(mut slf: PyRefMut<Self>) -> PyRefMut<Self> {
        slf.text = leetspeak(&slf.text);
        slf
    }

    /// Applies ROT13 transformation.
    fn rot13(mut slf: PyRefMut<Self>) -> PyRefMut<Self> {
        slf.text = rot13(&slf.text);
        slf
    }

    /// Reverses the string.
    fn reverse(mut slf: PyRefMut<Self>) -> PyRefMut<Self> {
        slf.text = reverse_string(&slf.text);
        slf
    }

    /// Builds and returns the final transformed string.
    ///
    /// Returns:
    ///     str: The transformed text
    fn build(&self) -> String {
        self.text.clone()
    }
}

// ============================================================================
// Module Definition
// ============================================================================

/// Python module for redstr transformations.
///
/// This module provides access to all string transformation functions
/// for security testing, penetration testing, and red/blue/purple team operations.
#[pymodule]
fn _redstr(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Case transformations
    m.add_function(wrap_pyfunction!(py_alternate_case, m)?)?;
    m.add_function(wrap_pyfunction!(py_case_swap, m)?)?;
    m.add_function(wrap_pyfunction!(py_inverse_case, m)?)?;
    m.add_function(wrap_pyfunction!(py_randomize_capitalization, m)?)?;
    m.add_function(wrap_pyfunction!(py_to_camel_case, m)?)?;
    m.add_function(wrap_pyfunction!(py_to_kebab_case, m)?)?;
    m.add_function(wrap_pyfunction!(py_to_snake_case, m)?)?;

    // Encoding transformations
    m.add_function(wrap_pyfunction!(py_base64_encode, m)?)?;
    m.add_function(wrap_pyfunction!(py_hex_encode, m)?)?;
    m.add_function(wrap_pyfunction!(py_hex_encode_mixed, m)?)?;
    m.add_function(wrap_pyfunction!(py_html_entity_encode, m)?)?;
    m.add_function(wrap_pyfunction!(py_mixed_encoding, m)?)?;
    m.add_function(wrap_pyfunction!(py_url_encode, m)?)?;

    // Unicode transformations
    m.add_function(wrap_pyfunction!(py_homoglyph_substitution, m)?)?;
    m.add_function(wrap_pyfunction!(py_space_variants, m)?)?;
    m.add_function(wrap_pyfunction!(py_unicode_normalize_variants, m)?)?;
    m.add_function(wrap_pyfunction!(py_unicode_variations, m)?)?;
    m.add_function(wrap_pyfunction!(py_zalgo_text, m)?)?;

    // Injection transformations
    m.add_function(wrap_pyfunction!(py_command_injection, m)?)?;
    m.add_function(wrap_pyfunction!(py_couchdb_injection, m)?)?;
    m.add_function(wrap_pyfunction!(py_dynamodb_obfuscate, m)?)?;
    m.add_function(wrap_pyfunction!(py_mongodb_injection, m)?)?;
    m.add_function(wrap_pyfunction!(py_nosql_operator_injection, m)?)?;
    m.add_function(wrap_pyfunction!(py_null_byte_injection, m)?)?;
    m.add_function(wrap_pyfunction!(py_path_traversal, m)?)?;
    m.add_function(wrap_pyfunction!(py_sql_comment_injection, m)?)?;
    m.add_function(wrap_pyfunction!(py_ssti_framework_variation, m)?)?;
    m.add_function(wrap_pyfunction!(py_ssti_injection, m)?)?;
    m.add_function(wrap_pyfunction!(py_ssti_syntax_obfuscate, m)?)?;
    m.add_function(wrap_pyfunction!(py_xss_tag_variations, m)?)?;

    // Obfuscation transformations
    m.add_function(wrap_pyfunction!(py_double_characters, m)?)?;
    m.add_function(wrap_pyfunction!(py_js_string_concat, m)?)?;
    m.add_function(wrap_pyfunction!(py_leetspeak, m)?)?;
    m.add_function(wrap_pyfunction!(py_reverse_string, m)?)?;
    m.add_function(wrap_pyfunction!(py_rot13, m)?)?;
    m.add_function(wrap_pyfunction!(py_vowel_swap, m)?)?;
    m.add_function(wrap_pyfunction!(py_whitespace_padding, m)?)?;

    // Phishing transformations
    m.add_function(wrap_pyfunction!(py_advanced_domain_spoof, m)?)?;
    m.add_function(wrap_pyfunction!(py_domain_typosquat, m)?)?;
    m.add_function(wrap_pyfunction!(py_email_obfuscation, m)?)?;
    m.add_function(wrap_pyfunction!(py_url_shortening_pattern, m)?)?;

    // Bot detection transformations
    m.add_function(wrap_pyfunction!(py_accept_language_variation, m)?)?;
    m.add_function(wrap_pyfunction!(py_cloudflare_challenge_variation, m)?)?;
    m.add_function(wrap_pyfunction!(py_http2_header_order, m)?)?;
    m.add_function(wrap_pyfunction!(py_random_user_agent, m)?)?;
    m.add_function(wrap_pyfunction!(py_tls_fingerprint_variation, m)?)?;

    // Cloudflare transformations
    m.add_function(wrap_pyfunction!(py_canvas_fingerprint_variation, m)?)?;
    m.add_function(wrap_pyfunction!(py_cloudflare_challenge_response, m)?)?;
    m.add_function(wrap_pyfunction!(py_cloudflare_turnstile_variation, m)?)?;
    m.add_function(wrap_pyfunction!(py_font_fingerprint_consistency, m)?)?;
    m.add_function(wrap_pyfunction!(py_tls_handshake_pattern, m)?)?;
    m.add_function(wrap_pyfunction!(py_webgl_fingerprint_obfuscate, m)?)?;

    // Shell transformations
    m.add_function(wrap_pyfunction!(py_bash_obfuscate, m)?)?;
    m.add_function(wrap_pyfunction!(py_env_var_obfuscate, m)?)?;
    m.add_function(wrap_pyfunction!(py_file_path_obfuscate, m)?)?;
    m.add_function(wrap_pyfunction!(py_powershell_obfuscate, m)?)?;

    // Web security transformations
    m.add_function(wrap_pyfunction!(py_api_endpoint_variation, m)?)?;
    m.add_function(wrap_pyfunction!(py_graphql_introspection_bypass, m)?)?;
    m.add_function(wrap_pyfunction!(py_graphql_obfuscate, m)?)?;
    m.add_function(wrap_pyfunction!(py_graphql_variable_injection, m)?)?;
    m.add_function(wrap_pyfunction!(py_http_header_variation, m)?)?;
    m.add_function(wrap_pyfunction!(py_jwt_algorithm_confusion, m)?)?;
    m.add_function(wrap_pyfunction!(py_jwt_header_manipulation, m)?)?;
    m.add_function(wrap_pyfunction!(py_jwt_payload_obfuscate, m)?)?;
    m.add_function(wrap_pyfunction!(py_jwt_signature_bypass, m)?)?;
    m.add_function(wrap_pyfunction!(py_session_token_variation, m)?)?;

    // Builder class
    m.add_class::<PyTransformBuilder>()?;

    Ok(())
}
