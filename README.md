# redstr

[![Crates.io](https://img.shields.io/crates/v/redstr.svg)](https://crates.io/crates/redstr)
[![Documentation](https://docs.rs/redstr/badge.svg)](https://docs.rs/redstr)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/arvid-berndtsson/redstr/blob/main/LICENSE)

**Red team string transformation library for offensive security operations, penetration testing, and evasion techniques.**

A comprehensive Rust library providing 60+ string obfuscation and transformation functions for red team, blue team, and purple team security operations. Perfect for building security tools like Caido and Burp Suite extensions, phishing frameworks, WAF bypass testing, and bot detection evasion.

## 🎯 Use Cases

- **Red Team**: WAF bypass, XSS/SQL injection evasion, phishing domain generation, payload obfuscation
- **Blue Team**: Security control testing, filter validation, detection system testing
- **Purple Team**: Collaborative testing, baseline establishment, continuous improvement

👉 **[View Detailed Use Cases](docs/use-cases.md)** - Comprehensive security testing scenarios

## 🚀 Features

- **60+ transformation functions** - Encoding, obfuscation, injection testing, web security
- **Zero dependencies** - Core library uses only Rust's standard library
- **Builder pattern API** - Chain transformations with `TransformBuilder`
- **Multi-language support** - Rust, JavaScript, Python, Go via HTTP API
- **CLI tool** - Optional command-line interface
- **Production-ready** - Performance optimized, well-documented, thoroughly tested

📊 **[Performance Benchmarks](docs/performance.md)** | 🔧 **[API Reference](docs/api-reference.md)**

## 🤔 Why redstr?

- **For Tool Developers**: Integrate into Caido, Burp Suite, or custom security tools
- **For Pen Testers**: Generate payload variations, bypass filters, test input validation
- **For Researchers**: Test detection engines, research evasion techniques

**Advantages:**
- Native Rust performance with type-safe API
- Zero required dependencies - easy to audit
- 60+ functions covering modern security techniques

## 📦 Installation

### Rust (Native)

Add this to your `Cargo.toml`:

```toml
[dependencies]
redstr = "0.2.3"
```

### With Serde Support (for web tools)

```toml
[dependencies]
redstr = { version = "0.2.3", features = ["serde"] }
```

### Other Languages & Platforms

**API Server (Recommended for all languages)**:
```bash
# Install the HTTP API server
cargo install redstr-server

# Or use Docker
docker pull arvid-berndtsson/redstr-server
```
See [API Server Documentation](docs/api_server.md) for usage examples in JavaScript, Python, Go, Ruby, and more.

**Coming Soon**:
- 🔜 **Homebrew**: `brew install redstr` (macOS/Linux)
- 🔜 **npm**: `npm install @redstr/core` (JavaScript/TypeScript)
- 🔜 **PyPI**: `pip install redstr` (Python)
- 🔜 **Go Module**: `go get github.com/arvid-berndtsson/redstr-go`
- 🔜 **Raycast Extension**: Quick launcher integration

Track progress in our [roadmap](roadmap/TASKS.md#15-distribution--language-bindings).

## Quick Start

### Basic Usage

```rust
use redstr::{
    randomize_capitalization, leetspeak, homoglyph_substitution,
    base64_encode, random_user_agent, domain_typosquat
};

fn main() {
    // Random capitalization
    let result = randomize_capitalization("Hello World");
    println!("{}", result);  // "HeLlO wOrLd"

    // Leetspeak for filter testing
    let obfuscated = leetspeak("password");
    println!("{}", obfuscated);  // "p@55w0rd"

    // Homoglyph substitution for phishing tests
    let spoofed = homoglyph_substitution("admin@example.com");
    println!("{}", spoofed);  // "аdmіn@еxаmple.com" (Cyrillic)

    // Random user agent for bot evasion
    let ua = random_user_agent();
}
```

### Builder Pattern

Chain multiple transformations:

```rust
use redstr::TransformBuilder;

// Complex payload generation
let payload = TransformBuilder::new("SELECT * FROM users")
    .case_swap()
    .base64()
    .build();
```

📚 **[View More Examples](examples/)** | 🔧 **[API Reference](docs/api-reference.md)**

## Integration Examples

```rust
use redstr::{random_user_agent, domain_typosquat, TransformBuilder};

// Web security testing - randomize requests
let ua = random_user_agent();

// Phishing detection - generate domain variations
let suspicious = domain_typosquat("paypal.com");

// WAF bypass - chain transformations
let payload = TransformBuilder::new("SELECT * FROM users")
    .case_swap()
    .url_encode()
    .build();
```

🔗 **[Detailed Integration Examples](docs/integration-examples.md)** - Caido, Burp Suite, EvilJinx, and more

## Command-Line Tool

An optional CLI is available for quick testing:

```bash
# Install with CLI feature
cargo install redstr --features cli

# Usage
redstr [mode] <text>
redstr leetspeak "password"    # → "p@55w0rd"
redstr base64 "hello"          # → "aGVsbG8="
```

📖 **[Complete CLI Reference](docs/cli-reference.md)** - All transformation modes and examples

## Library API

All functions accept `&str` and return `String`. Key functions include:

**Encoding**: `base64_encode`, `url_encode`, `hex_encode`, `html_entity_encode`  
**Obfuscation**: `leetspeak`, `homoglyph_substitution`, `case_swap`, `unicode_variations`  
**Injection Testing**: `sql_comment_injection`, `xss_tag_variations`, `path_traversal`  
**Web Security**: `random_user_agent`, `domain_typosquat`, `js_string_concat`  
**Case Conversion**: `to_camel_case`, `to_snake_case`, `to_kebab_case`

📖 **[Complete API Reference](docs/api-reference.md)** - All 60+ functions with examples



## Dependencies

**Zero required dependencies** - Core library uses only Rust's standard library.

Optional: `serde` feature for serialization support.

## Building & Testing

```bash
# Build library
cargo build --release

# Build with CLI
cargo build --release --features cli

# Run tests
cargo test
```

## 🔌 Integrations

Official integrations (separate repositories):
- **[redstr-server](https://github.com/arvid-berndtsson/redstr-server)** - HTTP API server
- **[redstr-burp](https://github.com/arvid-berndtsson/redstr-burp)** - Burp Suite extension
- **[redstr-caido](https://github.com/arvid-berndtsson/redstr-caido)** - Caido plugin
- **[redstr-eviljinx](https://github.com/arvid-berndtsson/redstr-eviljinx)** - EvilJinx integration
- **[redstr-owasp-zap](https://github.com/arvid-berndtsson/redstr-owasp-zap)** - OWASP ZAP add-on

**Building your own integration?** See [Integration Guidelines](docs/INTEGRATION_GUIDELINES.md)

## 🤝 Contributing

Contributions are welcome! This library is designed for the security community. Whether you're adding new transformation functions, improving documentation, or reporting issues, your input helps make security testing more effective.

See [CONTRIBUTING.md](CONTRIBUTING.md) for development guidelines and [docs/INTEGRATION_GUIDELINES.md](docs/INTEGRATION_GUIDELINES.md) for integration guidance.

## 📖 Documentation

- **[API Reference](docs/api-reference.md)** - Complete function reference
- **[CLI Reference](docs/cli-reference.md)** - Command-line tool documentation
- **[Use Cases](docs/use-cases.md)** - Security testing scenarios
- **[Integration Examples](docs/integration-examples.md)** - Tool integration patterns
- **[docs.rs/redstr](https://docs.rs/redstr)** - Full library documentation
- **[examples/](examples/)** - Code examples

## ⚠️ Responsible Use

This library is intended for authorized security testing, research, and defensive security purposes only. Users are responsible for ensuring they have proper authorization before using these techniques on systems they do not own or have explicit permission to test.

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/arvid-berndtsson/redstr/blob/main/LICENSE) file for details.