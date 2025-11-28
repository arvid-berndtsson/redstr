# Integration Tests for redstr Go Bindings

This directory contains comprehensive integration tests that verify the Go bindings for the redstr library work correctly end-to-end.

## Overview

These tests verify that:
- The FFI layer correctly bridges Rust and Go
- All transformation functions work as expected
- Memory management is handled properly
- The builder pattern works correctly
- Edge cases are handled gracefully

## Running the Tests

### Prerequisites

1. Build the Rust library with FFI support:
```bash
cd /home/runner/work/redstr/redstr
cargo build --release --features ffi
cp target/release/libredstr.so bindings/go/
```

### Run Tests

```bash
cd bindings/go/test
LD_LIBRARY_PATH=.. CGO_ENABLED=1 go test -v
```

On macOS:
```bash
DYLD_LIBRARY_PATH=.. CGO_ENABLED=1 go test -v
```

## Test Categories

### TestBasicTransformations
Verifies core transformation functions:
- Base64 encoding
- URL encoding
- Hex encoding
- ROT13 cipher

### TestCaseTransformations
Verifies case manipulation functions:
- CamelCase conversion
- snake_case conversion
- kebab-case conversion

### TestObfuscationFunctions
Verifies obfuscation transformations:
- Leetspeak
- String reversal
- Vowel swapping

### TestSecurityFunctions
Verifies security-related transformations:
- SQL comment injection
- Path traversal
- XSS tag variations

### TestUnicodeFunctions
Verifies Unicode transformations:
- Homoglyph substitution
- Zalgo text generation

### TestBuilderPattern
Verifies the fluent builder interface:
- Single transformations
- Chained transformations
- Complex transformation chains

### TestBotDetectionFunctions
Verifies bot detection evasion:
- Random user agent generation

### TestMultiParameterFunctions
Verifies functions with multiple parameters:
- SSTI framework variations

### TestEdgeCases
Verifies edge case handling:
- Empty strings
- Unicode input
- Long strings

### TestMemoryManagement
Verifies no memory leaks:
- Repeated function calls
- Proper cleanup of C strings

### TestRealWorldScenarios
Demonstrates practical usage:
- WAF bypass techniques
- Phishing domain generation
- Payload encoding chains

## Test Output

When all tests pass, you should see:
```
PASS
ok      github.com/arvid-berndtsson/redstr-go/test      0.004s
```

## Notes

- Some functions are non-deterministic (use RNG) - tests verify properties rather than exact output
- All tests verify the FFI bridge works correctly
- Memory management is automatically handled by the Go wrapper
- Tests run quickly (< 5ms total) due to direct FFI to native Rust code

## Troubleshooting

### Library Not Found

If you see "cannot open shared object file", ensure:
1. The Rust library is built: `cargo build --release --features ffi`
2. The library is in `bindings/go/`: `cp target/release/libredstr.so bindings/go/`
3. `LD_LIBRARY_PATH` is set correctly

### CGO Disabled

If you see "C source files not allowed", ensure:
```bash
export CGO_ENABLED=1
```

## Contributing

When adding new functions to the Go bindings:
1. Add a test case in the appropriate test function
2. Verify it passes with `go test -v`
3. Update this README if adding a new test category
