# Changelog - redstr Go Bindings

## [0.1.0] - 2024-11-27

### Added
- Initial release of Go bindings for redstr library
- C FFI layer with 60+ transformation functions
- Go wrapper with idiomatic API design
- TransformBuilder for fluent transformation chaining
- Comprehensive test suite (19 tests)
- Cross-platform support (Linux, macOS, Windows)

#### Case Transformations (7 functions)
- `RandomizeCapitalization` - Random capitalization for filter bypass
- `CaseSwap` - Swap uppercase/lowercase
- `AlternateCase` - Alternate between cases
- `InverseCase` - Invert case
- `ToCamelCase` - Convert to camelCase
- `ToSnakeCase` - Convert to snake_case
- `ToKebabCase` - Convert to kebab-case

#### Encoding (6 functions)
- `Base64Encode` - Base64 encoding
- `URLEncode` - URL percent encoding
- `HexEncode` - Hexadecimal encoding
- `HexEncodeMixed` - Mixed-case hex encoding
- `HTMLEntityEncode` - HTML entity encoding
- `MixedEncoding` - Random mixed encoding

#### Unicode Transformations (5 functions)
- `HomoglyphSubstitution` - Replace with similar Unicode characters
- `UnicodeVariations` - Apply Unicode variation selectors
- `ZalgoText` - Add combining diacritical marks
- `SpaceVariants` - Replace spaces with Unicode variants
- `UnicodeNormalizeVariants` - Different normalization forms

#### Injection Testing (12 functions)
- `SQLCommentInjection` - Insert SQL comments
- `XSSTagVariations` - Generate XSS variations
- `NullByteInjection` - Insert null bytes
- `PathTraversal` - Generate path traversal sequences
- `CommandInjection` - Command injection patterns
- `MongoDBInjection` - MongoDB injection patterns
- `CouchDBInjection` - CouchDB injection patterns
- `DynamoDBObfuscate` - DynamoDB query obfuscation
- `NoSQLOperatorInjection` - NoSQL operator injection
- `SSTIInjection` - Server-side template injection
- `SSTIFrameworkVariation` - Framework-specific SSTI
- `SSTISyntaxObfuscate` - Obfuscate SSTI syntax

#### Obfuscation (7 functions)
- `Leetspeak` - Convert to leetspeak
- `ROT13` - Apply ROT13 cipher
- `VowelSwap` - Swap vowels
- `DoubleCharacters` - Double each character
- `ReverseString` - Reverse the string
- `WhitespacePadding` - Add random whitespace
- `JSStringConcat` - JavaScript string concatenation

#### Phishing (4 functions)
- `DomainTyposquat` - Generate typosquatting domains
- `AdvancedDomainSpoof` - Advanced IDN homograph attacks
- `EmailObfuscation` - Obfuscate email addresses
- `URLShorteningPattern` - URL shortening patterns

#### Bot Detection Evasion (5 functions)
- `RandomUserAgent` - Generate random user agent
- `TLSFingerprintVariation` - TLS fingerprint variations
- `AcceptLanguageVariation` - Accept-Language variations
- `CloudflareChallengeVariation` - Cloudflare challenge variations
- `HTTP2HeaderOrder` - HTTP/2 header order variations

#### Cloudflare Evasion (6 functions)
- `CloudflareTurnstileVariation` - Turnstile challenge variations
- `CloudflareChallengeResponse` - Challenge response patterns
- `TLSHandshakePattern` - TLS handshake patterns
- `CanvasFingerprintVariation` - Canvas fingerprint variations
- `WebGLFingerprintObfuscate` - WebGL fingerprint obfuscation
- `FontFingerprintConsistency` - Font fingerprint consistency

#### Web Security (10 functions)
- `HTTPHeaderVariation` - HTTP header variations
- `APIEndpointVariation` - API endpoint variations
- `GraphQLObfuscate` - GraphQL query obfuscation
- `SessionTokenVariation` - Session token variations
- `GraphQLVariableInjection` - GraphQL variable injection
- `GraphQLIntrospectionBypass` - Introspection bypass
- `JWTHeaderManipulation` - JWT header manipulation
- `JWTPayloadObfuscate` - JWT payload obfuscation
- `JWTAlgorithmConfusion` - JWT algorithm confusion
- `JWTSignatureBypass` - JWT signature bypass

#### Shell Commands (4 functions)
- `PowershellObfuscate` - Obfuscate PowerShell commands
- `BashObfuscate` - Obfuscate Bash commands
- `EnvVarObfuscate` - Obfuscate environment variables
- `FilePathObfuscate` - Obfuscate file paths

### Examples
- Basic usage example with common transformations
- Security testing example with WAF bypass, XSS, injection testing
- Builder pattern examples

### Documentation
- Comprehensive README with API reference
- Installation instructions for Linux, macOS, Windows
- Usage examples for all function categories
- Troubleshooting guide
- Performance notes

### CI/CD
- GitHub Actions workflow for automated testing
- Cross-platform testing (Linux, macOS, Windows)
- Multiple Go version testing (1.19, 1.20, 1.21)
- Build verification for all platforms

### Testing
- 19 unit tests covering all major functions
- Tests for edge cases (empty strings, non-deterministic functions)
- Builder pattern tests
- Example programs verified

## [Upcoming]

### Planned Features
- Go module publishing to pkg.go.dev
- Additional utility functions
- Performance benchmarks
- More comprehensive examples
