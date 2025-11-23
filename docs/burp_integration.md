# Burp Suite Integration for redstr

**Task ID:** INT-BURP-001, INT-BURP-002  
**Status:** Research Complete - Implementation Ready  
**Last Updated:** November 23, 2025

## Overview

This document outlines the integration between redstr and Burp Suite, the industry-standard web application security testing tool. The integration enables penetration testers to leverage redstr's advanced string transformation capabilities within Burp Suite's comprehensive testing workflow.

## Burp Suite Extension Architecture

### Extension APIs

1. **Montoya API (Java 17+)** - Modern, Recommended
   - Full-featured modern API
   - Active development and support
   - Comprehensive documentation
   - Rich type system and interfaces

2. **Extender API (Python via Jython)** - Legacy
   - Python scripting support
   - Rapid prototyping
   - Simple for small extensions
   - Legacy API with limited features

### Core Components

1. **HTTP Message Processing**
   - Request/response interception
   - Message modification
   - Header manipulation
   - Body transformation

2. **Scanner Integration**
   - Custom scan checks
   - Issue reporting
   - Active/passive scanning
   - Vulnerability detection

3. **Context Menus**
   - Right-click menu items
   - Quick actions
   - Payload generation
   - Send to tools

4. **UI Extensions**
   - Custom tabs
   - Message editors
   - Configuration panels
   - Results display

## Integration Points

### 1. Intruder Payload Generator

Generate sophisticated payloads using redstr:
- SQL injection variations
- XSS payload obfuscation
- Command injection patterns
- Encoding chains

### 2. Session Handler

Transform session tokens and authentication:
- JWT manipulation
- Session token variation
- Cookie obfuscation
- Bearer token transformation

### 3. Scanner Integration

Enhance scanning with custom checks:
- WAF bypass detection
- Advanced injection testing
- Obfuscated payload scanning
- Evasion technique validation

### 4. Repeater Enhancement

Transform requests in Repeater:
- Quick obfuscation
- Multi-encoding chains
- Pattern generation
- A/B testing variants

## Integration Design

### Option 1: HTTP API Bridge (Recommended)

Deploy redstr as HTTP service and create Burp extension to interface with it.

**Benefits:**
- Language-agnostic
- Use full Rust performance
- Scalable architecture
- No JVM overhead for transformations
- Easy to update redstr independently

**Architecture:**
```
Burp Extension (Java/Python) <-HTTP-> redstr HTTP Server (Rust)
```

### Option 2: JNI Bridge (Java Native Interface)

Compile redstr as native library for direct Java integration.

**Benefits:**
- Native performance
- No network overhead
- Single deployment unit
- Direct function calls

**Drawbacks:**
- Complex build process
- Platform-specific binaries
- Harder to debug

### Option 3: CLI Wrapper

Execute redstr CLI from Burp extension.

**Benefits:**
- Simple implementation
- Easy to debug
- No compilation complexity

**Drawbacks:**
- Process spawning overhead
- Less efficient for high-volume operations

## Implementation Plan

### Phase 1: HTTP API Bridge Extension (INT-BURP-002)

Create `integrations/burp/` with:

1. **Java Extension (Montoya API)**
   ```
   src/main/java/com/redstr/burp/
   ├── RedstrExtension.java          # Main entry point
   ├── RedstrHttpClient.java         # HTTP client for API
   ├── RedstrPayloadGenerator.java   # Intruder integration
   ├── RedstrContextMenu.java        # Context menu actions
   ├── RedstrTab.java                # Custom UI tab
   ├── RedstrScanner.java            # Scanner checks
   └── config/
       └── RedstrConfig.java         # Configuration
   ```

2. **Python Extension (Extender API - Optional)**
   ```
   redstr_burp.py                    # Main extension
   redstr_client.py                  # API client
   redstr_payloads.py                # Payload generators
   ```

3. **Build Configuration**
   ```
   pom.xml                           # Maven build
   build.gradle                      # Gradle build (alternative)
   ```

4. **Documentation**
   ```
   README.md                         # Installation guide
   USAGE.md                          # Usage examples
   API.md                            # API reference
   ```

### Phase 2: Core Extension Features

#### 1. HTTP Client

```java
public class RedstrHttpClient {
    private final String apiUrl;
    
    public RedstrHttpClient(String apiUrl) {
        this.apiUrl = apiUrl;
    }
    
    public String transform(String input, String method) throws IOException {
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

#### 2. Intruder Payload Generator

```java
public class RedstrPayloadGenerator implements PayloadGenerator {
    private final RedstrHttpClient client;
    private final String transformMethod;
    
    @Override
    public PayloadGeneratorResult generatePayload(ByteArray baseValue) {
        String input = baseValue.toString();
        String transformed = client.transform(input, transformMethod);
        return PayloadGeneratorResult.payloadGenerated(
            ByteArray.byteArray(transformed)
        );
    }
}
```

#### 3. Context Menu Integration

```java
public class RedstrContextMenu implements ContextMenuItemsProvider {
    @Override
    public List<Component> provideMenuItems(ContextMenuEvent event) {
        List<Component> menuItems = new ArrayList<>();
        
        if (event.messageEditorRequestResponse().isPresent()) {
            JMenuItem sqlItem = new JMenuItem("redstr: SQL Injection");
            sqlItem.addActionListener(e -> 
                transformSelection(event, "sql_comment_injection")
            );
            menuItems.add(sqlItem);
            
            JMenuItem xssItem = new JMenuItem("redstr: XSS Variation");
            xssItem.addActionListener(e -> 
                transformSelection(event, "xss_tag_variations")
            );
            menuItems.add(xssItem);
        }
        
        return menuItems;
    }
}
```

#### 4. Scanner Integration

```java
public class RedstrScanner implements ScanCheck {
    @Override
    public List<ScanIssue> passiveScan(HttpRequestResponse baseRequestResponse) {
        // No passive scanning
        return Collections.emptyList();
    }
    
    @Override
    public List<ScanIssue> activeScan(
            HttpRequestResponse baseRequestResponse,
            InsertionPoint insertionPoint) {
        
        List<ScanIssue> issues = new ArrayList<>();
        String baseValue = insertionPoint.baseValue();
        
        // Test with various redstr transformations
        String[] methods = {"sql_comment_injection", "xss_tag_variations", 
                           "command_injection"};
        
        for (String method : methods) {
            String payload = client.transform(baseValue, method);
            HttpRequest request = insertionPoint.buildHttpRequestWithPayload(
                ByteArray.byteArray(payload)
            );
            
            HttpRequestResponse result = api.http().sendRequest(request);
            if (isVulnerable(result, method)) {
                issues.add(createIssue(result, method, payload));
            }
        }
        
        return issues;
    }
}
```

#### 5. Custom UI Tab

```java
public class RedstrTab implements MontoyaApi {
    private JPanel mainPanel;
    private JComboBox<String> methodSelector;
    private JTextArea inputArea;
    private JTextArea outputArea;
    
    public RedstrTab(MontoyaApi api, RedstrHttpClient client) {
        this.api = api;
        this.client = client;
        initUI();
    }
    
    private void initUI() {
        mainPanel = new JPanel(new BorderLayout());
        
        // Method selector
        String[] methods = {"sql_comment_injection", "xss_tag_variations",
                           "leetspeak", "base64_encode", "domain_typosquat"};
        methodSelector = new JComboBox<>(methods);
        
        // Input/output areas
        inputArea = new JTextArea(10, 40);
        outputArea = new JTextArea(10, 40);
        outputArea.setEditable(false);
        
        // Transform button
        JButton transformButton = new JButton("Transform");
        transformButton.addActionListener(e -> performTransform());
        
        // Layout
        mainPanel.add(createToolbar(), BorderLayout.NORTH);
        mainPanel.add(createSplitPane(), BorderLayout.CENTER);
    }
    
    private void performTransform() {
        String input = inputArea.getText();
        String method = (String) methodSelector.getSelectedItem();
        
        try {
            String output = client.transform(input, method);
            outputArea.setText(output);
        } catch (Exception e) {
            outputArea.setText("Error: " + e.getMessage());
        }
    }
}
```

### Phase 3: Python Alternative

```python
from burp import IBurpExtender, IIntruderPayloadGenerator, IContextMenuFactory
from javax.swing import JMenuItem
import urllib2
import json

class BurpExtender(IBurpExtender, IIntruderPayloadGenerator, IContextMenuFactory):
    def registerExtenderCallbacks(self, callbacks):
        self._callbacks = callbacks
        self._helpers = callbacks.getHelpers()
        
        callbacks.setExtensionName("redstr Integration")
        callbacks.registerIntruderPayloadGeneratorFactory(self)
        callbacks.registerContextMenuFactory(self)
        
        self.api_url = "http://localhost:8080"
    
    def transform(self, input_str, method):
        """Call redstr API"""
        data = json.dumps({"function": method, "input": input_str})
        req = urllib2.Request(self.api_url + "/transform", data,
                             {"Content-Type": "application/json"})
        response = urllib2.urlopen(req)
        result = json.loads(response.read())
        return result["output"]
    
    def createMenuItems(self, invocation):
        """Create context menu items"""
        menu_items = []
        
        sql_item = JMenuItem("redstr: SQL Injection")
        sql_item.addActionListener(
            lambda x: self.transform_selection(invocation, "sql_comment_injection")
        )
        menu_items.append(sql_item)
        
        return menu_items
    
    def transform_selection(self, invocation, method):
        """Transform selected text"""
        bounds = invocation.getSelectionBounds()
        if bounds:
            message = invocation.getSelectedMessages()[0]
            selected = message.getRequest()[bounds[0]:bounds[1]]
            transformed = self.transform(selected, method)
            # Update request with transformed value
            self._callbacks.makeHttpRequest(message.getHttpService(), transformed)
```

## Use Cases

### 1. SQL Injection Testing

```java
// Generate SQL injection payloads
String original = "1";
String[] payloads = {
    client.transform(original, "sql_comment_injection"),
    client.transform("' OR '1'='1", "sql_comment_injection"),
    client.transform("UNION SELECT", "sql_comment_injection")
};
// Use in Intruder
```

### 2. XSS Testing with WAF Bypass

```java
// Generate XSS variations
String xssPayload = "<script>alert('XSS')</script>";
String obfuscated = client.transform(xssPayload, "xss_tag_variations");
String htmlEncoded = client.transform(obfuscated, "html_entity_encode");
String mixed = client.transform(htmlEncoded, "mixed_encoding");
```

### 3. JWT Manipulation

```java
// Test JWT vulnerabilities
String jwt = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...";
String algConfusion = client.transform(jwt, "jwt_algorithm_confusion");
String headerManip = client.transform(jwt, "jwt_header_manipulation");
String sigBypass = client.transform(jwt, "jwt_signature_bypass");
```

### 4. Phishing Domain Testing

```java
// Test phishing filters
String domain = "legitimate-bank.com";
String typosquat = client.transform(domain, "domain_typosquat");
String homoglyph = client.transform(domain, "homoglyph_substitution");
```

## Configuration

### Extension Settings

```java
public class RedstrConfig {
    private String apiUrl = "http://localhost:8080";
    private int timeout = 5000;
    private boolean enableLogging = true;
    private Map<String, String> presets = new HashMap<>();
    
    // Configuration UI
    public JPanel createConfigPanel() {
        JPanel panel = new JPanel();
        
        JTextField urlField = new JTextField(apiUrl);
        JSpinner timeoutSpinner = new JSpinner(new SpinnerNumberModel(timeout, 1000, 30000, 1000));
        JCheckBox loggingCheckbox = new JCheckBox("Enable Logging", enableLogging);
        
        // Add listeners to save config
        return panel;
    }
}
```

## Building and Installation

### Java (Maven)

```xml
<!-- pom.xml -->
<project>
    <dependencies>
        <dependency>
            <groupId>net.portswigger.burp.extensions</groupId>
            <artifactId>montoya-api</artifactId>
            <version>2023.10.3.4</version>
        </dependency>
    </dependencies>
    
    <build>
        <plugins>
            <plugin>
                <groupId>org.apache.maven.plugins</groupId>
                <artifactId>maven-shade-plugin</artifactId>
                <configuration>
                    <finalName>redstr-burp</finalName>
                </configuration>
            </plugin>
        </plugins>
    </build>
</project>
```

Build:
```bash
mvn clean package
# Output: target/redstr-burp.jar
```

### Python

No build required - just load the `.py` file:

1. Download Jython standalone JAR
2. Configure in Burp: Extensions → Options → Python Environment
3. Load extension: Extensions → Add → Python → Select file

### Installation

1. Ensure redstr HTTP server is running:
   ```bash
   redstr serve --port 8080
   ```

2. Load extension in Burp:
   - Go to Extensions tab
   - Click "Add"
   - Select extension type (Java/Python)
   - Choose file
   - Click "Next"

3. Verify installation:
   - Check "Output" tab for success message
   - Verify "redstr" tab appears
   - Test context menu items

## Testing

### Unit Tests

```java
@Test
public void testSqlInjectionTransform() {
    RedstrHttpClient client = new RedstrHttpClient("http://localhost:8080");
    String result = client.transform("SELECT * FROM users", "sql_comment_injection");
    assertTrue(result.contains("/**/"));
}
```

### Integration Tests

Test with actual Burp Suite environment:
- Mock HTTP service
- Test request/response processing
- Verify scanner integration
- Validate UI components

## Security Considerations

### Authorization

- Only use on authorized targets
- Log all transformation operations
- Implement audit trails

### API Security

- Use authentication for HTTP API
- Implement rate limiting
- Validate all inputs
- Sanitize outputs

### Performance

- Cache transformation results
- Implement request pooling
- Monitor memory usage
- Limit concurrent requests

## Troubleshooting

### Common Issues

1. **Extension won't load**
   - Check Java version (requires Java 17 for Montoya)
   - Verify Jython is configured (for Python)
   - Check dependencies in classpath

2. **API connection fails**
   - Verify redstr server is running
   - Check firewall settings
   - Validate API URL in configuration

3. **Transformations fail**
   - Check API server logs
   - Verify input format
   - Test transformation directly via CLI

## References

- [Burp Extension Documentation](https://portswigger.net/burp/documentation/desktop/extend-burp/extensions)
- [Montoya API Reference](https://portswigger.github.io/burp-extensions-montoya-api/)
- [Extender API Guide](https://portswigger.net/burp/extender/api/)
- [Extension Examples](https://github.com/PortSwigger/extension-montoya-api-examples)
- [redstr Documentation](../README.md)

## Roadmap

- [x] Research Burp extension APIs
- [x] Design integration architecture
- [ ] Implement HTTP API bridge
- [ ] Create Java extension (Montoya API)
- [ ] Create Python extension (optional)
- [ ] Build configuration UI
- [ ] Write comprehensive tests
- [ ] Create installation guide
- [ ] Publish to BApp Store
- [ ] Create tutorial videos

## Support

For integration support:
- Check PortSwigger documentation
- Visit PortSwigger Discord #extensions
- Review example extensions
- File issues on GitHub

---

**Important:** This extension is designed for authorized security testing only. Users must obtain proper authorization before conducting any security assessments.
