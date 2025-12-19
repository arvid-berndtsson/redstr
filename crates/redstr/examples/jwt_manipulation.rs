//! JWT Token Manipulation Example
//!
//! This example demonstrates JWT (JSON Web Token) manipulation techniques
//! for security testing. Includes header manipulation, payload obfuscation,
//! algorithm confusion, and signature bypass attempts.

use redstr::{
    jwt_algorithm_confusion, jwt_header_manipulation, jwt_payload_obfuscate, jwt_signature_bypass,
};

fn main() {
    println!("=== JWT Token Manipulation Examples ===\n");

    // Example 1: JWT Header Manipulation
    println!("1. JWT Header Manipulation");
    println!("   Purpose: Modify JWT header for security testing");
    println!("   Use Case: Testing JWT validation and algorithm checks\n");

    let jwt_header = r#"{"alg":"HS256","typ":"JWT"}"#;
    println!("   Original header: {}", jwt_header);
    let modified_header = jwt_header_manipulation(jwt_header);
    println!("   Modified header: {}\n", modified_header);

    // Example 2: JWT Payload Obfuscation
    println!("2. JWT Payload Obfuscation");
    println!("   Purpose: Obfuscate JWT payload data");
    println!("   Use Case: Testing payload validation and claim verification\n");

    let jwt_payload = r#"{"sub":"1234567890","name":"John Doe","admin":true}"#;
    println!("   Original payload: {}", jwt_payload);
    let obfuscated_payload = jwt_payload_obfuscate(jwt_payload);
    println!("   Obfuscated payload: {}\n", obfuscated_payload);

    // Example 3: Algorithm Confusion Attack
    println!("3. JWT Algorithm Confusion");
    println!("   Purpose: Test algorithm confusion vulnerabilities");
    println!("   Use Case: Testing JWT signature verification bypass\n");

    let jwt_token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
    println!("   Original token: {}...", &jwt_token[..50]);
    let confused_token = jwt_algorithm_confusion(jwt_token);
    println!("   Confused token: {}...\n", &confused_token[..50]);

    // Example 4: Signature Bypass Attempts
    println!("4. JWT Signature Bypass");
    println!("   Purpose: Attempt signature bypass techniques");
    println!("   Use Case: Testing JWT signature validation\n");

    let token_to_bypass = "eyJhbGciOiJub25lIiwidHlwIjoiSldUIn0.eyJzdWIiOiIxMjM0NTY3ODkwIn0.";
    println!("   Original token: {}...", &token_to_bypass[..50]);
    let bypassed_token = jwt_signature_bypass(token_to_bypass);
    println!("   Bypass attempt: {}...\n", &bypassed_token[..50]);

    // Example 5: Red Team Scenario - Privilege Escalation
    println!("5. Red Team Scenario: JWT Privilege Escalation");
    println!("   Purpose: Test JWT-based privilege escalation");
    println!("   Use Case: Comprehensive JWT security testing\n");

    let user_payload = r#"{"sub":"user123","role":"user","admin":false}"#;
    println!("   Step 1 - Original payload: {}", user_payload);

    let obfuscated = jwt_payload_obfuscate(user_payload);
    println!("   Step 2 - Obfuscated: {}", obfuscated);

    let header = r#"{"alg":"HS256","typ":"JWT"}"#;
    let modified = jwt_header_manipulation(header);
    println!("   Step 3 - Modified header: {}", modified);

    // Example 6: Testing Different Algorithms
    println!("\n6. Testing Algorithm Variations");
    println!("   Purpose: Test various JWT algorithms");
    println!("   Use Case: Algorithm downgrade attacks\n");

    let algorithms = vec![
        r#"{"alg":"HS256","typ":"JWT"}"#,
        r#"{"alg":"RS256","typ":"JWT"}"#,
        r#"{"alg":"none","typ":"JWT"}"#,
    ];

    for alg_header in algorithms {
        println!("   Testing: {}", alg_header);
        let result = jwt_header_manipulation(alg_header);
        println!("   Result: {}\n", result);
    }

    // Example 7: Common JWT Attack Vectors
    println!("7. Common JWT Attack Vectors");
    println!("   Purpose: Generate common JWT attack patterns\n");

    println!("   a) None Algorithm Attack:");
    let none_alg = r#"{"alg":"none"}"#;
    println!("      {}", jwt_header_manipulation(none_alg));

    println!("\n   b) Algorithm Confusion (HS256 -> RS256):");
    let hs256_token = "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ0ZXN0In0.xyz";
    println!("      {}", jwt_algorithm_confusion(hs256_token));

    println!("\n   c) Payload Manipulation:");
    let payload = r#"{"iat":1516239022,"exp":1516242622}"#;
    println!("      {}", jwt_payload_obfuscate(payload));

    println!("\n   d) Signature Removal:");
    let sig_token = "header.payload.signature";
    println!("      {}\n", jwt_signature_bypass(sig_token));

    // Example 8: Blue Team Testing
    println!("8. Blue Team: JWT Validation Testing");
    println!("   Purpose: Test JWT validation mechanisms");
    println!("   Use Case: Validating security controls\n");

    let valid_token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.signature";
    println!("   Valid token: {}...", &valid_token[..50]);

    println!("\n   Test 1: Algorithm manipulation");
    println!("   {}", jwt_algorithm_confusion(valid_token));

    println!("\n   Test 2: Signature bypass");
    println!("   {}", jwt_signature_bypass(valid_token));

    let payload = r#"{"sub":"1234567890","admin":true}"#;
    println!("\n   Test 3: Payload modification");
    println!("   {}\n", jwt_payload_obfuscate(payload));

    println!("=== Security Testing Tips ===");
    println!("• Test 'none' algorithm acceptance");
    println!("• Test algorithm confusion (HS256 ↔ RS256)");
    println!("• Test signature validation (remove/modify)");
    println!("• Test payload manipulation (claims, expiry)");
    println!("• Test key confusion attacks");
    println!("• Check for weak signing keys");
    println!("\n⚠️  Use responsibly: Only test systems you own or have permission to test");
}
