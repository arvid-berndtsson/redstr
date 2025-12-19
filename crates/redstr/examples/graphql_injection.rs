//! GraphQL Injection and Obfuscation Example
//!
//! This example demonstrates GraphQL injection patterns and obfuscation
//! techniques for security testing. Useful for testing GraphQL API security
//! and bypassing input validation.

use redstr::{graphql_introspection_bypass, graphql_obfuscate, graphql_variable_injection};

fn main() {
    println!("=== GraphQL Injection and Obfuscation Examples ===\n");

    // Example 1: GraphQL Query Obfuscation
    println!("1. GraphQL Query Obfuscation");
    println!("   Purpose: Obfuscate GraphQL queries to bypass WAF detection");
    println!("   Use Case: Testing GraphQL security controls\n");

    let query = "query { users { id name email } }";
    println!("   Original query: {}", query);
    let obfuscated = graphql_obfuscate(query);
    println!("   Obfuscated: {}\n", obfuscated);

    // Example 2: GraphQL Variable Injection
    println!("2. GraphQL Variable Injection");
    println!("   Purpose: Test variable injection vulnerabilities");
    println!("   Use Case: Red team testing of GraphQL endpoints\n");

    let query_with_vars = "query GetUser($id: ID!) { user(id: $id) { name } }";
    println!("   Original query: {}", query_with_vars);
    let injected = graphql_variable_injection(query_with_vars);
    println!("   Injected: {}\n", injected);

    // Example 3: GraphQL Introspection Bypass
    println!("3. GraphQL Introspection Bypass");
    println!("   Purpose: Bypass introspection query restrictions");
    println!("   Use Case: Testing GraphQL schema exposure\n");

    let introspection = "__schema { types { name } }";
    println!("   Original introspection: {}", introspection);
    let bypassed = graphql_introspection_bypass(introspection);
    println!("   Bypassed: {}\n", bypassed);

    // Example 4: Complex Query Testing
    println!("4. Complex Query Obfuscation");
    println!("   Purpose: Test complex GraphQL query obfuscation");
    println!("   Use Case: Advanced security testing\n");

    let complex_query = r#"
        mutation CreateUser($input: UserInput!) {
            createUser(input: $input) {
                user {
                    id
                    name
                    email
                }
            }
        }
    "#;
    println!("   Original mutation: {}", complex_query.trim());
    let obfuscated_complex = graphql_obfuscate(complex_query);
    println!("   Obfuscated: {}\n", obfuscated_complex);

    // Example 5: Red Team Scenario
    println!("5. Red Team Scenario: Chaining Techniques");
    println!("   Purpose: Combine multiple techniques for comprehensive testing\n");

    let admin_query = "query { admin { password token } }";
    println!("   Step 1 - Original: {}", admin_query);

    let obf1 = graphql_obfuscate(admin_query);
    println!("   Step 2 - Obfuscated: {}", obf1);

    let obf2 = graphql_variable_injection(&obf1);
    println!("   Step 3 - With injection: {}", obf2);

    let obf3 = graphql_introspection_bypass(&obf2);
    println!("   Step 4 - Introspection bypass: {}\n", obf3);

    println!("=== Security Testing Tips ===");
    println!("• Use graphql_obfuscate() to test WAF and filter bypass");
    println!("• Use graphql_variable_injection() to test injection vulnerabilities");
    println!("• Use graphql_introspection_bypass() to test schema exposure");
    println!("• Combine techniques for comprehensive security testing");
    println!("\n⚠️  Use responsibly: Only test systems you own or have permission to test");
}
