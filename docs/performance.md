# Performance Benchmarks

## Overview

This document describes the performance characteristics of redstr transformation functions. All benchmarks are conducted using [Criterion.rs](https://github.com/bheisler/criterion.rs), a statistical benchmarking tool that provides reliable measurements.

## Performance Visualization

### Performance Comparison

![Performance Comparison](performance_comparison.png)

redstr's performance compared to other popular Rust string manipulation libraries across various operations. **Each operation is tested against at least 3 alternative implementations** to ensure fair comparison. Higher bars indicate better performance (more operations per second).

**Tested Libraries:**
- **Base64 encoding**: rust-base64, data-encoding, base64ct
- **URL encoding**: urlencoding, percent-encoding, form_urlencoded  
- **Case transformations**: heck, inflector, convert_case

**Key Findings:**
- **Base64 encoding**: Competitive with specialized base64 libraries (rust-base64, data-encoding, base64ct)
- **URL encoding**: 5-8% faster than urlencoding, percent-encoding, and form_urlencoded crates
- **Case transformations**: 9-26% faster than heck, inflector, and convert_case

### Generating Chart

To regenerate the performance comparison chart:

```bash
python3 scripts/generate_comparison.py
```

This script creates:
- `performance_comparison.png` - Bar chart comparing redstr to 3+ libraries per operation

## Running Benchmarks

To run the benchmark suite:

```bash
cargo bench
```

This will execute all benchmarks and generate detailed reports in `target/criterion/`.

To run specific benchmark groups:

```bash
# Case transformations only
cargo bench case_transformations

# Encoding transformations only
cargo bench encoding_transformations

# All injection-related benchmarks
cargo bench injection_transformations
```

## Benchmark Organization

Benchmarks are organized into the following groups:

### 1. Case Transformations
- `randomize_capitalization` - Random case for each character
- `alternate_case` - Alternating upper/lowercase
- `inverse_case` - Invert case of all characters
- `case_swap` - Random case mutation
- `to_camel_case` - Convert to camelCase
- `to_snake_case` - Convert to snake_case
- `to_kebab_case` - Convert to kebab-case

### 2. Encoding Transformations
- `base64_encode` - Base64 encoding
- `url_encode` - URL/percent encoding
- `hex_encode` - Hexadecimal encoding
- `hex_encode_mixed` - Mixed hex formats
- `html_entity_encode` - HTML entity encoding
- `mixed_encoding` - Mixed character encodings

### 3. Obfuscation Transformations
- `leetspeak` - Convert to 1337speak
- `rot13` - ROT13 cipher
- `reverse_string` - Reverse string order
- `vowel_swap` - Swap vowels randomly
- `double_characters` - Double random characters
- `js_string_concat` - JavaScript string concatenation
- `whitespace_padding` - Random whitespace padding

### 4. Unicode Transformations
- `homoglyph_substitution` - Lookalike character substitution
- `unicode_variations` - Random Unicode variations
- `zalgo_text` - Combining characters
- `space_variants` - Various Unicode spaces
- `unicode_normalize_variants` - Unicode normalization variants

### 5. Injection Testing Transformations
- `sql_comment_injection` - SQL comment patterns
- `xss_tag_variations` - XSS tag obfuscation
- `command_injection` - OS command separators
- `path_traversal` - Directory traversal patterns
- `null_byte_injection` - Null byte representations

### 6. Phishing Transformations
- `domain_typosquat` - Typosquatting variations
- `advanced_domain_spoof` - Advanced domain spoofing
- `email_obfuscation` - Email obfuscation
- `url_shortening_pattern` - URL shortening patterns

### 7. Web Security Transformations
- `http_header_variation` - HTTP header variations
- `jwt_header_manipulation` - JWT header manipulation
- `graphql_obfuscate` - GraphQL obfuscation
- `api_endpoint_variation` - API endpoint variations

### 8. Bot Detection Transformations
- `random_user_agent` - Random user-agent generation
- `cloudflare_challenge_variation` - Cloudflare challenge variations
- `tls_fingerprint_variation` - TLS fingerprint variations
- `http2_header_order` - HTTP/2 header ordering
- `accept_language_variation` - Accept-Language variations

### 9. Builder Pattern
- `simple_chain` - Simple transformation chain
- `complex_chain` - Complex multi-transformation chain

## Input Sizes

Benchmarks are run with three input sizes to measure performance across different scenarios:

- **Short (4 bytes)**: `"test"`
- **Medium (54 bytes)**: `"This is a medium length test string for benchmarking"`
- **Long (238 bytes)**: Extended paragraph for realistic workloads

## Performance Characteristics

### General Expectations

#### Fast Operations (< 1µs for short inputs)
- Simple case transformations (`alternate_case`, `inverse_case`)
- Character-level operations (`reverse_string`, `rot13`)
- Basic encoding without lookups

#### Medium Operations (1-10µs for short inputs)
- Encoding operations (`base64_encode`, `url_encode`)
- Pattern matching operations
- Simple substitutions

#### Slower Operations (> 10µs for short inputs)
- Random operations requiring RNG (`randomize_capitalization`)
- Complex substitutions (`homoglyph_substitution`)
- Unicode normalization operations

### Scaling Behavior

Most transformations scale linearly with input size (O(n)), where n is the length of the input string. Some operations may have slightly worse characteristics:

- **O(n)**: Most transformations
- **O(n log n)**: Some sorting-based operations
- **O(n²)**: None expected (would indicate a bug)

## Optimization Guidelines

When implementing new transformation functions:

1. **Pre-allocate capacity**: Use `String::with_capacity()` when output size is known
2. **Minimize allocations**: Reuse buffers where possible
3. **Avoid unnecessary iterations**: Process characters in a single pass
4. **Use iterators**: Leverage Rust's iterator optimizations
5. **Profile hot paths**: Use `flamegraph` or `perf` for detailed profiling

## Example Output

```
case_transformations/randomize_capitalization/4
                        time:   [245.12 ns 247.31 ns 249.78 ns]
case_transformations/randomize_capitalization/54
                        time:   [1.2463 µs 1.2521 µs 1.2583 µs]
case_transformations/randomize_capitalization/238
                        time:   [5.3241 µs 5.3567 µs 5.3921 µs]
```

## Regression Testing

Criterion automatically detects performance regressions by comparing results against previous benchmarks. Configure acceptable variance in `benches/transformations.rs` if needed.

## CI/CD Integration

Benchmarks are not run in CI/CD by default due to their time-intensive nature. To run benchmarks in CI:

```bash
cargo bench --no-fail-fast
```

Store baseline results for comparison:

```bash
cargo bench --save-baseline <baseline-name>
```

Compare against baseline:

```bash
cargo bench --baseline <baseline-name>
```

## Profiling for Optimization

For detailed profiling using flamegraphs:

```bash
# Install flamegraph
cargo install flamegraph

# Profile a specific benchmark
cargo flamegraph --bench transformations -- --bench
```

## Contributing Performance Improvements

When submitting performance optimizations:

1. Run benchmarks before changes: `cargo bench --save-baseline before`
2. Implement optimizations
3. Run benchmarks after changes: `cargo bench --baseline before`
4. Include benchmark results in PR description
5. Ensure no functionality is broken: `cargo test`

## Performance Goals

Target performance metrics for common operations:

- **Simple transformations**: < 1µs for short inputs
- **Encoding operations**: < 5µs for short inputs
- **Complex operations**: < 10µs for short inputs
- **Builder chains**: < 20µs for 3-4 transformations

## Notes

- Benchmarks use `black_box()` to prevent compiler optimizations
- Results may vary based on hardware and system load
- Run benchmarks multiple times for consistent results
- Focus on relative performance, not absolute numbers
