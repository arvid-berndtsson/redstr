# @arvid-berndtsson/redstr

[![npm version](https://badge.fury.io/js/%40redstr%2Fcore.svg)](https://www.npmjs.com/package/@arvid-berndtsson/redstr)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/arvid-berndtsson/redstr/blob/main/LICENSE)

> **Note:** This package is located at `bindings/npm/` in the [redstr repository](https://github.com/arvid-berndtsson/redstr).

**Red team string transformation library for offensive security operations - Node.js bindings**

High-performance Node.js bindings for [redstr](https://github.com/arvid-berndtsson/redstr), a comprehensive Rust library providing 60+ string obfuscation and transformation functions for security testing, penetration testing, and red/blue/purple team operations.

## ⚡ Features

- **Native Performance**: Rust-powered transformations via napi-rs
- **60+ Transformations**: Encoding, obfuscation, injection testing, web security
- **Zero JavaScript Dependencies**: Lean and secure
- **Cross-Platform**: Pre-built binaries for Linux, macOS, Windows (x64, ARM64)
- **TypeScript Support**: Full type definitions included

## 📦 Installation

```bash
npm install @arvid-berndtsson/redstr
# or
yarn add @arvid-berndtsson/redstr
# or
pnpm add @arvid-berndtsson/redstr
```

## 🚀 Quick Start

```javascript
const redstr = require('@arvid-berndtsson/redstr');

// Random capitalization for filter bypass
const result = redstr.randomizeCapitalization('Hello World');
console.log(result); // "HeLlO wOrLd" (varies each run)

// Leetspeak for content filter evasion
const obfuscated = redstr.leetspeak('password');
console.log(obfuscated); // "p@55w0rd" or "p4$$w0rd"

// Homoglyph substitution for phishing tests
const spoofed = redstr.homoglyphSubstitution('admin@example.com');
console.log(spoofed); // "аdmіn@еxаmple.com" (Cyrillic characters)

// Random user agent for bot evasion
const ua = redstr.randomUserAgent();
console.log(ua); // Random user agent string
```

## 📚 TypeScript Usage

```typescript
import {
  randomizeCapitalization,
  leetspeak,
  homoglyphSubstitution,
  base64Encode,
  sqlCommentInjection,
  xssTagVariations
} from '@arvid-berndtsson/redstr';

// Type-safe with full IntelliSense support
const obfuscated: string = leetspeak('test');
const encoded: string = base64Encode('payload');
```

## 🎯 Use Cases

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

## 📖 API Reference

All functions accept a string and return a transformed string. Functions are organized by category:

### Case Transformations
```javascript
randomizeCapitalization(input)  // Random upper/lowercase
alternateCase(input)            // Alternating case
caseSwap(input)                 // Swap all case
inverseCase(input)              // Inverse case
toCamelCase(input)              // camelCase
toSnakeCase(input)              // snake_case
toKebabCase(input)              // kebab-case
```

### Encoding
```javascript
base64Encode(input)            // Base64 encoding
urlEncode(input)               // URL encoding
hexEncode(input)               // Hex encoding
hexEncodeMixed(input)          // Mixed case hex
htmlEntityEncode(input)        // HTML entities
mixedEncoding(input)           // Mixed URL+HTML
```

### Unicode
```javascript
homoglyphSubstitution(input)   // Homoglyph replacement
unicodeVariations(input)       // Unicode variations
zalgoText(input)               // Zalgo text
spaceVariants(input)           // Unicode spaces
unicodeNormalizeVariants(input) // Normalization
```

### Injection Testing
```javascript
sqlCommentInjection(input)     // SQL comments
xssTagVariations(input)        // XSS tags
nullByteInjection(input)       // Null bytes
pathTraversal(input)           // Path traversal
commandInjection(input)        // Command injection
mongodbInjection(input)        // MongoDB patterns
couchdbInjection(input)        // CouchDB patterns
dynamodbObfuscate(input)       // DynamoDB obfuscation
nosqlOperatorInjection(input)  // NoSQL operators
sstiInjection(input)           // SSTI patterns
sstiFrameworkVariation(input)  // SSTI frameworks
sstiSyntaxObfuscate(input)     // SSTI syntax
```

### Obfuscation
```javascript
leetspeak(input)               // L33t speak
rot13(input)                   // ROT13 cipher
vowelSwap(input)               // Swap vowels
doubleCharacters(input)        // Double chars
reverseString(input)           // Reverse
whitespacePadding(input)       // Add whitespace
jsStringConcat(input)          // JS concatenation
```

### Phishing
```javascript
domainTyposquat(input)         // Domain typos
advancedDomainSpoof(input)     // Advanced spoofing
emailObfuscation(input)        // Email obfuscation
urlShorteningPattern(input)    // URL patterns
```

### Bot Detection
```javascript
randomUserAgent()              // Random UA
tlsFingerprintVariation(input) // TLS fingerprint
http2HeaderOrder(input)        // HTTP/2 headers
acceptLanguageVariation()      // Accept-Language
cloudflareChallengeVariation(input) // CF challenge
```

### Cloudflare Evasion
```javascript
cloudflareTurnstileVariation(input) // Turnstile
cloudflareChallengeResponse(input)  // Challenge response
tlsHandshakePattern(input)          // TLS handshake
canvasFingerprintVariation(input)   // Canvas fingerprint
webglFingerprintObfuscate(input)    // WebGL fingerprint
fontFingerprintConsistency(input)   // Font fingerprint
```

### Web Security
```javascript
httpHeaderVariation(input)          // HTTP headers
apiEndpointVariation(input)         // API endpoints
graphqlObfuscate(input)             // GraphQL queries
sessionTokenVariation(input)        // Session tokens
graphqlVariableInjection(input)     // GraphQL variables
graphqlIntrospectionBypass(input)   // Introspection bypass
jwtHeaderManipulation(input)        // JWT headers
jwtPayloadObfuscate(input)          // JWT payloads
jwtAlgorithmConfusion(input)        // JWT algorithm
jwtSignatureBypass(input)           // JWT signature
```

### Shell
```javascript
powershellObfuscate(input)     // PowerShell obfuscation
bashObfuscate(input)           // Bash obfuscation
envVarObfuscate(input)         // Environment variables
filePathObfuscate(input)       // File paths
```

## 🔧 Examples

See the [examples directory](./examples) for more usage examples:

- [Node.js Basic Usage](./examples/basic.js)
- [TypeScript Usage](./examples/typescript-example.ts)
- [WAF Bypass](./examples/waf-bypass.js)
- [Phishing Testing](./examples/phishing.js)

## 🏗️ Architecture

This package uses [napi-rs](https://napi.rs/) to provide native Node.js bindings to the Rust redstr library. Pre-built binaries are available for:

- **Linux**: x64 (glibc, musl), ARM64, ARMv7
- **macOS**: x64, ARM64 (Apple Silicon), Universal
- **Windows**: x64, ia32, ARM64

## 🔒 Security & Responsible Use

This library is intended for **authorized security testing, research, and defensive security purposes only**. Users are responsible for ensuring they have proper authorization before using these techniques on systems they do not own or have explicit permission to test.

## 📄 License

MIT License - see [LICENSE](https://github.com/arvid-berndtsson/redstr/blob/main/LICENSE)

## 🔗 Links

- [Main Repository](https://github.com/arvid-berndtsson/redstr)
- [Documentation](https://docs.rs/redstr)
- [Rust Crate](https://crates.io/crates/redstr)
- [npm Package](https://www.npmjs.com/package/@arvid-berndtsson/redstr)

## 🤝 Contributing

Contributions are welcome! See [CONTRIBUTING.md](https://github.com/arvid-berndtsson/redstr/blob/main/CONTRIBUTING.md) in the main repository.
