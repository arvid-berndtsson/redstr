# redstr

Native Node.js bindings for [redstr](https://github.com/arvid-berndtsson/redstr) - high-performance string transformations for security testing.

## Installation

```bash
npm install redstr
```

## Features

- 🚀 **Native performance** - Zero-overhead Rust bindings via napi-rs
- 🔒 **Security-focused** - 60+ transformations for WAF bypass, XSS, SQL injection, etc.
- 📦 **Zero dependencies** - No JavaScript dependencies
- 🎯 **TypeScript support** - Full type definitions included
- 🔗 **Builder pattern** - Chain multiple transformations fluently

## Quick Start

```javascript
const { leetspeak, base64Encode, TransformBuilder } = require('redstr');

// Direct function calls
console.log(leetspeak('password'));  // 'p@55w0rd'
console.log(base64Encode('hello'));  // 'aGVsbG8='

// Builder pattern for chaining
const payload = new TransformBuilder('<script>alert(1)</script>')
    .caseSwap()
    .urlEncode()
    .build();
console.log(payload);
```

## TypeScript

```typescript
import { leetspeak, TransformBuilder, randomUserAgent } from 'redstr';

const encoded: string = leetspeak('password');

const builder = new TransformBuilder('test')
    .base64()
    .urlEncode();
    
const result: string = builder.build();
```

## Available Functions

### Case Transformations
- `randomizeCapitalization(input)` - Random case for each character
- `caseSwap(input)` - Swap uppercase/lowercase
- `alternateCase(input)` - Alternate case pattern
- `toCamelCase(input)` - Convert to camelCase
- `toSnakeCase(input)` - Convert to snake_case
- `toKebabCase(input)` - Convert to kebab-case

### Encoding
- `base64Encode(input)` - Base64 encoding
- `urlEncode(input)` - URL encoding
- `hexEncode(input)` - Hex encoding
- `htmlEntityEncode(input)` - HTML entity encoding
- `mixedEncoding(input)` - Mixed encoding techniques

### Obfuscation
- `leetspeak(input)` - Leetspeak transformation
- `rot13(input)` - ROT13 cipher
- `reverseString(input)` - Reverse the string
- `doubleCharacters(input)` - Double each character
- `jsStringConcat(input)` - JS string concatenation

### Unicode
- `homoglyphSubstitution(input)` - Replace with similar-looking characters
- `zalgoText(input)` - Zalgo text effect
- `unicodeVariations(input)` - Unicode variations
- `spaceVariants(input)` - Alternative space characters

### Phishing
- `domainTyposquat(input)` - Generate typosquatted domains
- `emailObfuscation(input)` - Obfuscate email addresses
- `advancedDomainSpoof(input)` - Domain spoofing techniques

### Injection
- `xssTagVariations(input)` - XSS payload variations
- `sqlCommentInjection(input)` - SQL injection patterns
- `commandInjection(input)` - Command injection
- `pathTraversal(input)` - Path traversal patterns
- `sstiInjection(input)` - Server-side template injection

### Bot Detection
- `randomUserAgent()` - Generate random user agent
- `tlsFingerprintVariation(input)` - TLS fingerprint variations

### Shell
- `powershellObfuscate(input)` - PowerShell obfuscation
- `bashObfuscate(input)` - Bash obfuscation

## TransformBuilder

Chain multiple transformations:

```javascript
const { TransformBuilder } = require('redstr');

const result = new TransformBuilder('payload')
    .leetspeak()
    .base64()
    .urlEncode()
    .build();
```

Available builder methods:
- `.leetspeak()` - Apply leetspeak
- `.base64()` - Apply Base64 encoding
- `.urlEncode()` - Apply URL encoding
- `.caseSwap()` - Apply case swap
- `.rot13()` - Apply ROT13
- `.hexEncode()` - Apply hex encoding
- `.homoglyphs()` - Apply homoglyph substitution
- `.reverse()` - Reverse the string
- `.build()` - Get the final result

## Performance

redstr uses native Rust code via napi-rs, providing near-native performance:

| Operation | Time |
|-----------|------|
| `leetspeak()` | ~0.01ms |
| `base64Encode()` | ~0.005ms |
| `TransformBuilder` (5 ops) | ~0.05ms |

## License

MIT
