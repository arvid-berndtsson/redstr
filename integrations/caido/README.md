# Caido Plugin for redstr

A Caido plugin that integrates redstr's string transformation capabilities into Caido's workflow automation and request manipulation features.

## Overview

This plugin enables security professionals to use redstr transformations directly within Caido for:
- Request/response transformation
- Payload generation
- WAF bypass testing
- Workflow automation

## Prerequisites

- Caido desktop application
- Node.js 18+ or 20+
- pnpm package manager
- redstr HTTP server running

## Installation

### 1. Start redstr HTTP Server

```bash
cd ../http-server
cargo run --release
```

The server should be running at `http://localhost:8080`.

### 2. Install Plugin (Coming Soon)

The Caido plugin implementation is planned. The structure will include:

```
caido/
├── package.json          # NPM dependencies
├── caido.config.ts      # Plugin configuration
├── src/
│   ├── frontend/        # UI components
│   │   ├── TransformPanel.tsx
│   │   ├── PayloadLibrary.tsx
│   │   └── ContextMenu.tsx
│   └── backend/         # Processing logic
│       ├── index.ts
│       ├── redstr-client.ts
│       └── workflow-nodes.ts
└── README.md
```

## Planned Features

### 1. Context Menu Integration

Right-click on any request/response text to apply redstr transformations:
- SQL Injection patterns
- XSS payload variations
- Encoding chains
- Obfuscation methods

### 2. Transformation Panel

Dedicated UI panel for:
- Selecting transformation methods
- Input/output text areas
- Transformation history
- Quick copy/apply buttons

### 3. Workflow Automation Nodes

Custom workflow nodes for:
- Payload obfuscation
- Multi-step encoding
- WAF bypass generation
- Attack chain automation

### 4. Payload Library

Pre-built payload templates for:
- SQL injection
- XSS attacks
- Path traversal
- Command injection
- NoSQL injection

## Usage Examples (Planned)

### Transform Selected Text

```typescript
// In Caido, select text and right-click
Context Menu → redstr → SQL Comment Injection
```

### Workflow Automation

```typescript
// Create workflow with redstr nodes
Request → redstr: Obfuscate → redstr: Encode → Send
```

### Payload Generation

```typescript
// Use payload library
Payload Library → SQL Injection → Select preset → Insert
```

## API Integration

The plugin will communicate with the redstr HTTP server:

```typescript
// Example client code
class RedstrClient {
  private apiUrl = 'http://localhost:8080';
  
  async transform(method: string, input: string): Promise<string> {
    const response = await fetch(`${this.apiUrl}/transform`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ function: method, input }),
    });
    
    const data = await response.json();
    return data.output;
  }
}
```

## Configuration

Plugin settings will include:
- API endpoint URL
- Default transformation methods
- Payload library paths
- Workflow presets

## Development

### Setup Development Environment

```bash
# Install dependencies
pnpm install

# Build plugin
pnpm build

# Watch mode for development
pnpm watch
```

### Testing

```bash
# Run unit tests
pnpm test

# Run integration tests
pnpm test:integration
```

## Use Cases

### 1. WAF Bypass Testing

Test web application firewalls with obfuscated payloads:
```typescript
const payload = "SELECT * FROM users";
const obfuscated = await redstr.transform('sql_comment_injection', payload);
// Output: SELECT/**/FROM/**/users
```

### 2. XSS Testing

Generate XSS payload variations:
```typescript
const xss = "<script>alert('XSS')</script>";
const variations = await redstr.transform('xss_tag_variations', xss);
```

### 3. API Fuzzing

Automate API fuzzing with workflow nodes:
```
Input → redstr: Injection → redstr: Encode → HTTP Request → Analyze
```

### 4. Session Token Testing

Test session token handling:
```typescript
const token = "session-token-value";
const varied = await redstr.transform('session_token_variation', token);
```

## Troubleshooting

### Plugin Won't Load

- Check Node.js version (18+ or 20+ required)
- Verify pnpm installation
- Check Caido application logs

### API Connection Fails

- Ensure redstr HTTP server is running
- Verify API URL in plugin settings
- Check firewall settings

### Transformations Fail

- Verify function name is correct
- Check API server logs
- Test transformation directly via curl

## Future Enhancements

- [ ] Implement plugin structure
- [ ] Create UI components
- [ ] Add workflow nodes
- [ ] Build payload library
- [ ] Publish to Caido Community Store
- [ ] Create video tutorials
- [ ] Add more transformation presets

## Documentation

See the comprehensive integration guide:
- [Caido Integration Documentation](../../docs/caido_integration.md)

## Support

For support and questions:
- Check [Caido Developer Docs](https://developer.caido.io/)
- Join Caido Discord
- File issues on GitHub

## Contributing

Contributions welcome! Please:
1. Fork the repository
2. Create a feature branch
3. Submit a pull request
4. Follow the coding standards

## License

MIT License - See LICENSE file in repository root.

---

**Status:** This integration is currently in planning phase. The HTTP API bridge is ready for use with custom Caido plugins. Full plugin implementation coming soon.

**Important:** This plugin is designed for authorized security testing only. Users must obtain proper authorization before conducting any security assessments.
