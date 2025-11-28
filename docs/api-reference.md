# API Reference

Complete reference for all 60+ transformation functions in the redstr library.

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

## NoSQL Injection Testing

### mongodb_injection
MongoDB injection patterns for NoSQL testing.

**Signature:** `fn mongodb_injection(query: &str) -> String`

**Example:**
```rust
use redstr::mongodb_injection;
let query = r#"{"username": "admin"}"#;
let result = mongodb_injection(query);
// Adds MongoDB injection patterns
```

### couchdb_injection
CouchDB injection patterns for NoSQL testing.

**Signature:** `fn couchdb_injection(query: &str) -> String`

**Example:**
```rust
use redstr::couchdb_injection;
let query = r#"{"selector": {"name": "admin"}}"#;
let result = couchdb_injection(query);
```

### dynamodb_obfuscate
DynamoDB query obfuscation for NoSQL testing.

**Signature:** `fn dynamodb_obfuscate(query: &str) -> String`

**Example:**
```rust
use redstr::dynamodb_obfuscate;
let query = r#"{"Key": {"id": {"S": "123"}}}"#;
let result = dynamodb_obfuscate(query);
```

### nosql_operator_injection
Generic NoSQL operator injection patterns.

**Signature:** `fn nosql_operator_injection(input: &str) -> String`

**Example:**
```rust
use redstr::nosql_operator_injection;
let input = "username";
let result = nosql_operator_injection(input);
// Adds $ne, $gt, etc. operators
```

## Server-Side Template Injection (SSTI)

### ssti_injection
Server-side template injection patterns.

**Signature:** `fn ssti_injection(input: &str) -> String`

**Example:**
```rust
use redstr::ssti_injection;
let template = "{{user.name}}";
let result = ssti_injection(template);
```

### ssti_framework_variation
Framework-specific SSTI variations (Jinja2, Twig, etc.).

**Signature:** `fn ssti_framework_variation(template: &str, framework: &str) -> String`

**Example:**
```rust
use redstr::ssti_framework_variation;
let payload = "{{7*7}}";
let result = ssti_framework_variation(payload, "jinja2");
```

### ssti_syntax_obfuscate
Obfuscate SSTI syntax for filter bypass.

**Signature:** `fn ssti_syntax_obfuscate(input: &str) -> String`

**Example:**
```rust
use redstr::ssti_syntax_obfuscate;
let payload = "{{config}}";
let result = ssti_syntax_obfuscate(payload);
```

## Bot Detection & Cloudflare Evasion

### accept_language_variation
HTTP Accept-Language header variations.

**Signature:** `fn accept_language_variation(input: &str) -> String`

**Example:**
```rust
use redstr::accept_language_variation;
let lang = "en-US,en;q=0.9";
let result = accept_language_variation(lang);
```

### http2_header_order
HTTP/2 header order variations for fingerprinting evasion.

**Signature:** `fn http2_header_order(input: &str) -> String`

**Example:**
```rust
use redstr::http2_header_order;
let headers = "accept-language: en-US\naccept-encoding: gzip";
let result = http2_header_order(headers);
```

### tls_fingerprint_variation
TLS fingerprint variations for bot detection bypass.

**Signature:** `fn tls_fingerprint_variation(input: &str) -> String`

**Example:**
```rust
use redstr::tls_fingerprint_variation;
let cipher = "TLS_AES_256_GCM_SHA384";
let result = tls_fingerprint_variation(cipher);
```

### cloudflare_challenge_variation
Cloudflare challenge response variations.

**Signature:** `fn cloudflare_challenge_variation(input: &str) -> String`

**Example:**
```rust
use redstr::cloudflare_challenge_variation;
let challenge = "cf_clearance=abc123";
let result = cloudflare_challenge_variation(challenge);
```

### cloudflare_turnstile_variation
Cloudflare Turnstile CAPTCHA variations.

**Signature:** `fn cloudflare_turnstile_variation(input: &str) -> String`

**Example:**
```rust
use redstr::cloudflare_turnstile_variation;
let challenge = "challenge-token";
let result = cloudflare_turnstile_variation(challenge);
```

### cloudflare_challenge_response
Generate Cloudflare challenge response patterns.

**Signature:** `fn cloudflare_challenge_response(input: &str) -> String`

**Example:**
```rust
use redstr::cloudflare_challenge_response;
let input = "challenge-data";
let result = cloudflare_challenge_response(input);
```

### canvas_fingerprint_variation
Canvas fingerprinting evasion variations.

**Signature:** `fn canvas_fingerprint_variation(input: &str) -> String`

**Example:**
```rust
use redstr::canvas_fingerprint_variation;
let data = "canvas-data";
let result = canvas_fingerprint_variation(data);
```

### font_fingerprint_consistency
Font fingerprinting consistency variations.

**Signature:** `fn font_fingerprint_consistency(input: &str) -> String`

**Example:**
```rust
use redstr::font_fingerprint_consistency;
let fonts = "Arial,Helvetica,sans-serif";
let result = font_fingerprint_consistency(fonts);
```

### tls_handshake_pattern
TLS handshake pattern variations.

**Signature:** `fn tls_handshake_pattern(input: &str) -> String`

**Example:**
```rust
use redstr::tls_handshake_pattern;
let pattern = "TLS_AES_256_GCM_SHA384:TLS_CHACHA20_POLY1305_SHA256";
let result = tls_handshake_pattern(pattern);
```

### webgl_fingerprint_obfuscate
WebGL fingerprinting obfuscation.

**Signature:** `fn webgl_fingerprint_obfuscate(input: &str) -> String`

**Example:**
```rust
use redstr::webgl_fingerprint_obfuscate;
let data = "webgl-renderer-data";
let result = webgl_fingerprint_obfuscate(data);
```

## Web Security & API Testing

### http_header_variation
HTTP header value variations.

**Signature:** `fn http_header_variation(input: &str) -> String`

**Example:**
```rust
use redstr::http_header_variation;
let header = "application/json";
let result = http_header_variation(header);
```

### api_endpoint_variation
API endpoint path variations for fuzzing.

**Signature:** `fn api_endpoint_variation(input: &str) -> String`

**Example:**
```rust
use redstr::api_endpoint_variation;
let endpoint = "/api/users";
let result = api_endpoint_variation(endpoint);
```

### session_token_variation
Session token format variations.

**Signature:** `fn session_token_variation(input: &str) -> String`

**Example:**
```rust
use redstr::session_token_variation;
let token = "session123";
let result = session_token_variation(token);
```

### graphql_obfuscate
GraphQL query obfuscation.

**Signature:** `fn graphql_obfuscate(input: &str) -> String`

**Example:**
```rust
use redstr::graphql_obfuscate;
let query = "query { user { name } }";
let result = graphql_obfuscate(query);
```

### graphql_introspection_bypass
GraphQL introspection bypass techniques.

**Signature:** `fn graphql_introspection_bypass(input: &str) -> String`

**Example:**
```rust
use redstr::graphql_introspection_bypass;
let query = "__schema";
let result = graphql_introspection_bypass(query);
```

### graphql_variable_injection
GraphQL variable injection patterns.

**Signature:** `fn graphql_variable_injection(input: &str) -> String`

**Example:**
```rust
use redstr::graphql_variable_injection;
let variable = "userId";
let result = graphql_variable_injection(variable);
```

## JWT Security Testing

### jwt_header_manipulation
JWT header manipulation for algorithm confusion.

**Signature:** `fn jwt_header_manipulation(input: &str) -> String`

**Example:**
```rust
use redstr::jwt_header_manipulation;
let header = r#"{"alg":"HS256","typ":"JWT"}"#;
let result = jwt_header_manipulation(header);
```

### jwt_payload_obfuscate
JWT payload obfuscation.

**Signature:** `fn jwt_payload_obfuscate(input: &str) -> String`

**Example:**
```rust
use redstr::jwt_payload_obfuscate;
let payload = r#"{"sub":"1234567890"}"#;
let result = jwt_payload_obfuscate(payload);
```

### jwt_algorithm_confusion
JWT algorithm confusion attack patterns.

**Signature:** `fn jwt_algorithm_confusion(input: &str) -> String`

**Example:**
```rust
use redstr::jwt_algorithm_confusion;
let token = "eyJhbGc...";
let result = jwt_algorithm_confusion(token);
```

### jwt_signature_bypass
JWT signature bypass techniques.

**Signature:** `fn jwt_signature_bypass(input: &str) -> String`

**Example:**
```rust
use redstr::jwt_signature_bypass;
let token = "eyJhbGc...";
let result = jwt_signature_bypass(token);
```

## Phishing & Social Engineering

### email_obfuscation
Email address obfuscation for phishing testing.

**Signature:** `fn email_obfuscation(input: &str) -> String`

**Example:**
```rust
use redstr::email_obfuscation;
let email = "admin@example.com";
let result = email_obfuscation(email);
```

### advanced_domain_spoof
Advanced domain spoofing with multiple techniques.

**Signature:** `fn advanced_domain_spoof(domain: &str) -> String`

**Example:**
```rust
use redstr::advanced_domain_spoof;
let domain = "paypal.com";
let result = advanced_domain_spoof(domain);
```

### url_shortening_pattern
URL shortening pattern generation.

**Signature:** `fn url_shortening_pattern(url: &str) -> String`

**Example:**
```rust
use redstr::url_shortening_pattern;
let url = "https://example.com/long/path";
let result = url_shortening_pattern(url);
```

## Shell & Command Obfuscation

### powershell_obfuscate
PowerShell command obfuscation.

**Signature:** `fn powershell_obfuscate(cmd: &str) -> String`

**Example:**
```rust
use redstr::powershell_obfuscate;
let cmd = "Get-Process";
let result = powershell_obfuscate(cmd);
```

### bash_obfuscate
Bash command obfuscation.

**Signature:** `fn bash_obfuscate(cmd: &str) -> String`

**Example:**
```rust
use redstr::bash_obfuscate;
let cmd = "cat /etc/passwd";
let result = bash_obfuscate(cmd);
```

### env_var_obfuscate
Environment variable obfuscation.

**Signature:** `fn env_var_obfuscate(var: &str) -> String`

**Example:**
```rust
use redstr::env_var_obfuscate;
let var = "$PATH";
let result = env_var_obfuscate(var);
```

### file_path_obfuscate
File path obfuscation for command injection.

**Signature:** `fn file_path_obfuscate(path: &str) -> String`

**Example:**
```rust
use redstr::file_path_obfuscate;
let path = "/etc/passwd";
let result = file_path_obfuscate(path);
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
