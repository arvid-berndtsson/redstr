# redstr-wasm

WebAssembly bindings for [redstr](https://github.com/arvid-berndtsson/redstr) - high-performance string transformations for browser-based security testing.

## Installation

```bash
npm install redstr-wasm
```

## Features

- 🚀 **Near-native performance** - Rust compiled to WebAssembly
- 🌐 **Browser-compatible** - Works in all modern browsers
- 🔒 **Security-focused** - 60+ transformations for WAF bypass, XSS, SQL injection, etc.
- 📦 **Small bundle size** - Optimized WASM binary
- 🎯 **TypeScript support** - Full type definitions included

## Quick Start

```javascript
import init, { leetspeak, base64_encode, randomUserAgent } from 'redstr-wasm';

// Initialize the WASM module (required once)
await init();

// Use transformations
console.log(leetspeak('password'));  // 'p@55w0rd'
console.log(base64_encode('hello'));  // 'aGVsbG8='
console.log(randomUserAgent());  // Random browser user agent
```

## React Example

```tsx
import { useEffect, useState } from 'react';
import init, { leetspeak, urlEncode } from 'redstr-wasm';

function App() {
  const [ready, setReady] = useState(false);
  const [input, setInput] = useState('');
  const [output, setOutput] = useState('');

  useEffect(() => {
    init().then(() => setReady(true));
  }, []);

  const transform = () => {
    if (ready) {
      setOutput(leetspeak(input));
    }
  };

  return (
    <div>
      <input value={input} onChange={e => setInput(e.target.value)} />
      <button onClick={transform} disabled={!ready}>Transform</button>
      <p>{output}</p>
    </div>
  );
}
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

### Obfuscation
- `leetspeak(input)` - Leetspeak transformation
- `rot13(input)` - ROT13 cipher
- `reverseString(input)` - Reverse the string
- `doubleCharacters(input)` - Double each character

### Unicode
- `homoglyphSubstitution(input)` - Replace with similar-looking characters
- `zalgoText(input)` - Zalgo text effect
- `unicodeVariations(input)` - Unicode variations

### Phishing
- `domainTyposquat(input)` - Generate typosquatted domains
- `emailObfuscation(input)` - Obfuscate email addresses

### Injection
- `xssTagVariations(input)` - XSS payload variations
- `sqlCommentInjection(input)` - SQL injection patterns
- `commandInjection(input)` - Command injection
- `sstiInjection(input)` - Server-side template injection

### Bot Detection
- `randomUserAgent()` - Generate random user agent

### Shell
- `powershellObfuscate(input)` - PowerShell obfuscation
- `bashObfuscate(input)` - Bash obfuscation

## Building from Source

```bash
# Install wasm-pack
cargo install wasm-pack

# Build the package
wasm-pack build --target web

# The output will be in pkg/
```

## Bundle Size

The optimized WASM binary is approximately 200-400KB gzipped, suitable for web applications.

## Browser Compatibility

Works in all browsers that support WebAssembly:
- Chrome 57+
- Firefox 52+
- Safari 11+
- Edge 16+

## License

MIT
