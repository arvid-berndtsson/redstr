# Caido Integration for redstr

**Task ID:** INT-CAIDO-001, INT-CAIDO-002  
**Status:** Research Complete - Implementation Ready  
**Last Updated:** November 23, 2025

## Overview

This document outlines the integration between redstr and Caido, a modern web security testing toolkit. The integration enables security professionals to use redstr's string transformation capabilities directly within Caido's workflow automation and request manipulation features.

## Caido Architecture

### Core Components

1. **Proxy Engine**
   - Intercepts HTTP/HTTPS/WebSocket traffic
   - Provides request/response manipulation
   - Supports automatic and manual interception

2. **Plugin System**
   - JavaScript/TypeScript-based plugins
   - Frontend UI components for user interaction
   - Backend processing for automation and data manipulation
   - Hot-reloading for rapid development

3. **Workflow Automation**
   - Automate repetitive security tasks
   - Custom nodes for processing pipelines
   - Integration with external tools

4. **Developer SDK**
   - `@caido/sdk-frontend` - UI components and state management
   - `@caido/sdk-backend` - Request/response processing
   - `@caido/sdk-workflow` - Automation node creation

## Integration Points

### 1. Request Transformation

Caido plugins can transform intercepted requests using redstr:
- SQL injection pattern generation
- XSS payload obfuscation
- Command injection variants
- Path traversal encoding

### 2. Response Analysis

Analyze and transform responses:
- Extract and decode obfuscated content
- Pattern recognition for security issues
- Automated vulnerability detection

### 3. Workflow Automation

Create custom workflow nodes:
- Obfuscation node for payloads
- Encoding transformation node
- WAF bypass generation node
- Multi-step attack chain node

### 4. UI Enhancement

Add redstr functionality to Caido's UI:
- Context menu items for quick transforms
- Dedicated transformation panel
- Preset payload libraries
- Custom encoder/decoder

## Integration Design

### Plugin Architecture

Caido plugins consist of:

1. **Configuration** (`caido.config.ts`)
2. **Frontend** (UI components)
3. **Backend** (Processing logic)

### Integration Approaches

#### Option 1: Native JavaScript Plugin (Recommended)

Compile redstr to WebAssembly and use in JavaScript:

```typescript
// caido.config.ts
import { defineConfig } from '@caido/cli';

export default defineConfig({
  name: 'redstr-integration',
  version: '1.0.0',
  description: 'redstr string transformations for Caido',
  frontend: {
    path: './frontend',
  },
  backend: {
    path: './backend',
  },
});
```

Benefits:
- Native Caido integration
- No external dependencies
- Fast performance via WASM
- Full SDK access

#### Option 2: HTTP Bridge Plugin

Create a plugin that communicates with redstr HTTP API:

```typescript
// backend/index.ts
export function init(sdk: SDK) {
  sdk.api.register('redstr.transform', async (input, method) => {
    const response = await fetch('http://localhost:8080/transform', {
      method: 'POST',
      body: JSON.stringify({ function: method, input }),
    });
    return await response.json();
  });
}
```

Benefits:
- Easy to implement
- Language-agnostic
- Can use full redstr Rust library
- Scalable architecture

#### Option 3: CLI Wrapper Plugin

Execute redstr CLI from plugin backend:

```typescript
import { exec } from 'child_process';
import { promisify } from 'util';

const execAsync = promisify(exec);

export async function transform(input: string, method: string): Promise<string> {
  const { stdout } = await execAsync(`redstr ${method} "${input}"`);
  return stdout.trim();
}
```

Benefits:
- Simple implementation
- No compilation needed
- Direct access to redstr CLI
- Easy debugging

## Implementation Plan

### Phase 1: Plugin Structure (INT-CAIDO-002)

Create `integrations/caido/` with:

1. **package.json** - NPM dependencies
2. **caido.config.ts** - Plugin configuration
3. **frontend/** - UI components
   - TransformPanel.tsx
   - PayloadLibrary.tsx
   - ContextMenu.tsx
4. **backend/** - Processing logic
   - index.ts (main entry)
   - transforms.ts (redstr bindings)
   - workflow-nodes.ts (custom nodes)
5. **README.md** - Installation and usage

### Phase 2: Core Features

1. **Request Interceptor**
   ```typescript
   sdk.intercept.onRequest((request) => {
     // Transform request using redstr
     const transformed = await redstr.transform(request.body);
     return { ...request, body: transformed };
   });
   ```

2. **Context Menu Integration**
   ```typescript
   sdk.contextMenu.register({
     label: 'redstr: SQL Injection',
     action: async (selection) => {
       const payload = await redstr.sqlInjection(selection);
       sdk.clipboard.write(payload);
     },
   });
   ```

3. **Workflow Nodes**
   ```typescript
   sdk.workflow.registerNode({
     id: 'redstr.obfuscate',
     name: 'redstr Obfuscation',
     inputs: ['string'],
     outputs: ['obfuscated'],
     execute: async (input) => ({
       obfuscated: await redstr.obfuscate(input.string),
     }),
   });
   ```

### Phase 3: UI Components

1. **Transformation Panel**
   - Dropdown for transformation type
   - Input/output text areas
   - Copy/apply buttons
   - Transformation history

2. **Payload Library**
   - Pre-built payload templates
   - Custom payload storage
   - Import/export functionality
   - Search and filter

3. **Settings Panel**
   - Configuration options
   - API endpoint settings
   - Transformation presets
   - Logging level

## Use Cases

### 1. WAF Bypass Testing

```typescript
// Transform request to bypass WAF
const original = "SELECT * FROM users WHERE id=1";
const obfuscated = await redstr.sqlCommentInjection(original);
// Output: SELECT/**/*/**/FROM/**/users/**/WHERE/**/id=1
```

### 2. XSS Testing

```typescript
// Generate XSS payload variations
const payload = "<script>alert('XSS')</script>";
const variations = await redstr.xssTagVariations(payload);
// Output: [
//   "<script>alert('XSS')</script>",
//   "<ScRiPt>alert('XSS')</sCrIpT>",
//   "&#60;script&#62;alert('XSS')&#60;/script&#62;"
// ]
```

### 3. Phishing Domain Testing

```typescript
// Test phishing detection
const domain = "example.com";
const typosquats = await redstr.domainTyposquat(domain);
// Output: ["exampl3.com", "examp1e.com", "exarnple.com"]
```

### 4. API Fuzzing

```typescript
// Workflow node for API fuzzing
sdk.workflow.registerNode({
  id: 'redstr.api.fuzz',
  name: 'API Fuzzing',
  execute: async (input) => {
    const results = [];
    for (const method of ['sql', 'xss', 'command']) {
      const fuzzed = await redstr.transform(input.endpoint, method);
      const response = await sdk.http.request(fuzzed);
      results.push({ method, response });
    }
    return { results };
  },
});
```

## API Reference

### Backend API

```typescript
interface RedstrAPI {
  // Case transformations
  randomizeCapitalization(input: string): Promise<string>;
  leetspeak(input: string): Promise<string>;
  
  // Encoding
  base64Encode(input: string): Promise<string>;
  urlEncode(input: string): Promise<string>;
  htmlEntityEncode(input: string): Promise<string>;
  
  // Injection
  sqlCommentInjection(input: string): Promise<string>;
  xssTagVariations(input: string): Promise<string[]>;
  commandInjection(input: string): Promise<string>;
  
  // Phishing
  domainTyposquat(domain: string): Promise<string[]>;
  emailObfuscation(email: string): Promise<string>;
  
  // Cloudflare evasion
  cloudflareChallenge(token: string): Promise<string>;
  tlsFingerprintVariation(fp: string): Promise<string>;
}
```

### Frontend Components

```typescript
// Transformation Panel
<TransformPanel
  input={selectedText}
  onTransform={(method, output) => applyTransform(output)}
  methods={['sql', 'xss', 'encoding', 'phishing']}
/>

// Payload Library
<PayloadLibrary
  categories={['injection', 'xss', 'traversal']}
  onSelect={(payload) => insertPayload(payload)}
/>
```

## Development Setup

### Prerequisites

- Node.js 18+ or 20+
- pnpm package manager
- Caido desktop application
- redstr installed (for CLI mode) or HTTP server running

### Installation

```bash
# Clone plugin template
pnpm create @caido-community/plugin

# Install dependencies
cd redstr-caido-plugin
pnpm install

# Build plugin
pnpm build

# Watch mode for development
pnpm watch
```

### Loading in Caido

1. Open Caido
2. Install Devtools plugin from Community Store
3. Navigate to Extensions
4. Click "Load from folder"
5. Select plugin directory
6. Enable hot-reload in Devtools

## Testing

### Unit Tests

```typescript
import { describe, it, expect } from 'vitest';
import { redstr } from './transforms';

describe('redstr transforms', () => {
  it('should obfuscate SQL injection', async () => {
    const input = "SELECT * FROM users";
    const output = await redstr.sqlCommentInjection(input);
    expect(output).toContain('/**/');
  });
});
```

### Integration Tests

Test with real Caido SDK:

```typescript
import { mockSDK } from '@caido/test-utils';

describe('Plugin integration', () => {
  it('should register context menu', () => {
    const sdk = mockSDK();
    init(sdk);
    expect(sdk.contextMenu.items).toContainEqual({
      label: 'redstr: Transform',
    });
  });
});
```

## Security Considerations

### Authorization

- Ensure proper authorization for security testing
- Log all transformation operations
- Implement rate limiting for API calls

### Data Privacy

- Don't log sensitive data
- Clear transformation history on exit
- Respect Caido's privacy settings

### Performance

- Cache transformation results
- Implement request throttling
- Use worker threads for heavy operations

## References

- [Caido Developer Documentation](https://developer.caido.io/)
- [Caido SDK Reference](https://deepwiki.com/caido/sdk-js)
- [Plugin Examples](https://github.com/caido/examples)
- [Community Plugins](https://caido.io/plugins)
- [redstr Documentation](../README.md)

## Roadmap

- [x] Research Caido extension API
- [x] Design integration architecture
- [ ] Implement plugin structure
- [ ] Create UI components
- [ ] Add workflow nodes
- [ ] Write tests
- [ ] Publish to Community Store
- [ ] Create video tutorial

## Support

For integration support:
- Join Caido Discord
- Check developer documentation
- Review example plugins
- File issues on GitHub

---

**Note:** This integration is designed for authorized security testing. Users must obtain proper authorization before conducting any security assessments.
