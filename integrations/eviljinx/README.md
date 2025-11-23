# EvilJinx Integration for redstr

Integration between redstr and EvilJinx phishing framework for enhanced evasion capabilities.

## Overview

This integration provides EvilJinx operators with redstr's string transformation capabilities to improve phishing campaign effectiveness and evade detection.

## Prerequisites

- EvilJinx installed and configured
- redstr HTTP server running (see `../http-server/`)
- Go 1.18+ (for Go integration examples)
- Bash/Python (for script examples)

## Installation

### Option 1: HTTP API Integration (Recommended)

1. Start the redstr HTTP server:
```bash
cd ../http-server
cargo run --release
```

2. Use the provided helper scripts to integrate with EvilJinx workflows.

### Option 2: CLI Integration

1. Install redstr CLI:
```bash
cargo install --path ../.. --features cli
```

2. Use redstr commands directly in your EvilJinx scripts.

## Integration Scripts

### Domain Generation Script

`scripts/generate_domains.sh`:
```bash
#!/bin/bash
# Generate typosquatted domains for phishing

TARGET_DOMAIN="$1"

if [ -z "$TARGET_DOMAIN" ]; then
    echo "Usage: $0 <target-domain>"
    exit 1
fi

# Call redstr HTTP API
RESULT=$(curl -s -X POST http://localhost:8080/transform \
    -H "Content-Type: application/json" \
    -d "{\"function\":\"domain_typosquat\",\"input\":\"$TARGET_DOMAIN\"}")

echo "$RESULT" | jq -r '.output'
```

### Email Obfuscation Script

`scripts/obfuscate_email.sh`:
```bash
#!/bin/bash
# Obfuscate email addresses for phishing campaigns

EMAIL="$1"

if [ -z "$EMAIL" ]; then
    echo "Usage: $0 <email-address>"
    exit 1
fi

curl -s -X POST http://localhost:8080/transform \
    -H "Content-Type: application/json" \
    -d "{\"function\":\"email_obfuscation\",\"input\":\"$EMAIL\"}" \
    | jq -r '.output'
```

### Template Transformation Script

`scripts/transform_template.py`:
```python
#!/usr/bin/env python3
"""
Transform EvilJinx phishlet templates with redstr obfuscation
"""
import sys
import json
import requests

def transform_text(text, method):
    """Transform text using redstr API"""
    url = "http://localhost:8080/transform"
    payload = {"function": method, "input": text}
    response = requests.post(url, json=payload)
    return response.json()["output"]

def process_phishlet(phishlet_path):
    """Process phishlet file and obfuscate JavaScript"""
    with open(phishlet_path, 'r') as f:
        content = f.read()
    
    # Find and obfuscate JavaScript sections
    # This is a simplified example
    obfuscated = transform_text(content, "js_string_concat")
    
    return obfuscated

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print(f"Usage: {sys.argv[0]} <phishlet-file>")
        sys.exit(1)
    
    result = process_phishlet(sys.argv[1])
    print(result)
```

## Usage Examples

### Example 1: Generate Phishing Domains

```bash
# Generate typosquatted domain variations
./scripts/generate_domains.sh "microsoft.com"

# Output examples:
# m1crosoft.com
# micr0soft.com
# microsof7.com
# mïcrosoft.com (homoglyph)
```

### Example 2: Create Obfuscated Phishing Emails

```bash
# Obfuscate sender email
./scripts/obfuscate_email.sh "security@microsoft.com"

# Output:
# s&#101;curity&#64;microso&#102;t&#46;com
```

### Example 3: Transform Phishlet Templates

```python
import requests

def enhance_phishlet(phishlet_content):
    """Add obfuscation to phishlet"""
    
    # Obfuscate JavaScript
    js_obfuscated = requests.post('http://localhost:8080/transform', json={
        'function': 'js_string_concat',
        'input': phishlet_content
    }).json()['output']
    
    # Add Cloudflare evasion
    cf_enhanced = requests.post('http://localhost:8080/transform', json={
        'function': 'cloudflare_challenge_response',
        'input': js_obfuscated
    }).json()['output']
    
    return cf_enhanced
```

### Example 4: Automated Campaign Setup

```bash
#!/bin/bash
# Automated phishing campaign setup with redstr integration

LEGIT_DOMAIN="example.com"
CAMPAIGN_NAME="test-campaign"

echo "Setting up campaign: $CAMPAIGN_NAME"

# 1. Generate phishing domain
PHISH_DOMAIN=$(./scripts/generate_domains.sh "$LEGIT_DOMAIN" | head -1)
echo "Phishing domain: $PHISH_DOMAIN"

# 2. Obfuscate sender email
SENDER=$(./scripts/obfuscate_email.sh "security@$LEGIT_DOMAIN")
echo "Sender email: $SENDER"

# 3. Configure EvilJinx
# (EvilJinx-specific commands would go here)

echo "Campaign setup complete!"
```

## Use Cases

### 1. Domain Generation for Campaigns

Generate convincing typosquatted domains:
- Character substitution (1 → l, 0 → o)
- Character omission
- Character addition
- Homoglyph substitution (Latin → Cyrillic)

### 2. Email Content Obfuscation

Obfuscate phishing emails to bypass spam filters:
- HTML entity encoding
- Unicode variations
- Mixed encoding techniques

### 3. JavaScript Obfuscation

Protect phishing page JavaScript from analysis:
- String concatenation
- Variable name obfuscation
- Function call obfuscation

### 4. Cloudflare Challenge Evasion

Generate challenge responses to evade bot detection:
- Turnstile variation
- TLS fingerprint randomization
- Canvas fingerprint variation

### 5. Session Token Manipulation

Obfuscate captured session tokens:
- JWT header manipulation
- Token variation
- Payload obfuscation

## Go Integration Example

For direct Go integration with EvilJinx:

```go
package main

import (
    "bytes"
    "encoding/json"
    "fmt"
    "net/http"
)

type TransformRequest struct {
    Function string `json:"function"`
    Input    string `json:"input"`
}

type TransformResponse struct {
    Output string `json:"output"`
}

func redstrTransform(function, input string) (string, error) {
    url := "http://localhost:8080/transform"
    
    reqBody, _ := json.Marshal(TransformRequest{
        Function: function,
        Input:    input,
    })
    
    resp, err := http.Post(url, "application/json", bytes.NewBuffer(reqBody))
    if err != nil {
        return "", err
    }
    defer resp.Body.Close()
    
    var result TransformResponse
    if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
        return "", err
    }
    
    return result.Output, nil
}

func main() {
    // Generate typosquatted domain
    domain, _ := redstrTransform("domain_typosquat", "example.com")
    fmt.Printf("Phishing domain: %s\n", domain)
    
    // Obfuscate email
    email, _ := redstrTransform("email_obfuscation", "admin@example.com")
    fmt.Printf("Obfuscated email: %s\n", email)
    
    // Generate JavaScript obfuscation
    js, _ := redstrTransform("js_string_concat", "alert('phishing')")
    fmt.Printf("Obfuscated JS: %s\n", js)
}
```

## Configuration

Create a configuration file for your integration:

`config.json`:
```json
{
  "redstr_api_url": "http://localhost:8080",
  "default_transformations": {
    "domain": "domain_typosquat",
    "email": "email_obfuscation",
    "javascript": "js_string_concat"
  },
  "cloudflare_evasion": true,
  "logging": {
    "enabled": true,
    "log_file": "eviljinx-redstr.log"
  }
}
```

## Security Considerations

### Authorized Use Only

This integration is strictly for:
- Authorized red team engagements
- Penetration testing with written permission
- Security awareness training
- Defensive security research

### Operational Security

- Deploy on isolated infrastructure
- Use proper logging and auditing
- Maintain engagement documentation
- Follow responsible disclosure practices
- Rotate techniques regularly

### Detection Avoidance

- Combine multiple transformation methods
- Monitor for detection signatures
- Update patterns based on campaign feedback
- Use A/B testing for effectiveness

## Troubleshooting

**API connection fails:**
- Ensure redstr HTTP server is running
- Check firewall settings
- Verify API URL in scripts

**Transformations produce unexpected results:**
- Test transformations individually
- Check input format
- Verify function name spelling

**EvilJinx doesn't accept transformed domains:**
- Validate domain format
- Check DNS configuration
- Ensure TLD is valid

## References

- [EvilJinx Documentation](../../docs/eviljinx_integration.md)
- [redstr HTTP Server](../http-server/README.md)
- [EvilJinx Official Docs](https://help.evilginx.com/)

## Contributing

Contributions are welcome! Please submit issues and pull requests for:
- New integration scripts
- Improved automation
- Additional use cases
- Bug fixes

## License

MIT License - See LICENSE file in repository root.

---

**Important:** This integration is provided for legitimate security testing only. Users must obtain proper authorization before conducting any phishing campaigns or security assessments.
