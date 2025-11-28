// Security testing examples for redstr Go bindings
package main

import (
	"fmt"
	redstr "github.com/arvid-berndtsson/redstr-go"
)

func main() {
	fmt.Println("=== redstr Go Bindings - Security Testing Examples ===\n")

	// WAF Bypass Techniques
	fmt.Println("WAF Bypass:")
	sqlQuery := "SELECT * FROM users WHERE id = 1"
	fmt.Printf("  Original: %s\n", sqlQuery)
	fmt.Printf("  SQL Comment Injection: %s\n", redstr.SQLCommentInjection(sqlQuery))
	fmt.Printf("  Case Swap + URL Encode: %s\n",
		redstr.NewTransformBuilder(sqlQuery).
			CaseSwap().
			URLEncode().
			Build())
	fmt.Println()

	// XSS Evasion
	fmt.Println("XSS Evasion:")
	xssPayload := "<script>alert(1)</script>"
	fmt.Printf("  Original: %s\n", xssPayload)
	fmt.Printf("  XSS Tag Variations: %s\n", redstr.XSSTagVariations(xssPayload))
	fmt.Printf("  Case Randomization: %s\n", redstr.RandomizeCapitalization(xssPayload))
	fmt.Println()

	// Injection Testing
	fmt.Println("Injection Testing:")
	fmt.Printf("  Path Traversal: %s\n", redstr.PathTraversal("etc/passwd"))
	fmt.Printf("  Command Injection: %s\n", redstr.CommandInjection("ls"))
	fmt.Printf("  Null Byte: %s\n", redstr.NullByteInjection("file.txt"))
	fmt.Printf("  MongoDB Injection: %s\n", redstr.MongoDBInjection("username"))
	fmt.Println()

	// Phishing Domain Generation
	fmt.Println("Phishing Domain Testing:")
	domain := "paypal.com"
	fmt.Printf("  Original: %s\n", domain)
	fmt.Printf("  Typosquat: %s\n", redstr.DomainTyposquat(domain))
	fmt.Printf("  Advanced Spoof: %s\n", redstr.AdvancedDomainSpoof(domain))
	fmt.Printf("  Homoglyph: %s\n", redstr.HomoglyphSubstitution(domain))
	fmt.Println()

	// SSTI Testing
	fmt.Println("SSTI Testing:")
	template := "{{ 7*7 }}"
	fmt.Printf("  Original: %s\n", template)
	fmt.Printf("  SSTI Injection: %s\n", redstr.SSTIInjection(template))
	fmt.Printf("  Jinja2 Variation: %s\n", redstr.SSTIFrameworkVariation(template, "jinja2"))
	fmt.Printf("  Syntax Obfuscate: %s\n", redstr.SSTISyntaxObfuscate(template))
	fmt.Println()

	// Shell Command Obfuscation
	fmt.Println("Shell Command Obfuscation:")
	psCommand := "Get-Process"
	bashCommand := "ls -la"
	fmt.Printf("  PowerShell: %s -> %s\n", psCommand, redstr.PowershellObfuscate(psCommand))
	fmt.Printf("  Bash: %s -> %s\n", bashCommand, redstr.BashObfuscate(bashCommand))
	fmt.Printf("  File Path: %s\n", redstr.FilePathObfuscate("/etc/passwd"))
	fmt.Println()

	// JWT Testing
	fmt.Println("JWT Testing:")
	jwtHeader := "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9"
	fmt.Printf("  Header Manipulation: %s\n", redstr.JWTHeaderManipulation(jwtHeader))
	fmt.Printf("  Algorithm Confusion: %s\n", redstr.JWTAlgorithmConfusion("HS256"))
	fmt.Println()

	// GraphQL Testing
	fmt.Println("GraphQL Testing:")
	gqlQuery := "query { users { id name email } }"
	fmt.Printf("  Original: %s\n", gqlQuery)
	fmt.Printf("  Obfuscated: %s\n", redstr.GraphQLObfuscate(gqlQuery))
	fmt.Printf("  Introspection Bypass: %s\n", redstr.GraphQLIntrospectionBypass("__schema"))
	fmt.Println()

	// Bot Detection Evasion
	fmt.Println("Bot Detection Evasion:")
	fmt.Printf("  Random User Agent: %s\n", redstr.RandomUserAgent())
	fmt.Printf("  TLS Fingerprint Variation: %s\n", redstr.TLSFingerprintVariation("ja3_hash"))
	fmt.Printf("  Cloudflare Turnstile: %s\n", redstr.CloudflareTurnstileVariation("token"))
	fmt.Println()

	// Complex Payload Building
	fmt.Println("Complex Payload Building:")
	complexPayload := redstr.NewTransformBuilder("<script>alert('XSS')</script>").
		RandomizeCapitalization().
		HTMLEntity().
		URLEncode().
		Build()
	fmt.Printf("  Multi-step encoding: %s\n", complexPayload)
}
