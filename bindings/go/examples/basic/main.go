// Basic usage examples for redstr Go bindings
package main

import (
	"fmt"
	redstr "github.com/arvid-berndtsson/redstr-go"
)

func main() {
	fmt.Println("=== redstr Go Bindings - Basic Examples ===")
	fmt.Println()

	// Case transformations
	fmt.Println("Case Transformations:")
	fmt.Printf("  Random Capitalization: %s\n", redstr.RandomizeCapitalization("Hello World"))
	fmt.Printf("  Case Swap: %s\n", redstr.CaseSwap("Hello World"))
	fmt.Printf("  Camel Case: %s\n", redstr.ToCamelCase("hello world"))
	fmt.Printf("  Snake Case: %s\n", redstr.ToSnakeCase("HelloWorld"))
	fmt.Printf("  Kebab Case: %s\n", redstr.ToKebabCase("HelloWorld"))
	fmt.Println()

	// Encoding
	fmt.Println("Encoding:")
	fmt.Printf("  Base64: %s\n", redstr.Base64Encode("hello"))
	fmt.Printf("  URL Encode: %s\n", redstr.URLEncode("hello world"))
	fmt.Printf("  Hex Encode: %s\n", redstr.HexEncode("hi"))
	fmt.Printf("  HTML Entity: %s\n", redstr.HTMLEntityEncode("<script>"))
	fmt.Println()

	// Obfuscation
	fmt.Println("Obfuscation:")
	fmt.Printf("  Leetspeak: %s\n", redstr.Leetspeak("password"))
	fmt.Printf("  ROT13: %s\n", redstr.ROT13("Hello"))
	fmt.Printf("  Reverse: %s\n", redstr.ReverseString("hello"))
	fmt.Printf("  Double Chars: %s\n", redstr.DoubleCharacters("abc"))
	fmt.Println()

	// Unicode
	fmt.Println("Unicode Transformations:")
	fmt.Printf("  Homoglyphs: %s\n", redstr.HomoglyphSubstitution("admin"))
	fmt.Printf("  Zalgo: %s\n", redstr.ZalgoText("hello"))
	fmt.Println()

	// Builder pattern
	fmt.Println("Builder Pattern:")
	result := redstr.NewTransformBuilder("SELECT * FROM users").
		CaseSwap().
		Base64().
		Build()
	fmt.Printf("  Chained transformations: %s\n", result)
	fmt.Println()

	// Random user agent
	fmt.Println("Random User Agent:")
	fmt.Printf("  %s\n", redstr.RandomUserAgent())
}
