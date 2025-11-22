# random-cap

A versatile string obfuscation and transformation library for security testing, useful for red team, blue team, and purple team activities.

## Features

This library provides multiple string transformation functions designed to help security professionals test various scenarios. It can be used both as a library in your Rust projects or as a command-line tool.

**Primary use case:** Library for integration into security tools  
**Secondary use case:** Command-line binary for quick testing

## Library Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
random-cap = "0.1.0"
```

### Quick Example

```rust
use random_cap::{
    randomize_capitalization, leetspeak, homoglyph_substitution,
    base64_encode, sql_comment_injection, xss_tag_variations
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

    // Base64 encoding for payload obfuscation
    let encoded = base64_encode("alert('XSS')");
    println!("{}", encoded);  // "YWxlcnQoJ1hTUycp"

    // SQL injection testing
    let sql_test = sql_comment_injection("SELECT * FROM users");
    println!("{}", sql_test);  // e.g., "SELECT --* FROM users"

    // XSS filter evasion
    let xss_test = xss_tag_variations("<script>alert(1)</script>");
    println!("{}", xss_test);  // Encoded variations
}
```

See the [examples directory](examples/) for more detailed usage examples.

## Command-Line Usage

### Installation

```bash
cargo install random-cap
# Or build from source:
cargo build --release
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

This tool uses only Rust's standard library and has zero external dependencies, making it lightweight and easy to audit.

## Building

```bash
cargo build --release
```

## Running Tests

```bash
cargo test
```

## License

See LICENSE file for details.