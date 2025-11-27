# Python Bindings for redstr

This document describes the Python bindings implementation for the redstr library.

## Overview

The Python bindings allow Python developers to use all 60+ redstr transformation functions natively in Python code. The bindings are built using PyO3 and maturin, providing native performance with a Pythonic API.

## Features

- ✅ **All 60+ transformations available** - Every Rust function is wrapped for Python
- ✅ **Native performance** - Built with PyO3 for C-level speed
- ✅ **Type hints included** - Full `.pyi` stub files for IDE support and mypy
- ✅ **TransformBuilder class** - Fluent API for chaining transformations
- ✅ **Comprehensive tests** - 39 passing tests covering all function categories
- ✅ **Examples included** - `basic_usage.py` and `security_testing.py` examples
- ✅ **Documentation** - Complete usage guide in `docs/python_usage.md`

## Implementation Details

### Files Added

```
├── Cargo.toml                      # Added PyO3 dependency with "python" feature
├── pyproject.toml                  # Maturin build configuration
├── src/
│   ├── lib.rs                      # Conditionally includes python_bindings module
│   └── python_bindings.rs          # PyO3 wrapper functions (1400+ lines)
├── python/
│   ├── redstr/
│   │   ├── __init__.py            # Python package entry point with re-exports
│   │   ├── __init__.pyi           # Type stubs for all functions
│   │   └── py.typed               # PEP 561 marker file
│   ├── examples/
│   │   ├── basic_usage.py         # Demonstrates all transformation categories
│   │   └── security_testing.py   # Security testing use cases
│   └── tests/
│       └── test_transformations.py # 39 pytest tests
├── docs/
│   └── python_usage.md            # Complete usage guide with examples
└── .gitignore                      # Added Python-specific patterns
```

### Architecture

The Python bindings follow this architecture:

1. **Rust Module** (`src/python_bindings.rs`):
   - Contains PyO3 wrapper functions for each Rust function
   - Each function is prefixed with `py_` (e.g., `py_leetspeak`)
   - Uses `#[pyfunction]` macro for function wrappers
   - Uses `#[pyclass]` and `#[pymethods]` for TransformBuilder
   - Module exported as `_redstr` (internal)

2. **Python Package** (`python/redstr/__init__.py`):
   - Re-exports all functions with clean names
   - Imports from the internal `_redstr` module
   - Provides `__version__` and `__all__` exports

3. **Type Stubs** (`python/redstr/__init__.pyi`):
   - Full type annotations for all functions
   - Enables IDE autocomplete and mypy type checking
   - Marked with `py.typed` for PEP 561 compliance

### Building

The bindings use maturin for building:

```bash
# Development build (editable install)
maturin develop --features python --release

# Production build (wheel)
maturin build --features python --release
```

### Testing

Python tests are located in `python/tests/` and run with pytest:

```bash
pytest python/tests/ -v
```

All 39 tests pass, covering:
- Case transformations (7 tests)
- Encoding transformations (4 tests)
- Unicode transformations (3 tests)
- Injection transformations (6 tests)
- Obfuscation transformations (4 tests)
- Phishing transformations (2 tests)
- Bot detection transformations (1 test)
- Shell transformations (2 tests)
- Web security transformations (3 tests)
- TransformBuilder (5 tests)
- Module metadata (2 tests)

## API Compatibility

The Python API mirrors the Rust API with these differences:

1. **Function Names**: Snake_case in both languages (consistent)
2. **TransformBuilder**: Uses mutable methods that return `self` for chaining
3. **random_user_agent**: Takes no arguments (in both languages)
4. **ssti_framework_variation**: Takes 2 arguments (template, framework)

## Usage Example

```python
import redstr

# Basic transformation
result = redstr.leetspeak("password")
print(result)  # "p@55w0rd"

# Chaining transformations
builder = redstr.TransformBuilder("SELECT * FROM users")
payload = builder.case_swap().url().build()
print(payload)
```

## Feature Flag

The Python bindings are controlled by the `python` feature flag in `Cargo.toml`:

```toml
[features]
python = ["dep:pyo3"]

[dependencies]
pyo3 = { version = "0.22", optional = true, features = ["extension-module"] }
```

This keeps the core library dependency-free while allowing Python bindings when needed.

## PyO3 Version

Using PyO3 0.22.x which provides:
- Python 3.8+ support
- `Bound<'_, PyModule>` API (new in 0.21+)
- Improved error handling
- Better performance

## Future Enhancements

- [ ] Publish to PyPI
- [ ] Add CI/CD for building wheels (manylinux, macOS, Windows)
- [ ] Add more comprehensive error handling
- [ ] Add async support for long-running transformations
- [ ] Add performance benchmarks comparing to pure Python

## Documentation

See [docs/python_usage.md](docs/python_usage.md) for:
- Installation instructions
- Complete API reference
- Usage examples for all functions
- Integration examples (requests, selenium, burp, caido)
- Best practices
- Troubleshooting

## Zero-Dependency Principle

The Python bindings maintain redstr's zero-dependency principle:
- PyO3 is only included when `python` feature is enabled
- Core library remains dependency-free
- Python bindings are optional and don't affect the main library

## Security

- PyO3 0.22 is a recent, maintained version
- All transformations are safe (no unsafe code in bindings)
- Functions remain pure (no side effects)
- Same security considerations as Rust library apply

## License

Python bindings are licensed under MIT, same as the main library.
