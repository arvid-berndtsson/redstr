# Burp Suite Extension for redstr

A Burp Suite extension that integrates redstr's string transformation capabilities into Burp Suite's testing workflow.

## Overview

This extension enables penetration testers to use redstr transformations within Burp Suite for:
- Intruder payload generation
- Request/response transformation
- Scanner integration
- Context menu actions
- Custom tab for batch transformations

## Prerequisites

- Burp Suite Professional or Community Edition
- Java 17 (for Montoya API) or Jython (for Python)
- redstr HTTP server running
- Maven or Gradle (for Java extension)

## Installation

### 1. Start redstr HTTP Server

```bash
cd ../http-server
cargo run --release
```

The server should be running at `http://localhost:8080`.

### 2. Install Extension (Coming Soon)

The Burp Suite extension implementation is planned. The structure will include:

```
burp/
├── pom.xml                    # Maven build
├── src/
│   └── main/
│       └── java/
│           └── com/
│               └── redstr/
│                   └── burp/
│                       ├── RedstrExtension.java
│                       ├── RedstrHttpClient.java
│                       ├── RedstrPayloadGenerator.java
│                       ├── RedstrContextMenu.java
│                       ├── RedstrTab.java
│                       └── RedstrScanner.java
└── README.md
```

## Planned Features

### 1. Intruder Integration

Generate payloads using redstr transformations:
- SQL injection variations
- XSS payload obfuscation
- Command injection patterns
- Encoding chains

### 2. Context Menu

Right-click menu items for quick transformations:
- Transform selection with redstr
- Generate payload variations
- Encode/decode operations
- Send to redstr tab

### 3. Scanner Integration

Custom scan checks using redstr:
- WAF bypass detection
- Advanced injection testing
- Obfuscated payload scanning

### 4. Custom Tab

Dedicated tab for batch operations:
- Input/output text areas
- Method selector dropdown
- Transform button
- History of transformations

### 5. Repeater Enhancement

Quick transformation actions in Repeater:
- Transform request body
- Encode parameters
- Generate variations

## Usage Examples (Planned)

### Intruder Payload Generator

```java
// Configure Intruder to use redstr payload generator
Intruder → Positions → Payload Sets → Generator → redstr
Select method: sql_comment_injection
```

### Context Menu

```
Right-click on request/response → redstr → SQL Injection
Right-click on parameter → redstr → XSS Variation
Right-click on header → redstr → Encode
```

### Custom Tab

```
1. Open redstr tab in Burp
2. Enter input text
3. Select transformation method
4. Click Transform
5. Copy output to clipboard or send to Repeater
```

## API Integration

The extension communicates with the redstr HTTP server:

```java
public class RedstrHttpClient {
    private final String apiUrl = "http://localhost:8080";
    
    public String transform(String method, String input) throws IOException {
        HttpRequest request = HttpRequest.newBuilder()
            .uri(URI.create(apiUrl + "/transform"))
            .header("Content-Type", "application/json")
            .POST(BodyPublishers.ofString(
                String.format("{\"function\":\"%s\",\"input\":\"%s\"}", 
                    method, escapeJson(input))
            ))
            .build();
        
        HttpResponse<String> response = httpClient.send(request, 
            BodyHandlers.ofString());
        
        return parseResponse(response.body());
    }
}
```

## Building

### Java Extension (Maven)

```bash
cd burp
mvn clean package
```

Output: `target/redstr-burp.jar`

### Loading Extension

1. Open Burp Suite
2. Go to Extensions tab
3. Click "Add"
4. Select "Java" as extension type
5. Choose `redstr-burp.jar`
6. Click "Next"

## Configuration

Extension settings will include:
- API endpoint URL
- Request timeout
- Default transformation methods
- Logging options

## Use Cases

### 1. SQL Injection Testing

```
Input: SELECT * FROM users WHERE id=1
Method: sql_comment_injection
Output: SELECT/**/FROM/**/users/**/WHERE/**/id=1
```

### 2. XSS with WAF Bypass

```
Input: <script>alert('XSS')</script>
Chain: xss_tag_variations → html_entity_encode → mixed_encoding
Output: Highly obfuscated XSS payload
```

### 3. JWT Manipulation

```
Input: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
Methods: 
  - jwt_algorithm_confusion
  - jwt_header_manipulation
  - jwt_signature_bypass
```

### 4. Phishing Domain Testing

```
Input: legitimate-bank.com
Methods:
  - domain_typosquat
  - homoglyph_substitution
  - advanced_domain_spoof
```

### 5. API Fuzzing

```
Use Intruder with redstr payload generator:
1. Mark insertion points
2. Select payload type: redstr
3. Configure methods: sql, xss, command injection
4. Start attack
```

## Python Extension (Alternative)

A simpler Python extension using the Extender API:

```python
from burp import IBurpExtender, IIntruderPayloadGenerator
import urllib2
import json

class BurpExtender(IBurpExtender, IIntruderPayloadGenerator):
    def registerExtenderCallbacks(self, callbacks):
        callbacks.setExtensionName("redstr Integration")
        callbacks.registerIntruderPayloadGeneratorFactory(self)
        self.api_url = "http://localhost:8080"
    
    def transform(self, method, input_str):
        data = json.dumps({"function": method, "input": input_str})
        req = urllib2.Request(self.api_url + "/transform", data)
        response = urllib2.urlopen(req)
        return json.loads(response.read())["output"]
```

## Troubleshooting

### Extension Won't Load

**Java Extension:**
- Check Java version (must be 17 for Montoya API)
- Verify JAR is properly built
- Check Burp Output tab for errors

**Python Extension:**
- Verify Jython is configured
- Check Python Environment settings in Burp
- Ensure jython-standalone.jar is loaded

### API Connection Fails

- Ensure redstr HTTP server is running on `localhost:8080`
- Check firewall settings
- Verify API URL in extension configuration
- Test connection with curl

### Transformations Produce Errors

- Check function name spelling
- Verify input format
- Check API server logs
- Test transformation directly via curl

### Performance Issues

- Reduce request timeout
- Use connection pooling
- Implement request caching
- Monitor memory usage

## Development

### Java Development

```bash
# Build
mvn clean package

# Run tests
mvn test

# Install to local Burp
cp target/redstr-burp.jar ~/burp-extensions/
```

### Python Development

```bash
# No build needed - just edit and reload
# Test with:
python test_extension.py
```

## Testing

### Unit Tests

```java
@Test
public void testSqlTransformation() {
    RedstrHttpClient client = new RedstrHttpClient();
    String result = client.transform("sql_comment_injection", "SELECT * FROM users");
    assertTrue(result.contains("/**/"));
}
```

### Integration Tests

Test with mock Burp environment:
- Mock HTTP service
- Test payload generation
- Verify scanner integration
- Validate context menu actions

## Future Enhancements

- [ ] Implement Java extension (Montoya API)
- [ ] Create Python extension (optional)
- [ ] Add configuration UI
- [ ] Implement scanner checks
- [ ] Build payload library
- [ ] Add authentication support
- [ ] Create detailed documentation
- [ ] Submit to BApp Store
- [ ] Create video tutorials

## Documentation

See the comprehensive integration guide:
- [Burp Suite Integration Documentation](../../docs/burp_integration.md)

## References

- [Burp Extension Documentation](https://portswigger.net/burp/documentation/desktop/extend-burp/extensions)
- [Montoya API](https://portswigger.github.io/burp-extensions-montoya-api/)
- [Extension Examples](https://github.com/PortSwigger/extension-montoya-api-examples)

## Support

For support and questions:
- Check PortSwigger documentation
- Visit PortSwigger Discord #extensions
- File issues on GitHub

## Contributing

Contributions welcome! Please:
1. Fork the repository
2. Create a feature branch
3. Implement changes with tests
4. Submit a pull request
5. Follow Burp extension best practices

## License

MIT License - See LICENSE file in repository root.

---

**Status:** This integration is currently in planning phase. The HTTP API bridge is ready for use with custom Burp extensions. Full extension implementation coming soon.

**Important:** This extension is designed for authorized security testing only. Users must obtain proper authorization before conducting any security assessments.
