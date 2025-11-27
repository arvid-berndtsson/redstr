#!/usr/bin/env python3
"""
Basic usage examples for redstr Python bindings.

This script demonstrates common string transformations for security testing.
"""

import redstr


def main():
    print("=" * 70)
    print("redstr - Python Bindings Basic Usage Examples")
    print("=" * 70)
    print()

    # Case transformations
    print("Case Transformations:")
    print("-" * 70)
    text = "Hello World"
    print(f"Original: {text}")
    print(f"alternate_case: {redstr.alternate_case(text)}")
    print(f"case_swap: {redstr.case_swap(text)}")
    print(f"randomize_capitalization: {redstr.randomize_capitalization(text)}")
    print(f"to_camel_case: {redstr.to_camel_case(text)}")
    print(f"to_snake_case: {redstr.to_snake_case(text)}")
    print(f"to_kebab_case: {redstr.to_kebab_case(text)}")
    print()

    # Encoding transformations
    print("Encoding Transformations:")
    print("-" * 70)
    text = "hello"
    print(f"Original: {text}")
    print(f"base64_encode: {redstr.base64_encode(text)}")
    print(f"hex_encode: {redstr.hex_encode(text)}")
    print(f"url_encode: {redstr.url_encode('hello world')}")
    print(f"html_entity_encode: {redstr.html_entity_encode('<script>')}")
    print()

    # Obfuscation transformations
    print("Obfuscation Transformations:")
    print("-" * 70)
    text = "password"
    print(f"Original: {text}")
    print(f"leetspeak: {redstr.leetspeak(text)}")
    print(f"rot13: {redstr.rot13(text)}")
    print(f"reverse_string: {redstr.reverse_string(text)}")
    print(f"double_characters: {redstr.double_characters(text)}")
    print()

    # Unicode transformations
    print("Unicode Transformations:")
    print("-" * 70)
    text = "admin"
    print(f"Original: {text}")
    print(f"homoglyph_substitution: {redstr.homoglyph_substitution(text)}")
    print(f"unicode_variations: {redstr.unicode_variations(text)}")
    print()

    # Injection testing
    print("Injection Testing:")
    print("-" * 70)
    sql = "SELECT * FROM users"
    print(f"Original: {sql}")
    print(f"sql_comment_injection: {redstr.sql_comment_injection(sql)}")
    print()

    xss = "alert(1)"
    print(f"Original: {xss}")
    print(f"xss_tag_variations: {redstr.xss_tag_variations(xss)}")
    print()

    path = "etc/passwd"
    print(f"Original: {path}")
    print(f"path_traversal: {redstr.path_traversal(path)}")
    print()

    # Phishing tools
    print("Phishing Tools:")
    print("-" * 70)
    domain = "google.com"
    print(f"Original: {domain}")
    print(f"domain_typosquat: {redstr.domain_typosquat(domain)}")
    print(f"advanced_domain_spoof: {redstr.advanced_domain_spoof(domain)}")
    print()

    # Bot detection evasion
    print("Bot Detection Evasion:")
    print("-" * 70)
    print(f"random_user_agent: {redstr.random_user_agent()[:80]}...")
    print()

    # TransformBuilder - chaining transformations
    print("TransformBuilder - Chaining Transformations:")
    print("-" * 70)
    text = "SELECT * FROM users"
    print(f"Original: {text}")
    
    builder = redstr.TransformBuilder(text)
    result = builder.case_swap().base64().build()
    print(f"case_swap().base64(): {result}")
    
    builder = redstr.TransformBuilder("hello world")
    result = builder.leetspeak().url().build()
    print(f"leetspeak().url() on 'hello world': {result}")
    print()

    print("=" * 70)
    print("Examples complete!")
    print("=" * 70)


if __name__ == "__main__":
    main()
