"""
redstr - Native string transformations for security testing.

This module provides high-performance string transformation functions
for security testing, including WAF bypass, XSS, SQL injection,
phishing detection, and evasion testing.

Example:
    >>> from redstr import leetspeak, base64_encode
    >>> leetspeak('password')
    'p@55w0rd'
    >>> base64_encode('hello')
    'aGVsbG8='
"""

from redstr._redstr import (
    # Case transformations
    randomize_capitalization,
    case_swap,
    alternate_case,
    inverse_case,
    to_camel_case,
    to_snake_case,
    to_kebab_case,
    # Encoding transformations
    base64_encode,
    url_encode,
    hex_encode,
    hex_encode_mixed,
    html_entity_encode,
    mixed_encoding,
    # Obfuscation transformations
    leetspeak,
    rot13,
    reverse_string,
    double_characters,
    vowel_swap,
    whitespace_padding,
    js_string_concat,
    # Unicode transformations
    homoglyph_substitution,
    zalgo_text,
    unicode_variations,
    space_variants,
    unicode_normalize_variants,
    # Phishing transformations
    domain_typosquat,
    email_obfuscation,
    url_shortening_pattern,
    advanced_domain_spoof,
    # Injection transformations
    xss_tag_variations,
    sql_comment_injection,
    command_injection,
    path_traversal,
    null_byte_injection,
    ssti_injection,
    mongodb_injection,
    # Bot detection transformations
    random_user_agent,
    tls_fingerprint_variation,
    accept_language_variation,
    # Shell transformations
    powershell_obfuscate,
    bash_obfuscate,
    env_var_obfuscate,
    file_path_obfuscate,
    # Web security transformations
    graphql_obfuscate,
    jwt_header_manipulation,
    jwt_payload_obfuscate,
    # Classes
    TransformBuilder,
)

__version__ = "0.2.6"
__all__ = [
    # Case transformations
    "randomize_capitalization",
    "case_swap",
    "alternate_case",
    "inverse_case",
    "to_camel_case",
    "to_snake_case",
    "to_kebab_case",
    # Encoding transformations
    "base64_encode",
    "url_encode",
    "hex_encode",
    "hex_encode_mixed",
    "html_entity_encode",
    "mixed_encoding",
    # Obfuscation transformations
    "leetspeak",
    "rot13",
    "reverse_string",
    "double_characters",
    "vowel_swap",
    "whitespace_padding",
    "js_string_concat",
    # Unicode transformations
    "homoglyph_substitution",
    "zalgo_text",
    "unicode_variations",
    "space_variants",
    "unicode_normalize_variants",
    # Phishing transformations
    "domain_typosquat",
    "email_obfuscation",
    "url_shortening_pattern",
    "advanced_domain_spoof",
    # Injection transformations
    "xss_tag_variations",
    "sql_comment_injection",
    "command_injection",
    "path_traversal",
    "null_byte_injection",
    "ssti_injection",
    "mongodb_injection",
    # Bot detection transformations
    "random_user_agent",
    "tls_fingerprint_variation",
    "accept_language_variation",
    # Shell transformations
    "powershell_obfuscate",
    "bash_obfuscate",
    "env_var_obfuscate",
    "file_path_obfuscate",
    # Web security transformations
    "graphql_obfuscate",
    "jwt_header_manipulation",
    "jwt_payload_obfuscate",
    # Classes
    "TransformBuilder",
]
