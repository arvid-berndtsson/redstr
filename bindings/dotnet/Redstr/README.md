# Redstr - .NET Bindings

[![NuGet](https://img.shields.io/nuget/v/Redstr.svg)](https://www.nuget.org/packages/Redstr/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/arvid-berndtsson/redstr/blob/main/LICENSE)

.NET bindings for the [redstr](https://github.com/arvid-berndtsson/redstr) library - a comprehensive string obfuscation and transformation library for security testing, penetration testing, and red/blue/purple team operations.

## Features

- **60+ transformation functions** - Encoding, obfuscation, injection testing, web security
- **Zero managed dependencies** - Pure .NET with native Rust core
- **Cross-platform** - Windows, Linux, macOS (x64 and ARM64)
- **Type-safe API** - Idiomatic C# API with XML documentation
- **Production-ready** - Performance optimized, well-documented, thoroughly tested

## Installation

Install via NuGet Package Manager:

```bash
dotnet add package Redstr
```

Or via Package Manager Console:

```powershell
Install-Package Redstr
```

## Quick Start

```csharp
using Redstr;

// Random capitalization for filter bypass
var result = RedstrTransform.RandomizeCapitalization("Hello World");
Console.WriteLine(result);  // "HeLlO wOrLd" (varies each run)

// Leetspeak for content filter evasion
var obfuscated = RedstrTransform.Leetspeak("password");
Console.WriteLine(obfuscated);  // "p@55w0rd" or "p4$$w0rd"

// Homoglyph substitution for phishing tests
var spoofed = RedstrTransform.HomoglyphSubstitution("admin@example.com");
Console.WriteLine(spoofed);  // "аdmіn@еxаmple.com" (Cyrillic)

// Random user agent for bot evasion
var ua = RedstrTransform.RandomUserAgent();
Console.WriteLine(ua);
```

## API Categories

### Case Transformations
- `RandomizeCapitalization` - Random case variation
- `CaseSwap` - Swap upper/lowercase
- `AlternateCase` - Alternate character case
- `ToCamelCase`, `ToSnakeCase`, `ToKebabCase` - Case conversions

### Encoding
- `Base64Encode` - Base64 encoding
- `UrlEncode` - URL encoding
- `HexEncode`, `HexEncodeMixed` - Hexadecimal encoding
- `HtmlEntityEncode` - HTML entity encoding
- `MixedEncoding` - Mixed encoding techniques

### Unicode Transformations
- `HomoglyphSubstitution` - Visually similar Unicode characters
- `UnicodeVariations` - Unicode variations
- `ZalgoText` - Combining characters for "zalgo" effect
- `SpaceVariants` - Unicode space variations

### Injection Testing
- `SqlCommentInjection` - SQL injection patterns
- `XssTagVariations` - XSS tag variations
- `CommandInjection` - Command injection patterns
- `PathTraversal` - Path traversal patterns
- `MongoDbInjection`, `CouchDbInjection`, `DynamoDbObfuscate` - NoSQL injection

### Obfuscation
- `Leetspeak` - 1337 speak conversion
- `Rot13` - ROT13 cipher
- `DoubleCharacters` - Character doubling
- `ReverseString` - String reversal
- `JsStringConcat` - JavaScript string concatenation

### Phishing
- `DomainTyposquat` - Typosquatting variations
- `AdvancedDomainSpoof` - Domain spoofing
- `EmailObfuscation` - Email obfuscation

### Web Security
- `JwtHeaderManipulation` - JWT header manipulation
- `JwtPayloadObfuscate` - JWT payload obfuscation
- `GraphqlObfuscate` - GraphQL query obfuscation
- `ApiEndpointVariation` - API endpoint variations

### Bot Detection
- `RandomUserAgent` - Random user agent generation
- `TlsFingerprintVariation` - TLS fingerprint variations
- `Http2HeaderOrder` - HTTP/2 header order variations
- `CloudflareChallengeVariation` - Cloudflare challenge variations

### Shell Commands
- `PowershellObfuscate` - PowerShell command obfuscation
- `BashObfuscate` - Bash command obfuscation
- `EnvVarObfuscate` - Environment variable obfuscation
- `FilePathObfuscate` - File path obfuscation

## Use Cases

### Red Team / Offensive Security
- WAF bypass with case variations and encoding
- XSS payload obfuscation
- SQL injection with comment insertion
- Phishing domain generation
- Command injection testing

### Blue Team / Defensive Security
- Test security control effectiveness
- Validate input sanitization
- Test filter and detection systems
- Verify Unicode handling

### Purple Team / Security Testing
- Collaborative red/blue exercises
- Security tool validation
- Baseline security testing

## Requirements

- .NET 8.0 or later
- Native library included for:
  - Windows (x64)
  - Linux (x64)
  - macOS (x64, ARM64)

## Building from Source

```bash
# Build the Rust library with FFI support
cargo build --release --features ffi

# Build the .NET library
cd bindings/dotnet/Redstr
dotnet build

# Run tests
dotnet test

# Create NuGet package
dotnet pack -c Release
```

## Documentation

- [Full API Reference](https://docs.rs/redstr)
- [Main Repository](https://github.com/arvid-berndtsson/redstr)
- [Use Cases Guide](https://github.com/arvid-berndtsson/redstr/blob/main/docs/use-cases.md)

## ⚠️ Responsible Use

This library is intended for authorized security testing, research, and defensive security purposes only. Users are responsible for ensuring they have proper authorization before using these techniques on systems they do not own or have explicit permission to test.

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/arvid-berndtsson/redstr/blob/main/LICENSE) file for details.

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](https://github.com/arvid-berndtsson/redstr/blob/main/CONTRIBUTING.md) for guidelines.
