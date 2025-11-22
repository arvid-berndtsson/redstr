#!/usr/bin/env python3
"""Extract functions from lib.rs into modules."""
import re
import sys

# Function to module mapping
FUNCTION_MODULES = {
    # Encoding
    'base64_encode': 'encoding',
    'url_encode': 'encoding',
    'hex_encode': 'encoding',
    'hex_encode_mixed': 'encoding',
    'html_entity_encode': 'encoding',
    'mixed_encoding': 'encoding',
    
    # Unicode
    'unicode_variations': 'unicode',
    'zalgo_text': 'unicode',
    'unicode_normalize_variants': 'unicode',
    'homoglyph_substitution': 'unicode',
    'space_variants': 'unicode',
    
    # Injection
    'sql_comment_injection': 'injection',
    'xss_tag_variations': 'injection',
    'null_byte_injection': 'injection',
    'path_traversal': 'injection',
    'command_injection': 'injection',
    
    # Obfuscation
    'leetspeak': 'obfuscation',
    'rot13': 'obfuscation',
    'vowel_swap': 'obfuscation',
    'double_characters': 'obfuscation',
    'reverse_string': 'obfuscation',
    'whitespace_padding': 'obfuscation',
    'js_string_concat': 'obfuscation',
    
    # Phishing
    'domain_typosquat': 'phishing',
    'advanced_domain_spoof': 'phishing',
    'email_obfuscation': 'phishing',
    'url_shortening_pattern': 'phishing',
    
    # Bot detection
    'random_user_agent': 'bot_detection',
    'http2_header_order': 'bot_detection',
    'tls_fingerprint_variation': 'bot_detection',
    'cloudflare_challenge_variation': 'bot_detection',
    'accept_language_variation': 'bot_detection',
    
    # Web security
    'http_header_variation': 'web_security',
    'api_endpoint_variation': 'web_security',
    'graphql_obfuscate': 'web_security',
    'session_token_variation': 'web_security',
    
    # Shell
    'powershell_obfuscate': 'shell',
    'bash_obfuscate': 'shell',
    'env_var_obfuscate': 'shell',
    'file_path_obfuscate': 'shell',
}

print("Function to module mapping created")
