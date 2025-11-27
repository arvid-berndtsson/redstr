# Getting Started with redstr Go Bindings

This guide will help you get up and running with the redstr Go bindings quickly.

## Prerequisites

- **Go 1.19 or higher**
- **Rust toolchain** (for building the library)
- **C compiler** (for CGO)
- **cbindgen** (for generating C headers)

## Installation

### 1. Install Dependencies

#### Ubuntu/Debian
```bash
# Install build essentials
sudo apt-get update
sudo apt-get install build-essential

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install cbindgen
cargo install cbindgen
```

#### macOS
```bash
# Install Xcode Command Line Tools
xcode-select --install

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install cbindgen
cargo install cbindgen
```

#### Windows
```powershell
# Install Rust from https://rustup.rs/
# Install Visual Studio Build Tools or MinGW-w64

# Install cbindgen
cargo install cbindgen
```

### 2. Clone and Build

```bash
# Clone the repository
git clone https://github.com/arvid-berndtsson/redstr
cd redstr

# Build the library
cd bindings/go
./build.sh release
```

This will:
- Build the Rust library with FFI support
- Generate the C header file
- Copy the library to the Go bindings directory

### 3. Verify Installation

```bash
cd bindings/go
LD_LIBRARY_PATH=. go test -v
```

All tests should pass.

## Your First Program

Create a file `hello.go`:

```go
package main

import (
    "fmt"
    redstr "github.com/arvid-berndtsson/redstr-go"
)

func main() {
    // Simple transformations
    fmt.Println("Leetspeak:", redstr.Leetspeak("password"))
    fmt.Println("Base64:", redstr.Base64Encode("hello"))
    fmt.Println("ROT13:", redstr.ROT13("Hello World"))
    
    // Builder pattern
    result := redstr.NewTransformBuilder("admin@example.com").
        Homoglyphs().
        URLEncode().
        Build()
    fmt.Println("Chained:", result)
}
```

Run it:
```bash
LD_LIBRARY_PATH=/path/to/redstr/bindings/go go run hello.go
```

## Common Use Cases

### WAF Bypass Testing

```go
package main

import (
    "fmt"
    redstr "github.com/arvid-berndtsson/redstr-go"
)

func main() {
    sqlQuery := "SELECT * FROM users WHERE id = 1"
    
    // Try different bypass techniques
    fmt.Println("SQL Comment Injection:")
    fmt.Println(redstr.SQLCommentInjection(sqlQuery))
    
    fmt.Println("\nCase Swap + URL Encode:")
    result := redstr.NewTransformBuilder(sqlQuery).
        CaseSwap().
        URLEncode().
        Build()
    fmt.Println(result)
    
    fmt.Println("\nMixed Encoding:")
    fmt.Println(redstr.MixedEncoding(sqlQuery))
}
```

### XSS Payload Generation

```go
package main

import (
    "fmt"
    redstr "github.com/arvid-berndtsson/redstr-go"
)

func main() {
    xss := "<script>alert(1)</script>"
    
    // Generate variations
    fmt.Println("XSS Tag Variations:")
    fmt.Println(redstr.XSSTagVariations(xss))
    
    fmt.Println("\nRandom Capitalization:")
    fmt.Println(redstr.RandomizeCapitalization(xss))
    
    fmt.Println("\nHTML Entity + URL Encode:")
    result := redstr.NewTransformBuilder(xss).
        HTMLEntity().
        URLEncode().
        Build()
    fmt.Println(result)
}
```

### Phishing Domain Testing

```go
package main

import (
    "fmt"
    redstr "github.com/arvid-berndtsson/redstr-go"
)

func main() {
    domain := "paypal.com"
    
    // Generate suspicious domains
    fmt.Println("Original:", domain)
    fmt.Println("Typosquat:", redstr.DomainTyposquat(domain))
    fmt.Println("Homoglyph:", redstr.HomoglyphSubstitution(domain))
    fmt.Println("Advanced Spoof:", redstr.AdvancedDomainSpoof(domain))
}
```

### Bot Detection Evasion

```go
package main

import (
    "fmt"
    redstr "github.com/arvid-berndtsson/redstr-go"
)

func main() {
    // Generate random user agents
    fmt.Println("User Agent 1:", redstr.RandomUserAgent())
    fmt.Println("User Agent 2:", redstr.RandomUserAgent())
    fmt.Println("User Agent 3:", redstr.RandomUserAgent())
    
    // TLS fingerprint variation
    fmt.Println("\nTLS Fingerprint:", 
        redstr.TLSFingerprintVariation("ja3_hash"))
    
    // Cloudflare challenge
    fmt.Println("Cloudflare Turnstile:", 
        redstr.CloudflareTurnstileVariation("challenge_token"))
}
```

## Using the Builder Pattern

The builder pattern allows you to chain multiple transformations:

```go
result := redstr.NewTransformBuilder("input").
    Leetspeak().
    CaseSwap().
    Base64().
    URLEncode().
    Build()
```

Available builder methods:
- `RandomizeCapitalization()`, `CaseSwap()`, `AlternateCase()`
- `Base64()`, `URLEncode()`, `HexEncode()`, `HTMLEntity()`
- `Homoglyphs()`, `Zalgo()`, `UnicodeVariations()`
- `Leetspeak()`, `ROT13()`, `Reverse()`, `DoubleChars()`
- `SQLComment()`, `XSSTag()`, `PathTraversal()`
- `GraphQL()`, `JWTHeader()`, `Powershell()`, `Bash()`

## Running Examples

The repository includes working examples:

```bash
# Basic examples
cd bindings/go/examples/basic
LD_LIBRARY_PATH=../.. go run main.go

# Security testing examples
cd bindings/go/examples/security
LD_LIBRARY_PATH=../.. go run main.go
```

## Setting Up Your Environment

### Permanent Library Path (Linux/macOS)

Add to your `~/.bashrc` or `~/.zshrc`:

```bash
export LD_LIBRARY_PATH=/path/to/redstr/bindings/go:$LD_LIBRARY_PATH
```

Or on macOS:
```bash
export DYLD_LIBRARY_PATH=/path/to/redstr/bindings/go:$DYLD_LIBRARY_PATH
```

### Using in Your Project

1. **Copy the library** to your project directory or install it system-wide
2. **Update go.mod** to reference the module
3. **Set CGO flags** if needed:

```go
package main

/*
#cgo LDFLAGS: -L/path/to/library -lredstr
*/
import "C"
```

## Troubleshooting

### Library Not Found

**Error:** `cannot open shared object file: No such file or directory`

**Solution:**
- Ensure library is built: `./build.sh release`
- Set library path: `export LD_LIBRARY_PATH=/path/to/redstr/bindings/go`
- Or copy library to system path: `sudo cp libredstr.so /usr/local/lib/`

### CGO Disabled

**Error:** `C source files not allowed when not using cgo`

**Solution:**
```bash
export CGO_ENABLED=1
go build
```

### Missing C Compiler

**Error:** `gcc: command not found`

**Solution:**
- Ubuntu/Debian: `sudo apt-get install build-essential`
- macOS: `xcode-select --install`
- Windows: Install MinGW-w64 or Visual Studio Build Tools

## Next Steps

- Read the [full API documentation](README.md)
- Explore the [examples](examples/)
- Check out [security use cases](examples/security/main.go)
- Join the community and contribute

## Resources

- **Main Repository**: https://github.com/arvid-berndtsson/redstr
- **Rust Documentation**: https://docs.rs/redstr
- **Issue Tracker**: https://github.com/arvid-berndtsson/redstr/issues
- **Contributing Guide**: ../../CONTRIBUTING.md

## License

MIT License - see [LICENSE](../../LICENSE) for details.

## Responsible Use

This library is intended for **authorized security testing only**. Always ensure you have proper authorization before testing systems you do not own.
