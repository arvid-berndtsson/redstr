# redstr

[![Crates.io](https://img.shields.io/crates/v/redstr.svg)](https://crates.io/crates/redstr)
[![Documentation](https://docs.rs/redstr/badge.svg)](https://docs.rs/redstr)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/arvid-berndtsson/redstr/blob/main/LICENSE)

**Red team string transformation library for offensive security operations, penetration testing, and evasion techniques.**

A comprehensive Rust library providing 30+ string obfuscation and transformation functions for red team operations, blue team defense testing, and purple team collaboration. Perfect for security professionals building tools like Caido, Burp Suite extensions, phishing frameworks (EvilJinx), WAF bypass testing, XSS detection, SQL injection testing, and bot detection evasion.

## 🎯 Use Cases

**Red Team / Offensive Security:**
- WAF (Web Application Firewall) bypass techniques
- XSS (Cross-Site Scripting) payload obfuscation
- SQL injection evasion patterns
- Phishing domain generation (typosquatting, homoglyphs)
- Command injection testing
- Path traversal attacks
- Bot detection evasion
- Payload encoding and obfuscation

**Blue Team / Defensive Security:**
- Security control testing
- Filter and detection validation
- Input sanitization testing
- Threat intelligence enrichment
- Malicious content detection
- Log analysis and normalization

**Purple Team / Security Testing:**
- Collaborative red/blue exercises
- Baseline security testing
- Security awareness training
- Vulnerability assessment
- Penetration testing automation

## 🚀 Features

**Production-ready library for security professionals and tool developers:**

- **Minimal dependencies** - Core library uses only Rust's standard library (optional serde for serialization)
- **30+ transformation functions** - Encoding, obfuscation, injection testing, and web-focused transformations
- **Builder pattern API** - Chain multiple transformations fluently with `TransformBuilder`
- **Serialization support** - Optional serde integration for web APIs and tool integration
- **Performance optimized** - Efficient string operations for high-throughput scenarios
- **Security-focused** - Designed for red/blue/purple team workflows and bot detection testing
- **Well-documented** - Complete API documentation with real-world integration examples
- **CLI tool included** - Optional command-line interface for quick testing

See [Performance Documentation](docs/performance.md) for detailed benchmarks and methodology.

## 🤔 Why redstr?

**For Security Tool Developers:**
- Integrate into Caido, Burp Suite, or custom security proxies
- Build phishing frameworks and social engineering tools
- Create WAF testing and bypass automation
- Develop bot detection evasion systems
- Build URL scanners and malware analysis tools

**For Penetration Testers:**
- Generate payload variations for manual testing
- Bypass security controls and filters
- Test input validation and encoding
- Create phishing campaigns with typosquatting
- Obfuscate attack payloads

**For Security Researchers:**
- Test detection engines and security controls
- Research evasion techniques
- Validate security implementations
- Create proof-of-concept exploits
- Analyze filter bypass methods

**Compared to alternatives:**
- **Native Rust performance** - No Python or JavaScript overhead
- **Type-safe API** - Compile-time guarantees
- **Minimal dependencies** - Core library has no required dependencies
- **Comprehensive coverage** - 30+ functions in one library
- **Active development** - Modern security techniques

## 📦 Installation

### Basic Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
redstr = "0.1.0"
```

### With Serde Support (for web tools)

```toml
[dependencies]
redstr = { version = "0.1.0", features = ["serde"] }
```

## Quick Start

### Basic Usage

```rust
use redstr::{
    randomize_capitalization, leetspeak, homoglyph_substitution,
    base64_encode, sql_comment_injection, xss_tag_variations,
    random_user_agent, domain_typosquat
};

fn main() {
    // Random capitalization
    let result = randomize_capitalization("Hello World");
    println!("{}", result);  // e.g., "HeLlO wOrLd"

    // Leetspeak for filter testing
    let obfuscated = leetspeak("password");
    println!("{}", obfuscated);  // e.g., "p@55w0rd"

    // Homoglyph substitution for phishing tests
    let spoofed = homoglyph_substitution("admin@example.com");
    println!("{}", spoofed);  // e.g., "аdmіn@еxаmple.com"

    // Web scraping with random user agents
    let ua = random_user_agent();
    println!("{}", ua);  // Random modern browser UA

    // Domain typosquatting for phishing detection
    let typo = domain_typosquat("example.com");
    println!("{}", typo);  // e.g., "examp1e.com", "exmaple.com"
}
```

### Builder Pattern (New!)

Chain multiple transformations fluently:

```rust
use redstr::TransformBuilder;

fn main() {
    // Chain transformations
    let result = TransformBuilder::new("admin@example.com")
        .homoglyphs()
        .url_encode()
        .build();
    
    // Perfect for creating complex payloads
    let payload = TransformBuilder::new("SELECT * FROM users")
        .case_swap()
        .base64()
        .build();
}
```

## Integration Examples

### Caido / Web Security Testing Tools

```rust
use redstr::{random_user_agent, url_encode, xss_tag_variations};

// Randomize requests to avoid fingerprinting
let headers = vec![
    ("User-Agent", random_user_agent()),
];

// Test XSS payloads with variations
let payload = "<script>alert(1)</script>";
let variations = vec![
    xss_tag_variations(payload),
    url_encode(&xss_tag_variations(payload)),
];
```

### EvilJinx / Phishing Frameworks

```rust
use redstr::{domain_typosquat, homoglyph_substitution, html_entity_encode};

// Generate phishing domains
let target = "paypal.com";
let typosquat = domain_typosquat(target);
let homoglyph = homoglyph_substitution(target);

// Obfuscate phishing page content
let link = "https://secure.paypal.com/login";
let obfuscated = html_entity_encode(&homoglyph_substitution(link));
```

### Bot Detection Testing

```rust
use redstr::{
    random_user_agent, 
    js_string_concat, 
    unicode_normalize_variants,
    whitespace_padding
};

// Simulate various bot evasion techniques
let user_agent = random_user_agent();
let obfuscated_js = js_string_concat("document.cookie");
let normalized = unicode_normalize_variants("robot");
```

### URL Scanner / Web Crawler Integration

```rust
use redstr::{url_encode, base64_encode, TransformBuilder};

// Encode URLs for safe storage/transmission
let suspicious_url = "http://example.com/path?param=<script>";
let safe_url = url_encode(suspicious_url);

// Create encoded payloads for analysis
let payload = TransformBuilder::new("malicious-content")
    .base64()
    .url_encode()
    .build();
```

See the [examples directory](examples/) for more detailed usage examples.

## Optional Command-Line Tool

An optional CLI binary is available for quick testing and experimentation.

### Installation

To install with the CLI tool:

```bash
cargo install redstr --features cli
```

Or build from source with the CLI feature:

```bash
cargo build --release --features cli
```

The binary will be available at `target/release/redstr`.

### CLI Usage

```bash
redstr [mode] <text>
```

If no mode is specified, random capitalization is used by default.

## Library API

All transformation functions accept a `&str` and return a `String`. Here are the available functions:

### Basic Transformations
- `randomize_capitalization(input: &str) -> String` - Random capitalization
- `alternate_case(input: &str) -> String` - Alternate upper/lower case
- `inverse_case(input: &str) -> String` - Invert case of each letter
- `reverse_string(input: &str) -> String` - Reverse the string

### Case Conversion
- `to_camel_case(input: &str) -> String` - Convert to camelCase
- `to_snake_case(input: &str) -> String` - Convert to snake_case
- `to_kebab_case(input: &str) -> String` - Convert to kebab-case

### Security Testing Functions
- `leetspeak(input: &str) -> String` - Convert to leetspeak
- `homoglyph_substitution(input: &str) -> String` - Substitute with lookalike characters
- `unicode_variations(input: &str) -> String` - Random unicode variations
- `zalgo_text(input: &str) -> String` - Add zalgo combining characters
- `rot13(input: &str) -> String` - Apply ROT13 cipher
- `vowel_swap(input: &str) -> String` - Swap vowels randomly
- `double_characters(input: &str) -> String` - Double random characters
- `space_variants(input: &str) -> String` - Use various space characters
- `mixed_encoding(input: &str) -> String` - Mix character encodings

### Encoding and Obfuscation Functions
- `base64_encode(input: &str) -> String` - Encode to Base64
- `url_encode(input: &str) -> String` - URL/percent encoding
- `hex_encode(input: &str) -> String` - Encode to hexadecimal
- `hex_encode_mixed(input: &str) -> String` - Mixed hex formats (\\x, %, 0x, &#x)

### Injection Testing Functions
- `sql_comment_injection(input: &str) -> String` - Insert SQL comment patterns
- `xss_tag_variations(input: &str) -> String` - Generate XSS tag variations
- `case_swap(input: &str) -> String` - Random case swapping for WAF bypass
- `null_byte_injection(input: &str) -> String` - Insert null byte representations
- `path_traversal(input: &str) -> String` - Generate path traversal patterns
- `command_injection(input: &str) -> String` - Insert command injection separators

### Web Security Functions (NEW!)
- `random_user_agent() -> String` - Generate random browser user-agent strings
- `domain_typosquat(domain: &str) -> String` - Create typosquatting variations for phishing
- `html_entity_encode(input: &str) -> String` - Encode using HTML entities
- `js_string_concat(input: &str) -> String` - JavaScript string concatenation obfuscation
- `unicode_normalize_variants(input: &str) -> String` - Unicode normalization variations
- `whitespace_padding(input: &str) -> String` - Add random whitespace for filter bypass

### Builder API (NEW!)
- `TransformBuilder::new(input: &str)` - Create a transformation chain
  - `.leetspeak()` - Apply leetspeak
  - `.base64()` - Apply base64 encoding
  - `.url_encode()` - Apply URL encoding
  - `.redstrs()` - Apply random capitalization
  - `.homoglyphs()` - Apply homoglyph substitution
  - `.case_swap()` - Apply case swapping
  - `.hex_encode()` - Apply hex encoding
  - `.rot13()` - Apply ROT13
  - `.build()` - Get the final result

See the [library documentation](https://docs.rs/redstr) for detailed API documentation.

### CLI Transformation Modes

#### Basic Transformations

- **random, r** - Random capitalization (default)
  - Example: `redstr "Hello World"` → `HeLlO wOrLd`
  
- **alternate, a** - Alternate upper/lower case
  - Example: `redstr alternate "Hello World"` → `HeLlO wOrLd`
  
- **inverse, i** - Invert the case of each letter
  - Example: `redstr inverse "Hello World"` → `hELLO wORLD`

- **reverse, rv** - Reverse the string
  - Example: `redstr reverse "Hello World"` → `dlroW olleH`

#### Case Conversion

- **camel, c** - Convert to camelCase
  - Example: `redstr camel "hello world test"` → `helloWorldTest`
  
- **snake, s** - Convert to snake_case
  - Example: `redstr snake "HelloWorldTest"` → `hello_world_test`
  
- **kebab, k** - Convert to kebab-case
  - Example: `redstr kebab "HelloWorldTest"` → `hello-world-test`

#### Security Testing Modes

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

#### Encoding and Obfuscation Modes

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

#### Injection Testing Modes

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

## Security Testing Use Cases

### Red Team Activities

#### Phishing and Social Engineering
- **Domain spoofing**: Use `homoglyph_substitution` to create convincing lookalike domains
  - `paypal.com` → `pаypаl.com` (using Cyrillic characters)
- **Email obfuscation**: Combine unicode variations with case swapping to evade email filters

#### Filter and WAF Evasion
- **Content filter bypass**: Use `leetspeak`, `unicode_variations`, or `case_swap` to bypass content filters
  - `malware` → `m@1w@r3` or `mAlWaRe`
- **SQL injection**: Use `sql_comment_injection` to insert SQL comments and evade WAF detection
  - `SELECT * FROM users` → `SELECT --* FROM /**/users`
- **XSS filter evasion**: Use `xss_tag_variations` to bypass XSS filters
  - `<script>` → `&#x3C;sCrIpT&#x3E;`
- **Command injection**: Use `command_injection` to test command separator filtering

#### Payload Obfuscation
- **Encoding obfuscation**: Use `base64_encode`, `url_encode`, `hex_encode`, or `hex_encode_mixed`
- **Mixed encoding**: Combine `mixed_encoding` with other transformations to evade detection systems
- **Path traversal**: Use `path_traversal` to test directory traversal vulnerabilities
  - `/etc/passwd` → `../etc/../passwd`
- **Null byte injection**: Use `null_byte_injection` to test null byte vulnerabilities
  - `file.txt` → `file%00.txt`

### Blue Team Activities

#### Detection and Validation Testing
- **Filter testing**: Test if your content filters catch variations like leetspeak or homoglyphs
  - Generate test cases for blocked words using multiple transformations
- **XSS detection**: Verify your XSS filters catch obfuscated payloads
- **SQL injection detection**: Test if your WAF detects SQL injection patterns with comments
- **Input validation**: Verify systems handle Unicode properly and reject malformed input
- **Encoding detection**: Test if your systems properly detect and decode various encoding schemes

#### Security Control Testing
- **URL encoding validation**: Use `url_encode` to test URL parsers and validators
- **Path validation**: Use `path_traversal` to test path sanitization functions
- **Command validation**: Use `command_injection` to test command sanitization
- **Null byte handling**: Use `null_byte_injection` to verify proper null byte handling

#### Monitoring and Logging
- **Log analysis**: Test logging systems with various character encodings to ensure proper logging
- **Alert generation**: Verify security monitoring systems trigger on obfuscated attacks
- **Normalization testing**: Test if logs properly normalize Unicode and encoded strings

### Purple Team Activities

#### Collaborative Testing
- **Shared test cases**: Use transformations to create consistent test payloads for both teams
- **Baseline establishment**: Generate standard test cases for security controls
- **Detection validation**: Red team uses transformations, blue team validates detection

#### Training and Documentation
- **Security awareness**: Generate examples for security training programs
  - Show how phishing domains can be spoofed with homoglyphs
  - Demonstrate filter evasion techniques
- **Playbook development**: Create standard attack patterns and detection rules
- **Tool validation**: Test security tools against various obfuscation techniques

#### Continuous Improvement
- **Coverage testing**: Ensure security controls cover all transformation variations
- **Gap analysis**: Identify missing detection rules using transformation permutations
- **Effectiveness metrics**: Measure detection rates across different obfuscation techniques

## Dependencies

The core library has **no required dependencies** and uses only Rust's standard library, making it lightweight and easy to audit.

**Optional dependencies:**
- `serde` - For serialization support when using the `serde` feature flag

**Development dependencies:**
- `cc-check` - For conventional commit checking in CI/CD

## Building

To build the library:

```bash
cargo build --release
```

To build with the optional CLI tool:

```bash
cargo build --release --features cli
```

## Running Tests

```bash
cargo test
```

## 📚 Complete Function Reference

### Encoding & Obfuscation
- **base64_encode** - Base64 encoding for payload obfuscation
- **url_encode** - RFC 3986 URL/percent encoding with UTF-8 support
- **hex_encode** - Hexadecimal encoding (lowercase)
- **hex_encode_mixed** - Mixed hex formats (`\x`, `%`, `0x`, `&#x`)
- **html_entity_encode** - HTML entity encoding for XSS testing
- **mixed_encoding** - Mixed character encodings (HTML entities + Unicode)

### String Transformation
- **randomize_capitalization** - Random case for each character
- **alternate_case** - Alternating upper/lowercase
- **inverse_case** - Invert case of all characters
- **case_swap** - Random case mutation for WAF bypass
- **leetspeak** - Convert to 1337speak for filter evasion
- **rot13** - ROT13 cipher transformation
- **reverse_string** - Reverse string order

### Unicode & Homoglyphs
- **homoglyph_substitution** - Lookalike character substitution for phishing
- **unicode_variations** - Random Unicode character variations
- **unicode_normalize_variants** - Unicode normalization testing
- **zalgo_text** - Combining characters for display corruption

### Case Conversion
- **to_camel_case** - Convert to camelCase
- **to_snake_case** - Convert to snake_case
- **to_kebab_case** - Convert to kebab-case

### Injection Testing
- **sql_comment_injection** - SQL comment patterns (`--`, `/**/`, `#`)
- **xss_tag_variations** - XSS tag obfuscation and encoding
- **command_injection** - OS command separators (`;`, `|`, `&&`)
- **path_traversal** - Directory traversal patterns (`../`, `..\\`)
- **null_byte_injection** - Null byte representations (`%00`, `\0`)

### Web Security
- **random_user_agent** - Generate random browser user-agents
- **domain_typosquat** - Typosquatting variations for phishing
- **js_string_concat** - JavaScript string concatenation obfuscation
- **whitespace_padding** - Random whitespace for filter bypass

### Utility Functions
- **vowel_swap** - Swap vowels for pattern matching tests
- **double_characters** - Random character doubling
- **space_variants** - Various Unicode space characters

### Builder Pattern
- **TransformBuilder** - Fluent API for chaining transformations

## 🔍 SEO Keywords

**Security Testing:** WAF bypass, XSS evasion, SQL injection, phishing detection, bot detection, security automation, penetration testing tools, red team tools, blue team defense, purple team testing

**Techniques:** String obfuscation, payload encoding, filter bypass, evasion techniques, homoglyph attacks, typosquatting, domain spoofing, Unicode normalization, character encoding

**Tool Integration:** Caido integration, Burp Suite, EvilJinx, urlscan.io, Cloudflare bypass, security proxies, phishing frameworks, malware analysis

**Technologies:** Rust security library, minimal-dependency Rust, type-safe security, polymorphic strings, transformation library

## 🔌 Building Integrations

### Want to integrate redstr with your security tool?

**For tool-specific add-ons/extensions (Burp Suite, OWASP ZAP, Caido, etc.):**
- Create a **separate repository** for your integration
- See our [Integration Guidelines](docs/INTEGRATION_GUIDELINES.md) for detailed guidance
- We'll help promote your integration!

**For code examples and library enhancements:**
- Contribute to this repository
- Add examples to `examples/` directory
- Submit documentation to `docs/`

**Common questions:**
- **Q: Should I build an OWASP ZAP add-on in another repo?**  
  **A: Yes!** ZAP add-ons should be separate repos. [See why →](docs/INTEGRATION_GUIDELINES.md#owasp-zap-add-on-specific-guidance)

- **Q: What about Burp Suite extensions?**  
  **A: Also separate repos.** [Learn more →](docs/INTEGRATION_GUIDELINES.md)

📚 **Read the full [Integration Guidelines](docs/INTEGRATION_GUIDELINES.md)** for complete details.

## 🤝 Contributing

Contributions are welcome! This library is designed for the security community. Whether you're adding new transformation functions, improving documentation, or reporting issues, your input helps make security testing more effective.

See [CONTRIBUTING.md](CONTRIBUTING.md) for development guidelines and [docs/INTEGRATION_GUIDELINES.md](docs/INTEGRATION_GUIDELINES.md) for integration guidance.

## 📖 Learn More

- **Documentation**: [docs.rs/redstr](https://docs.rs/redstr)
- **Repository**: [GitHub](https://github.com/arvid-berndtsson/redstr)
- **Examples**: See the `examples/` directory for comprehensive integration patterns
- **Integration Guidelines**: [docs/INTEGRATION_GUIDELINES.md](docs/INTEGRATION_GUIDELINES.md)
- **Blog Posts**: Check for community blog posts and integration guides (coming soon)

## ⚠️ Responsible Use

This library is intended for authorized security testing, research, and defensive security purposes only. Users are responsible for ensuring they have proper authorization before using these techniques on systems they do not own or have explicit permission to test.

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/arvid-berndtsson/redstr/blob/main/LICENSE) file for details.