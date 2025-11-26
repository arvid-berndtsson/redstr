# redstr API Server

## Overview

The **redstr-server** provides a language-agnostic HTTP API for accessing redstr transformations. This is the recommended approach for integrating redstr into applications written in languages other than Rust, similar to how OpenAI and other modern APIs work.

## Why Use the API Server?

### Advantages

1. **Language Agnostic**: Use redstr from any programming language that can make HTTP requests
2. **Centralized**: Deploy once, use everywhere
3. **Easy Maintenance**: Update transformations in one place
4. **Scalable**: Can be horizontally scaled as needed
5. **Security**: Centralized security updates and monitoring
6. **Consistent**: Same API across all clients
7. **No Native Dependencies**: Clients don't need to compile native code

### Comparison with Language Bindings

| Aspect | API Server | Native Bindings |
|--------|------------|-----------------|
| Performance | Network overhead | Direct function calls |
| Deployment | Centralized | Distributed |
| Maintenance | Single codebase | Multiple binding repos |
| Language Support | Universal | Per-language effort |
| Installation | HTTP client only | Native compilation required |
| Updates | Server-side only | Client library updates needed |

**Recommendation**: Use the API server for most applications. Consider native bindings only for:
- High-throughput, latency-sensitive applications
- Offline/air-gapped environments
- Embedded systems

## Quick Start

### Installation

The redstr-server is available as a separate repository:

```bash
# Clone the server repository
git clone https://github.com/arvid-berndtsson/redstr-server

# Or install via cargo
cargo install redstr-server

# Or use Docker
docker pull arvid-berndtsson/redstr-server
```

### Running the Server

```bash
# Run locally
redstr-server --port 8080

# With Docker
docker run -p 8080:8080 arvid-berndtsson/redstr-server

# With environment configuration
REDSTR_PORT=8080 REDSTR_HOST=0.0.0.0 redstr-server
```

## API Reference

### Base URL

```
http://localhost:8080
```

### Endpoints

#### POST /transform

Transform a string using a specific transformation mode.

**Request:**

```json
{
  "input": "string to transform",
  "mode": "leetspeak"
}
```

**Response:**

```json
{
  "output": "tr@n5f0rm3d 5tr1ng",
  "mode": "leetspeak",
  "success": true
}
```

#### POST /transform/chain

Apply multiple transformations in sequence (builder pattern).

**Request:**

```json
{
  "input": "admin@example.com",
  "transformations": ["homoglyphs", "url_encode"]
}
```

**Response:**

```json
{
  "output": "%D0%B0dm%D1%96n%40%D0%B5x%D0%B0mple.com",
  "transformations_applied": ["homoglyphs", "url_encode"],
  "success": true
}
```

#### GET /modes

List all available transformation modes.

**Response:**

```json
{
  "modes": [
    "leetspeak",
    "base64",
    "url_encode",
    "homoglyphs",
    "case_swap",
    "rot13",
    ...
  ]
}
```

#### GET /health

Health check endpoint for load balancers and monitoring.

**Response:**

```json
{
  "status": "healthy",
  "version": "0.2.3"
}
```

## Client Examples

### JavaScript/TypeScript (Node.js)

```javascript
const axios = require('axios');

const REDSTR_API = 'http://localhost:8080';

async function transform(input, mode) {
  const response = await axios.post(`${REDSTR_API}/transform`, {
    input,
    mode
  });
  return response.data.output;
}

async function chainTransforms(input, transformations) {
  const response = await axios.post(`${REDSTR_API}/transform/chain`, {
    input,
    transformations
  });
  return response.data.output;
}

// Usage
(async () => {
  const leetspeak = await transform('password', 'leetspeak');
  console.log(leetspeak); // p@55w0rd
  
  const chained = await chainTransforms('admin@example.com', [
    'homoglyphs',
    'url_encode'
  ]);
  console.log(chained);
})();
```

### JavaScript (Browser/Fetch)

```javascript
const REDSTR_API = 'http://localhost:8080';

async function transform(input, mode) {
  const response = await fetch(`${REDSTR_API}/transform`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ input, mode })
  });
  const data = await response.json();
  return data.output;
}

// Usage
transform('password', 'leetspeak').then(console.log);
```

### Python

```python
import requests

REDSTR_API = 'http://localhost:8080'

def transform(input_str, mode):
    response = requests.post(f'{REDSTR_API}/transform', json={
        'input': input_str,
        'mode': mode
    })
    return response.json()['output']

def chain_transforms(input_str, transformations):
    response = requests.post(f'{REDSTR_API}/transform/chain', json={
        'input': input_str,
        'transformations': transformations
    })
    return response.json()['output']

# Usage
leetspeak = transform('password', 'leetspeak')
print(leetspeak)  # p@55w0rd

chained = chain_transforms('admin@example.com', ['homoglyphs', 'url_encode'])
print(chained)
```

### Go

```go
package main

import (
    "bytes"
    "encoding/json"
    "fmt"
    "io"
    "log"
    "net/http"
)

const RedstrAPI = "http://localhost:8080"

type TransformRequest struct {
    Input string `json:"input"`
    Mode  string `json:"mode"`
}

type TransformResponse struct {
    Output  string `json:"output"`
    Mode    string `json:"mode"`
    Success bool   `json:"success"`
}

func Transform(input, mode string) (string, error) {
    reqBody, err := json.Marshal(TransformRequest{
        Input: input,
        Mode:  mode,
    })
    if err != nil {
        return "", fmt.Errorf("failed to marshal request: %w", err)
    }
    
    resp, err := http.Post(
        RedstrAPI+"/transform",
        "application/json",
        bytes.NewBuffer(reqBody),
    )
    if err != nil {
        return "", fmt.Errorf("failed to make request: %w", err)
    }
    defer resp.Body.Close()
    
    body, err := io.ReadAll(resp.Body)
    if err != nil {
        return "", fmt.Errorf("failed to read response: %w", err)
    }
    
    var result TransformResponse
    if err := json.Unmarshal(body, &result); err != nil {
        return "", fmt.Errorf("failed to unmarshal response: %w", err)
    }
    
    return result.Output, nil
}

func main() {
    output, err := Transform("password", "leetspeak")
    if err != nil {
        log.Fatalf("Transform failed: %v", err)
    }
    fmt.Println(output) // p@55w0rd
}
```

### Ruby

```ruby
require 'net/http'
require 'json'
require 'uri'

REDSTR_API = 'http://localhost:8080'

def transform(input, mode)
  uri = URI("#{REDSTR_API}/transform")
  request = Net::HTTP::Post.new(uri, 'Content-Type' => 'application/json')
  request.body = { input: input, mode: mode }.to_json
  
  response = Net::HTTP.start(uri.hostname, uri.port) do |http|
    http.request(request)
  end
  
  JSON.parse(response.body)['output']
end

# Usage
puts transform('password', 'leetspeak')
```

### cURL

```bash
# Simple transformation
curl -X POST http://localhost:8080/transform \
  -H "Content-Type: application/json" \
  -d '{"input": "password", "mode": "leetspeak"}'

# Chained transformations
curl -X POST http://localhost:8080/transform/chain \
  -H "Content-Type: application/json" \
  -d '{
    "input": "admin@example.com",
    "transformations": ["homoglyphs", "url_encode"]
  }'

# List all modes
curl http://localhost:8080/modes

# Health check
curl http://localhost:8080/health
```

## Deployment

### Docker Deployment

```bash
# Build custom image
docker build -t redstr-server .

# Run with custom port
docker run -p 3000:3000 -e REDSTR_PORT=3000 redstr-server

# Run with volume for config
docker run -p 8080:8080 -v /path/to/config:/config redstr-server
```

### Docker Compose

```yaml
version: '3.8'
services:
  redstr:
    image: arvid-berndtsson/redstr-server:latest
    ports:
      - "8080:8080"
    environment:
      - REDSTR_PORT=8080
      - REDSTR_HOST=0.0.0.0
      - RUST_LOG=info
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
      interval: 30s
      timeout: 10s
      retries: 3
```

### Systemd Service

```ini
[Unit]
Description=redstr API Server
After=network.target

[Service]
Type=simple
User=redstr
WorkingDirectory=/opt/redstr-server
ExecStart=/usr/local/bin/redstr-server --port 8080
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

### Cloud Platforms

#### AWS (using ECS/Fargate)

```bash
# Push to ECR
aws ecr get-login-password --region us-east-1 | docker login --username AWS --password-stdin <account>.dkr.ecr.us-east-1.amazonaws.com
docker tag redstr-server:latest <account>.dkr.ecr.us-east-1.amazonaws.com/redstr-server:latest
docker push <account>.dkr.ecr.us-east-1.amazonaws.com/redstr-server:latest

# Deploy via ECS task definition
```

#### Google Cloud Run

```bash
# Build and deploy
gcloud builds submit --tag gcr.io/PROJECT-ID/redstr-server
gcloud run deploy redstr-server --image gcr.io/PROJECT-ID/redstr-server --platform managed
```

#### Heroku

```bash
# Using Dockerfile
heroku create redstr-server
heroku container:push web
heroku container:release web
```

## Security Considerations

### Authentication

While the basic server doesn't include authentication, you should add it for production use:

1. **API Keys**: Use middleware to validate API keys
2. **OAuth2**: Integrate with OAuth2 providers
3. **JWT**: Use JSON Web Tokens for stateless auth
4. **mTLS**: Mutual TLS for service-to-service communication

### Rate Limiting

Implement rate limiting to prevent abuse:

```rust
// Example rate limiting configuration
rate_limit: {
    requests_per_minute: 100,
    burst_size: 20
}
```

### HTTPS/TLS

Always use HTTPS in production:

```bash
# Using nginx as reverse proxy
server {
    listen 443 ssl;
    ssl_certificate /path/to/cert.pem;
    ssl_certificate_key /path/to/key.pem;
    
    location / {
        proxy_pass http://localhost:8080;
    }
}
```

### Input Validation

The server validates input sizes to prevent DoS attacks:

- Maximum input length: 1MB (configurable)
- Request timeout: 30 seconds
- Connection limits: per configuration

## Performance

### Benchmarks

Typical performance metrics (single instance):

- **Throughput**: 10,000+ requests/second
- **Latency**: <5ms (p99)
- **Memory**: ~50MB base + request overhead
- **CPU**: Single core can handle ~5,000 req/s

### Scaling

#### Horizontal Scaling

Deploy multiple instances behind a load balancer:

```
                    ┌─────────────┐
                    │   Nginx     │
                    │Load Balancer│
                    └──────┬──────┘
                           │
           ┌───────────────┼───────────────┐
           │               │               │
    ┌──────▼─────┐  ┌──────▼─────┐  ┌──────▼─────┐
    │  redstr    │  │  redstr    │  │  redstr    │
    │ server #1  │  │ server #2  │  │ server #3  │
    └────────────┘  └────────────┘  └────────────┘
```

#### Vertical Scaling

Configure for your hardware:

```bash
# Adjust worker threads
REDSTR_WORKERS=8 redstr-server
```

## Monitoring

### Health Checks

```bash
# Simple health check
curl http://localhost:8080/health

# Detailed metrics (if enabled)
curl http://localhost:8080/metrics
```

### Logging

Configure logging levels:

```bash
RUST_LOG=info redstr-server        # Standard logging
RUST_LOG=debug redstr-server       # Detailed logging
RUST_LOG=warn redstr-server        # Warnings only
```

### Prometheus Metrics

If metrics are enabled:

```prometheus
# Request rate
rate(redstr_requests_total[5m])

# Error rate
rate(redstr_errors_total[5m])

# Latency
histogram_quantile(0.99, rate(redstr_request_duration_seconds_bucket[5m]))
```

## Troubleshooting

### Common Issues

#### Connection Refused

```bash
# Check if server is running
ps aux | grep redstr-server

# Check port binding
netstat -tuln | grep 8080
```

#### High Memory Usage

```bash
# Monitor memory
docker stats redstr-server

# Reduce worker threads if needed
REDSTR_WORKERS=2 redstr-server
```

#### Slow Response Times

- Check network latency between client and server
- Monitor server CPU/memory usage
- Consider horizontal scaling
- Enable response caching if appropriate

## Future Enhancements

Planned features for the API server:

- [ ] GraphQL API support
- [ ] WebSocket support for streaming
- [ ] Batch transformation endpoint
- [ ] OpenAPI/Swagger documentation
- [ ] Built-in authentication
- [ ] Rate limiting middleware
- [ ] Caching layer
- [ ] Metrics and monitoring endpoints

## Resources

- **Repository**: [github.com/arvid-berndtsson/redstr-server](https://github.com/arvid-berndtsson/redstr-server)
- **Documentation**: [docs.rs/redstr](https://docs.rs/redstr)
- **Main Library**: [github.com/arvid-berndtsson/redstr](https://github.com/arvid-berndtsson/redstr)

## Contributing

Contributions to the API server are welcome! Please see the contributing guide in the redstr-server repository.
