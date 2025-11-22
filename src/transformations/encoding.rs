use crate::rng::SimpleRng;

/// Encodes characters using mixed encoding formats (HTML entities, Unicode escapes).
///
/// Useful for testing encoding vulnerabilities and XSS.
///
/// # Examples
///
/// ```
/// use redstr::mixed_encoding;
/// let result = mixed_encoding("test");
/// assert!(result.contains("&#") || result.contains("\\u"));
/// ```
pub fn mixed_encoding(input: &str) -> String {
    let mut rng = SimpleRng::new();
    let mut result = String::new();

    for c in input.chars() {
        match rng.next() % 4 {
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
/// Useful for red team payload obfuscation and blue team testing of encoding detection.
///
/// # Examples
///
/// ```
/// use redstr::base64_encode;
/// let result = base64_encode("hello");
/// assert_eq!(result, "aGVsbG8=");
/// ```
pub fn base64_encode(input: &str) -> String {
    const BASE64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let bytes = input.as_bytes();
    let mut result = String::new();

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

/// Encodes text with URL/percent encoding.
///
/// Useful for red team web security testing and blue team input validation testing.
/// Properly encodes multi-byte UTF-8 characters.
///
/// # Examples
///
/// ```
/// use redstr::url_encode;
/// let result = url_encode("hello world");
/// assert!(result.contains("%20"));
/// ```
pub fn url_encode(input: &str) -> String {
    let mut result = String::new();
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
/// Useful for red team payload encoding and blue team encoding detection.
/// Uses lowercase hex digits which is the most common format.
///
/// # Examples
///
/// ```
/// use redstr::hex_encode;
/// let result = hex_encode("test");
/// assert_eq!(result, "74657374");
/// ```
pub fn hex_encode(input: &str) -> String {
    input.bytes().fold(String::new(), |mut acc, b| {
        use std::fmt::Write;
        write!(&mut acc, "{:02x}", b).unwrap();
        acc
    })
}

/// Encodes text with mixed hexadecimal formats (0x, \x, %, etc.).
///
/// Useful for red team obfuscation and blue team detection testing.
///
/// # Examples
///
/// ```
/// use redstr::hex_encode_mixed;
/// let result = hex_encode_mixed("ab");
/// assert!(result.contains("\\x") || result.contains("%") || result.contains("0x"));
/// ```
pub fn hex_encode_mixed(input: &str) -> String {
    let mut rng = SimpleRng::new();

    input
        .bytes()
        .map(|b| match rng.next() % 4 {
            0 => format!("\\x{:02x}", b),
            1 => format!("%{:02x}", b),
            2 => format!("0x{:02x}", b),
            _ => format!("&#x{:02x};", b),
        })
        .collect()
}

/// Encodes text using various HTML entity formats.
///
/// Useful for XSS testing and HTML parser bypass techniques.
///
/// # Examples
///
/// ```
/// use redstr::html_entity_encode;
/// let result = html_entity_encode("test");
/// assert!(result.contains("&#") || result.contains("&"));
/// ```
pub fn html_entity_encode(input: &str) -> String {
    let mut rng = SimpleRng::new();
    let mut result = String::new();

    for c in input.chars() {
        match rng.next() % 4 {
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
    fn test_url_encode() {
        let result = url_encode("hello world");
        assert!(result.contains("%20"));

        let result2 = url_encode("test@example.com");
        assert!(result2.contains("%40"));
    }

    #[test]
    fn test_hex_encode() {
        assert_eq!(hex_encode("test"), "74657374");
        assert_eq!(hex_encode("ab"), "6162");
    }

    #[test]
    fn test_hex_encode_mixed() {
        let result = hex_encode_mixed("ab");
        // Should contain hexadecimal encoding
        assert!(result.len() > 2);
    }

    #[test]
    fn test_html_entity_encode() {
        let result = html_entity_encode("test");
        assert!(result.len() >= 4);
    }
}
