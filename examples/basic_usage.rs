use redstr::*;

fn main() {
    println!("=== Random Cap Library Examples ===\n");

    // Example 1: Random capitalization
    let text = "Hello World";
    println!("1. Random Capitalization:");
    println!("   Input:  {}", text);
    println!("   Output: {}\n", randomize_capitalization(text));

    // Example 2: Leetspeak for filter testing
    let password = "password123";
    println!("2. Leetspeak (testing password filters):");
    println!("   Input:  {}", password);
    println!("   Output: {}\n", leetspeak(password));

    // Example 3: Homoglyph substitution for phishing tests
    let domain = "microsoft.com";
    println!("3. Homoglyph Substitution (phishing tests):");
    println!("   Input:  {}", domain);
    println!("   Output: {}\n", homoglyph_substitution(domain));

    // Example 4: ROT13 cipher
    let secret = "secret message";
    println!("4. ROT13 Cipher:");
    println!("   Input:    {}", secret);
    let encrypted = rot13(secret);
    println!("   Encoded:  {}", encrypted);
    println!("   Decoded:  {}\n", rot13(&encrypted));

    // Example 5: Case conversions
    let text = "hello world test";
    println!("5. Case Conversions:");
    println!("   Input:      {}", text);
    println!("   camelCase:  {}", to_camel_case(text));
    println!("   snake_case: {}", to_snake_case(text));
    println!("   kebab-case: {}\n", to_kebab_case(text));

    // Example 6: Unicode variations for normalization testing
    let admin = "administrator";
    println!("6. Unicode Variations (testing normalization):");
    println!("   Input:  {}", admin);
    println!("   Output: {}\n", unicode_variations(admin));

    // Example 7: Multiple transformations
    let input = "SecurityTest";
    println!("7. Multiple Transformations on '{}':", input);
    println!("   Alternate: {}", alternate_case(input));
    println!("   Inverse:   {}", inverse_case(input));
    println!("   Reverse:   {}", reverse_string(input));
    println!("   Zalgo:     {}\n", zalgo_text(input));
}
