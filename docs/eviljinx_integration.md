# EvilJinx Integration for redstr

**Task ID:** INT-EJ-001, INT-EJ-002  
**Status:** Research Complete - Implementation Ready  
**Last Updated:** November 23, 2025

## Overview

This document outlines the integration between redstr and EvilJinx, a sophisticated adversary-in-the-middle (AiTM) phishing framework. The integration enables red team operators to leverage redstr's string obfuscation capabilities within EvilJinx phishing campaigns.

## EvilJinx Architecture

### Core Components

1. **Reverse Proxy (AiTM) Engine**
   - Operates as a transparent reverse proxy between victim and legitimate authentication services
   - Captures credentials, MFA tokens, and session cookies
   - Enables session hijacking without direct credential use

2. **Phishlets System**
   - YAML-based configuration files defining proxy behavior
   - Configurable for specific services (Google, Microsoft, Okta, etc.)
   - Supports traffic manipulation, content injection, and path rewrites

3. **Session Hijacking & Logging**
   - Logs all POST data and intercepted session tokens
   - Stores authentication cookies for later session reuse

4. **Infrastructure & Evasion**
   - Cloudflare integration for IP masking and WAF bypass
   - User-agent and IP filtering
   - Dynamic HTML/JavaScript obfuscation
   - Path rewriting for URL disguising

## Integration Points

### 1. Domain Generation Hooks

EvilJinx requires convincing domain names for phishing campaigns. redstr can provide:
- Domain typosquatting variations via `domain_typosquat()`
- Advanced domain spoofing via `advanced_domain_spoof()`
- Homoglyph substitution for visual similarity

### 2. Email Obfuscation

Pre-campaign email generation can leverage:
- `email_obfuscation()` for bypassing spam filters
- Unicode variations for evading detection
- HTML entity encoding for email payloads

### 3. Template Transformation

EvilJinx phishlets contain templates that can be enhanced:
- JavaScript obfuscation via `js_string_concat()`
- HTML entity encoding via `html_entity_encode()`
- Mixed encoding via `mixed_encoding()` for WAF bypass

### 4. Cloudflare Evasion

Leverage redstr's Cloudflare-specific functions:
- `cloudflare_turnstile_variation()`
- `cloudflare_challenge_response()`
- `tls_fingerprint_variation()`
- `canvas_fingerprint_variation()`

## Integration Design

### Library Integration (Recommended)

Since EvilJinx is written in Go and redstr is written in Rust, integration is achieved through:

1. **HTTP API Bridge** - redstr provides transformation via REST API
2. **CLI Wrapper** - EvilJinx scripts call redstr CLI
3. **Standalone Library** - Compile redstr as C-compatible library

### Option 1: HTTP API Bridge (Preferred)

```rust
// Simple HTTP server wrapping redstr transformations
// Can be deployed alongside EvilJinx
use redstr::*;

// POST /transform
// Body: {"function": "domain_typosquat", "input": "example.com"}
// Response: {"output": "exampl3.com"}
```

Benefits:
- Language-agnostic integration
- Easy to deploy and scale
- No compilation complexity

### Option 2: CLI Wrapper

```bash
# EvilJinx script calls redstr CLI
SPOOFED_DOMAIN=$(redstr domain_typosquat "example.com")
OBFUSCATED_EMAIL=$(redstr email_obfuscation "admin@example.com")
```

Benefits:
- Simple integration
- No additional infrastructure
- Direct shell integration

### Option 3: FFI Library (Advanced)

Compile redstr to C-compatible shared library for direct Go integration:

```rust
#[no_mangle]
pub extern "C" fn redstr_domain_typosquat(input: *const c_char) -> *mut c_char {
    // C-compatible wrapper
}
```

Benefits:
- Native performance
- No external dependencies
- Direct function calls

## Implementation Plan

### Phase 1: Core Integration Library (INT-EJ-002)

Create `integrations/eviljinx/` with:

1. **plugin.rs** - Main integration logic
   - HTTP server implementation
   - Request/response handling
   - Error handling

2. **hooks.rs** - Transformation hooks
   - Domain generation hooks
   - Email obfuscation hooks
   - Template transformation hooks

3. **config.rs** - Configuration management
   - Server settings
   - Transformation options
   - Logging configuration

4. **README.md** - Usage documentation
   - Setup instructions
   - API documentation
   - Example workflows

### Phase 2: CLI Integration

Enhance redstr CLI with EvilJinx-specific commands:

```bash
redstr eviljinx domain --input example.com
redstr eviljinx email --input admin@example.com
redstr eviljinx template --file phishlet.yaml
```

### Phase 3: Testing & Documentation

- Integration tests with sample phishlets
- Performance benchmarks
- Security best practices documentation
- Real-world usage examples

## Use Cases

### 1. Credential Harvesting Campaigns

```rust
use redstr::{domain_typosquat, email_obfuscation};

let legit_domain = "microsoft.com";
let phishing_domain = domain_typosquat(legit_domain);
// Output: m1crosoft.com, micr0soft.com, etc.

let sender = email_obfuscation("security@microsoft.com");
// Output: s&#101;curity&#64;microsoft&#46;com
```

### 2. MFA Bypass Operations

```rust
use redstr::{cloudflare_challenge_response, tls_fingerprint_variation};

// Evade bot detection during MFA flow
let challenge_token = "cf-challenge-token";
let varied_token = cloudflare_challenge_response(challenge_token);

// Randomize TLS fingerprint
let tls_fp = tls_fingerprint_variation("client-fingerprint");
```

### 3. Session Hijacking

```rust
use redstr::{jwt_header_manipulation, session_token_variation};

// Obfuscate captured session tokens
let session = "captured-session-token";
let obfuscated = session_token_variation(session);
```

## Security Considerations

### Ethical Use Only

This integration is strictly for:
- Authorized red team engagements
- Penetration testing with written permission
- Security awareness training
- Defensive research

### Detection Avoidance

- Use rotation of obfuscation techniques
- Combine multiple transformation methods
- Monitor for detection signatures
- Update patterns regularly

### Operational Security

- Deploy on isolated infrastructure
- Use proper logging and auditing
- Maintain engagement documentation
- Follow responsible disclosure

## API Reference

### Domain Generation

```rust
pub fn eviljinx_domain_spoof(domain: &str) -> Vec<String>
```

Returns multiple domain variations optimized for phishing campaigns.

### Email Obfuscation

```rust
pub fn eviljinx_email_obfuscate(email: &str, level: ObfuscationLevel) -> String
```

Obfuscates email addresses with configurable intensity.

### Template Processing

```rust
pub fn eviljinx_template_transform(template: &str) -> String
```

Processes EvilJinx phishlet templates with obfuscation.

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_generation() {
        let domain = "example.com";
        let variations = eviljinx_domain_spoof(domain);
        assert!(!variations.is_empty());
    }
}
```

### Integration Tests

Test with real phishlet structures and validate output format.

## References

- [EvilJinx Official Documentation](https://help.evilginx.com/)
- [EvilJinx Pro Features](https://evilginx.com/)
- [Phishlet Configuration Guide](https://github.com/kgretzky/evilginx2)
- [redstr Core Documentation](../README.md)

## Roadmap

- [x] Research EvilJinx architecture
- [x] Design integration approach
- [ ] Implement HTTP API bridge
- [ ] Create CLI wrappers
- [ ] Write integration tests
- [ ] Document deployment
- [ ] Publish examples

## Support

For integration support:
- File issues on GitHub
- Consult documentation
- Review example implementations
- Contact maintainers for authorized engagements

---

**Important:** This integration is provided for legitimate security testing only. Users must obtain proper authorization before conducting any phishing campaigns or security assessments.
