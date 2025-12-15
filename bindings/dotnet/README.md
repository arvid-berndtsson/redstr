# Redstr

Native .NET bindings for [redstr](https://github.com/arvid-berndtsson/redstr) - high-performance string transformations for security testing.

## Installation

```bash
dotnet add package Redstr
```

## Features

- 🚀 **Native performance** - Zero-overhead Rust bindings via P/Invoke
- 🔒 **Security-focused** - 60+ transformations for WAF bypass, XSS, SQL injection, etc.
- 📦 **Zero dependencies** - No additional NuGet dependencies
- 🎯 **Full documentation** - XML docs for IntelliSense
- 🔗 **Builder pattern** - Chain multiple transformations fluently

## Quick Start

```csharp
using Redstr;

// Direct function calls
Console.WriteLine(Transforms.Leetspeak("password"));  // "p@55w0rd"
Console.WriteLine(Transforms.Base64Encode("hello"));  // "aGVsbG8="

// Builder pattern for chaining
var payload = new TransformBuilder("<script>alert(1)</script>")
    .CaseSwap()
    .UrlEncode()
    .Build();
Console.WriteLine(payload);
```

## Available Functions

### Case Transformations
- `Transforms.RandomizeCapitalization(input)` - Random case for each character
- `Transforms.CaseSwap(input)` - Swap uppercase/lowercase
- `Transforms.AlternateCase(input)` - Alternate case pattern
- `Transforms.InverseCase(input)` - Inverse case

### Encoding
- `Transforms.Base64Encode(input)` - Base64 encoding
- `Transforms.UrlEncode(input)` - URL encoding
- `Transforms.HexEncode(input)` - Hex encoding
- `Transforms.HtmlEntityEncode(input)` - HTML entity encoding

### Obfuscation
- `Transforms.Leetspeak(input)` - Leetspeak transformation
- `Transforms.Rot13(input)` - ROT13 cipher
- `Transforms.ReverseString(input)` - Reverse the string
- `Transforms.DoubleCharacters(input)` - Double each character

### Unicode
- `Transforms.HomoglyphSubstitution(input)` - Replace with similar-looking characters
- `Transforms.ZalgoText(input)` - Zalgo text effect

### Phishing
- `Transforms.DomainTyposquat(input)` - Generate typosquatted domains
- `Transforms.EmailObfuscation(input)` - Obfuscate email addresses

### Injection
- `Transforms.XssTagVariations(input)` - XSS payload variations
- `Transforms.SqlCommentInjection(input)` - SQL injection patterns
- `Transforms.CommandInjection(input)` - Command injection
- `Transforms.PathTraversal(input)` - Path traversal patterns

### Bot Detection
- `Transforms.RandomUserAgent()` - Generate random user agent

### Shell
- `Transforms.PowershellObfuscate(input)` - PowerShell obfuscation
- `Transforms.BashObfuscate(input)` - Bash obfuscation

## TransformBuilder

Chain multiple transformations fluently:

```csharp
using Redstr;

var result = new TransformBuilder("payload")
    .Leetspeak()
    .Base64()
    .UrlEncode()
    .Build();

// Implicit string conversion
string encoded = new TransformBuilder("test").Leetspeak();
```

Available builder methods:
- `.Leetspeak()` - Apply leetspeak
- `.Base64()` - Apply Base64 encoding
- `.UrlEncode()` - Apply URL encoding
- `.CaseSwap()` - Apply case swap
- `.Rot13()` - Apply ROT13
- `.HexEncode()` - Apply hex encoding
- `.Homoglyphs()` - Apply homoglyph substitution
- `.Reverse()` - Reverse the string
- `.HtmlEncode()` - Apply HTML entity encoding
- `.XssVariations()` - Apply XSS variations
- `.SqlComments()` - Apply SQL comment injection
- `.Build()` - Get the final result

## Supported Platforms

- Windows x64
- Linux x64
- macOS x64
- macOS ARM64 (Apple Silicon)
- Linux ARM64

## License

MIT
