# Cross-Framework Support Strategy

## Overview

This document outlines the strategy for supporting redstr in various application frameworks including React apps, .NET backends, and other ecosystems. The approach follows the architecture principles established in our [Language Bindings](LANGUAGE_BINDINGS.md) documentation.

## Decision Framework

### When to Use Each Approach

```
                    ┌─────────────────────────────────────┐
                    │   What's your use case?             │
                    └─────────────────┬───────────────────┘
                                      │
                    ┌─────────────────▼───────────────────┐
                    │ Is network latency acceptable?       │
                    │ (5-10ms per request overhead)        │
                    └─────────────────┬───────────────────┘
                           YES │              │ NO
                               ▼              ▼
                    ┌──────────────┐ ┌──────────────────┐
                    │ API Server   │ │ Native Bindings  │
                    │ (Recommended)│ │ or WebAssembly   │
                    └──────────────┘ └──────────────────┘
```

| Factor | API Server | Native Bindings | WebAssembly |
|--------|------------|-----------------|-------------|
| **Setup Complexity** | Low | Medium-High | Medium |
| **Network Latency** | Yes (~5-10ms) | No | No |
| **Offline Support** | No | Yes | Yes (browser) |
| **Bundle Size Impact** | None | Platform libraries | ~500KB-2MB |
| **Update Process** | Server-side | Client library update | Rebuild needed |
| **Best For** | Most apps | High-throughput | Browser-only |

---

## React / JavaScript / TypeScript Support

### Option 1: API Server (Recommended)

**Best for:** Most React applications, rapid development, microservices architecture

The `redstr-server` provides a language-agnostic HTTP API. See [API Server Documentation](api_server.md).

#### Client Implementation

```typescript
// lib/redstr.ts - TypeScript client for redstr API
const REDSTR_API = process.env.REDSTR_API_URL || 'http://localhost:8080';

export interface TransformResult {
  output: string;
}

export interface BatchTransform {
  function: string;
  input: string;
}

export async function transform(func: string, input: string): Promise<string> {
  const response = await fetch(`${REDSTR_API}/transform`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ function: func, input: input })
  });
  
  if (!response.ok) {
    const error = await response.json();
    throw new Error(error.error || 'Transform failed');
  }
  
  const data: TransformResult = await response.json();
  return data.output;
}

export async function batchTransform(transforms: BatchTransform[]): Promise<string[]> {
  const response = await fetch(`${REDSTR_API}/batch`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ transforms })
  });
  
  if (!response.ok) {
    const error = await response.json();
    throw new Error(error.error || 'Batch transform failed');
  }
  
  const data = await response.json();
  return data.results.map((r: TransformResult) => r.output);
}

// Convenience functions
export const leetspeak = (input: string) => transform('leetspeak', input);
export const base64Encode = (input: string) => transform('base64_encode', input);
export const urlEncode = (input: string) => transform('url_encode', input);
export const rot13 = (input: string) => transform('rot13', input);
// ... add more as needed
```

#### React Hook Example

```typescript
// hooks/useRedstr.ts
import { useState, useCallback } from 'react';
import { transform, batchTransform, BatchTransform } from '../lib/redstr';

export function useRedstr() {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<Error | null>(null);

  const doTransform = useCallback(async (func: string, input: string) => {
    setLoading(true);
    setError(null);
    try {
      const result = await transform(func, input);
      return result;
    } catch (err) {
      setError(err as Error);
      throw err;
    } finally {
      setLoading(false);
    }
  }, []);

  const doBatchTransform = useCallback(async (transforms: BatchTransform[]) => {
    setLoading(true);
    setError(null);
    try {
      const results = await batchTransform(transforms);
      return results;
    } catch (err) {
      setError(err as Error);
      throw err;
    } finally {
      setLoading(false);
    }
  }, []);

  return { transform: doTransform, batchTransform: doBatchTransform, loading, error };
}
```

#### React Component Example

```tsx
// components/TransformDemo.tsx
import React, { useState } from 'react';
import { useRedstr } from '../hooks/useRedstr';

export function TransformDemo() {
  const [input, setInput] = useState('');
  const [output, setOutput] = useState('');
  const [selectedFunc, setSelectedFunc] = useState('leetspeak');
  const { transform, loading, error } = useRedstr();

  const handleTransform = async () => {
    try {
      const result = await transform(selectedFunc, input);
      setOutput(result);
    } catch (err) {
      console.error('Transform failed:', err);
    }
  };

  return (
    <div>
      <select value={selectedFunc} onChange={e => setSelectedFunc(e.target.value)}>
        <option value="leetspeak">Leetspeak</option>
        <option value="base64_encode">Base64 Encode</option>
        <option value="url_encode">URL Encode</option>
        <option value="rot13">ROT13</option>
      </select>
      <input 
        type="text" 
        value={input} 
        onChange={e => setInput(e.target.value)}
        placeholder="Enter text to transform"
      />
      <button onClick={handleTransform} disabled={loading}>
        {loading ? 'Transforming...' : 'Transform'}
      </button>
      {error && <div className="error">{error.message}</div>}
      {output && <div className="output">{output}</div>}
    </div>
  );
}
```

### Option 2: npm Package with napi-rs (Future)

**Best for:** Node.js/SSR React, high-throughput applications, offline requirements

The `redstr-npm` package (planned) will provide native bindings via napi-rs.

#### Repository Structure

```
redstr-npm/
├── README.md
├── package.json
├── Cargo.toml              # napi-rs configuration
├── src/
│   └── lib.rs              # Rust bindings with #[napi] macros
├── index.js                # Platform detection
├── index.d.ts              # TypeScript definitions
├── npm/                    # Platform-specific packages
│   ├── darwin-arm64/
│   ├── darwin-x64/
│   ├── linux-arm64-gnu/
│   ├── linux-x64-gnu/
│   ├── win32-x64-msvc/
│   └── ...
└── __test__/
    └── index.spec.ts
```

#### Expected API

```typescript
// Using native bindings (future)
import { leetspeak, base64Encode, urlEncode, TransformBuilder } from 'redstr';

// Direct function calls (no network)
const encoded = leetspeak('password');  // Synchronous, ~0.001ms

// Builder pattern
const result = new TransformBuilder('test input')
  .caseSwap()
  .urlEncode()
  .build();
```

### Option 3: WebAssembly (Browser-Only)

**Best for:** Client-side only applications, offline-first PWAs, security-sensitive apps

#### Repository Structure

```
redstr-wasm/
├── README.md
├── Cargo.toml
├── src/
│   └── lib.rs              # wasm-bindgen exports
├── pkg/                    # Generated WASM package
│   ├── redstr_wasm.js
│   ├── redstr_wasm.d.ts
│   ├── redstr_wasm_bg.wasm
│   └── package.json
└── crates/redstr/examples/
    └── react-app/
```

#### Expected API

```typescript
// Using WASM in React
import init, { leetspeak, TransformBuilder } from 'redstr-wasm';

// Initialize WASM module (once on app load)
await init();

// Then use synchronously
const encoded = leetspeak('password');
```

---

## .NET / C# Support

### Option 1: API Server (Recommended)

**Best for:** Most .NET applications, microservices, ASP.NET backends

#### Client Implementation

```csharp
// RedstrClient.cs
using System.Net.Http.Json;
using System.Text.Json;

namespace YourApp.Services;

public class RedstrClient : IDisposable
{
    private readonly HttpClient _client;
    private readonly JsonSerializerOptions _jsonOptions;

    public RedstrClient(string baseUrl = "http://localhost:8080")
    {
        _client = new HttpClient { BaseAddress = new Uri(baseUrl) };
        _jsonOptions = new JsonSerializerOptions { PropertyNamingPolicy = JsonNamingPolicy.CamelCase };
    }

    public async Task<string> TransformAsync(string function, string input, CancellationToken ct = default)
    {
        var request = new { Function = function, Input = input };
        var response = await _client.PostAsJsonAsync("/transform", request, _jsonOptions, ct);
        response.EnsureSuccessStatusCode();
        
        var result = await response.Content.ReadFromJsonAsync<TransformResponse>(_jsonOptions, ct);
        return result?.Output ?? throw new InvalidOperationException("Empty response");
    }

    public async Task<string[]> BatchTransformAsync(
        IEnumerable<(string Function, string Input)> transforms, 
        CancellationToken ct = default)
    {
        var request = new
        {
            Transforms = transforms.Select(t => new { Function = t.Function, Input = t.Input })
        };
        
        var response = await _client.PostAsJsonAsync("/batch", request, _jsonOptions, ct);
        response.EnsureSuccessStatusCode();
        
        var result = await response.Content.ReadFromJsonAsync<BatchResponse>(_jsonOptions, ct);
        return result?.Results.Select(r => r.Output).ToArray() ?? Array.Empty<string>();
    }

    // Convenience methods
    public Task<string> LeetspeakAsync(string input, CancellationToken ct = default) 
        => TransformAsync("leetspeak", input, ct);
    
    public Task<string> Base64EncodeAsync(string input, CancellationToken ct = default) 
        => TransformAsync("base64_encode", input, ct);
    
    public Task<string> UrlEncodeAsync(string input, CancellationToken ct = default) 
        => TransformAsync("url_encode", input, ct);

    public void Dispose() => _client.Dispose();

    private record TransformResponse(string Output);
    private record BatchResponse(TransformResponse[] Results);
}
```

#### ASP.NET Core Integration

```csharp
// Program.cs or Startup.cs
builder.Services.AddSingleton<RedstrClient>(sp => 
    new RedstrClient(builder.Configuration["Redstr:ApiUrl"] ?? "http://localhost:8080"));

// appsettings.json
{
    "Redstr": {
        "ApiUrl": "http://redstr-server:8080"
    }
}
```

#### Usage in Controllers

```csharp
// SecurityController.cs
[ApiController]
[Route("api/[controller]")]
public class SecurityController : ControllerBase
{
    private readonly RedstrClient _redstr;

    public SecurityController(RedstrClient redstr)
    {
        _redstr = redstr;
    }

    [HttpPost("encode")]
    public async Task<IActionResult> EncodePayload([FromBody] EncodeRequest request)
    {
        var encoded = await _redstr.TransformAsync(request.Function, request.Input);
        return Ok(new { encoded });
    }

    [HttpPost("batch")]
    public async Task<IActionResult> BatchEncode([FromBody] BatchEncodeRequest request)
    {
        var transforms = request.Items.Select(i => (i.Function, i.Input));
        var results = await _redstr.BatchTransformAsync(transforms);
        return Ok(new { results });
    }
}
```

### Option 2: NuGet Package with Native Bindings (Future)

**Best for:** High-throughput applications, offline requirements, embedded scenarios

The `redstr-csharp` package (planned) will provide native bindings via C FFI.

#### Repository Structure

```
redstr-csharp/
├── README.md
├── src/
│   └── Redstr/
│       ├── Redstr.csproj
│       ├── RedstrNative.cs       # P/Invoke declarations
│       ├── Redstr.cs             # High-level API
│       └── TransformBuilder.cs   # Builder pattern
├── native/
│   └── redstr-ffi/               # Rust C FFI library
│       ├── Cargo.toml
│       └── src/lib.rs
├── tests/
│   └── Redstr.Tests/
└── crates/redstr/examples/
```

#### Expected API

```csharp
// Using native bindings (future)
using Redstr;

// Direct function calls (no network)
var encoded = Transforms.Leetspeak("password");  // Synchronous, ~0.001ms

// Builder pattern
var result = new TransformBuilder("test input")
    .CaseSwap()
    .UrlEncode()
    .Build();
```

---

## Implementation Roadmap

### Phase 1: API Server Enhancements (Immediate)
1. ✅ `redstr-server` exists with basic functionality
2. Add OpenAPI/Swagger documentation
3. Add client SDKs for common languages (thin wrappers around HTTP)
4. Improve error handling and validation

### Phase 2: npm Package (Q1 2026)
1. Create `redstr-npm` repository
2. Implement napi-rs bindings
3. Set up cross-platform CI/CD
4. Publish to npm registry

### Phase 3: .NET Package (Q2 2026)
1. Create `redstr-csharp` repository
2. Implement C FFI layer in core redstr
3. Create P/Invoke wrappers
4. Publish to NuGet

### Phase 4: WebAssembly (Q2-Q3 2026)
1. Create `redstr-wasm` repository
2. Implement wasm-bindgen exports
3. Create React/Vue/Angular examples
4. Publish to npm as separate package

---

## Architecture Decisions

### Why Separate Repositories?

1. **Independent Release Cycles**: npm, NuGet, and WASM packages can be updated independently
2. **Language-Specific Tooling**: Each ecosystem has its own CI/CD, testing, and packaging requirements
3. **Cleaner Core**: The Rust library stays focused on core transformations
4. **Clear Ownership**: Different maintainers can own different bindings

### Why API Server as Default?

1. **Universal Compatibility**: Works with any language that can make HTTP requests
2. **Centralized Updates**: Update transformations in one place
3. **No Native Compilation**: Clients don't need Rust toolchain
4. **Easy Deployment**: Single service to deploy and scale
5. **Acceptable Latency**: 5-10ms overhead is fine for most use cases

### When Native Bindings Are Worth It

Consider native bindings when:
- Processing >1000 transformations per second
- Latency requirements <5ms per transformation
- Offline/air-gapped environments
- Embedded systems or edge computing
- Bundle size is not a concern

---

## Quick Start Recommendations

### For React/Next.js Apps
1. Deploy `redstr-server` (Docker or Cloud Run)
2. Use the TypeScript client above
3. Call API from your components or server-side code

### For .NET Apps
1. Deploy `redstr-server` (Docker or Kubernetes)
2. Use the C# client above
3. Register as singleton in DI container

### For High-Performance Needs
1. Wait for native bindings (`redstr-npm`, `redstr-csharp`)
2. Or contribute to binding development!

---

## Resources

- [API Server Documentation](api_server.md)
- [Language Bindings Strategy](LANGUAGE_BINDINGS.md)
- [Integration Guidelines](INTEGRATION_GUIDELINES.md)
- [redstr-server Repository](https://github.com/arvid-berndtsson/redstr-server)

---

**Last Updated**: December 2025  
**Version**: 1.0
