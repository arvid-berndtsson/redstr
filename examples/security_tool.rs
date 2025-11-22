/// Example: Using redstr as part of a security testing tool
///
/// This demonstrates how to integrate the library into a security tool
/// for generating test payloads and obfuscated strings.
use redstr::*;

fn main() {
    println!("=== Security Tool Integration Example ===\n");

    // Red Team: Generate phishing domain variations
    println!("Red Team - Phishing Domain Generation:");
    let target_domain = "paypal.com";
    println!("  Target: {}", target_domain);
    for i in 1..=3 {
        println!("  Variant {}: {}", i, homoglyph_substitution(target_domain));
    }
    println!();

    // Blue Team: Test filter bypass techniques
    println!("Blue Team - Testing Content Filters:");
    let test_words = vec!["malware", "exploit", "password"];
    for word in test_words {
        println!("  Original: {}", word);
        println!("    Leetspeak:   {}", leetspeak(word));
        println!("    Unicode:     {}", unicode_variations(word));
        println!("    Random Cap:  {}", randomize_capitalization(word));
        println!();
    }

    // Purple Team: Generate test cases
    println!("Purple Team - Test Case Generation:");
    let test_input = "SELECT * FROM users";
    println!("  SQL Query: {}", test_input);
    println!("  Mixed Case:    {}", alternate_case(test_input));
    println!("  Mixed Encoded: {}", mixed_encoding(test_input));
    println!();

    // Payload obfuscation testing
    println!("Payload Obfuscation Examples:");
    let payload = "alert('XSS')";
    println!("  Original:  {}", payload);
    println!("  Doubled:   {}", double_characters(payload));
    println!("  Encoded:   {}", mixed_encoding(payload));
    println!();

    // Input validation testing
    println!("Input Validation Testing:");
    let normal_input = "John Doe";
    println!("  Normal:        {}", normal_input);
    println!("  Zalgo:         {}", zalgo_text(normal_input));
    println!("  Space Variants: {}", space_variants(normal_input));
    println!("  Vowel Swap:    {}", vowel_swap(normal_input));
}
