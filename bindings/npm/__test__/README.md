# NPM Bindings Tests

This directory contains tests for the `@redstr/core` npm package bindings.

## Test Files

### `index.spec.js`
Tests the functionality of all transformation functions exported by the npm package:
- Case transformations (7 functions)
- Encoding transformations (6 functions)
- Unicode transformations (5 functions)
- Injection testing (12 functions)
- Obfuscation (7 functions)
- Phishing (4 functions)
- Bot detection (5 functions)
- Cloudflare (6 functions)
- Web security (10 functions)
- Shell transformations (4 functions)

**Total: 28 functional tests**

### `bindings.spec.js`
Tests that verify the native bindings can be generated and loaded:
- Verifies the `.node` native module exists for the current platform
- Verifies the module can be loaded without errors
- Verifies all 60+ transformation functions are exported
- Verifies functions can be called and return expected types

**Total: 4 bindings verification tests**

## Running Tests

```bash
# Run all tests
npm test

# Run specific test file
npm test __test__/index.spec.js
npm test __test__/bindings.spec.js
```

## CI/CD Integration

These tests are automatically run in GitHub Actions on:
- Pull requests that modify `npm/**`, `src/**`, or `Cargo.toml`
- Pushes to the `main` branch

The CI workflow (`.github/workflows/npm-bindings-test.yml`) runs tests on:
- Multiple operating systems: Ubuntu, macOS, Windows
- Multiple Node.js versions: 18, 20

## Test Coverage

The tests ensure:
1. ✅ Native bindings compile successfully
2. ✅ Native module can be loaded on all supported platforms
3. ✅ All functions are properly exported to JavaScript
4. ✅ Functions work correctly and return expected types
5. ✅ Examples run without errors
