# redstr

Native Python bindings for [redstr](https://github.com/arvid-berndtsson/redstr) - high-performance string transformations for security testing.

## Installation

```bash
pip install redstr
```

## Features

- 🚀 **Native performance** - Zero-overhead Rust bindings via PyO3
- 🔒 **Security-focused** - 60+ transformations for WAF bypass, XSS, SQL injection, etc.
- 📦 **Zero dependencies** - No Python dependencies
- 🎯 **Type hints** - Full type annotations included
- 🔗 **Builder pattern** - Chain multiple transformations

## Quick Start

```python
from redstr import leetspeak, base64_encode, TransformBuilder

# Direct function calls
print(leetspeak('password'))  # 'p@55w0rd'
print(base64_encode('hello'))  # 'aGVsbG8='

# Builder pattern for chaining
builder = TransformBuilder('<script>alert(1)</script>')
builder.case_swap()
builder.url_encode()
payload = builder.build()
print(payload)
```

## Available Functions

### Case Transformations
- `randomize_capitalization(input)` - Random case for each character
- `case_swap(input)` - Swap uppercase/lowercase
- `alternate_case(input)` - Alternate case pattern
- `to_camel_case(input)` - Convert to camelCase
- `to_snake_case(input)` - Convert to snake_case
- `to_kebab_case(input)` - Convert to kebab-case

### Encoding
- `base64_encode(input)` - Base64 encoding
- `url_encode(input)` - URL encoding
- `hex_encode(input)` - Hex encoding
- `html_entity_encode(input)` - HTML entity encoding
- `mixed_encoding(input)` - Mixed encoding techniques

### Obfuscation
- `leetspeak(input)` - Leetspeak transformation
- `rot13(input)` - ROT13 cipher
- `reverse_string(input)` - Reverse the string
- `double_characters(input)` - Double each character
- `js_string_concat(input)` - JS string concatenation

### Unicode
- `homoglyph_substitution(input)` - Replace with similar-looking characters
- `zalgo_text(input)` - Zalgo text effect
- `unicode_variations(input)` - Unicode variations
- `space_variants(input)` - Alternative space characters

### Phishing
- `domain_typosquat(input)` - Generate typosquatted domains
- `email_obfuscation(input)` - Obfuscate email addresses
- `advanced_domain_spoof(input)` - Domain spoofing techniques

### Injection
- `xss_tag_variations(input)` - XSS payload variations
- `sql_comment_injection(input)` - SQL injection patterns
- `command_injection(input)` - Command injection
- `path_traversal(input)` - Path traversal patterns
- `ssti_injection(input)` - Server-side template injection

### Bot Detection
- `random_user_agent()` - Generate random user agent
- `tls_fingerprint_variation(input)` - TLS fingerprint variations

### Shell
- `powershell_obfuscate(input)` - PowerShell obfuscation
- `bash_obfuscate(input)` - Bash obfuscation

## TransformBuilder

Chain multiple transformations:

```python
from redstr import TransformBuilder

builder = TransformBuilder('payload')
builder.leetspeak()
builder.base64()
builder.url_encode()
result = builder.build()
```

Available builder methods:
- `.leetspeak()` - Apply leetspeak
- `.base64()` - Apply Base64 encoding
- `.url_encode()` - Apply URL encoding
- `.case_swap()` - Apply case swap
- `.rot13()` - Apply ROT13
- `.hex_encode()` - Apply hex encoding
- `.homoglyphs()` - Apply homoglyph substitution
- `.reverse()` - Reverse the string
- `.build()` - Get the final result

## Performance

redstr uses native Rust code via PyO3, providing near-native performance:

| Operation | Time |
|-----------|------|
| `leetspeak()` | ~0.01ms |
| `base64_encode()` | ~0.005ms |
| `TransformBuilder` (5 ops) | ~0.05ms |

## License

MIT
