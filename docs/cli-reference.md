# CLI Reference Guide

Complete reference for the redstr command-line interface. The CLI is an optional binary for quick testing and experimentation with transformation functions.

## Installation

To install with the CLI tool:

```bash
cargo install redstr --features cli
```

Or build from source with the CLI feature:

```bash
cargo build --release --features cli
```

The binary will be available at `target/release/redstr`.

## Usage

```bash
redstr [mode] <text>
```

If no mode is specified, random capitalization is used by default.

## Transformation Modes

### Basic Transformations

- **random, r** - Random capitalization (default)
  - Example: `redstr "Hello World"` → `HeLlO wOrLd`
  
- **alternate, a** - Alternate upper/lower case
  - Example: `redstr alternate "Hello World"` → `HeLlO wOrLd`
  
- **inverse, i** - Invert the case of each letter
  - Example: `redstr inverse "Hello World"` → `hELLO wORLD`

- **reverse, rv** - Reverse the string
  - Example: `redstr reverse "Hello World"` → `dlroW olleH`

### Case Conversion

- **camel, c** - Convert to camelCase
  - Example: `redstr camel "hello world test"` → `helloWorldTest`
  
- **snake, s** - Convert to snake_case
  - Example: `redstr snake "HelloWorldTest"` → `hello_world_test`
  
- **kebab, k** - Convert to kebab-case
  - Example: `redstr kebab "HelloWorldTest"` → `hello-world-test`

### Security Testing Modes

- **leetspeak, l** - Convert to leetspeak
  - Useful for testing password filters and content detection
  - Example: `redstr leetspeak "password123"` → `p@55w0rd123`

- **homoglyph, h** - Substitute with similar-looking characters
  - Useful for testing homograph attacks and IDN spoofing
  - Example: `redstr homoglyph "admin@example.com"` → `аdmіn@еxаmple.com`

- **unicode, u** - Random unicode variations
  - Useful for testing Unicode handling and normalization
  - Example: `redstr unicode "administrator"` → `ádmïnïštrâtör`

- **zalgo, z** - Add zalgo combining characters
  - Useful for testing display issues and Unicode handling
  - Example: `redstr zalgo "test"` → `t̃̂e̊̋s̈̃t̂̃`

- **rot13** - Apply ROT13 cipher
  - Classic cipher transformation
  - Example: `redstr rot13 "Hello World"` → `Uryyb Jbeyq`

- **vowel-swap, vs** - Swap vowels randomly
  - Useful for testing pattern matching and filters
  - Example: `redstr vowel-swap "testing"` → `tistong`

- **double, d** - Double random characters
  - Useful for testing input validation
  - Example: `redstr double "test"` → `tteesstt`

- **space-variants, sv** - Use various space characters
  - Useful for testing whitespace handling (uses various Unicode spaces)
  - Example: `redstr space-variants "hello world"`

- **mixed-encoding, me** - Mix character encodings
  - Useful for testing encoding vulnerabilities and XSS
  - Example: `redstr mixed-encoding "test"` → Mix of HTML entities and Unicode escapes

### Encoding and Obfuscation Modes

- **base64, b64** - Encode to Base64
  - Useful for red team payload obfuscation
  - Example: `redstr base64 "hello"` → `aGVsbG8=`

- **url-encode, url** - URL/percent encoding
  - Useful for web security testing
  - Example: `redstr url-encode "test @example.com"` → `test%20%40example.com`

- **hex-encode, hex** - Encode to hexadecimal
  - Useful for encoding obfuscation
  - Example: `redstr hex-encode "test"` → `74657374`

- **hex-mixed, hm** - Mixed hex formats (\\x, %, 0x, &#x)
  - Useful for testing encoding detection
  - Example: `redstr hex-mixed "ab"` → `\x61%62` (varies)

### Injection Testing Modes

- **sql-comment, sql** - Insert SQL comment patterns
  - Useful for red team SQL injection testing
  - Example: `redstr sql-comment "SELECT * FROM users"` → `SELECT --* FROM users`

- **xss-tags, xss** - Generate XSS tag variations
  - Useful for testing XSS filters
  - Example: `redstr xss-tags "<script>alert(1)</script>"` → Encoded variations

- **case-swap, cs** - Random case swapping
  - Useful for WAF/filter bypass testing
  - Example: `redstr case-swap "SELECT"` → `SeLeCt`

- **null-byte, nb** - Insert null byte representations
  - Useful for testing null byte vulnerabilities
  - Example: `redstr null-byte "test.txt"` → `test%00.txt` (varies)

- **path-traversal, pt** - Generate path traversal patterns
  - Useful for directory traversal testing
  - Example: `redstr path-traversal "/etc/passwd"` → `../etc/../passwd` (varies)

- **command-injection, ci** - Insert command injection separators
  - Useful for OS command injection testing
  - Example: `redstr command-injection "ping example.com"` → `ping;example.com` (varies)

## See Also

- [Use Cases Documentation](use-cases.md) - Detailed security testing scenarios
- [API Reference](api-reference.md) - Complete library function reference
- [Integration Examples](integration-examples.md) - Tool integration patterns
