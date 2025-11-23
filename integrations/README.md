# redstr Integrations

This directory contains integrations between redstr and various security testing platforms and frameworks.

## Overview

The integrations enable security professionals to use redstr's string transformation capabilities within their existing security testing workflows. Each integration is designed to be lightweight, easy to deploy, and follows best practices for the target platform.

## Available Integrations

### 1. HTTP Server (`http-server/`)

A standalone HTTP API server that provides REST endpoints for all redstr transformations. This server acts as a bridge between redstr and external tools.

**Status:** ✅ Implemented and tested

**Features:**
- RESTful API for all redstr functions
- JSON request/response format
- CORS enabled
- Zero external dependencies
- Thread-per-connection model

**Use Cases:**
- Bridge for EvilJinx integration
- Backend for Caido plugins
- API for Burp Suite extensions
- Custom tool integration

**Quick Start:**
```bash
cd http-server
cargo build --release
cargo run --release
```

See: [http-server/README.md](http-server/README.md)

### 2. EvilJinx Integration (`eviljinx/`)

Integration with EvilJinx phishing framework for enhanced domain generation, email obfuscation, and template transformation.

**Status:** 📝 Documentation and scripts provided

**Features:**
- Domain typosquatting generation
- Email obfuscation for spam bypass
- Template transformation
- Cloudflare evasion capabilities
- Go integration examples

**Use Cases:**
- Phishing campaign domain generation
- Email content obfuscation
- JavaScript obfuscation
- Session token manipulation

**Quick Start:**
```bash
# Start HTTP server first
cd ../http-server && cargo run --release &

# Use helper scripts
cd eviljinx/scripts
./generate_domains.sh example.com
./obfuscate_email.sh admin@example.com
```

See: [eviljinx/README.md](eviljinx/README.md)

### 3. Caido Plugin (`caido/`)

Plugin for Caido web security testing toolkit, providing UI integration and workflow automation.

**Status:** 🔄 Planned - HTTP API bridge ready for use

**Planned Features:**
- Context menu integration
- Transformation panel
- Workflow automation nodes
- Payload library
- Request/response transformation

**Use Cases:**
- WAF bypass testing
- XSS payload generation
- API fuzzing
- Request transformation

**Future Implementation:**
```bash
cd caido
pnpm install
pnpm build
# Load in Caido Extensions
```

See: [caido/README.md](caido/README.md)

### 4. Burp Suite Extension (`burp/`)

Extension for Burp Suite providing Intruder integration, scanner checks, and context menu actions.

**Status:** 🔄 Planned - HTTP API bridge ready for use

**Planned Features:**
- Intruder payload generator
- Context menu transformations
- Custom scanner checks
- Dedicated transformation tab
- Session handler integration

**Use Cases:**
- SQL injection testing
- XSS with WAF bypass
- JWT manipulation
- Phishing domain testing

**Future Implementation:**
```bash
cd burp
mvn clean package
# Load redstr-burp.jar in Burp Suite Extensions
```

See: [burp/README.md](burp/README.md)

## Integration Architecture

All integrations follow a common architecture pattern:

```
┌─────────────────┐
│  External Tool  │ (EvilJinx, Caido, Burp, etc.)
└────────┬────────┘
         │
         │ HTTP/CLI/FFI
         │
┌────────▼────────┐
│  HTTP Server    │ (redstr HTTP API)
│   or CLI/FFI    │
└────────┬────────┘
         │
         │ Function calls
         │
┌────────▼────────┐
│  redstr Core    │ (Rust library)
└─────────────────┘
```

### Integration Options

1. **HTTP API Bridge** (Recommended)
   - Language-agnostic
   - Easy to deploy
   - Scalable
   - No compilation complexity

2. **CLI Wrapper**
   - Simple integration
   - Direct shell use
   - No infrastructure needed

3. **FFI Library** (Advanced)
   - Native performance
   - Direct function calls
   - Platform-specific builds

## Common Setup

### 1. Build redstr

```bash
cd /path/to/redstr
cargo build --release
```

### 2. Start HTTP Server (for HTTP-based integrations)

```bash
cd integrations/http-server
cargo run --release
```

Server will be available at `http://localhost:8080`

### 3. Test API

```bash
# Health check
curl http://localhost:8080/health

# Transform text
curl -X POST http://localhost:8080/transform \
  -H "Content-Type: application/json" \
  -d '{"function":"leetspeak","input":"Hello World"}'
```

## API Reference

### Endpoints

- `GET /` - Server information
- `GET /health` - Health check
- `POST /transform` - Transform string

### Request Format

```json
{
  "function": "method_name",
  "input": "text to transform"
}
```

### Response Format

```json
{
  "output": "transformed text"
}
```

### Error Format

```json
{
  "error": "error message"
}
```

## Available Transformation Functions

See [http-server/README.md](http-server/README.md) for complete list of available functions including:

- Case transformations
- Encoding/decoding
- Injection patterns (SQL, XSS, NoSQL, SSTI)
- Phishing (domain, email)
- Obfuscation
- Unicode variations
- Cloudflare evasion
- Web security (JWT, GraphQL, API)
- Shell obfuscation
- Bot detection evasion

## Security Considerations

### Authorized Use Only

All integrations are designed for:
- Authorized penetration testing
- Red team engagements with permission
- Security awareness training
- Defensive security research

### Best Practices

1. **Authorization**: Always obtain written permission before testing
2. **Logging**: Enable comprehensive logging for audit trails
3. **Isolation**: Deploy on isolated infrastructure
4. **Documentation**: Maintain engagement documentation
5. **Responsible Disclosure**: Follow responsible disclosure practices

### Network Security

- HTTP server binds to localhost by default
- Implement authentication for network exposure
- Use TLS for production deployments
- Apply rate limiting as needed

## Development

### Adding New Integrations

1. Create directory under `integrations/`
2. Implement integration using HTTP API or CLI
3. Write comprehensive README
4. Add tests
5. Update this main README

### Testing Integrations

Each integration should include:
- Unit tests
- Integration tests
- Example usage
- Documentation

## Troubleshooting

### HTTP Server Won't Start

```
Error: Address already in use (os error 98)
```

**Solution**: Kill process using port 8080 or change port in `http-server/src/main.rs`

### API Connection Fails

**Checklist:**
- [ ] HTTP server is running
- [ ] Correct URL (default: http://localhost:8080)
- [ ] Firewall allows connection
- [ ] No proxy interference

### Transformations Fail

**Checklist:**
- [ ] Function name is spelled correctly
- [ ] Input format is valid
- [ ] Check API server logs
- [ ] Test with curl directly

## Contributing

Contributions are welcome! To contribute:

1. Fork the repository
2. Create a feature branch
3. Implement integration following existing patterns
4. Add comprehensive documentation
5. Submit pull request

### Integration Guidelines

- Follow existing directory structure
- Provide complete README with examples
- Include error handling
- Add troubleshooting section
- Document security considerations

## Documentation

- [EvilJinx Integration Guide](../docs/eviljinx_integration.md)
- [Caido Integration Guide](../docs/caido_integration.md)
- [Burp Suite Integration Guide](../docs/burp_integration.md)

## Support

For integration support:
- Check integration-specific README
- Review documentation in `docs/`
- File issues on GitHub
- Consult platform-specific resources

## Roadmap

### Completed
- [x] HTTP API server
- [x] EvilJinx helper scripts
- [x] Integration documentation

### In Progress
- [ ] Caido plugin implementation
- [ ] Burp Suite extension implementation

### Planned
- [ ] Metasploit integration
- [ ] OWASP ZAP extension
- [ ] Nuclei template integration
- [ ] Custom payloads for SQLMap
- [ ] GitHub Actions integration

## License

MIT License - See LICENSE file in repository root.

---

**Important:** These integrations are provided for legitimate security testing only. Users must obtain proper authorization before conducting any security assessments or phishing campaigns.
