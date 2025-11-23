# redstr HTTP Server

A simple HTTP API server for redstr string transformations. This server provides a REST API that allows external tools to use redstr's transformation functions over HTTP.

## Features

- Zero-dependency HTTP server (uses only Rust standard library)
- REST API with JSON request/response
- CORS enabled for browser access
- Simple request/response format
- Thread-per-connection model

## Installation

```bash
cd integrations/http-server
cargo build --release
```

The binary will be available at `target/release/redstr-serve`.

## Usage

Start the server:

```bash
cargo run --release
```

Or run the compiled binary:

```bash
./target/release/redstr-serve
```

The server will listen on `http://127.0.0.1:8080` by default.

## API Endpoints

### GET /

Returns server information and available endpoints.

**Response:**
```json
{
  "service": "redstr",
  "version": "0.2.0",
  "endpoints": ["/transform", "/health"]
}
```

### GET /health

Health check endpoint.

**Response:**
```json
{
  "status": "healthy"
}
```

### POST /transform

Transform a string using a redstr function.

**Request:**
```json
{
  "function": "leetspeak",
  "input": "Hello World"
}
```

**Response:**
```json
{
  "output": "H3ll0 W0rld"
}
```

**Error Response:**
```json
{
  "error": "Unknown function: invalid_function"
}
```

## Available Functions

### Case Transformations
- `randomize_capitalization`
- `leetspeak`
- `alternate_case`
- `case_swap`
- `to_camel_case`
- `to_snake_case`
- `to_kebab_case`

### Encoding
- `base64_encode`
- `url_encode`
- `hex_encode`
- `html_entity_encode`
- `mixed_encoding`

### Injection Patterns
- `sql_comment_injection`
- `xss_tag_variations`
- `command_injection`
- `path_traversal`
- `null_byte_injection`
- `mongodb_injection`
- `couchdb_injection`
- `dynamodb_obfuscate`
- `nosql_operator_injection`
- `ssti_injection`
- `ssti_syntax_obfuscate`
- `ssti_framework_variation`

### Phishing
- `domain_typosquat`
- `advanced_domain_spoof`
- `email_obfuscation`
- `url_shortening_pattern`

### Obfuscation
- `rot13`
- `reverse_string`
- `vowel_swap`
- `double_characters`
- `whitespace_padding`
- `js_string_concat`

### Unicode
- `homoglyph_substitution`
- `unicode_variations`
- `zalgo_text`
- `space_variants`
- `unicode_normalize_variants`

### Cloudflare Evasion
- `cloudflare_turnstile_variation`
- `cloudflare_challenge_response`
- `tls_fingerprint_variation`
- `tls_handshake_pattern`
- `canvas_fingerprint_variation`
- `webgl_fingerprint_obfuscate`
- `font_fingerprint_consistency`

### Web Security
- `http_header_variation`
- `api_endpoint_variation`
- `graphql_obfuscate`
- `graphql_variable_injection`
- `graphql_introspection_bypass`
- `session_token_variation`
- `jwt_header_manipulation`
- `jwt_payload_obfuscate`
- `jwt_algorithm_confusion`
- `jwt_signature_bypass`

### Shell
- `bash_obfuscate`
- `powershell_obfuscate`
- `env_var_obfuscate`
- `file_path_obfuscate`

### Bot Detection
- `random_user_agent`
- `http2_header_order`
- `cloudflare_challenge_variation`
- `accept_language_variation`

## Example Usage

### Using curl

```bash
# Basic transformation
curl -X POST http://localhost:8080/transform \
  -H "Content-Type: application/json" \
  -d '{"function":"leetspeak","input":"password"}'

# SQL injection pattern
curl -X POST http://localhost:8080/transform \
  -H "Content-Type: application/json" \
  -d '{"function":"sql_comment_injection","input":"SELECT * FROM users"}'

# Domain typosquatting
curl -X POST http://localhost:8080/transform \
  -H "Content-Type: application/json" \
  -d '{"function":"domain_typosquat","input":"example.com"}'
```

### Using Python

```python
import requests

url = "http://localhost:8080/transform"
payload = {
    "function": "xss_tag_variations",
    "input": "<script>alert('XSS')</script>"
}

response = requests.post(url, json=payload)
print(response.json()["output"])
```

### Using JavaScript

```javascript
fetch('http://localhost:8080/transform', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    function: 'base64_encode',
    input: 'Hello World'
  })
})
.then(res => res.json())
.then(data => console.log(data.output));
```

## Integration with External Tools

This HTTP server is designed to be used as a bridge between redstr and external security testing tools:

- **EvilJinx**: Use for domain generation and email obfuscation
- **Caido**: Create plugins that call this API for transformations
- **Burp Suite**: Build extensions that interface with this server
- **Custom Tools**: Any tool that can make HTTP requests

See the integration documentation in the `docs/` directory for specific examples.

## Security Considerations

- The server binds to localhost (127.0.0.1) by default for security
- No authentication is implemented - add your own if exposing to network
- Designed for local use and authorized security testing only
- Log all transformation requests for audit purposes

## Performance

- Thread-per-connection model
- Synchronous I/O (suitable for moderate load)
- No external dependencies
- Minimal memory footprint

For high-performance scenarios, consider using an async runtime like Tokio.

## Troubleshooting

**Port already in use:**
```
Error: Address already in use (os error 98)
```
Solution: Change the port in `main.rs` or kill the process using port 8080.

**Connection refused:**
Ensure the server is running and accessible at the configured address.

## Future Enhancements

- [ ] Configuration file support
- [ ] Custom port binding
- [ ] Authentication/authorization
- [ ] Rate limiting
- [ ] Request logging
- [ ] Metrics endpoint
- [ ] Async I/O with Tokio
- [ ] TLS support

## License

MIT License - See LICENSE file in the repository root.
