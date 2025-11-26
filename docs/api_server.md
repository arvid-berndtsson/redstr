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
# Clone and build from source
git clone https://github.com/arvid-berndtsson/redstr-server
cd redstr-server
cargo build --release

# The binary will be at target/release/redstr-server
```

### Running the Server

```bash
# Run from source
cargo run --release

# Or run the compiled binary
./target/release/redstr-server

# With custom port (via PORT environment variable)
PORT=3000 cargo run --release

# With custom log level
RUST_LOG=debug cargo run --release
```

The server will listen on `http://0.0.0.0:8080` by default (or the port specified by the `PORT` environment variable).

**Note:** The server binds to `0.0.0.0` (all interfaces) for Railway and cloud deployment compatibility. For local development only, consider using a reverse proxy or firewall rules for added security.

## API Reference

### Base URL

```
http://localhost:8080
```

### Endpoints

#### GET /

Get server information and available endpoints.

**Response:**

```json
{
  "service": "redstr-server",
  "version": "0.2.0",
  "endpoints": ["/transform", "/batch", "/functions", "/health", "/version"]
}
```

#### POST /transform

Transform a string using a specific transformation function.

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

#### POST /batch

Apply multiple transformations (each with its own function and input).

**Request:**

```json
{
  "transforms": [
    {"function": "leetspeak", "input": "Hello"},
    {"function": "base64_encode", "input": "World"}
  ]
}
```

**Response:**

```json
{
  "results": [
    {"output": "H3ll0"},
    {"output": "V29ybGQ="}
  ]
}
```

**Note:** For chained transformations (applying multiple functions to the same input), call `/transform` multiple times with the output of each call, or use the batch endpoint with intermediate results.

#### GET /functions

List all available transformation functions.

**Response:**

```json
{
  "functions": [
    "leetspeak",
    "base64_encode",
    "url_encode",
    "homoglyph_substitution",
    "case_swap",
    "rot13",
    ...
  ],
  "count": 62
}
```

#### GET /health

Health check endpoint for load balancers and monitoring.

**Response:**

```json
{
  "status": "healthy"
}
```

#### GET /version

Get detailed version information.

**Response:**

```json
{
  "service": "redstr-server",
  "version": "0.2.0",
  "redstr_version": "0.2.0"
}
```

## Client Examples

### JavaScript/TypeScript (Node.js)

```javascript
const axios = require('axios');

const REDSTR_API = 'http://localhost:8080';

async function transform(func, input) {
  try {
    const response = await axios.post(`${REDSTR_API}/transform`, {
      function: func,
      input: input
    });
    return response.data.output;
  } catch (error) {
    if (error.response) {
      throw new Error(error.response.data.error);
    }
    throw error;
  }
}

async function batchTransforms(transforms) {
  try {
    const response = await axios.post(`${REDSTR_API}/batch`, {
      transforms: transforms
    });
    return response.data.results;
  } catch (error) {
    if (error.response) {
      throw new Error(error.response.data.error);
    }
    throw error;
  }
}

// Usage
(async () => {
  // Single transformation
  const leetspeak = await transform('leetspeak', 'password');
  console.log(leetspeak); // p@55w0rd
  
  // Batch transformations
  const results = await batchTransforms([
    {function: 'leetspeak', input: 'Hello'},
    {function: 'base64_encode', input: 'World'}
  ]);
  console.log(results); // [{output: 'H3ll0'}, {output: 'V29ybGQ='}]
  
  // Chained transformations (manual)
  let text = 'admin@example.com';
  text = await transform('homoglyph_substitution', text);
  text = await transform('url_encode', text);
  console.log(text);
})();
```

### JavaScript (Browser/Fetch)

```javascript
const REDSTR_API = 'http://localhost:8080';

async function transform(func, input) {
  const response = await fetch(`${REDSTR_API}/transform`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ 
      function: func,
      input: input
    })
  });
  
  if (!response.ok) {
    const error = await response.json();
    throw new Error(error.error || 'Transform failed');
  }
  
  const data = await response.json();
  return data.output;
}

// Usage
transform('leetspeak', 'password').then(console.log);

// With error handling
transform('leetspeak', 'password')
  .then(result => console.log(result))
  .catch(error => console.error('Error:', error.message));
```

### Python

```python
import requests

REDSTR_API = 'http://localhost:8080'

def transform(function, input_str):
    """Transform a string using a redstr function."""
    response = requests.post(f'{REDSTR_API}/transform', json={
        'function': function,
        'input': input_str
    })
    response.raise_for_status()  # Raises HTTPError for bad status codes
    data = response.json()
    return data.get('output', '')

def batch_transforms(transforms):
    """Apply multiple transformations in a batch."""
    response = requests.post(f'{REDSTR_API}/batch', json={
        'transforms': transforms
    })
    response.raise_for_status()
    data = response.json()
    return data.get('results', [])

# Usage
try:
    # Single transformation
    leetspeak = transform('leetspeak', 'password')
    print(leetspeak)  # p@55w0rd
    
    # Batch transformations
    results = batch_transforms([
        {'function': 'leetspeak', 'input': 'Hello'},
        {'function': 'base64_encode', 'input': 'World'}
    ])
    for result in results:
        print(result['output'])
    
    # Chained transformations (manual)
    text = 'admin@example.com'
    text = transform('homoglyph_substitution', text)
    text = transform('url_encode', text)
    print(text)
    
except requests.exceptions.RequestException as e:
    print(f"Error: {e}")
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
    Function string `json:"function"`
    Input    string `json:"input"`
}

type TransformResponse struct {
    Output string `json:"output"`
}

type ErrorResponse struct {
    Error string `json:"error"`
}

type BatchRequest struct {
    Transforms []TransformRequest `json:"transforms"`
}

type BatchResponse struct {
    Results []TransformResponse `json:"results"`
}

func Transform(function, input string) (string, error) {
    reqBody, err := json.Marshal(TransformRequest{
        Function: function,
        Input:    input,
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
    
    if resp.StatusCode != http.StatusOK {
        var errResp ErrorResponse
        if err := json.Unmarshal(body, &errResp); err == nil {
            return "", fmt.Errorf("transform failed: %s", errResp.Error)
        }
        return "", fmt.Errorf("request failed with status: %d", resp.StatusCode)
    }
    
    var result TransformResponse
    if err := json.Unmarshal(body, &result); err != nil {
        return "", fmt.Errorf("failed to unmarshal response: %w", err)
    }
    
    return result.Output, nil
}

func BatchTransform(transforms []TransformRequest) ([]string, error) {
    reqBody, err := json.Marshal(BatchRequest{Transforms: transforms})
    if err != nil {
        return nil, fmt.Errorf("failed to marshal request: %w", err)
    }
    
    resp, err := http.Post(
        RedstrAPI+"/batch",
        "application/json",
        bytes.NewBuffer(reqBody),
    )
    if err != nil {
        return nil, fmt.Errorf("failed to make request: %w", err)
    }
    defer resp.Body.Close()
    
    body, err := io.ReadAll(resp.Body)
    if err != nil {
        return nil, fmt.Errorf("failed to read response: %w", err)
    }
    
    if resp.StatusCode != http.StatusOK {
        var errResp ErrorResponse
        if err := json.Unmarshal(body, &errResp); err == nil {
            return nil, fmt.Errorf("batch transform failed: %s", errResp.Error)
        }
        return nil, fmt.Errorf("request failed with status: %d", resp.StatusCode)
    }
    
    var result BatchResponse
    if err := json.Unmarshal(body, &result); err != nil {
        return nil, fmt.Errorf("failed to unmarshal response: %w", err)
    }
    
    outputs := make([]string, len(result.Results))
    for i, r := range result.Results {
        outputs[i] = r.Output
    }
    
    return outputs, nil
}

func main() {
    // Single transformation
    output, err := Transform("leetspeak", "password")
    if err != nil {
        log.Fatalf("Transform failed: %v", err)
    }
    fmt.Println(output) // p@55w0rd
    
    // Batch transformations
    results, err := BatchTransform([]TransformRequest{
        {Function: "leetspeak", Input: "Hello"},
        {Function: "base64_encode", Input: "World"},
    })
    if err != nil {
        log.Fatalf("Batch transform failed: %v", err)
    }
    for _, result := range results {
        fmt.Println(result)
    }
}
```

### Ruby

```ruby
require 'net/http'
require 'json'
require 'uri'

REDSTR_API = 'http://localhost:8080'

def transform(function, input)
  uri = URI("#{REDSTR_API}/transform")
  request = Net::HTTP::Post.new(uri, 'Content-Type' => 'application/json')
  request.body = { function: function, input: input }.to_json
  
  response = Net::HTTP.start(uri.hostname, uri.port) do |http|
    http.request(request)
  end
  
  if response.code.to_i != 200
    error_data = JSON.parse(response.body)
    raise "Transform failed: #{error_data['error']}"
  end
  
  JSON.parse(response.body)['output']
end

def batch_transform(transforms)
  uri = URI("#{REDSTR_API}/batch")
  request = Net::HTTP::Post.new(uri, 'Content-Type' => 'application/json')
  request.body = { transforms: transforms }.to_json
  
  response = Net::HTTP.start(uri.hostname, uri.port) do |http|
    http.request(request)
  end
  
  if response.code.to_i != 200
    error_data = JSON.parse(response.body)
    raise "Batch transform failed: #{error_data['error']}"
  end
  
  JSON.parse(response.body)['results']
end

# Usage
begin
  puts transform('leetspeak', 'password')
  
  results = batch_transform([
    { function: 'leetspeak', input: 'Hello' },
    { function: 'base64_encode', input: 'World' }
  ])
  results.each { |r| puts r['output'] }
rescue StandardError => e
  puts "Error: #{e.message}"
end
```

### cURL

```bash
# Get server info
curl http://localhost:8080/

# List all available functions
curl http://localhost:8080/functions

# Health check
curl http://localhost:8080/health

# Version information
curl http://localhost:8080/version

# Simple transformation
curl -X POST http://localhost:8080/transform \
  -H "Content-Type: application/json" \
  -d '{"function": "leetspeak", "input": "password"}'

# Batch transformations
curl -X POST http://localhost:8080/batch \
  -H "Content-Type: application/json" \
  -d '{
    "transforms": [
      {"function": "leetspeak", "input": "Hello"},
      {"function": "base64_encode", "input": "World"}
    ]
  }'

# URL encoding
curl -X POST http://localhost:8080/transform \
  -H "Content-Type: application/json" \
  -d '{"function": "url_encode", "input": "hello world"}'

# SQL injection testing
curl -X POST http://localhost:8080/transform \
  -H "Content-Type: application/json" \
  -d '{"function": "sql_comment_injection", "input": "SELECT * FROM users"}'

# Domain typosquatting
curl -X POST http://localhost:8080/transform \
  -H "Content-Type: application/json" \
  -d '{"function": "domain_typosquat", "input": "example.com"}'
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
