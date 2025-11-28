# Bindings Generator Design Document

## Overview

This document describes the design and requirements for an automated bindings generator for the redstr library. The generator will parse Rust function signatures and automatically generate FFI wrappers and language-specific bindings for multiple target languages.

## Problem Statement

With plans to scale to 500+ transformation functions across 5 language targets (Go, npm/JavaScript, Python, Java, C#/.NET), manual bindings maintenance becomes impractical:

- **Manual effort**: ~2,500 lines per language × 5 languages = 12,500+ lines to maintain
- **Consistency risk**: Each function requires manual updates across 5+ files
- **Error-prone**: Easy to miss functions or introduce signature mismatches
- **Maintenance burden**: Every new function requires updates in all language bindings

## Architecture

The bindings generator should be a standalone Rust tool that can be:
- Run as a CLI tool during development
- Integrated into the build process via `build.rs`
- Added as a dependency via Git or crates.io

### High-Level Flow

```
┌─────────────────┐
│  src/lib.rs     │  Parse Rust source
│  (Public API)   │────────┐
└─────────────────┘        │
                           ▼
                    ┌──────────────┐
                    │  Generator   │
                    │  Parser      │
                    └──────────────┘
                           │
          ┌────────────────┼────────────────┐
          ▼                ▼                ▼
   ┌──────────┐     ┌──────────┐    ┌──────────┐
   │   FFI    │     │   Go     │    │   npm    │
   │Generator │     │Generator │    │Generator │
   └──────────┘     └──────────┘    └──────────┘
          │                │                │
          ▼                ▼                ▼
   src/ffi.rs    bindings/go/     bindings/npm/
                 redstr.go         src/lib.rs
```

## Input Requirements

### Rust Source Parsing

The generator needs to extract from `src/lib.rs` and `src/transformations/**/*.rs`:

1. **Public functions** with signatures:
   ```rust
   pub fn function_name(param1: &str, param2: Option<&str>) -> String
   ```

2. **Function metadata**:
   - Function name
   - Parameter names and types
   - Return type
   - Documentation comments (for generating docs in target languages)
   - Module/category (from file path)

3. **Type mappings**:
   - Rust types → C FFI types
   - Rust types → Target language types

### Example Function to Parse

```rust
/// Randomly changes the capitalization of each character.
/// Useful for bypassing case-sensitive filters.
pub fn randomize_capitalization(input: &str) -> String {
    // implementation
}
```

**Extracted metadata**:
- Name: `randomize_capitalization`
- Parameters: `[{name: "input", type: "&str"}]`
- Return type: `String`
- Doc: "Randomly changes the capitalization..."
- Category: `case` (from file path)

## Output Requirements

### 1. FFI Layer (`src/ffi.rs`)

Generate C-compatible FFI wrappers:

```rust
/// # Safety
/// The caller must ensure `input` is a valid null-terminated C string.
/// The returned pointer must be freed using `redstr_free_string`.
#[no_mangle]
pub unsafe extern "C" fn redstr_randomize_capitalization(input: *const c_char) -> *mut c_char {
    let rust_input = c_str_to_rust(input);
    rust_str_to_c(randomize_capitalization(&rust_input))
}
```

**Requirements**:
- Memory management helpers (`redstr_free_string`, `c_str_to_rust`, `rust_str_to_c`)
- Safety documentation
- Consistent naming: `redstr_<function_name>`

### 2. Go Bindings (`bindings/go/redstr.go`)

Generate Go wrappers using CGO:

```go
// RandomizeCapitalization randomly changes the capitalization of each character.
// Useful for bypassing case-sensitive filters.
//
// Example:
//
//	result := redstr.RandomizeCapitalization("Hello World")
//	// Output: "HeLlO wOrLd" (varies each run)
func RandomizeCapitalization(input string) string {
    cInput := cString(input)
    defer C.free(unsafe.Pointer(cInput))
    return goString(C.redstr_randomize_capitalization(cInput))
}
```

**Requirements**:
- PascalCase function names
- CGO function calls
- Memory management (defer C.free)
- Documentation with examples
- Helper functions for string conversion

### 3. npm/JavaScript Bindings (`bindings/npm/src/lib.rs`)

Generate napi-rs wrappers:

```rust
/// Randomly changes the capitalization of each character.
/// Useful for bypassing case-sensitive filters.
#[napi]
pub fn randomize_capitalization(input: String) -> String {
    redstr::randomize_capitalization(&input)
}
```

**Requirements**:
- `#[napi]` macro annotation
- snake_case function names (matches JS convention)
- Direct delegation to redstr functions
- Documentation for TypeScript definitions

### 4. Python Bindings (`bindings/python/redstr.pyi`)

Generate PyO3 wrappers (future):

```rust
#[pyfunction]
fn randomize_capitalization(input: &str) -> PyResult<String> {
    Ok(redstr::randomize_capitalization(input))
}
```

**Requirements**:
- `#[pyfunction]` macro
- Error handling with `PyResult`
- Type stubs generation

### 5. Java Bindings (`bindings/java/`)

Generate JNI wrappers (future):

```java
public class Redstr {
    public static native String randomizeCapitalization(String input);
}
```

### 6. C# Bindings (`bindings/dotnet/`)

Generate P/Invoke wrappers (future):

```csharp
[DllImport("redstr")]
public static extern IntPtr redstr_randomize_capitalization(string input);
```

## Implementation Strategy

### Phase 1: Parser (Weeks 1-2)

Create a Rust source code parser:

1. **Use `syn` crate** to parse Rust syntax trees
2. **Extract public functions** from modules
3. **Build intermediate representation** (IR) with:
   - Function signatures
   - Documentation
   - Type information
   - Module hierarchy

```rust
struct FunctionInfo {
    name: String,
    params: Vec<Parameter>,
    return_type: Type,
    docs: String,
    module: String,
}

struct Parameter {
    name: String,
    ty: Type,
    is_optional: bool,
}
```

### Phase 2: FFI Generator (Weeks 2-3)

Generate C FFI layer:

1. **Template-based generation** using `askama` or `tera`
2. **Type mapping**: Rust → C FFI
3. **Memory management**: Proper string handling
4. **Safety documentation**: Generated doc comments

### Phase 3: Language Generators (Weeks 3-6)

Implement generators for each target language:

1. **Go generator**: CGO bindings
2. **npm generator**: napi-rs bindings
3. **Python generator**: PyO3 bindings
4. **Java generator**: JNI bindings
5. **C# generator**: P/Invoke bindings

Each generator should:
- Use templates for code generation
- Handle type conversions
- Generate idiomatic code for target language
- Include documentation and examples

### Phase 4: Testing & Validation (Weeks 6-7)

1. **Compare generated vs manual bindings**: Ensure output matches
2. **Integration tests**: Verify generated bindings compile and work
3. **Consistency checks**: All languages have same functions
4. **Performance benchmarks**: No regression from manual bindings

## Tool Interface

### CLI Usage

```bash
# Generate all bindings
redstr-bindgen generate --input src/lib.rs --output .

# Generate specific language
redstr-bindgen generate --language go --output bindings/go

# Validate without writing files
redstr-bindgen validate --input src/lib.rs

# Show diff between generated and existing
redstr-bindgen diff --language npm
```

### As Build Dependency

In `Cargo.toml`:

```toml
[build-dependencies]
redstr-bindgen = { git = "https://github.com/your-org/redstr-bindgen" }
```

In `build.rs`:

```rust
fn main() {
    redstr_bindgen::generate_all_bindings()
        .expect("Failed to generate bindings");
}
```

### Configuration File

`bindings.toml`:

```toml
[generator]
input_paths = ["src/lib.rs", "src/transformations"]
output_dir = "."

[languages.go]
enabled = true
output_path = "bindings/go/redstr.go"
package_name = "redstr"

[languages.npm]
enabled = true
output_path = "bindings/npm/src/lib.rs"
package_name = "@arvid-berndtsson/redstr"

[languages.python]
enabled = false  # Not yet implemented
output_path = "bindings/python/"

[type_mappings]
"String" = { c = "char*", go = "string", js = "string", python = "str" }
"&str" = { c = "const char*", go = "string", js = "string", python = "str" }
"Option<&str>" = { c = "const char*", go = "*string", js = "string | null" }
```

## Type System

### Supported Rust Types

1. **Primitive types**:
   - `String`, `&str` → strings in all languages
   - `bool` → boolean types
   - `i32`, `u32`, `i64`, `u64` → integers
   - `f32`, `f64` → floats

2. **Optional types**:
   - `Option<T>` → nullable/optional in target languages

3. **Complex types** (future):
   - `Vec<T>` → arrays/lists
   - Custom structs → mapped to target language structs

### Type Mapping Table

| Rust Type     | C FFI          | Go          | npm/JS      | Python | Java   | C#     |
|---------------|----------------|-------------|-------------|--------|--------|--------|
| `String`      | `*mut c_char` | `string`    | `string`    | `str`  | `String` | `string` |
| `&str`        | `*const c_char`| `string`   | `string`    | `str`  | `String` | `string` |
| `bool`        | `bool`        | `bool`      | `boolean`   | `bool` | `boolean` | `bool` |
| `i32`         | `i32`         | `int32`     | `number`    | `int`  | `int`  | `int`  |
| `Option<&str>`| `*const c_char`| `*string`  | `string?`   | `Optional[str]` | `String` | `string?` |

## Dependencies

### Generator Tool Dependencies

```toml
[dependencies]
syn = { version = "2.0", features = ["full", "extra-traits"] }
quote = "1.0"
proc-macro2 = "1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
toml = "0.8"
clap = { version = "4.0", features = ["derive"] }
askama = "0.12"  # For templates
anyhow = "1.0"
```

### Template Files

Store templates in `templates/` directory:

- `templates/ffi.rs.j2` - FFI layer template
- `templates/go.go.j2` - Go bindings template
- `templates/npm.rs.j2` - npm bindings template
- etc.

## Testing Strategy

### Unit Tests

Test each component independently:

```rust
#[test]
fn test_parse_function() {
    let source = r#"
        pub fn test_fn(input: &str) -> String {
            input.to_string()
        }
    "#;
    let func = parse_function(source).unwrap();
    assert_eq!(func.name, "test_fn");
    assert_eq!(func.params.len(), 1);
}
```

### Integration Tests

Verify generated code compiles:

```rust
#[test]
fn test_generated_ffi_compiles() {
    let generated = generate_ffi(&functions);
    // Write to temp file
    // Compile with rustc
    // Assert success
}
```

### Validation Tests

Compare generated vs. manual bindings:

```rust
#[test]
fn test_matches_manual_bindings() {
    let generated = generate_npm_bindings(&functions);
    let manual = read_to_string("bindings/npm/src/lib.rs").unwrap();
    // Compare normalized versions
    assert_similar(generated, manual);
}
```

## Maintenance & Evolution

### Adding New Languages

1. Implement new generator in `src/generators/<language>.rs`
2. Add templates in `templates/<language>/`
3. Update configuration schema
4. Add integration tests
5. Document in this file

### Handling Breaking Changes

When Rust API changes:

1. Run generator to update all bindings
2. Review generated changes
3. Run full test suite
4. Commit generated files with clear message

### Version Compatibility

- Generator version should match redstr library version
- Tag generator releases to match library releases
- Document compatibility matrix

## Example Repository Structure

```
redstr-bindgen/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs           # CLI entry point
│   ├── lib.rs            # Library interface
│   ├── parser.rs         # Rust source parser
│   ├── ir.rs             # Intermediate representation
│   ├── generators/
│   │   ├── mod.rs
│   │   ├── ffi.rs        # FFI generator
│   │   ├── go.rs         # Go generator
│   │   ├── npm.rs        # npm generator
│   │   ├── python.rs     # Python generator
│   │   ├── java.rs       # Java generator
│   │   └── csharp.rs     # C# generator
│   ├── templates/
│   │   ├── ffi.rs.j2
│   │   ├── go.go.j2
│   │   └── npm.rs.j2
│   └── config.rs         # Configuration handling
├── tests/
│   ├── integration_tests.rs
│   └── fixtures/         # Test input files
└── examples/
    └── generate.rs       # Example usage
```

## Performance Considerations

- **Incremental generation**: Only regenerate changed files
- **Parallel generation**: Generate languages in parallel
- **Caching**: Cache parsed IR between runs
- **Fast compilation**: Generated code should compile quickly

## Future Enhancements

1. **Interactive mode**: CLI wizard for configuring generator
2. **Watch mode**: Auto-regenerate on source changes
3. **Custom templates**: Allow users to provide custom templates
4. **Plugins**: Support for custom generators
5. **Documentation generation**: Auto-generate API docs for each language
6. **Benchmark generation**: Auto-generate performance tests

## References

- [Mozilla Diplomat](https://github.com/rust-diplomat/diplomat) - Multi-language FFI generator
- [rust-bindgen](https://github.com/rust-lang/rust-bindgen) - C/C++ bindings generator
- [PyO3](https://github.com/PyO3/pyo3) - Python bindings
- [napi-rs](https://napi.rs/) - Node.js bindings
- [autocxx](https://github.com/google/autocxx) - C++ bindings

## Contact & Support

For questions or contributions to the generator tool:
- Create issues in the redstr-bindgen repository
- Reference this design document in discussions
- Follow conventional commits for changes

---

**Document Version**: 1.0  
**Last Updated**: 2025-11-28  
**Authors**: Copilot Agent  
**Status**: Design/Planning Phase
