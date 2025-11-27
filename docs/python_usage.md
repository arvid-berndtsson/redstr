# Python Usage Guide

This guide covers how to use the redstr Python bindings for security testing and penetration testing workflows.

## Installation

### From PyPI (Recommended)

```bash
pip install redstr
```

### From Source

```bash
# Clone the repository
git clone https://github.com/arvid-berndtsson/redstr
cd redstr

# Install maturin
pip install maturin

# Build and install
maturin develop --release
```

## Quick Start

```python
import redstr

# Basic transformations
result = redstr.leetspeak("password")
print(result)  # Output: "p@55w0rd"

encoded = redstr.base64_encode("hello")
print(encoded)  # Output: "aGVsbG8="

# Chaining transformations
builder = redstr.TransformBuilder("SELECT * FROM users")
payload = builder.case_swap().url().build()
print(payload)
```

## API Overview

All transformation functions follow a simple pattern:
- Accept a string as input
- Return a transformed string as output
- Are pure functions (no side effects)

### Function Categories

#### Case Transformations

```python
import redstr

redstr.alternate_case("hello")           # "hElLo"
redstr.case_swap("Hello World")          # "hELLO wORLD"
redstr.randomize_capitalization("test")  # "TeSt" (varies)
redstr.to_camel_case("hello world")      # "helloWorld"
redstr.to_snake_case("Hello World")      # "hello_world"
redstr.to_kebab_case("Hello World")      # "hello-world"
```

#### Encoding Transformations

```python
import redstr

redstr.base64_encode("hello")            # "aGVsbG8="
redstr.hex_encode("hi")                  # "6869"
redstr.url_encode("hello world")         # "hello%20world"
redstr.html_entity_encode("<script>")    # "&lt;script&gt;"
redstr.mixed_encoding("test")            # Mixed encoding (varies)
```

#### Unicode Transformations

```python
import redstr

# Homoglyph substitution (for phishing tests)
redstr.homoglyph_substitution("admin")   # "аdmіn" (Cyrillic)

# Unicode variations
redstr.unicode_variations("hello")       # "ℎℯ𝓁𝓁ℴ"

# Zalgo text (combining diacritics)
redstr.zalgo_text("hello")               # "h̷̢͇e̸̛͜l̵͙̀l̶̰̂ȯ̴̡"
```

#### Injection Testing

```python
import redstr

# SQL injection
redstr.sql_comment_injection("SELECT * FROM users")

# XSS testing
redstr.xss_tag_variations("alert(1)")

# Path traversal
redstr.path_traversal("etc/passwd")      # "../../../etc/passwd"

# Command injection
redstr.command_injection("cat file")

# NoSQL injection
redstr.mongodb_injection("admin")        # '{"$ne": "admin"}'
redstr.nosql_operator_injection("val")

# SSTI (Server-Side Template Injection)
redstr.ssti_injection("name")            # "{{name}}"
redstr.ssti_framework_variation("{{7*7}}")
```

#### Obfuscation

```python
import redstr

redstr.leetspeak("elite")                # "3l1t3"
redstr.rot13("hello")                    # "uryyb"
redstr.reverse_string("hello")           # "olleh"
redstr.double_characters("hi")           # "hhii"
redstr.js_string_concat("alert")         # '"al"+"er"+"t"'
redstr.vowel_swap("hello")               # Randomized vowels
redstr.whitespace_padding("test")        # With random padding
```

#### Phishing Tools

```python
import redstr

# Domain typosquatting
redstr.domain_typosquat("google.com")    # "gooogle.com" (varies)

# Advanced domain spoofing
redstr.advanced_domain_spoof("paypal.com")

# Email obfuscation
redstr.email_obfuscation("admin@example.com")  # "admin[at]example[dot]com"

# URL shortening patterns
redstr.url_shortening_pattern("https://example.com")
```

#### Bot Detection Evasion

```python
import redstr

# Random user agent
ua = redstr.random_user_agent()
print(ua)  # Mozilla/5.0 (Windows NT 10.0; ...)

# TLS fingerprint variation
redstr.tls_fingerprint_variation("771,4865-4866")

# Accept-Language variations
redstr.accept_language_variation("en")   # "en-US,en;q=0.9"

# HTTP/2 header order
redstr.http2_header_order("User-Agent: ...")
```

#### Cloudflare Evasion

```python
import redstr

redstr.cloudflare_turnstile_variation("data")
redstr.cloudflare_challenge_response("challenge")
redstr.canvas_fingerprint_variation("data")
redstr.webgl_fingerprint_obfuscate("webgl")
redstr.font_fingerprint_consistency("fonts")
redstr.tls_handshake_pattern("handshake")
```

#### Shell Obfuscation

```python
import redstr

# Bash obfuscation
redstr.bash_obfuscate("cat file.txt")   # "c\\at f\\ile.txt"

# PowerShell obfuscation
redstr.powershell_obfuscate("Get-Process")  # "&('Get'+'-Process')"

# Environment variable obfuscation
redstr.env_var_obfuscate("PATH")         # "$PATH"

# File path obfuscation
redstr.file_path_obfuscate("/etc/passwd")
```

#### Web Security Testing

```python
import redstr

# JWT manipulation
jwt = "eyJhbGci..."
redstr.jwt_signature_bypass(jwt)
redstr.jwt_algorithm_confusion(jwt)
redstr.jwt_header_manipulation(jwt)
redstr.jwt_payload_obfuscate(jwt)

# GraphQL testing
redstr.graphql_obfuscate("query { user { name } }")
redstr.graphql_introspection_bypass("query")
redstr.graphql_variable_injection("id")

# API testing
redstr.api_endpoint_variation("/api/users")
redstr.http_header_variation("User-Agent")
redstr.session_token_variation("token123")
```

## TransformBuilder - Chaining Transformations

The `TransformBuilder` class allows you to chain multiple transformations fluently:

```python
import redstr

# Create a builder with initial text
builder = redstr.TransformBuilder("hello world")

# Chain transformations
result = builder.leetspeak().base64().build()
print(result)

# Complex chaining for WAF bypass
payload = "SELECT * FROM users WHERE id=1"
builder = redstr.TransformBuilder(payload)
obfuscated = builder.case_swap().url().build()
```

### Available Builder Methods

- `alternate_case()` - Alternate case transformation
- `case_swap()` - Swap case of all characters
- `inverse_case()` - Inverse case transformation
- `randomize_capitalization()` - Random capitalization
- `camel_case()` - Convert to camelCase
- `kebab_case()` - Convert to kebab-case
- `snake_case()` - Convert to snake_case
- `base64()` - Base64 encode
- `hex()` - Hex encode
- `url()` - URL encode
- `leetspeak()` - Apply leetspeak transformation
- `rot13()` - Apply ROT13 cipher
- `reverse()` - Reverse the string
- `build()` - Get the final transformed string

## Use Cases

### WAF Bypass Testing

```python
import redstr

# Test if WAF blocks various SQL injection patterns
sql_base = "SELECT * FROM users WHERE id=1"

# Generate variations
variations = [
    redstr.sql_comment_injection(sql_base),
    redstr.TransformBuilder(sql_base).case_swap().build(),
    redstr.TransformBuilder(sql_base).url().build(),
    redstr.mixed_encoding(sql_base),
]

for i, variant in enumerate(variations, 1):
    print(f"Variant {i}: {variant}")
    # Test against WAF
```

### XSS Payload Generation

```python
import redstr

payload = "alert(document.cookie)"

# Generate different XSS vectors
vectors = [
    redstr.xss_tag_variations(payload),
    redstr.js_string_concat("alert"),
    redstr.html_entity_encode(f"<script>{payload}</script>"),
]

for vector in vectors:
    print(f"Testing: {vector}")
    # Test XSS vector
```

### Phishing Campaign Testing

```python
import redstr

target_domains = ["paypal.com", "amazon.com", "google.com"]

for domain in target_domains:
    # Generate typosquatting variations
    typo = redstr.domain_typosquat(domain)
    spoof = redstr.advanced_domain_spoof(domain)
    
    print(f"Target: {domain}")
    print(f"  Typosquat: {typo}")
    print(f"  Spoof: {spoof}")
```

### Bot Detection Evasion

```python
import redstr

# Generate different browser fingerprints
for i in range(5):
    ua = redstr.random_user_agent()
    tls = redstr.tls_fingerprint_variation("771,4865-4866")
    lang = redstr.accept_language_variation("en")
    
    print(f"Profile {i+1}:")
    print(f"  User-Agent: {ua[:60]}...")
    print(f"  TLS: {tls}")
    print(f"  Accept-Language: {lang}")
```

## Type Hints

The package includes full type hints and works with mypy:

```python
from typing import reveal_type
import redstr

# Type checking works
result: str = redstr.leetspeak("test")
reveal_type(result)  # str

# Builder also has types
builder: redstr.TransformBuilder = redstr.TransformBuilder("test")
reveal_type(builder.build())  # str
```

## Testing

The package includes comprehensive tests. Run them with pytest:

```bash
# Install test dependencies
pip install pytest

# Run tests
pytest python/tests/

# Run with coverage
pip install pytest-cov
pytest python/tests/ --cov=redstr --cov-report=html
```

## Performance

The Python bindings use native Rust code under the hood, providing excellent performance:

```python
import time
import redstr

# Benchmark example
text = "SELECT * FROM users WHERE id=1" * 100

start = time.time()
for _ in range(1000):
    _ = redstr.leetspeak(text)
elapsed = time.time() - start

print(f"1000 iterations in {elapsed:.2f} seconds")
```

## Error Handling

All functions are designed to be robust and not fail on normal input:

```python
import redstr

# Empty strings are handled
result = redstr.leetspeak("")
assert result == ""

# Special characters are preserved
result = redstr.leetspeak("!@#$%")
# Non-leetspeak characters remain unchanged
```

## Integration Examples

### With Requests

```python
import requests
import redstr

# Evade bot detection
ua = redstr.random_user_agent()
headers = {
    "User-Agent": ua,
    "Accept-Language": redstr.accept_language_variation("en"),
}

response = requests.get("https://example.com", headers=headers)
```

### With Selenium

```python
from selenium import webdriver
import redstr

# Use random user agent
options = webdriver.ChromeOptions()
ua = redstr.random_user_agent()
options.add_argument(f'user-agent={ua}')

driver = webdriver.Chrome(options=options)
```

### With Burp Suite / Caido

```python
import redstr

def process_request(request):
    """Process and obfuscate request for testing."""
    # Obfuscate SQL injection payloads
    payload = request.get_parameter("id")
    obfuscated = redstr.sql_comment_injection(payload)
    request.set_parameter("id", obfuscated)
    return request
```

## Best Practices

1. **Authorized Testing Only**: Only use on systems you own or have permission to test
2. **Combine Transformations**: Use `TransformBuilder` to chain multiple transformations
3. **Test Variations**: Generate multiple payload variations to test different bypass techniques
4. **Document Results**: Keep track of which transformations work against specific targets
5. **Respect Rate Limits**: Don't overwhelm target systems with automated requests

## Troubleshooting

### Import Error

If you get an import error:
```python
ImportError: No module named '_redstr'
```

Solution: Reinstall the package:
```bash
pip uninstall redstr
pip install redstr
```

### Type Checking Issues

If mypy complains about missing types:
```bash
# Ensure py.typed is included
pip install --force-reinstall redstr
```

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines.

## License

MIT License - see [LICENSE](../LICENSE) for details.

## Responsible Use

This library is intended for authorized security testing, research, and defensive security purposes only. Users are responsible for ensuring they have proper authorization before using these techniques on systems they do not own or have explicit permission to test.
