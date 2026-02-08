# redstr Scripts

This directory contains helper scripts and workflow wrappers for redstr.

## generate-cli-assets.sh

Generates CLI-derived assets from a single mode list:
- `completions/redstr.bash`
- `completions/redstr.zsh`
- `completions/redstr.fish`
- `man/redstr.1`

Usage:
```bash
scripts/generate-cli-assets.sh
```

Run this after changing CLI modes/options in `src/main.rs`.

## redstr-wrapper.sh

A convenience wrapper that provides common security testing workflows combining
multiple redstr transformations.

### Installation

```bash
# System-wide installation
sudo cp redstr-wrapper.sh /usr/local/bin/redstr-wrapper
sudo chmod +x /usr/local/bin/redstr-wrapper

# User installation
mkdir -p ~/.local/bin
cp redstr-wrapper.sh ~/.local/bin/redstr-wrapper
chmod +x ~/.local/bin/redstr-wrapper

# Add to PATH if needed (add to ~/.bashrc or ~/.zshrc)
export PATH="$HOME/.local/bin:$PATH"
```

### Usage

```bash
# Show help
redstr-wrapper --help

# WAF bypass workflow
redstr-wrapper waf-bypass "SELECT * FROM users"

# Phishing domain generation
redstr-wrapper phishing paypal.com

# XSS payload variations
redstr-wrapper xss-test "<script>alert(1)</script>"

# SQL injection testing
redstr-wrapper sql-test "SELECT * FROM users WHERE id=1"

# Encode in all formats
redstr-wrapper encode-all "secret data"

# Batch transform a file
redstr-wrapper batch-transform payloads.txt -o results.txt
```

### Workflows

#### WAF Bypass
Generates multiple variations of a payload for WAF bypass testing:
- Case variations (random case, case swap)
- Encoding variations (URL, hex, base64)
- SQL-specific patterns (comment injection)

Example:
```bash
$ redstr-wrapper waf-bypass "SELECT * FROM users"
Case Variations:
  Random Case: SeLeCt * FrOm UsErS
  Case Swap: sElEcT * fRoM uSeRs
Encoding Variations:
  URL Encoded: SELECT%20%2A%20FROM%20users
  Hex Encoded: 53454c454354202a2046524f4d207573657273
  Base64: U0VMRUNUICogRlJPTSB1c2Vycw==
SQL-Specific:
  SQL Comments: SELECT --* FROM /**/users
```

#### Phishing
Generates phishing domain variations using homoglyphs and Unicode:
- Homoglyph substitutions (lookalike characters)
- Unicode variations (IDN homograph attacks)

Example:
```bash
$ redstr-wrapper phishing paypal.com
Homoglyph Variations:
  pаypаl.com  (Cyrillic 'a')
  pаypаl.cοm  (Greek 'o')
  paypаl.cοm  (mixed)
Unicode Variations:
  pãypål.com
  pāypāl.com
```

#### XSS Testing
Generates XSS payload variations for filter bypass:
- Tag variations (case, spacing, encoding)
- Encoding variations (URL, HTML entities)
- Case obfuscation

Example:
```bash
$ redstr-wrapper xss-test "<script>alert(1)</script>"
Tag Variations:
  <ScRiPt>alert(1)</sCrIpT>
Encoding Variations:
  URL Encoded: %3Cscript%3Ealert(1)%3C%2Fscript%3E
  HTML Entities: &#x3C;script&#x3E;alert(1)&#x3C;/script&#x3E;
```

#### SQL Testing
Generates SQL injection variations:
- Comment injection patterns
- Case variations
- URL encoding

#### Encode All
Encodes input in all available formats:
- Base64
- URL encoding
- Hex (normal and mixed)
- ROT13

#### Batch Transform
Process multiple lines from a file:
```bash
# Transform all lines with random capitalization
redstr-wrapper batch-transform wordlist.txt -o output.txt

# With specific mode
redstr-wrapper batch-transform payloads.txt -o encoded.txt -m base64
```

### Environment Variables

- `REDSTR_BIN` - Path to redstr binary (default: `redstr`)

Example:
```bash
export REDSTR_BIN=/opt/redstr/bin/redstr
redstr-wrapper waf-bypass "test"
```

### Integration with Other Tools

#### With Burp Suite
```bash
# Generate payloads for Burp Intruder
redstr-wrapper waf-bypass "' OR 1=1--" > burp-payloads.txt
```

#### With sqlmap
```bash
# Generate obfuscated SQL payloads
redstr-wrapper sql-test "' UNION SELECT NULL--" | xargs -I {} sqlmap -u "http://target.com?id={}"
```

#### With ffuf
```bash
# Generate directory traversal payloads
for i in {1..10}; do
  redstr path-traversal "/etc/passwd" >> traversal-payloads.txt
done
ffuf -u http://target.com/FUZZ -w traversal-payloads.txt
```

### Color Output

The wrapper uses color output for better readability:
- Blue - Banners and headers
- Green - Success messages and section headers
- Yellow - Subsection headers
- Red - Error messages

Disable colors:
```bash
export NO_COLOR=1
redstr-wrapper waf-bypass "test"
```

### Tips

1. **Combine with other tools**: Pipe output to other security tools
2. **Save output**: Use `-o` flag to save results
3. **Batch processing**: Use batch-transform for wordlists
4. **Integration**: Set REDSTR_BIN for custom installations
5. **Workflows**: Customize workflows by editing the script

## Other Scripts

### install-commit-hook.sh
Installs git commit hooks for conventional commits.

## Contributing

To add new workflows to redstr-wrapper.sh:

1. Create a new workflow function: `workflow_yourname()`
2. Add help text in `print_usage()`
3. Add case statement in `main()`
4. Test thoroughly
5. Update this README

Example:
```bash
workflow_custom() {
    local input="$1"
    echo -e "${GREEN}[*] Running custom workflow...${NC}"
    # Your transformations here
}
```
