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
use random_cap::{randomize_capitalization, leetspeak, homoglyph_substitution};

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

## Security Testing Use Cases

### Red Team Activities
- **Phishing campaigns**: Use homoglyph substitution to create convincing lookalike domains
- **Filter evasion**: Use leetspeak, unicode variations, or character doubling to bypass content filters
- **Payload obfuscation**: Mix encodings to evade detection systems

### Blue Team Activities
- **Detection testing**: Test if your filters catch variations like leetspeak or homoglyphs
- **Input validation**: Verify systems handle Unicode properly and reject malformed input
- **Log analysis**: Test logging systems with various character encodings

### Purple Team Activities
- **Collaboration**: Share obfuscated test data between red and blue teams
- **Baseline testing**: Create test cases for security controls
- **Training**: Generate examples for security awareness training

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