# API Reference

Complete reference for all transformation functions in the redstr library.

## Function Signature Pattern

All transformation functions accept a `&str` and return a `String`:

```rust
fn function_name(input: &str) -> String
```

## Encoding & Obfuscation

### base64_encode
Base64 encoding for payload obfuscation.

**Signature:** `fn base64_encode(input: &str) -> String`

**Example:**
```rust
use redstr::base64_encode;
let encoded = base64_encode("hello");
println!("{}", encoded); // "aGVsbG8="
```

### url_encode
RFC 3986 URL/percent encoding with UTF-8 support.

**Signature:** `fn url_encode(input: &str) -> String`

**Example:**
```rust
use redstr::url_encode;
let encoded = url_encode("test @example.com");
println!("{}", encoded); // "test%20%40example.com"
```

### hex_encode
Hexadecimal encoding (lowercase).

**Signature:** `fn hex_encode(input: &str) -> String`

**Example:**
```rust
use redstr::hex_encode;
let encoded = hex_encode("test");
println!("{}", encoded); // "74657374"
```

### hex_encode_mixed
Mixed hex formats (`\x`, `%`, `0x`, `&#x`).

**Signature:** `fn hex_encode_mixed(input: &str) -> String`

**Example:**
```rust
use redstr::hex_encode_mixed;
let encoded = hex_encode_mixed("ab");
// Output varies: "\x61%62" or similar
```

### html_entity_encode
HTML entity encoding for XSS testing.

**Signature:** `fn html_entity_encode(input: &str) -> String`

**Example:**
```rust
use redstr::html_entity_encode;
let encoded = html_entity_encode("<script>");
// "&#60;script&#62;"
```

### mixed_encoding
Mixed character encodings (HTML entities + Unicode).

**Signature:** `fn mixed_encoding(input: &str) -> String`

**Example:**
```rust
use redstr::mixed_encoding;
let encoded = mixed_encoding("test");
// Mix of HTML entities and Unicode escapes
```

## String Transformation

### randomize_capitalization
Random case for each character.

**Signature:** `fn randomize_capitalization(input: &str) -> String`

**Example:**
```rust
use redstr::randomize_capitalization;
let result = randomize_capitalization("Hello World");
// "HeLlO wOrLd" (varies)
```

### alternate_case
Alternating upper/lowercase.

**Signature:** `fn alternate_case(input: &str) -> String`

**Example:**
```rust
use redstr::alternate_case;
let result = alternate_case("Hello World");
println!("{}", result); // "HeLlO wOrLd"
```

### inverse_case
Invert case of all characters.

**Signature:** `fn inverse_case(input: &str) -> String`

**Example:**
```rust
use redstr::inverse_case;
let result = inverse_case("Hello World");
println!("{}", result); // "hELLO wORLD"
```

### case_swap
Random case mutation for WAF bypass.

**Signature:** `fn case_swap(input: &str) -> String`

**Example:**
```rust
use redstr::case_swap;
let result = case_swap("SELECT");
// "SeLeCt" (varies)
```

### leetspeak
Convert to 1337speak for filter evasion.

**Signature:** `fn leetspeak(input: &str) -> String`

**Example:**
```rust
use redstr::leetspeak;
let result = leetspeak("password");
println!("{}", result); // "p@55w0rd"
```

### rot13
ROT13 cipher transformation.

**Signature:** `fn rot13(input: &str) -> String`

**Example:**
```rust
use redstr::rot13;
let result = rot13("Hello");
println!("{}", result); // "Uryyb"
```

### reverse_string
Reverse string order.

**Signature:** `fn reverse_string(input: &str) -> String`

**Example:**
```rust
use redstr::reverse_string;
let result = reverse_string("Hello");
println!("{}", result); // "olleH"
```

## Unicode & Homoglyphs

### homoglyph_substitution
Lookalike character substitution for phishing.

**Signature:** `fn homoglyph_substitution(input: &str) -> String`

**Example:**
```rust
use redstr::homoglyph_substitution;
let result = homoglyph_substitution("admin@example.com");
// "аdmіn@еxаmple.com" (using Cyrillic)
```

### unicode_variations
Random Unicode character variations.

**Signature:** `fn unicode_variations(input: &str) -> String`

**Example:**
```rust
use redstr::unicode_variations;
let result = unicode_variations("test");
// "tëšt" (varies)
```

### unicode_normalize_variants
Unicode normalization testing.

**Signature:** `fn unicode_normalize_variants(input: &str) -> String`

**Example:**
```rust
use redstr::unicode_normalize_variants;
let result = unicode_normalize_variants("test");
// Various normalized forms
```

### zalgo_text
Combining characters for display corruption.

**Signature:** `fn zalgo_text(input: &str) -> String`

**Example:**
```rust
use redstr::zalgo_text;
let result = zalgo_text("test");
// "t̃̂e̊̋s̈̃t̂̃"
```

## Case Conversion

### to_camel_case
Convert to camelCase.

**Signature:** `fn to_camel_case(input: &str) -> String`

**Example:**
```rust
use redstr::to_camel_case;
let result = to_camel_case("hello world test");
println!("{}", result); // "helloWorldTest"
```

### to_snake_case
Convert to snake_case.

**Signature:** `fn to_snake_case(input: &str) -> String`

**Example:**
```rust
use redstr::to_snake_case;
let result = to_snake_case("HelloWorldTest");
println!("{}", result); // "hello_world_test"
```

### to_kebab_case
Convert to kebab-case.

**Signature:** `fn to_kebab_case(input: &str) -> String`

**Example:**
```rust
use redstr::to_kebab_case;
let result = to_kebab_case("HelloWorldTest");
println!("{}", result); // "hello-world-test"
```

## Injection Testing

### sql_comment_injection
SQL comment patterns (`--`, `/**/`, `#`).

**Signature:** `fn sql_comment_injection(input: &str) -> String`

**Example:**
```rust
use redstr::sql_comment_injection;
let result = sql_comment_injection("SELECT * FROM users");
// "SELECT --* FROM users" (varies)
```

### xss_tag_variations
XSS tag obfuscation and encoding.

**Signature:** `fn xss_tag_variations(input: &str) -> String`

**Example:**
```rust
use redstr::xss_tag_variations;
let result = xss_tag_variations("<script>alert(1)</script>");
// Encoded variations
```

### command_injection
OS command separators (`;`, `|`, `&&`).

**Signature:** `fn command_injection(input: &str) -> String`

**Example:**
```rust
use redstr::command_injection;
let result = command_injection("ping example.com");
// "ping;example.com" (varies)
```

### path_traversal
Directory traversal patterns (`../`, `..\\`).

**Signature:** `fn path_traversal(input: &str) -> String`

**Example:**
```rust
use redstr::path_traversal;
let result = path_traversal("/etc/passwd");
// "../etc/../passwd" (varies)
```

### null_byte_injection
Null byte representations (`%00`, `\0`).

**Signature:** `fn null_byte_injection(input: &str) -> String`

**Example:**
```rust
use redstr::null_byte_injection;
let result = null_byte_injection("file.txt");
// "file%00.txt" (varies)
```

## Web Security

### random_user_agent
Generate random browser user-agents.

**Signature:** `fn random_user_agent() -> String`

**Example:**
```rust
use redstr::random_user_agent;
let ua = random_user_agent();
// Random modern browser UA
```

### domain_typosquat
Typosquatting variations for phishing.

**Signature:** `fn domain_typosquat(domain: &str) -> String`

**Example:**
```rust
use redstr::domain_typosquat;
let typo = domain_typosquat("example.com");
// "examp1e.com", "exmaple.com", etc. (varies)
```

### js_string_concat
JavaScript string concatenation obfuscation.

**Signature:** `fn js_string_concat(input: &str) -> String`

**Example:**
```rust
use redstr::js_string_concat;
let result = js_string_concat("document.cookie");
// "'doc'+'ument'+'.co'+'okie'" (varies)
```

### whitespace_padding
Random whitespace for filter bypass.

**Signature:** `fn whitespace_padding(input: &str) -> String`

**Example:**
```rust
use redstr::whitespace_padding;
let result = whitespace_padding("test");
// "  test  " (varies)
```

## Utility Functions

### vowel_swap
Swap vowels for pattern matching tests.

**Signature:** `fn vowel_swap(input: &str) -> String`

**Example:**
```rust
use redstr::vowel_swap;
let result = vowel_swap("testing");
// "tistong" (varies)
```

### double_characters
Random character doubling.

**Signature:** `fn double_characters(input: &str) -> String`

**Example:**
```rust
use redstr::double_characters;
let result = double_characters("test");
// "tteesstt" (varies)
```

### space_variants
Various Unicode space characters.

**Signature:** `fn space_variants(input: &str) -> String`

**Example:**
```rust
use redstr::space_variants;
let result = space_variants("hello world");
// Uses various Unicode spaces
```

## Builder Pattern

### TransformBuilder
Fluent API for chaining transformations.

**Methods:**
- `.new(input: &str)` - Create a new builder
- `.leetspeak()` - Apply leetspeak transformation
- `.base64()` - Apply base64 encoding
- `.url_encode()` - Apply URL encoding
- `.redstrs()` - Apply random capitalization
- `.homoglyphs()` - Apply homoglyph substitution
- `.case_swap()` - Apply case swapping
- `.hex_encode()` - Apply hex encoding
- `.rot13()` - Apply ROT13
- `.build()` - Get the final result

**Example:**
```rust
use redstr::TransformBuilder;

let result = TransformBuilder::new("admin@example.com")
    .homoglyphs()
    .url_encode()
    .build();
```

## See Also

- [CLI Reference](cli-reference.md) - Command-line interface documentation
- [Use Cases](use-cases.md) - Security testing scenarios
- [Integration Examples](integration-examples.md) - Tool integration patterns
- [Library Documentation](https://docs.rs/redstr) - Full docs on docs.rs
