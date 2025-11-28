# redstr-go - Go Bindings for redstr

Go bindings for the [redstr](https://github.com/arvid-berndtsson/redstr) Rust library, providing string transformation and obfuscation functions for security testing, penetration testing, and red/blue/purple team operations.

## Features

- **60+ Transformation Functions** - All redstr functions available in Go
- **Zero External Dependencies** - Only depends on the redstr Rust library
- **Native Performance** - Direct FFI to Rust via CGO
- **Idiomatic Go API** - Go-friendly function names and patterns
- **Builder Pattern** - Chain multiple transformations fluently
- **Cross-Platform** - Supports Linux, macOS, and Windows

## Installation

### Prerequisites

- Go 1.19 or higher
- Rust toolchain (for building the library)
- C compiler (for CGO)

### Building the Library

1. Clone the repository:
```bash
git clone https://github.com/arvid-berndtsson/redstr
cd redstr
```

2. Build the Rust library with FFI support:
```bash
./bindings/go/build.sh
```

This will:
- Build the Rust library with FFI features
- Generate the C header file
- Copy the library to the Go bindings directory

### Using in Your Go Project

The Go bindings are located in the `bindings/go/` directory of the main redstr repository. To use them in your project, use a local replace directive in your `go.mod`:

```go
module your-project

go 1.19

replace github.com/arvid-berndtsson/redstr-go => /path/to/redstr/bindings/go

require github.com/arvid-berndtsson/redstr-go v0.0.0
```

**Note:** These bindings are part of the main [redstr repository](https://github.com/arvid-berndtsson/redstr) and are not published as a separate Go module yet.

## Quick Start

### Basic Usage

```go
package main

import (
    "fmt"
    redstr "github.com/arvid-berndtsson/redstr-go"
)

func main() {
    // Random capitalization
    result := redstr.RandomizeCapitalization("Hello World")
    fmt.Println(result) // "HeLlO wOrLd"

    // Leetspeak
    obfuscated := redstr.Leetspeak("password")
    fmt.Println(obfuscated) // "p@55w0rd"

    // Base64 encoding
    encoded := redstr.Base64Encode("hello")
    fmt.Println(encoded) // "aGVsbG8="

    // Homoglyph substitution
    spoofed := redstr.HomoglyphSubstitution("admin")
    fmt.Println(spoofed) // "аdmіn" (Cyrillic)
}
```

### Builder Pattern

Chain multiple transformations:

```go
result := redstr.NewTransformBuilder("SELECT * FROM users").
    CaseSwap().
    Base64().
    Build()
fmt.Println(result)
```

### Security Testing Example

```go
// WAF bypass testing
sqlQuery := "SELECT * FROM users WHERE id = 1"
bypassed := redstr.SQLCommentInjection(sqlQuery)

// XSS payload obfuscation
xss := "<script>alert(1)</script>"
obfuscated := redstr.XSSTagVariations(xss)

// Phishing domain testing
domain := "paypal.com"
typosquat := redstr.DomainTyposquat(domain)
spoofed := redstr.AdvancedDomainSpoof(domain)
```

## API Reference

### Case Transformations

- `RandomizeCapitalization(input string) string` - Randomize case of each character
- `CaseSwap(input string) string` - Swap upper/lowercase
- `AlternateCase(input string) string` - Alternate between cases
- `InverseCase(input string) string` - Invert case
- `ToCamelCase(input string) string` - Convert to camelCase
- `ToSnakeCase(input string) string` - Convert to snake_case
- `ToKebabCase(input string) string` - Convert to kebab-case

### Encoding

- `Base64Encode(input string) string` - Base64 encoding
- `URLEncode(input string) string` - URL percent encoding
- `HexEncode(input string) string` - Hexadecimal encoding
- `HexEncodeMixed(input string) string` - Mixed-case hex
- `HTMLEntityEncode(input string) string` - HTML entity encoding
- `MixedEncoding(input string) string` - Random mixed encoding

### Unicode Transformations

- `HomoglyphSubstitution(input string) string` - Replace with similar Unicode chars
- `UnicodeVariations(input string) string` - Apply Unicode variation selectors
- `ZalgoText(input string) string` - Add combining diacritical marks
- `SpaceVariants(input string) string` - Replace spaces with Unicode variants
- `UnicodeNormalizeVariants(input string) string` - Different normalization forms

### Injection Testing

- `SQLCommentInjection(input string) string` - Insert SQL comments
- `XSSTagVariations(input string) string` - Generate XSS variations
- `NullByteInjection(input string) string` - Insert null bytes
- `PathTraversal(input string) string` - Generate path traversal
- `CommandInjection(input string) string` - Command injection patterns
- `MongoDBInjection(input string) string` - MongoDB injection
- `CouchDBInjection(input string) string` - CouchDB injection
- `DynamoDBObfuscate(input string) string` - DynamoDB obfuscation
- `NoSQLOperatorInjection(input string) string` - NoSQL operator injection
- `SSTIInjection(input string) string` - Server-side template injection
- `SSTIFrameworkVariation(template, framework string) string` - Framework-specific SSTI
- `SSTISyntaxObfuscate(input string) string` - Obfuscate SSTI syntax

### Obfuscation

- `Leetspeak(input string) string` - Convert to leetspeak
- `ROT13(input string) string` - Apply ROT13 cipher
- `VowelSwap(input string) string` - Swap vowels
- `DoubleCharacters(input string) string` - Double each character
- `ReverseString(input string) string` - Reverse the string
- `WhitespacePadding(input string) string` - Add random whitespace
- `JSStringConcat(input string) string` - JavaScript string concatenation

### Phishing

- `DomainTyposquat(input string) string` - Generate typosquatting domains
- `AdvancedDomainSpoof(input string) string` - Advanced IDN homograph attacks
- `EmailObfuscation(input string) string` - Obfuscate email addresses
- `URLShorteningPattern(input string) string` - URL shortening patterns

### Bot Detection Evasion

- `RandomUserAgent() string` - Generate random user agent
- `TLSFingerprintVariation(input string) string` - TLS fingerprint variations
- `AcceptLanguageVariation(input string) string` - Accept-Language variations
- `CloudflareChallengeVariation(input string) string` - Cloudflare challenge variations
- `HTTP2HeaderOrder(input string) string` - HTTP/2 header order variations

### Cloudflare Evasion

- `CloudflareTurnstileVariation(input string) string` - Turnstile challenge variations
- `CloudflareChallengeResponse(input string) string` - Challenge response patterns
- `TLSHandshakePattern(input string) string` - TLS handshake patterns
- `CanvasFingerprintVariation(input string) string` - Canvas fingerprint variations
- `WebGLFingerprintObfuscate(input string) string` - WebGL fingerprint obfuscation
- `FontFingerprintConsistency(input string) string` - Font fingerprint consistency

### Web Security

- `HTTPHeaderVariation(input string) string` - HTTP header variations
- `APIEndpointVariation(input string) string` - API endpoint variations
- `GraphQLObfuscate(input string) string` - GraphQL query obfuscation
- `SessionTokenVariation(input string) string` - Session token variations
- `GraphQLVariableInjection(input string) string` - GraphQL variable injection
- `GraphQLIntrospectionBypass(input string) string` - Introspection bypass
- `JWTHeaderManipulation(input string) string` - JWT header manipulation
- `JWTPayloadObfuscate(input string) string` - JWT payload obfuscation
- `JWTAlgorithmConfusion(input string) string` - JWT algorithm confusion
- `JWTSignatureBypass(input string) string` - JWT signature bypass

### Shell Commands

- `PowershellObfuscate(input string) string` - Obfuscate PowerShell
- `BashObfuscate(input string) string` - Obfuscate Bash commands
- `EnvVarObfuscate(input string) string` - Obfuscate environment variables
- `FilePathObfuscate(input string) string` - Obfuscate file paths

### Builder Pattern

```go
builder := redstr.NewTransformBuilder(input)
result := builder.
    Leetspeak().
    Base64().
    URLEncode().
    Build()
```

Available builder methods: `RandomizeCapitalization`, `CaseSwap`, `AlternateCase`, `Base64`, `URLEncode`, `HexEncode`, `Homoglyphs`, `Leetspeak`, `ROT13`, `Reverse`, `GraphQL`, `Powershell`, `Bash`, and more.

## Examples

See the [examples](examples/) directory for complete working examples:

- [basic/main.go](examples/basic/main.go) - Basic transformation examples
- [security/main.go](examples/security/main.go) - Security testing scenarios

Run examples:
```bash
cd examples/basic
go run main.go

cd examples/security
go run main.go
```

## Testing

Run the test suite:

```bash
cd bindings/go
go test -v
```

## Building

### Development Build

```bash
./build.sh debug
```

### Release Build

```bash
./build.sh release
```

## Use Cases

### Red Team / Offensive Security
- WAF bypass testing with encoding chains
- XSS payload obfuscation
- SQL injection with comment insertion
- Phishing domain generation
- Command injection testing
- Bot detection evasion

### Blue Team / Defensive Security
- Test security control effectiveness
- Validate input sanitization
- Test filter and detection systems
- Verify Unicode handling

### Purple Team
- Collaborative red/blue exercises
- Security tool validation
- Baseline security testing

## Performance

The Go bindings provide near-native Rust performance through CGO FFI. All transformations are performed in the Rust library with minimal overhead for string conversion.

## Cross-Platform Support

The bindings work on:
- **Linux** (x86_64, ARM64)
- **macOS** (Intel, Apple Silicon)
- **Windows** (x86_64)

The build script automatically detects the platform and builds the appropriate library format (`.so`, `.dylib`, or `.dll`).

## Troubleshooting

### CGO Errors

If you encounter CGO errors:
```bash
# Ensure CGO is enabled
export CGO_ENABLED=1

# On macOS, you may need to set C compiler
export CC=clang
```

### Library Not Found

Ensure the library is built:
```bash
./build.sh release
```

And that the path in `redstr.go` is correct:
```go
#cgo LDFLAGS: -L${SRCDIR}/../../target/release -lredstr
```

### Missing Dependencies

Install required build tools:
```bash
# Ubuntu/Debian
sudo apt-get install build-essential

# macOS
xcode-select --install

# Fedora/RHEL
sudo dnf install gcc
```

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](../../CONTRIBUTING.md) in the main repository.

## License

This project is licensed under the MIT License. See [LICENSE](../../LICENSE) for details.

## Responsible Use

This library is intended for authorized security testing, research, and defensive security purposes only. Users are responsible for ensuring they have proper authorization before using these techniques on systems they do not own or have explicit permission to test.

## Links

- **Main Repository**: https://github.com/arvid-berndtsson/redstr
- **Documentation**: https://docs.rs/redstr
- **Crates.io**: https://crates.io/crates/redstr
- **Issues**: https://github.com/arvid-berndtsson/redstr/issues
