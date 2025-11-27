# Security Testing Use Cases

Comprehensive guide to using redstr for red team, blue team, and purple team security operations.

## Red Team Activities

### Phishing and Social Engineering

#### Domain Spoofing
Use `homoglyph_substitution` to create convincing lookalike domains:
- `paypal.com` → `pаypаl.com` (using Cyrillic characters)
- Perfect for testing phishing detection systems
- Validates homograph attack defenses

#### Email Obfuscation
Combine unicode variations with case swapping to evade email filters:
- Test email security gateways
- Validate content filtering systems
- Check for Unicode normalization issues

### Filter and WAF Evasion

#### Content Filter Bypass
Use `leetspeak`, `unicode_variations`, or `case_swap` to bypass content filters:
- `malware` → `m@1w@r3` or `mAlWaRe`
- Test keyword blocking systems
- Validate case-sensitivity in filters

#### SQL Injection
Use `sql_comment_injection` to insert SQL comments and evade WAF detection:
- `SELECT * FROM users` → `SELECT --* FROM /**/users`
- Test SQL injection detection
- Bypass comment-based filters

#### XSS Filter Evasion
Use `xss_tag_variations` to bypass XSS filters:
- `<script>` → `&#x3C;sCrIpT&#x3E;`
- Test encoding detection
- Validate XSS prevention mechanisms

#### Command Injection
Use `command_injection` to test command separator filtering:
- Test command injection prevention
- Validate input sanitization
- Check for command chaining vulnerabilities

### Payload Obfuscation

#### Encoding Obfuscation
Multiple encoding options available:
- `base64_encode` - Standard Base64 encoding
- `url_encode` - URL/percent encoding
- `hex_encode` - Hexadecimal encoding
- `hex_encode_mixed` - Mixed hex formats for evasion

#### Mixed Encoding
Combine `mixed_encoding` with other transformations to evade detection systems:
- Layer multiple encoding schemes
- Test multi-stage decoding
- Validate comprehensive encoding detection

#### Path Traversal
Use `path_traversal` to test directory traversal vulnerabilities:
- `/etc/passwd` → `../etc/../passwd`
- Test path sanitization
- Validate directory access controls

#### Null Byte Injection
Use `null_byte_injection` to test null byte vulnerabilities:
- `file.txt` → `file%00.txt`
- Test null byte handling
- Validate file extension checks

## Blue Team Activities

### Detection and Validation Testing

#### Filter Testing
Test if your content filters catch variations like leetspeak or homoglyphs:
- Generate test cases for blocked words using multiple transformations
- Validate filter comprehensiveness
- Test for filter bypass techniques

#### XSS Detection
Verify your XSS filters catch obfuscated payloads:
- Test multiple encoding schemes
- Validate detection coverage
- Check for edge cases

#### SQL Injection Detection
Test if your WAF detects SQL injection patterns with comments:
- Validate comment filtering
- Test pattern recognition
- Check for evasion techniques

#### Input Validation
Verify systems handle Unicode properly and reject malformed input:
- Test Unicode normalization
- Validate character encoding
- Check for encoding vulnerabilities

#### Encoding Detection
Test if your systems properly detect and decode various encoding schemes:
- Multiple encoding layers
- Mixed encoding formats
- Edge case encodings

### Security Control Testing

#### URL Encoding Validation
Use `url_encode` to test URL parsers and validators:
- Test URL parsing edge cases
- Validate encoding/decoding consistency
- Check for bypass vulnerabilities

#### Path Validation
Use `path_traversal` to test path sanitization functions:
- Validate path normalization
- Test directory traversal prevention
- Check for canonicalization issues

#### Command Validation
Use `command_injection` to test command sanitization:
- Test command separator filtering
- Validate input sanitization
- Check for injection vulnerabilities

#### Null Byte Handling
Use `null_byte_injection` to verify proper null byte handling:
- Test null byte detection
- Validate file handling
- Check for truncation issues

### Monitoring and Logging

#### Log Analysis
Test logging systems with various character encodings to ensure proper logging:
- Validate log encoding
- Test for log injection
- Check for encoding issues

#### Alert Generation
Verify security monitoring systems trigger on obfuscated attacks:
- Test alert rules
- Validate detection thresholds
- Check for false negatives

#### Normalization Testing
Test if logs properly normalize Unicode and encoded strings:
- Validate Unicode normalization
- Test encoding standardization
- Check for consistency

## Purple Team Activities

### Collaborative Testing

#### Shared Test Cases
Use transformations to create consistent test payloads for both teams:
- Standardized test payloads
- Consistent testing methodology
- Reproducible results

#### Baseline Establishment
Generate standard test cases for security controls:
- Establish security baselines
- Define test standards
- Create benchmark tests

#### Detection Validation
Red team uses transformations, blue team validates detection:
- Collaborative testing
- Detection verification
- Gap identification

### Training and Documentation

#### Security Awareness
Generate examples for security training programs:
- Show how phishing domains can be spoofed with homoglyphs
- Demonstrate filter evasion techniques
- Educate on attack patterns

#### Playbook Development
Create standard attack patterns and detection rules:
- Document attack techniques
- Create detection signatures
- Build response playbooks

#### Tool Validation
Test security tools against various obfuscation techniques:
- Validate tool effectiveness
- Test detection coverage
- Identify tool limitations

### Continuous Improvement

#### Coverage Testing
Ensure security controls cover all transformation variations:
- Comprehensive coverage validation
- Gap identification
- Control improvement

#### Gap Analysis
Identify missing detection rules using transformation permutations:
- Systematic testing
- Pattern identification
- Rule enhancement

#### Effectiveness Metrics
Measure detection rates across different obfuscation techniques:
- Quantify detection effectiveness
- Track improvement over time
- Benchmark performance

## See Also

- [CLI Reference](cli-reference.md) - Command-line interface documentation
- [API Reference](api-reference.md) - Complete library function reference
- [Integration Examples](integration-examples.md) - Tool integration patterns
