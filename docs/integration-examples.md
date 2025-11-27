# Integration Examples

Detailed examples for integrating redstr into security tools and workflows.

## Caido / Web Security Testing Tools

Randomize requests and test payloads with multiple variations.

### Basic Request Randomization

```rust
use redstr::{random_user_agent, url_encode, xss_tag_variations};

// Randomize requests to avoid fingerprinting
let headers = vec![
    ("User-Agent", random_user_agent()),
];

// Make requests with random UA
let response = make_request_with_headers(url, headers);
```

### XSS Payload Testing

```rust
use redstr::{xss_tag_variations, url_encode, TransformBuilder};

// Test XSS payloads with variations
let payload = "<script>alert(1)</script>";
let variations = vec![
    xss_tag_variations(payload),
    url_encode(&xss_tag_variations(payload)),
    TransformBuilder::new(payload)
        .case_swap()
        .base64()
        .build(),
];

// Test each variation
for variant in variations {
    test_xss_payload(&variant);
}
```

### Advanced Payload Generation

```rust
use redstr::TransformBuilder;

// Create complex payloads
let payloads = vec![
    // Base64 encoded XSS
    TransformBuilder::new("<img src=x onerror=alert(1)>")
        .base64()
        .build(),
    
    // Case-swapped SQL injection
    TransformBuilder::new("UNION SELECT")
        .case_swap()
        .build(),
    
    // URL-encoded command injection
    TransformBuilder::new("$(whoami)")
        .url_encode()
        .build(),
];
```

## EvilJinx / Phishing Frameworks

Generate convincing phishing domains and obfuscate content.

### Phishing Domain Generation

```rust
use redstr::{domain_typosquat, homoglyph_substitution};

// Generate phishing domains
let target = "paypal.com";

// Typosquatting variations
let typo_domains = (0..10)
    .map(|_| domain_typosquat(target))
    .collect::<Vec<_>>();

// Homoglyph variations
let homoglyph_domains = (0..10)
    .map(|_| homoglyph_substitution(target))
    .collect::<Vec<_>>();

// Use for phishing campaign
for domain in typo_domains.iter().chain(homoglyph_domains.iter()) {
    register_phishing_domain(domain);
}
```

### Email Content Obfuscation

```rust
use redstr::{html_entity_encode, homoglyph_substitution, TransformBuilder};

// Obfuscate phishing page content
let link = "https://secure.paypal.com/login";

// Multiple obfuscation layers
let obfuscated = TransformBuilder::new(link)
    .homoglyphs()
    .base64()
    .build();

// HTML entity encoding for email
let email_safe = html_entity_encode(&homoglyph_substitution(link));
```

### Template Obfuscation

```rust
use redstr::{js_string_concat, html_entity_encode};

// Obfuscate JavaScript in phishing page
let js_code = "document.getElementById('password').value";
let obfuscated_js = js_string_concat(js_code);

// Obfuscate HTML attributes
let sensitive_text = "Enter your password";
let obfuscated_text = html_entity_encode(sensitive_text);
```

## Bot Detection Testing

Test bot detection systems with various evasion techniques.

### User-Agent Randomization

```rust
use redstr::random_user_agent;

// Simulate different browsers
let user_agents: Vec<String> = (0..100)
    .map(|_| random_user_agent())
    .collect();

// Test with different UAs
for ua in user_agents {
    test_bot_detection_with_ua(&ua);
}
```

### Content Obfuscation for Bots

```rust
use redstr::{
    js_string_concat,
    unicode_normalize_variants,
    whitespace_padding,
    TransformBuilder,
};

// Simulate bot evasion techniques
let content = "robot";

// Unicode normalization variants
let normalized = unicode_normalize_variants(content);

// JavaScript obfuscation
let js_obfuscated = js_string_concat("document.cookie");

// Whitespace padding
let padded = whitespace_padding(content);

// Complex transformation
let complex = TransformBuilder::new("bot detector")
    .homoglyphs()
    .case_swap()
    .build();
```

### Fingerprinting Evasion

```rust
use redstr::{random_user_agent, TransformBuilder};

// Generate diverse request patterns
struct RequestPattern {
    user_agent: String,
    cookies: String,
    headers: Vec<(String, String)>,
}

fn generate_request_pattern() -> RequestPattern {
    RequestPattern {
        user_agent: random_user_agent(),
        cookies: TransformBuilder::new("session_id=abc123")
            .base64()
            .build(),
        headers: vec![
            ("Accept-Language".to_string(), "en-US,en;q=0.9".to_string()),
            ("User-Agent".to_string(), random_user_agent()),
        ],
    }
}
```

## URL Scanner / Web Crawler Integration

Safely encode and analyze suspicious URLs.

### URL Encoding for Storage

```rust
use redstr::{url_encode, base64_encode, TransformBuilder};

// Encode URLs for safe storage/transmission
let suspicious_url = "http://example.com/path?param=<script>";
let safe_url = url_encode(suspicious_url);

// Store in database
store_url_in_database(&safe_url);

// Retrieve and decode
let retrieved = retrieve_url_from_database();
// Handle decoding separately
```

### Payload Analysis

```rust
use redstr::TransformBuilder;

// Create encoded payloads for analysis
let payloads = vec![
    "malicious-content",
    "<script>alert(1)</script>",
    "'; DROP TABLE users; --",
];

for payload in payloads {
    // Analyze in multiple encodings
    let base64_encoded = TransformBuilder::new(payload)
        .base64()
        .build();
    
    let url_encoded = TransformBuilder::new(payload)
        .url_encode()
        .build();
    
    let double_encoded = TransformBuilder::new(payload)
        .base64()
        .url_encode()
        .build();
    
    analyze_payload(&base64_encoded);
    analyze_payload(&url_encoded);
    analyze_payload(&double_encoded);
}
```

### Malicious Domain Detection

```rust
use redstr::{homoglyph_substitution, domain_typosquat};

// Detect potential phishing domains
let legitimate_domains = vec![
    "paypal.com",
    "google.com",
    "microsoft.com",
];

for domain in legitimate_domains {
    // Generate variations that might be phishing
    let homoglyph_variants: Vec<String> = (0..50)
        .map(|_| homoglyph_substitution(domain))
        .collect();
    
    let typosquat_variants: Vec<String> = (0..50)
        .map(|_| domain_typosquat(domain))
        .collect();
    
    // Check if any variants are registered
    for variant in homoglyph_variants.iter().chain(typosquat_variants.iter()) {
        if is_domain_registered(variant) {
            flag_potential_phishing(variant, domain);
        }
    }
}
```

## Burp Suite Extension

Integrate transformations into Burp Suite workflows.

### Request Transformer

```rust
use redstr::{TransformBuilder, random_user_agent};

// Transform request in Burp extension
fn transform_request(original_request: &str) -> String {
    // Parse request
    let (headers, body) = parse_http_request(original_request);
    
    // Transform user agent
    let new_ua = random_user_agent();
    
    // Transform payload
    let transformed_body = TransformBuilder::new(&body)
        .case_swap()
        .url_encode()
        .build();
    
    // Reconstruct request
    rebuild_http_request(headers, &transformed_body, &new_ua)
}
```

### Payload Generator

```rust
use redstr::{
    leetspeak, base64_encode, url_encode,
    xss_tag_variations, sql_comment_injection,
};

// Generate payload variations for Intruder
fn generate_payload_variations(base_payload: &str) -> Vec<String> {
    vec![
        base_payload.to_string(),
        leetspeak(base_payload),
        base64_encode(base_payload),
        url_encode(base_payload),
        xss_tag_variations(base_payload),
        sql_comment_injection(base_payload),
    ]
}
```

## Custom Security Tools

Build custom security testing tools with redstr.

### WAF Bypass Tester

```rust
use redstr::{
    case_swap, sql_comment_injection,
    xss_tag_variations, TransformBuilder,
};

struct WafBypassTester {
    target_url: String,
    payloads: Vec<String>,
}

impl WafBypassTester {
    fn new(url: &str) -> Self {
        WafBypassTester {
            target_url: url.to_string(),
            payloads: Vec::new(),
        }
    }
    
    fn generate_sql_injection_variants(&mut self, base: &str) {
        self.payloads.extend(vec![
            base.to_string(),
            case_swap(base),
            sql_comment_injection(base),
            TransformBuilder::new(base)
                .case_swap()
                .url_encode()
                .build(),
        ]);
    }
    
    fn test_all_variants(&self) -> Vec<TestResult> {
        self.payloads
            .iter()
            .map(|payload| self.test_payload(payload))
            .collect()
    }
    
    fn test_payload(&self, payload: &str) -> TestResult {
        // Implementation
        TestResult::default()
    }
}

#[derive(Default)]
struct TestResult {
    payload: String,
    blocked: bool,
    response_code: u16,
}
```

## See Also

- [CLI Reference](cli-reference.md) - Command-line interface documentation
- [Use Cases](use-cases.md) - Security testing scenarios
- [API Reference](api-reference.md) - Complete function reference
- [Official Integrations](../README.md#building-integrations) - Official tool integrations
