# Language Bindings for redstr

This document explains how redstr provides support for multiple programming languages through native bindings maintained in a monorepo structure.

## Overview

The `redstr` repository uses a **monorepo architecture** where the core Rust library and all language bindings live together. This follows the pattern used by successful projects like Polars, libsignal, swc, and deltachat.

## Architecture

```
redstr/
├── crates/
│   └── redstr/              # Core Rust library
├── ffi/                     # C FFI layer (shared by bindings)
├── bindings/
│   ├── node/                # Node.js/TypeScript (napi-rs)
│   ├── python/              # Python (PyO3)
│   ├── wasm/                # WebAssembly (wasm-bindgen)
│   └── dotnet/              # .NET/C# (P/Invoke)
└── .github/workflows/       # CI/CD for all languages
```

### Why Monorepo?

| Benefit | Description |
|---------|-------------|
| **Atomic updates** | Change core + bindings in one PR |
| **Version sync** | All bindings always match core version |
| **Shared CI** | One pipeline tests everything |
| **Easier maintenance** | No cross-repo version coordination |
| **Single source of truth** | All code in one place |

### Used By

This pattern is used by:
- **libsignal** (Signal) - `rust/`, `node/`, `java/`, `swift/` in one repo
- **Polars** - [`crates/polars/`](../crates/)polars/), `py-polars/`, `nodejs-polars/`
- **swc** - Rust core + Node.js bindings in monorepo
- **deltachat** - Core + FFI + all bindings together

## Supported Languages

| Language | Directory | Framework | Package Manager | Status |
|----------|-----------|-----------|-----------------|--------|
| **Rust** | [`crates/redstr/`](../crates/)redstr/) | Native | crates.io | ✅ Active |
| **Node.js/TypeScript** | [`bindings/node/`](../bindings/)node/) | napi-rs | npm | 🚧 In Progress |
| **Python** | [`bindings/python/`](../bindings/)python/) | PyO3 + maturin | PyPI | 🚧 In Progress |
| **WebAssembly** | [`bindings/wasm/`](../bindings/)wasm/) | wasm-bindgen | npm | 🚧 In Progress |
| **C#/.NET** | [`bindings/dotnet/`](../bindings/)dotnet/) | C FFI + P/Invoke | NuGet | 🚧 In Progress |
| **C/C++** | `ffi/` | cbindgen | Header file | 🚧 In Progress |
| **Go** | External | CGO | Go modules | ✅ `redstr-go` |

## Repository Structure

### Core Library ([`crates/redstr/`](../crates/)redstr/))

The core Rust library containing all transformation functions.

```
crates/redstr/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── builder.rs
    ├── rng.rs
    └── transformations/
        ├── mod.rs
        ├── encoding.rs
        ├── case.rs
        ├── obfuscation.rs
        └── ...
```

### C FFI Layer (`ffi/`)

A C-compatible FFI layer that enables bindings for languages that can call C functions (Python, .NET, Ruby, etc.).

```
ffi/
├── Cargo.toml
├── cbindgen.toml
├── src/
│   └── lib.rs           # C-compatible exports
└── include/
    └── redstr.h         # Generated C header
```

### Node.js Bindings ([`bindings/node/`](../bindings/)node/))

Native Node.js bindings using napi-rs for zero-overhead calls.

```
bindings/node/
├── Cargo.toml
├── package.json
├── src/
│   └── lib.rs           # napi-rs exports
├── index.js             # Platform loader
├── index.d.ts           # TypeScript definitions
└── npm/                 # Platform-specific packages
    ├── darwin-arm64/
    ├── darwin-x64/
    ├── linux-x64-gnu/
    └── win32-x64-msvc/
```

### Python Bindings ([`bindings/python/`](../bindings/)python/))

Native Python bindings using PyO3 and maturin.

```
bindings/python/
├── Cargo.toml
├── pyproject.toml
├── src/
│   └── lib.rs           # PyO3 exports
└── python/
    └── redstr/
        ├── __init__.py
        └── py.typed     # PEP 561 marker
```

### WebAssembly ([`bindings/wasm/`](../bindings/)wasm/))

Browser-compatible WebAssembly bindings.

```
bindings/wasm/
├── Cargo.toml
├── src/
│   └── lib.rs           # wasm-bindgen exports
└── pkg/                 # Generated package
    ├── redstr_wasm.js
    ├── redstr_wasm.d.ts
    └── redstr_wasm_bg.wasm
```

### .NET Bindings ([`bindings/dotnet/`](../bindings/)dotnet/))

.NET bindings using P/Invoke to call the C FFI layer.

```
bindings/dotnet/
├── src/
│   └── Redstr/
│       ├── Redstr.csproj
│       ├── Native.cs    # P/Invoke declarations
│       ├── Transforms.cs
│       └── TransformBuilder.cs
└── tests/
    └── Redstr.Tests/
```

## Development Workflow

### Building All Bindings

```bash
# Build core library
cargo build -p redstr

# Build FFI layer
cargo build -p redstr-ffi --features ffi

# Build Node.js bindings
cd bindings/node && npm run build

# Build Python bindings
cd bindings/python && maturin develop

# Build WASM bindings
cd bindings/wasm && wasm-pack build

# Build .NET bindings
cd bindings/dotnet && dotnet build
```

### Testing All Bindings

```bash
# Test core library
cargo test -p redstr

# Test Node.js bindings
cd bindings/node && npm test

# Test Python bindings
cd bindings/python && pytest

# Test WASM bindings
cd bindings/wasm && wasm-pack test --headless --firefox

# Test .NET bindings
cd bindings/dotnet && dotnet test
```

### Adding a New Function

When adding a new transformation function:

1. **Add to core** ([`crates/redstr/src/`](../crates/)redstr/src/))
2. **Export via FFI** ([`ffi/src/lib.rs`](../ffi/)src/lib.rs))
3. **Add to Node.js** ([`bindings/node/src/lib.rs`](../bindings/)node/src/lib.rs))
4. **Add to Python** ([`bindings/python/src/lib.rs`](../bindings/)python/src/lib.rs))
5. **Add to WASM** ([`bindings/wasm/src/lib.rs`](../bindings/)wasm/src/lib.rs))
6. **Add to .NET** ([`bindings/dotnet/src/Redstr/Transforms.cs`](../bindings/)dotnet/src/Redstr/Transforms.cs))
7. **Regenerate C header** (`cargo build -p redstr-ffi`)

### Code Generation (Future)

As the library grows, we'll add automated code generation:

```bash
# Generate all bindings from core library
cargo run -p redstr-bindgen

# This will:
# 1. Parse core library function signatures
# 2. Generate FFI exports
# 3. Generate Node.js bindings
# 4. Generate Python bindings
# 5. Generate WASM bindings
# 6. Generate .NET bindings
```

## Publishing

### Release Process

All packages are published from the same commit:

```bash
# 1. Update version in workspace Cargo.toml
# 2. Create release tag
git tag v<version>

# 3. CI publishes to all registries:
#    - crates.io (Rust)
#    - npm (Node.js, WASM)
#    - PyPI (Python)
#    - NuGet (.NET)
```

### Version Synchronization

All packages share the same version number:
- `redstr` on crates.io: `<version>`
- `redstr` on npm: `<version>`
- `redstr` on PyPI: `<version>`
- `Redstr` on NuGet: `<version>`

## Usage Examples

### Rust

```rust
use redstr::{leetspeak, TransformBuilder};

let encoded = leetspeak("password");

let payload = TransformBuilder::new("<script>")
    .case_swap()
    .url_encode()
    .build();
```

### Node.js / TypeScript

```typescript
import { leetspeak, TransformBuilder } from 'redstr';

const encoded = leetspeak('password');

const payload = new TransformBuilder('<script>')
    .caseSwap()
    .urlEncode()
    .build();
```

### Python

```python
from redstr import leetspeak, TransformBuilder

encoded = leetspeak('password')

payload = (TransformBuilder('<script>')
    .case_swap()
    .url_encode()
    .build())
```

### Browser (WASM)

```typescript
import init, { leetspeak } from 'redstr-wasm';

await init();
const encoded = leetspeak('password');
```

### C# / .NET

```csharp
using Redstr;

var encoded = Transforms.Leetspeak("password");

var payload = new TransformBuilder("<script>")
    .CaseSwap()
    .UrlEncode()
    .Build();
```

### C / C++

```c
#include "redstr.h"

char* encoded = redstr_leetspeak("password");
// Use encoded...
redstr_free_string(encoded);
```

## Performance

All bindings have minimal overhead:

| Binding | Overhead | Notes |
|---------|----------|-------|
| Rust | 0 | Native |
| Node.js (napi-rs) | ~0.01ms | Direct FFI, no serialization |
| Python (PyO3) | ~0.01ms | Direct FFI, no serialization |
| WASM | ~0.05ms | WASM call overhead |
| .NET (P/Invoke) | ~0.01ms | Direct FFI |
| C/C++ | 0 | Direct FFI |

## Contributing

### Adding a New Language Binding

1. Create directory: `bindings/<language>/`
2. Add to workspace in root [`Cargo.toml`](Cargo.toml) (if Rust-based)
3. Implement bindings following existing patterns
4. Add CI workflow in [`.github/workflows/`](../.github/)workflows/)
5. Add documentation to this file
6. Add usage example to README

### Guidelines

- All bindings must support all public functions
- TypeScript/Python type definitions required
- Tests required for all functions
- Documentation required with examples
- Follow language-specific conventions

## Resources

- **Repository**: https://github.com/arvid-berndtsson/redstr
- **Rust Docs**: https://docs.rs/redstr
- **npm Package**: https://www.npmjs.com/package/redstr
- **PyPI Package**: https://pypi.org/project/redstr/
- **NuGet Package**: https://www.nuget.org/packages/Redstr

## FAQ

### Why not separate repositories?

Separate repos cause version sync issues and maintenance overhead. With a monorepo:
- One PR updates everything
- CI catches binding breakages immediately
- No "binding lags behind core" problems

### Can I use just the Rust library?

Yes! The core library at [`crates/redstr/`](../crates/)redstr/) works standalone:
```toml
[dependencies]
redstr = "0.3"
```

### What about the API server?

The `redstr-server` HTTP API is in a separate repository for deployment flexibility. Use it for:
- Quick prototyping
- Microservices architecture
- Languages without bindings

But native bindings are preferred for performance.

---

**Last Updated**: December 2025  
**Version**: 2.0 (Monorepo Architecture)
