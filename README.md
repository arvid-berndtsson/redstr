# random-cap

A Rust library for string obfuscation and transformation designed for security testing and penetration testing tools. Ideal for integration into tools like Caido, EvilJinx, urlscan.io, and bot detection systems.

## Features

Production-ready library for security professionals and tool developers:

- **Zero required dependencies** - Core library uses only Rust's standard library
- **30+ transformation functions** - Encoding, obfuscation, injection testing, and web-focused transformations
- **Builder pattern** - Chain multiple transformations fluently
- **Serialization support** - Optional serde integration for web APIs
- **Security-focused** - Designed for red/blue/purple team workflows and bot detection testing
- **Well-documented** - Complete API documentation with real-world integration examples

## Installation

### Basic Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
random-cap = "0.1.0"
```

### With Serde Support (for web tools)

```toml
[dependencies]
random-cap = { version = "0.1.0", features = ["serde"] }
```

## Quick Start

### Basic Usage

```rust
use random_cap::{
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
use random_cap::TransformBuilder;

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
use random_cap::{random_user_agent, url_encode, xss_tag_variations};

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
use random_cap::{domain_typosquat, homoglyph_substitution, html_entity_encode};

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
use random_cap::{
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
use random_cap::{url_encode, base64_encode, TransformBuilder};

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
cargo install random-cap --features cli
```

Or build from source with the CLI feature:

```bash
cargo build --release --features cli
```

The binary will be available at `target/release/random-cap`.

### CLI Usage

```bash
random-cap [mode] <text>
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
  - `.random_caps()` - Apply random capitalization
  - `.homoglyphs()` - Apply homoglyph substitution
  - `.case_swap()` - Apply case swapping
  - `.hex_encode()` - Apply hex encoding
  - `.rot13()` - Apply ROT13
  - `.build()` - Get the final result

See the [library documentation](https://docs.rs/random-cap) for detailed API documentation.

### CLI Transformation Modes

#### Basic Transformations

- **random, r** - Random capitalization (default)
  - Example: `random-cap "Hello World"` → `HeLlO wOrLd`
  
- **alternate, a** - Alternate upper/lower case
  - Example: `random-cap alternate "Hello World"` → `HeLlO wOrLd`
  
- **inverse, i** - Invert the case of each letter
  - Example: `random-cap inverse "Hello World"` → `hELLO wORLD`

- **reverse, rv** - Reverse the string
  - Example: `random-cap reverse "Hello World"` → `dlroW olleH`

#### Case Conversion

- **camel, c** - Convert to camelCase
  - Example: `random-cap camel "hello world test"` → `helloWorldTest`
  
- **snake, s** - Convert to snake_case
  - Example: `random-cap snake "HelloWorldTest"` → `hello_world_test`
  
- **kebab, k** - Convert to kebab-case
  - Example: `random-cap kebab "HelloWorldTest"` → `hello-world-test`

#### Security Testing Modes

- **leetspeak, l** - Convert to leetspeak
  - Useful for testing password filters and content detection
  - Example: `random-cap leetspeak "password123"` → `p@55w0rd123`

- **homoglyph, h** - Substitute with similar-looking characters
  - Useful for testing homograph attacks and IDN spoofing
  - Example: `random-cap homoglyph "admin@example.com"` → `аdmіn@еxаmple.com`

- **unicode, u** - Random unicode variations
  - Useful for testing Unicode handling and normalization
  - Example: `random-cap unicode "administrator"` → `ádmïnïštrâtör`

- **zalgo, z** - Add zalgo combining characters
  - Useful for testing display issues and Unicode handling
  - Example: `random-cap zalgo "test"` → `t̃̂e̊̋s̈̃t̂̃`

- **rot13** - Apply ROT13 cipher
  - Classic cipher transformation
  - Example: `random-cap rot13 "Hello World"` → `Uryyb Jbeyq`

- **vowel-swap, vs** - Swap vowels randomly
  - Useful for testing pattern matching and filters
  - Example: `random-cap vowel-swap "testing"` → `tistong`

- **double, d** - Double random characters
  - Useful for testing input validation
  - Example: `random-cap double "test"` → `tteesstt`

- **space-variants, sv** - Use various space characters
  - Useful for testing whitespace handling (uses various Unicode spaces)
  - Example: `random-cap space-variants "hello world"`

- **mixed-encoding, me** - Mix character encodings
  - Useful for testing encoding vulnerabilities and XSS
  - Example: `random-cap mixed-encoding "test"` → Mix of HTML entities and Unicode escapes

#### Encoding and Obfuscation Modes

- **base64, b64** - Encode to Base64
  - Useful for red team payload obfuscation
  - Example: `random-cap base64 "hello"` → `aGVsbG8=`

- **url-encode, url** - URL/percent encoding
  - Useful for web security testing
  - Example: `random-cap url-encode "test @example.com"` → `test%20%40example.com`

- **hex-encode, hex** - Encode to hexadecimal
  - Useful for encoding obfuscation
  - Example: `random-cap hex-encode "test"` → `74657374`

- **hex-mixed, hm** - Mixed hex formats (\\x, %, 0x, &#x)
  - Useful for testing encoding detection
  - Example: `random-cap hex-mixed "ab"` → `\x61%62` (varies)

#### Injection Testing Modes

- **sql-comment, sql** - Insert SQL comment patterns
  - Useful for red team SQL injection testing
  - Example: `random-cap sql-comment "SELECT * FROM users"` → `SELECT --* FROM users`

- **xss-tags, xss** - Generate XSS tag variations
  - Useful for testing XSS filters
  - Example: `random-cap xss-tags "<script>alert(1)</script>"` → Encoded variations

- **case-swap, cs** - Random case swapping
  - Useful for WAF/filter bypass testing
  - Example: `random-cap case-swap "SELECT"` → `SeLeCt`

- **null-byte, nb** - Insert null byte representations
  - Useful for testing null byte vulnerabilities
  - Example: `random-cap null-byte "test.txt"` → `test%00.txt` (varies)

- **path-traversal, pt** - Generate path traversal patterns
  - Useful for directory traversal testing
  - Example: `random-cap path-traversal "/etc/passwd"` → `../etc/../passwd` (varies)

- **command-injection, ci** - Insert command injection separators
  - Useful for OS command injection testing
  - Example: `random-cap command-injection "ping example.com"` → `ping;example.com` (varies)

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

## No Dependencies

This library uses only Rust's standard library and has zero external dependencies, making it lightweight and easy to audit.

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

## License

See LICENSE file for details.