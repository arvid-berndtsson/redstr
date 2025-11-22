/// Example: Integration patterns for security testing tools
///
/// This example demonstrates how to integrate polystr into professional
/// security testing tools like Caido, EvilJinx, urlscan.io, and bot detection systems.

use polystr::*;

fn main() {
    println!("=== Tool Integration Examples ===\n");

    // ========================================
    // Caido / Web Security Testing Proxy
    // ========================================
    println!("=== CAIDO / WEB SECURITY PROXY ===\n");
    
    println!("1. Request Fingerprint Randomization:");
    for i in 1..=3 {
        let ua = random_user_agent();
        println!("   Request #{}: {}", i, &ua[..60]);
    }
    println!();
    
    println!("2. Payload Generation with Builder:");
    let payloads = vec![
        TransformBuilder::new("<script>alert(1)</script>")
            .case_swap()
            .build(),
        TransformBuilder::new("<script>alert(1)</script>")
            .url_encode()
            .build(),
        TransformBuilder::new("document.cookie")
            .base64()
            .url_encode()
            .build(),
    ];
    for (i, payload) in payloads.iter().enumerate() {
        println!("   Payload {}: {}", i + 1, payload);
    }
    println!();

    println!("3. SQL Injection Test Cases:");
    let sql_tests = vec![
        "SELECT * FROM users WHERE id=1",
        "UNION SELECT password FROM admin",
        "DROP TABLE users",
    ];
    for test in sql_tests {
        println!("   Original: {}", test);
        println!("   Modified: {}", sql_comment_injection(test));
        println!();
    }

    // ========================================
    // EvilJinx / Phishing Framework
    // ========================================
    println!("=== EVILJINX / PHISHING FRAMEWORK ===\n");
    
    println!("4. Phishing Domain Generation:");
    let targets = vec!["paypal.com", "microsoft.com", "google.com"];
    for target in targets {
        println!("   Target: {}", target);
        println!("     Typosquat 1: {}", domain_typosquat(target));
        println!("     Typosquat 2: {}", domain_typosquat(target));
        println!("     Homoglyph:   {}", homoglyph_substitution(target));
        println!();
    }

    println!("5. Email Content Obfuscation:");
    let email_content = "Click here to verify your account";
    println!("   Original: {}", email_content);
    println!("   HTML Entities: {}", html_entity_encode(email_content));
    println!("   Unicode Variants: {}", unicode_normalize_variants(email_content));
    println!();

    println!("6. Link Obfuscation Chain:");
    let phishing_link = "https://secure-login.example.com/verify";
    let obfuscated = TransformBuilder::new(phishing_link)
        .homoglyphs()
        .build();
    println!("   Original: {}", phishing_link);
    println!("   Obfuscated: {}", obfuscated);
    println!();

    // ========================================
    // URL Scanner / Web Crawler
    // ========================================
    println!("=== URLSCAN.IO / WEB CRAWLER ===\n");
    
    println!("7. URL Normalization Testing:");
    let test_urls = vec![
        "http://example.com/path",
        "http://example.com/path?param=value",
        "http://example.com/path?param=<script>",
    ];
    for url in test_urls {
        println!("   Original:    {}", url);
        println!("   URL Encoded: {}", url_encode(url));
        println!();
    }

    println!("8. Content Analysis Preparation:");
    let suspicious_content = "eval(atob('malicious'))";
    println!("   Original: {}", suspicious_content);
    println!("   Base64:   {}", base64_encode(suspicious_content));
    println!("   Hex:      {}", hex_encode(suspicious_content));
    println!();

    // ========================================
    // Cloudflare / Bot Detection
    // ========================================
    println!("=== CLOUDFLARE BOT PROTECTION / WAF BYPASS ===\n");
    
    println!("9. User-Agent Rotation:");
    println!("   Strategy: Use random modern browser UAs");
    for i in 1..=5 {
        println!("   UA {}: {}", i, &random_user_agent()[..70]);
    }
    println!();

    println!("10. JavaScript Obfuscation:");
    let js_code = "document.cookie";
    println!("   Original: {}", js_code);
    println!("   Concat:   {}", js_string_concat(js_code));
    println!("   Encoded:  {}", html_entity_encode(js_code));
    println!();

    println!("11. Filter Bypass Techniques:");
    let blocked_word = "malicious";
    println!("   Original: {}", blocked_word);
    println!("   Leetspeak:     {}", leetspeak(blocked_word));
    println!("   Case Swap:     {}", case_swap(blocked_word));
    println!("   Homoglyphs:    {}", homoglyph_substitution(blocked_word));
    println!("   Unicode:       {}", unicode_variations(blocked_word));
    println!("   Whitespace:    {:?}", whitespace_padding(blocked_word));
    println!();

    // ========================================
    // Advanced Chaining
    // ========================================
    println!("=== ADVANCED TRANSFORMATION CHAINS ===\n");
    
    println!("12. Multi-Stage Payload:");
    let payload = "admin' OR '1'='1";
    println!("   Stage 1 (Original):  {}", payload);
    
    let stage2 = case_swap(payload);
    println!("   Stage 2 (Case Swap): {}", stage2);
    
    let stage3 = url_encode(&stage2);
    println!("   Stage 3 (URL Enc):   {}", stage3);
    
    let stage4 = base64_encode(&stage3);
    println!("   Stage 4 (Base64):    {}", stage4);
    println!();

    println!("13. Builder Pattern for Complex Payloads:");
    let complex = TransformBuilder::new("SELECT password FROM users")
        .case_swap()
        .url_encode()
        .base64()
        .build();
    println!("   Result: {}", complex);
    println!();

    // ========================================
    // Evasion Testing Suite
    // ========================================
    println!("=== EVASION TESTING SUITE ===\n");
    
    println!("14. Complete Evasion Test:");
    let test_string = "DROP TABLE";
    println!("   Target String: {}", test_string);
    println!("\n   Transformations:");
    
    let transformations = vec![
        ("Leetspeak", leetspeak(test_string)),
        ("Case Swap", case_swap(test_string)),
        ("Homoglyphs", homoglyph_substitution(test_string)),
        ("Unicode Var", unicode_variations(test_string)),
        ("Double Char", double_characters(test_string)),
        ("Space Var", space_variants(test_string)),
        ("ROT13", rot13(test_string)),
        ("Alternate", alternate_case(test_string)),
    ];
    
    for (name, result) in transformations {
        println!("     {:<12} -> {}", name, result);
    }
    println!();

    // ========================================
    // Integration Best Practices
    // ========================================
    println!("=== INTEGRATION BEST PRACTICES ===\n");
    println!("1. Use TransformBuilder for complex workflows");
    println!("2. Combine multiple transformations for better evasion");
    println!("3. Test against your own defenses before deployment");
    println!("4. Use random_user_agent() to avoid fingerprinting");
    println!("5. Chain homoglyphs + URL encoding for maximum obfuscation");
    println!("6. Store transformed payloads for replay attacks");
    println!("7. Use domain_typosquat() for comprehensive phishing campaigns");
    println!();

    println!("=== End of Tool Integration Examples ===");
}
