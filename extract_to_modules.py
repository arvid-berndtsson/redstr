#!/usr/bin/env python3
"""Extract functions from lib.rs into module files."""
import re
import os

# Read lib.rs
with open('src/lib.rs', 'r') as f:
    content = f.read()

# Function to module mapping (excluding case which is already created)
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

# Extract function with its full body (simplified - finds pub fn and extracts until matching brace)
def extract_function(content, func_name):
    """Extract a function definition and its body."""
    # Find the function definition
    pattern = rf'(///.*?\n)*pub fn {func_name}\([^)]*\) -> [^{{]*\{{'
    match = re.search(pattern, content, re.DOTALL)
    if not match:
        return None
    
    start = match.start()
    # Find matching closing brace
    brace_count = 0
    in_string = False
    string_char = None
    i = match.end() - 1
    
    while i < len(content):
        char = content[i]
        if not in_string:
            if char == '{':
                brace_count += 1
            elif char == '}':
                brace_count -= 1
                if brace_count == 0:
                    return content[start:i+1]
            elif char in ('"', "'"):
                in_string = True
                string_char = char
        else:
            if char == string_char and content[i-1] != '\\':
                in_string = False
        i += 1
    
    return None

print("Script created. This is a complex parsing task.")
print("For now, let's create modules manually with key functions.")
