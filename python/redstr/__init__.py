"""
redstr - Red team string obfuscation and transformation library

A comprehensive string transformation library for security testing, penetration testing,
and red/blue/purple team operations.

Features:
    - 60+ transformation functions for security testing
    - Case transformations for filter bypass
    - Encoding functions (Base64, URL, hex, HTML entities)
    - Unicode transformations (homoglyphs, zalgo text)
    - Injection testing (SQL, XSS, command injection, NoSQL)
    - Obfuscation (leetspeak, ROT13, JavaScript)
    - Phishing tools (domain typosquatting, email obfuscation)
    - Bot detection evasion (user agents, fingerprinting)
    - Web security testing (JWT, GraphQL, APIs)
    - Shell obfuscation (PowerShell, Bash)

Example:
    >>> import redstr
    >>> redstr.leetspeak("password")
    'p@55w0rd'
    >>> redstr.base64_encode("hello")
    'aGVsbG8='
    >>> redstr.homoglyph_substitution("admin")
    'аdmіn'

Note:
    This library is intended for authorized security testing only.
    Users are responsible for ensuring proper authorization before use.
"""

from ._redstr import (
    # Case transformations
    py_alternate_case as alternate_case,
    py_case_swap as case_swap,
    py_inverse_case as inverse_case,
    py_randomize_capitalization as randomize_capitalization,
    py_to_camel_case as to_camel_case,
    py_to_kebab_case as to_kebab_case,
    py_to_snake_case as to_snake_case,
    # Encoding transformations
    py_base64_encode as base64_encode,
    py_hex_encode as hex_encode,
    py_hex_encode_mixed as hex_encode_mixed,
    py_html_entity_encode as html_entity_encode,
    py_mixed_encoding as mixed_encoding,
    py_url_encode as url_encode,
    # Unicode transformations
    py_homoglyph_substitution as homoglyph_substitution,
    py_space_variants as space_variants,
    py_unicode_normalize_variants as unicode_normalize_variants,
    py_unicode_variations as unicode_variations,
    py_zalgo_text as zalgo_text,
    # Injection transformations
    py_command_injection as command_injection,
    py_couchdb_injection as couchdb_injection,
    py_dynamodb_obfuscate as dynamodb_obfuscate,
    py_mongodb_injection as mongodb_injection,
    py_nosql_operator_injection as nosql_operator_injection,
    py_null_byte_injection as null_byte_injection,
    py_path_traversal as path_traversal,
    py_sql_comment_injection as sql_comment_injection,
    py_ssti_framework_variation as ssti_framework_variation,
    py_ssti_injection as ssti_injection,
    py_ssti_syntax_obfuscate as ssti_syntax_obfuscate,
    py_xss_tag_variations as xss_tag_variations,
    # Obfuscation transformations
    py_double_characters as double_characters,
    py_js_string_concat as js_string_concat,
    py_leetspeak as leetspeak,
    py_reverse_string as reverse_string,
    py_rot13 as rot13,
    py_vowel_swap as vowel_swap,
    py_whitespace_padding as whitespace_padding,
    # Phishing transformations
    py_advanced_domain_spoof as advanced_domain_spoof,
    py_domain_typosquat as domain_typosquat,
    py_email_obfuscation as email_obfuscation,
    py_url_shortening_pattern as url_shortening_pattern,
    # Bot detection transformations
    py_accept_language_variation as accept_language_variation,
    py_cloudflare_challenge_variation as cloudflare_challenge_variation,
    py_http2_header_order as http2_header_order,
    py_random_user_agent as random_user_agent,
    py_tls_fingerprint_variation as tls_fingerprint_variation,
    # Cloudflare transformations
    py_canvas_fingerprint_variation as canvas_fingerprint_variation,
    py_cloudflare_challenge_response as cloudflare_challenge_response,
    py_cloudflare_turnstile_variation as cloudflare_turnstile_variation,
    py_font_fingerprint_consistency as font_fingerprint_consistency,
    py_tls_handshake_pattern as tls_handshake_pattern,
    py_webgl_fingerprint_obfuscate as webgl_fingerprint_obfuscate,
    # Shell transformations
    py_bash_obfuscate as bash_obfuscate,
    py_env_var_obfuscate as env_var_obfuscate,
    py_file_path_obfuscate as file_path_obfuscate,
    py_powershell_obfuscate as powershell_obfuscate,
    # Web security transformations
    py_api_endpoint_variation as api_endpoint_variation,
    py_graphql_introspection_bypass as graphql_introspection_bypass,
    py_graphql_obfuscate as graphql_obfuscate,
    py_graphql_variable_injection as graphql_variable_injection,
    py_http_header_variation as http_header_variation,
    py_jwt_algorithm_confusion as jwt_algorithm_confusion,
    py_jwt_header_manipulation as jwt_header_manipulation,
    py_jwt_payload_obfuscate as jwt_payload_obfuscate,
    py_jwt_signature_bypass as jwt_signature_bypass,
    py_session_token_variation as session_token_variation,
    # Builder class
    PyTransformBuilder as TransformBuilder,
)

__version__ = "0.2.3"

__all__ = [
    # Case transformations
    "alternate_case",
    "case_swap",
    "inverse_case",
    "randomize_capitalization",
    "to_camel_case",
    "to_kebab_case",
    "to_snake_case",
    # Encoding transformations
    "base64_encode",
    "hex_encode",
    "hex_encode_mixed",
    "html_entity_encode",
    "mixed_encoding",
    "url_encode",
    # Unicode transformations
    "homoglyph_substitution",
    "space_variants",
    "unicode_normalize_variants",
    "unicode_variations",
    "zalgo_text",
    # Injection transformations
    "command_injection",
    "couchdb_injection",
    "dynamodb_obfuscate",
    "mongodb_injection",
    "nosql_operator_injection",
    "null_byte_injection",
    "path_traversal",
    "sql_comment_injection",
    "ssti_framework_variation",
    "ssti_injection",
    "ssti_syntax_obfuscate",
    "xss_tag_variations",
    # Obfuscation transformations
    "double_characters",
    "js_string_concat",
    "leetspeak",
    "reverse_string",
    "rot13",
    "vowel_swap",
    "whitespace_padding",
    # Phishing transformations
    "advanced_domain_spoof",
    "domain_typosquat",
    "email_obfuscation",
    "url_shortening_pattern",
    # Bot detection transformations
    "accept_language_variation",
    "cloudflare_challenge_variation",
    "http2_header_order",
    "random_user_agent",
    "tls_fingerprint_variation",
    # Cloudflare transformations
    "canvas_fingerprint_variation",
    "cloudflare_challenge_response",
    "cloudflare_turnstile_variation",
    "font_fingerprint_consistency",
    "tls_handshake_pattern",
    "webgl_fingerprint_obfuscate",
    # Shell transformations
    "bash_obfuscate",
    "env_var_obfuscate",
    "file_path_obfuscate",
    "powershell_obfuscate",
    # Web security transformations
    "api_endpoint_variation",
    "graphql_introspection_bypass",
    "graphql_obfuscate",
    "graphql_variable_injection",
    "http_header_variation",
    "jwt_algorithm_confusion",
    "jwt_header_manipulation",
    "jwt_payload_obfuscate",
    "jwt_signature_bypass",
    "session_token_variation",
    # Builder class
    "TransformBuilder",
]
