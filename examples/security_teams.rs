/// Example: Comprehensive security testing scenarios for Red, Blue, and Purple teams
///
/// This example demonstrates how to use polystr transformations for various
/// security testing scenarios across different security team roles.

use polystr::*;

fn main() {
    println!("=== Security Team Use Cases ===\n");

    // Red Team: Phishing and Social Engineering
    println!("=== RED TEAM: Phishing and Social Engineering ===");
    println!("\n1. Domain Spoofing with Homoglyphs:");
    let legitimate_domains = vec!["paypal.com", "microsoft.com", "google.com"];
    for domain in legitimate_domains {
        println!("  Original: {}", domain);
        for i in 1..=3 {
            let spoofed = homoglyph_substitution(domain);
            println!("  Variant {}: {}", i, spoofed);
        }
        println!();
    }

    // Red Team: Filter Evasion
    println!("=== RED TEAM: Filter and WAF Evasion ===");
    println!("\n2. Content Filter Bypass:");
    let blocked_words = vec!["malware", "exploit", "hack", "password"];
    for word in blocked_words {
        println!("  Original: {}", word);
        println!("    Leetspeak:   {}", leetspeak(word));
        println!("    Case Swap:   {}", case_swap(word));
        println!("    Unicode:     {}", unicode_variations(word));
        println!("    Double Char: {}", double_characters(word));
        println!();
    }

    println!("3. SQL Injection Evasion:");
    let sql_query = "SELECT password FROM users WHERE username = 'admin'";
    println!("  Original: {}", sql_query);
    println!("  With comments: {}", sql_comment_injection(sql_query));
    println!("  Case swapped:  {}", case_swap(sql_query));
    println!();

    println!("4. XSS Filter Evasion:");
    let xss_payloads = vec![
        "<script>alert('XSS')</script>",
        "<img src=x onerror=alert(1)>",
        "<svg onload=alert(1)>",
    ];
    for payload in xss_payloads {
        println!("  Original: {}", payload);
        println!("  Variation: {}", xss_tag_variations(payload));
        println!();
    }

    println!("5. Command Injection Testing:");
    let commands = vec!["ping 127.0.0.1", "cat /etc/passwd", "ls -la"];
    for cmd in commands {
        println!("  Original: {}", cmd);
        println!("  Injected: {}", command_injection(cmd));
        println!();
    }

    // Red Team: Payload Obfuscation
    println!("=== RED TEAM: Payload Obfuscation ===");
    println!("\n6. Encoding Obfuscation:");
    let payload = "alert('XSS')";
    println!("  Original:    {}", payload);
    println!("  Base64:      {}", base64_encode(payload));
    println!("  URL Encoded: {}", url_encode(payload));
    println!("  Hex:         {}", hex_encode(payload));
    println!("  Hex Mixed:   {}", hex_encode_mixed(payload));
    println!("  Mixed Enc:   {}", mixed_encoding(payload));
    println!();

    println!("7. Path Traversal:");
    let paths = vec!["/etc/passwd", "/var/www/html/index.php", "../../config.ini"];
    for path in paths {
        println!("  Original: {}", path);
        println!("  Traversal: {}", path_traversal(path));
        println!();
    }

    println!("8. Null Byte Injection:");
    let files = vec!["upload.jpg", "document.pdf", "script.php"];
    for file in files {
        println!("  Original: {}", file);
        println!("  Injected: {}", null_byte_injection(file));
        println!();
    }

    // Blue Team: Detection Testing
    println!("=== BLUE TEAM: Detection and Validation Testing ===");
    println!("\n9. Testing Content Filters:");
    let test_input = "This contains malicious content";
    println!("  Test string: {}", test_input);
    println!("  Filters should catch these variations:");
    println!("    1. {}", leetspeak(test_input));
    println!("    2. {}", case_swap(test_input));
    println!("    3. {}", unicode_variations(test_input));
    println!("    4. {}", alternate_case(test_input));
    println!();

    println!("10. Testing Input Validation:");
    let user_input = "John Doe <admin>";
    println!("  Normal input: {}", user_input);
    println!("  System should handle:");
    println!("    Unicode:   {}", unicode_variations(user_input));
    println!("    Zalgo:     {}", zalgo_text(user_input));
    println!("    Spaces:    {}", space_variants(user_input));
    println!();

    println!("11. Testing URL Parsers:");
    let url = "https://example.com/path?param=value";
    println!("  Normal URL: {}", url);
    println!("  Parser should handle: {}", url_encode(url));
    println!();

    println!("12. Testing Path Validators:");
    let safe_path = "/home/user/document.txt";
    println!("  Safe path: {}", safe_path);
    println!("  Should reject: {}", path_traversal(safe_path));
    println!();

    // Blue Team: Security Monitoring
    println!("=== BLUE TEAM: Security Monitoring ===");
    println!("\n13. Log Analysis Testing:");
    let log_entry = "User admin logged in from 192.168.1.1";
    println!("  Normal log: {}", log_entry);
    println!("  Logs should normalize:");
    println!("    1. {}", unicode_variations(log_entry));
    println!("    2. {}", mixed_encoding(log_entry));
    println!();

    // Purple Team: Collaborative Testing
    println!("=== PURPLE TEAM: Collaborative Testing ===");
    println!("\n14. Baseline Test Cases:");
    let baseline_payloads = vec![
        ("XSS", "<script>alert(1)</script>"),
        ("SQLi", "' OR '1'='1"),
        ("Command", "; cat /etc/passwd"),
        ("Path", "../../../etc/passwd"),
    ];
    
    println!("  Generating test cases for security controls:");
    for (attack_type, payload) in baseline_payloads {
        println!("  {} Attack:", attack_type);
        println!("    Original:    {}", payload);
        println!("    Obfuscated:  {}", case_swap(payload));
        println!("    Encoded:     {}", url_encode(payload));
        println!();
    }

    println!("15. Training Examples:");
    println!("  Phishing Domain Detection Training:");
    let training_domain = "secure-login.paypal.com";
    println!("    Legitimate: {}", training_domain);
    println!("    Spoofed 1:  {}", homoglyph_substitution(training_domain));
    println!("    Spoofed 2:  {}", homoglyph_substitution(training_domain));
    println!();

    println!("16. Coverage Testing:");
    let critical_input = "SELECT * FROM users WHERE admin=1";
    println!("  Testing detection coverage for: {}", critical_input);
    println!("  Transformation variants to test:");
    println!("    1. Case variation:  {}", case_swap(critical_input));
    println!("    2. With comments:   {}", sql_comment_injection(critical_input));
    println!("    3. URL encoded:     {}", url_encode(critical_input));
    println!("    4. Hex encoded:     {}", hex_encode(critical_input));
    println!("    5. Mixed encoding:  {}", mixed_encoding(critical_input));
    println!();

    // Advanced Combinations
    println!("=== ADVANCED: Multiple Transformations ===");
    println!("\n17. Stacked Transformations:");
    let sensitive_data = "admin password";
    println!("  Original: {}", sensitive_data);
    let step1 = leetspeak(sensitive_data);
    println!("  Step 1 (Leetspeak): {}", step1);
    let step2 = base64_encode(&step1);
    println!("  Step 2 (Base64):    {}", step2);
    let step3 = url_encode(&step2);
    println!("  Step 3 (URL):       {}", step3);
    println!();

    println!("18. Tool Validation:");
    let obfuscated_sql = case_swap("SELECT * FROM users");
    let encoded_sql = url_encode("SELECT * FROM users");
    let payloads = vec![
        ("Normal", "SELECT * FROM users".to_string()),
        ("Obfuscated", obfuscated_sql),
        ("Encoded", encoded_sql),
    ];
    println!("  Testing detection tools against:");
    for (variant_type, payload) in payloads {
        println!("    {}: {}", variant_type, payload);
    }
    println!();

    println!("=== End of Security Team Examples ===");
}
