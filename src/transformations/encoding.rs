use crate::rng::SimpleRng;

/// Encodes characters using mixed encoding formats (HTML entities, Unicode escapes).
///
/// Randomly encodes each character using one of four formats: plain text,
/// hexadecimal HTML entity (`&#x...;`), decimal HTML entity (`&#...;`),
/// or Unicode escape (`\u{...}`). This mixed approach can bypass filters
/// that only detect specific encoding formats.
///
/// # Use Cases
///
/// - **XSS Testing**: Bypass filters that don't handle all encoding formats
/// - **Red Team**: Evade detection systems with mixed encoding
/// - **Blue Team**: Test encoding normalization and parser robustness
///
/// # Examples
///
/// ```
/// use redstr::mixed_encoding;
///
/// let result = mixed_encoding("test");
/// // Example output: "t&#x65;&#115;\u{0074}" (varies each run)
/// assert!(result.contains("&#") || result.contains("\\u"));
///
/// // XSS payload with mixed encoding
/// let xss = mixed_encoding("<script>");
/// // Example: "&#x3c;s&#99;r\u{0069}pt&#x3e;"
/// ```
pub fn mixed_encoding(input: &str) -> String {
    let mut rng = SimpleRng::new();
    let mut result = String::with_capacity(input.len() * 8); // Encoded chars are longer

    for c in input.chars() {
        match rng.next_u64() % 4 {
            0 => result.push(c),
            1 => result.push_str(&format!("&#x{:x};", c as u32)),
            2 => result.push_str(&format!("&#{};", c as u32)),
            _ => result.push_str(&format!("\\u{{{:04x}}}", c as u32)),
        }
    }
    result
}

/// Encodes text to Base64.
///
/// Converts input text to Base64 encoding using the standard RFC 4648 alphabet.
/// This is a lossless encoding that increases the string length by approximately
/// 33% and is commonly used for transmitting binary data or obfuscating payloads.
///
/// # Use Cases
///
/// - **Red Team**: Obfuscate command payloads to evade detection
/// - **Data Transmission**: Safely encode binary data in text formats
/// - **Blue Team**: Test if security systems properly decode Base64
/// - **API Testing**: Encode credentials or tokens
///
/// # Examples
///
/// ```
/// use redstr::base64_encode;
///
/// assert_eq!(base64_encode("hello"), "aGVsbG8=");
/// assert_eq!(base64_encode("test"), "dGVzdA==");
///
/// // Obfuscate shell commands
/// let cmd = base64_encode("rm -rf /tmp/*");
/// assert_eq!(cmd, "cm0gLXJmIC90bXAvKg==");
///
/// // Encode credentials
/// let auth = base64_encode("username:password");
/// // Use in Authorization: Basic header
/// ```
pub fn base64_encode(input: &str) -> String {
    const BASE64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let bytes = input.as_bytes();
    let capacity = bytes.len().div_ceil(3) * 4; // Base64 expands by ~33%
    let mut result = String::with_capacity(capacity);

    for chunk in bytes.chunks(3) {
        let mut buf = [0u8; 3];
        for (i, &byte) in chunk.iter().enumerate() {
            buf[i] = byte;
        }

        let b1 = (buf[0] >> 2) as usize;
        let b2 = (((buf[0] & 0x03) << 4) | (buf[1] >> 4)) as usize;
        let b3 = (((buf[1] & 0x0F) << 2) | (buf[2] >> 6)) as usize;
        let b4 = (buf[2] & 0x3F) as usize;

        result.push(BASE64_CHARS[b1] as char);
        result.push(BASE64_CHARS[b2] as char);
        result.push(if chunk.len() > 1 {
            BASE64_CHARS[b3] as char
        } else {
            '='
        });
        result.push(if chunk.len() > 2 {
            BASE64_CHARS[b4] as char
        } else {
            '='
        });
    }

    result
}

/// Encodes text with URL/percent encoding (RFC 3986).
///
/// Converts characters to percent-encoded format (`%XX`) where unreserved
/// characters (A-Z, a-z, 0-9, `-`, `_`, `.`, `~`) remain unchanged. Properly
/// handles multi-byte UTF-8 characters by encoding each byte separately.
///
/// # Use Cases
///
/// - **URL Construction**: Safely encode query parameters and path segments
/// - **Red Team**: Bypass input filters with encoded payloads
/// - **API Testing**: Encode special characters in HTTP requests
/// - **Blue Team**: Test URL parser and decoder implementations
///
/// # Examples
///
/// ```
/// use redstr::url_encode;
///
/// assert_eq!(url_encode("hello world"), "hello%20world");
/// assert_eq!(url_encode("user@example.com"), "user%40example.com");
///
/// // XSS payload encoding
/// let xss = url_encode("<script>alert(1)</script>");
/// // Output: "%3Cscript%3Ealert%281%29%3C%2Fscript%3E"
///
/// // SQL injection encoding
/// let sql = url_encode("' OR '1'='1");
/// // Output: "%27%20OR%20%271%27%3D%271"
/// ```
pub fn url_encode(input: &str) -> String {
    let mut result = String::with_capacity(input.len() * 3); // URL encoding can triple size
    for c in input.chars() {
        if c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.' || c == '~' {
            result.push(c);
        } else {
            // Properly encode multi-byte UTF-8 characters
            let mut buf = [0; 4];
            let encoded = c.encode_utf8(&mut buf);
            for byte in encoded.bytes() {
                result.push_str(&format!("%{:02X}", byte));
            }
        }
    }
    result
}

/// Encodes text to hexadecimal representation (lowercase).
///
/// Converts each byte to a two-character lowercase hexadecimal string.
/// The output contains only characters 0-9 and a-f. This is a common
/// encoding format for binary data and raw byte representations.
///
/// # Use Cases
///
/// - **Binary Data**: Display binary content as readable hex
/// - **Debugging**: View exact byte values in strings
/// - **Red Team**: Encode shellcode or binary payloads
/// - **Cryptography**: Display hash values and signatures
///
/// # Examples
///
/// ```
/// use redstr::hex_encode;
///
/// assert_eq!(hex_encode("test"), "74657374");
/// assert_eq!(hex_encode("AB"), "4142");
/// assert_eq!(hex_encode("hello"), "68656c6c6f");
///
/// // Encode shellcode
/// let shellcode = vec![0x90, 0x90, 0xc3]; // NOP NOP RET
/// // Would encode to: "9090c3"
/// ```
pub fn hex_encode(input: &str) -> String {
    input.bytes().fold(String::new(), |mut acc, b| {
        use std::fmt::Write;
        write!(&mut acc, "{:02x}", b).unwrap();
        acc
    })
}

/// Encodes text with mixed hexadecimal formats (0x, \x, %, &#x).
///
/// Randomly encodes each byte using one of four hex formats: C-style escape
/// (`\x`), URL encoding (`%`), hex literal (`0x`), or HTML entity (`&#x;`).
/// This format mixing can evade detection systems that pattern-match specific
/// encoding styles.
///
/// # Use Cases
///
/// - **Red Team**: Bypass detection with varied encoding formats
/// - **XSS/SQLi Testing**: Evade filters that only recognize one format
/// - **Blue Team**: Test encoder/decoder robustness across formats
///
/// # Examples
///
/// ```
/// use redstr::hex_encode_mixed;
///
/// let result = hex_encode_mixed("AB");
/// // Example output: "\x41%42" or "0x41&#x42;" (varies each run)
/// assert!(result.len() >= 2);
///
/// // Mixed format payload obfuscation
/// let payload = hex_encode_mixed("<script>");
/// // Example: "\x3c%73&#x63;0x72\x69%70&#x74;\x3e"
/// ```
pub fn hex_encode_mixed(input: &str) -> String {
    let mut rng = SimpleRng::new();

    input
        .bytes()
        .map(|b| match rng.next_u64() % 4 {
            0 => format!("\\x{:02x}", b),
            1 => format!("%{:02x}", b),
            2 => format!("0x{:02x}", b),
            _ => format!("&#x{:02x};", b),
        })
        .collect()
}

/// Encodes text using various HTML entity formats.
///
/// Randomly encodes characters using plain text, decimal entities (`&#...;`),
/// hexadecimal entities (`&#x...;`), or named entities (`&lt;`, `&gt;`, etc.).
/// This mixed approach tests HTML parser robustness and can bypass filters.
///
/// # Use Cases
///
/// - **XSS Testing**: Bypass HTML sanitizers with entity encoding
/// - **Red Team**: Evade WAF rules that look for literal characters
/// - **Blue Team**: Test HTML entity decoder implementations
/// - **Web Scraping**: Handle various entity encoding formats
///
/// # Examples
///
/// ```
/// use redstr::html_entity_encode;
///
/// let result = html_entity_encode("<script>");
/// // Example: "&lt;&#115;&#x63;r&#105;pt&gt;" (varies each run)
///
/// // XSS payload with entity encoding
/// let xss = html_entity_encode("<img src=x onerror=alert(1)>");
/// // Bypasses filters looking for literal "<" and ">"
///
/// // Special character encoding
/// let special = html_entity_encode("A&B<C>D");
/// // Example: "A&amp;B&lt;C&gt;D"
/// ```
pub fn html_entity_encode(input: &str) -> String {
    let mut rng = SimpleRng::new();
    let mut result = String::new();

    for c in input.chars() {
        match rng.next_u64() % 4 {
            0 => result.push(c),
            1 => result.push_str(&format!("&#{};", c as u32)),
            2 => result.push_str(&format!("&#x{:X};", c as u32)),
            _ => {
                // Named entities for common characters
                match c {
                    '<' => result.push_str("&lt;"),
                    '>' => result.push_str("&gt;"),
                    '&' => result.push_str("&amp;"),
                    '"' => result.push_str("&quot;"),
                    '\'' => result.push_str("&apos;"),
                    ' ' => result.push_str("&nbsp;"),
                    _ => result.push_str(&format!("&#{};", c as u32)),
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_encode() {
        assert_eq!(base64_encode("hello"), "aGVsbG8=");
        assert_eq!(base64_encode("test"), "dGVzdA==");
        assert_eq!(base64_encode("a"), "YQ==");
    }

    #[test]
    fn test_base64_encode_empty_string() {
        assert_eq!(base64_encode(""), "");
    }

    #[test]
    fn test_base64_encode_single_char() {
        let result = base64_encode("A");
        assert_eq!(result, "QQ==");
    }

    #[test]
    fn test_base64_encode_numbers() {
        assert_eq!(base64_encode("123"), "MTIz");
    }

    #[test]
    fn test_base64_encode_special_chars() {
        let result = base64_encode("!@#");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_base64_encode_long_string() {
        let input = "The quick brown fox jumps over the lazy dog";
        let result = base64_encode(input);
        assert!(!result.is_empty());
        assert!(result.len() > input.len());
    }

    #[test]
    fn test_base64_encode_unicode() {
        let result = base64_encode("hello 世界");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_base64_encode_binary_data() {
        let result = base64_encode("\x00\x01\x02\x03");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_base64_encode_padding() {
        assert!(base64_encode("a").ends_with("=="));
        assert!(base64_encode("ab").ends_with("="));
        assert!(!base64_encode("abc").ends_with("="));
    }

    #[test]
    fn test_base64_encode_whitespace() {
        let result = base64_encode("hello world");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_base64_encode_sql_injection() {
        let result = base64_encode("SELECT * FROM users");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_url_encode() {
        let result = url_encode("hello world");
        assert!(result.contains("%20"));

        let result2 = url_encode("test@example.com");
        assert!(result2.contains("%40"));
    }

    #[test]
    fn test_url_encode_empty_string() {
        assert_eq!(url_encode(""), "");
    }

    #[test]
    fn test_url_encode_alphanumeric() {
        assert_eq!(url_encode("abc123"), "abc123");
    }

    #[test]
    fn test_url_encode_special_chars() {
        let result = url_encode("!@#$%^&*()");
        assert!(result.contains("%"));
    }

    #[test]
    fn test_url_encode_path() {
        let result = url_encode("/path/to/file");
        assert!(result.contains("%2F"));
    }

    #[test]
    fn test_url_encode_query_string() {
        let result = url_encode("key=value&foo=bar");
        assert!(result.contains("%3D"));
        assert!(result.contains("%26"));
    }

    #[test]
    fn test_url_encode_unicode() {
        let result = url_encode("hello 世界");
        assert!(result.contains("%"));
    }

    #[test]
    fn test_url_encode_plus_sign() {
        let result = url_encode("a+b");
        assert!(result.contains("%2B"));
    }

    #[test]
    fn test_url_encode_slash() {
        let result = url_encode("a/b");
        assert!(result.contains("%2F"));
    }

    #[test]
    fn test_url_encode_question_mark() {
        let result = url_encode("what?");
        assert!(result.contains("%3F"));
    }

    #[test]
    fn test_url_encode_hash() {
        let result = url_encode("#anchor");
        assert!(result.contains("%23"));
    }

    #[test]
    fn test_hex_encode() {
        assert_eq!(hex_encode("test"), "74657374");
        assert_eq!(hex_encode("ab"), "6162");
    }

    #[test]
    fn test_hex_encode_empty_string() {
        assert_eq!(hex_encode(""), "");
    }

    #[test]
    fn test_hex_encode_single_char() {
        assert_eq!(hex_encode("A"), "41");
    }

    #[test]
    fn test_hex_encode_numbers() {
        assert_eq!(hex_encode("123"), "313233");
    }

    #[test]
    fn test_hex_encode_special_chars() {
        let result = hex_encode("!@#");
        assert!(!result.is_empty());
        assert_eq!(result.len(), 6); // 3 chars * 2 hex digits
    }

    #[test]
    fn test_hex_encode_whitespace() {
        let result = hex_encode(" ");
        assert_eq!(result, "20");
    }

    #[test]
    fn test_hex_encode_lowercase() {
        let result = hex_encode("abc");
        assert_eq!(result, "616263");
        assert!(!result.contains("A")); // Should be lowercase hex
    }

    #[test]
    fn test_hex_encode_uppercase() {
        let result = hex_encode("ABC");
        assert_eq!(result, "414243");
    }

    #[test]
    fn test_hex_encode_mixed_case_input() {
        let result = hex_encode("Hello!");
        assert!(!result.is_empty());
        assert_eq!(result.len(), 12); // 6 chars * 2 hex digits
    }

    #[test]
    fn test_hex_encode_newline() {
        let result = hex_encode("\n");
        assert_eq!(result, "0a");
    }

    #[test]
    fn test_hex_encode_mixed() {
        let result = hex_encode_mixed("ab");
        // Should contain hexadecimal encoding
        assert!(result.len() > 2);
    }

    #[test]
    fn test_hex_encode_mixed_empty_string() {
        let result = hex_encode_mixed("");
        assert_eq!(result, "");
    }

    #[test]
    fn test_hex_encode_mixed_single_char() {
        let result = hex_encode_mixed("A");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_hex_encode_mixed_formats() {
        let result = hex_encode_mixed("test");
        // Should contain mix of formats: \x, %, 0x, &#x
        assert!(!result.is_empty());
    }

    #[test]
    fn test_hex_encode_mixed_special_chars() {
        let result = hex_encode_mixed("!@#");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_hex_encode_mixed_numbers() {
        let result = hex_encode_mixed("123");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_hex_encode_mixed_whitespace() {
        let result = hex_encode_mixed("a b");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_hex_encode_mixed_long_string() {
        let result = hex_encode_mixed("hello world");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_hex_encode_mixed_xss_payload() {
        let result = hex_encode_mixed("<script>");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_hex_encode_mixed_preserves_content() {
        let result = hex_encode_mixed("abc");
        // Result should be longer due to encoding
        assert!(result.len() >= 3);
    }

    #[test]
    fn test_html_entity_encode() {
        let result = html_entity_encode("test");
        assert!(result.len() >= 4);
    }

    #[test]
    fn test_html_entity_encode_empty_string() {
        let result = html_entity_encode("");
        assert_eq!(result, "");
    }

    #[test]
    fn test_html_entity_encode_angle_brackets() {
        let result = html_entity_encode("<>");
        assert!(result.contains("lt") || result.contains("gt") || result.contains("&#"));
    }

    #[test]
    fn test_html_entity_encode_ampersand() {
        let result = html_entity_encode("&");
        // Can be plain "&", "&amp;", "&#38;", or "&#x26;"
        assert!(result.contains("amp") || result.contains("&#") || result == "&");
    }

    #[test]
    fn test_html_entity_encode_quotes() {
        let result = html_entity_encode("\"'");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_html_entity_encode_space() {
        let result = html_entity_encode(" ");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_html_entity_encode_xss_payload() {
        let result = html_entity_encode("<script>alert(1)</script>");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_html_entity_encode_mixed_content() {
        let result = html_entity_encode("Hello <world> & \"test\"");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_html_entity_encode_numbers() {
        let result = html_entity_encode("123");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_html_entity_encode_special_chars() {
        let result = html_entity_encode("!@#$%");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_html_entity_encode_unicode() {
        let result = html_entity_encode("♥");
        assert!(!result.is_empty());
    }
}
