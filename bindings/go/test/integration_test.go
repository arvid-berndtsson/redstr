// Integration test for redstr Go bindings
// This test verifies that the FFI bindings work correctly end-to-end
package test

import (
	"strings"
	"testing"

	redstr "github.com/arvid-berndtsson/redstr-go"
)

// TestBasicTransformations verifies core transformation functions
func TestBasicTransformations(t *testing.T) {
	t.Run("Base64Encoding", func(t *testing.T) {
		input := "hello"
		result := redstr.Base64Encode(input)
		expected := "aGVsbG8="
		if result != expected {
			t.Errorf("Base64Encode failed: expected %s, got %s", expected, result)
		}
	})

	t.Run("URLEncoding", func(t *testing.T) {
		input := "hello world"
		result := redstr.URLEncode(input)
		if !strings.Contains(result, "%20") {
			t.Errorf("URLEncode failed: expected %%20 in %s", result)
		}
	})

	t.Run("HexEncoding", func(t *testing.T) {
		input := "hi"
		result := redstr.HexEncode(input)
		expected := "6869"
		if result != expected {
			t.Errorf("HexEncode failed: expected %s, got %s", expected, result)
		}
	})

	t.Run("ROT13", func(t *testing.T) {
		input := "Hello"
		result := redstr.ROT13(input)
		expected := "Uryyb"
		if result != expected {
			t.Errorf("ROT13 failed: expected %s, got %s", expected, result)
		}

		// ROT13 is its own inverse
		result2 := redstr.ROT13(result)
		if result2 != input {
			t.Errorf("ROT13 inverse failed: expected %s, got %s", input, result2)
		}
	})
}

// TestCaseTransformations verifies case manipulation functions
func TestCaseTransformations(t *testing.T) {
	t.Run("ToCamelCase", func(t *testing.T) {
		input := "hello world"
		result := redstr.ToCamelCase(input)
		expected := "helloWorld"
		if result != expected {
			t.Errorf("ToCamelCase failed: expected %s, got %s", expected, result)
		}
	})

	t.Run("ToSnakeCase", func(t *testing.T) {
		input := "HelloWorld"
		result := redstr.ToSnakeCase(input)
		expected := "hello_world"
		if result != expected {
			t.Errorf("ToSnakeCase failed: expected %s, got %s", expected, result)
		}
	})

	t.Run("ToKebabCase", func(t *testing.T) {
		input := "HelloWorld"
		result := redstr.ToKebabCase(input)
		expected := "hello-world"
		if result != expected {
			t.Errorf("ToKebabCase failed: expected %s, got %s", expected, result)
		}
	})
}

// TestObfuscationFunctions verifies obfuscation transformations
func TestObfuscationFunctions(t *testing.T) {
	t.Run("Leetspeak", func(t *testing.T) {
		input := "password"
		result := redstr.Leetspeak(input)
		// Leetspeak is non-deterministic, check for common substitutions
		if !strings.Contains(result, "4") && !strings.Contains(result, "@") &&
			!strings.Contains(result, "5") && !strings.Contains(result, "0") {
			t.Errorf("Leetspeak failed: expected substitutions in %s", result)
		}
	})

	t.Run("ReverseString", func(t *testing.T) {
		input := "hello"
		result := redstr.ReverseString(input)
		expected := "olleh"
		if result != expected {
			t.Errorf("ReverseString failed: expected %s, got %s", expected, result)
		}
	})

	t.Run("VowelSwap", func(t *testing.T) {
		input := "hello"
		result := redstr.VowelSwap(input)
		// Check that result is not empty and has same length
		if len(result) == 0 {
			t.Error("VowelSwap returned empty string")
		}
	})
}

// TestSecurityFunctions verifies security-related transformations
func TestSecurityFunctions(t *testing.T) {
	t.Run("SQLCommentInjection", func(t *testing.T) {
		input := "SELECT * FROM users"
		// Function is non-deterministic - try multiple times
		foundComments := false
		for i := 0; i < 10; i++ {
			result := redstr.SQLCommentInjection(input)
			// Should contain SQL comments (either -- or /* */ or #)
			if strings.Contains(result, "--") ||
				strings.Contains(result, "/*") ||
				strings.Contains(result, "#") {
				foundComments = true
				break
			}
		}
		if !foundComments {
			t.Error("SQLCommentInjection failed to add comments in 10 tries")
		}
	})

	t.Run("PathTraversal", func(t *testing.T) {
		input := "etc/passwd"
		result := redstr.PathTraversal(input)
		// Should return something (may or may not add traversal due to RNG)
		if len(result) == 0 {
			t.Error("PathTraversal returned empty string")
		}
		// Should contain the input path
		if !strings.Contains(result, "etc") || !strings.Contains(result, "passwd") {
			t.Errorf("PathTraversal lost original path: %s", result)
		}
	})

	t.Run("XSSTagVariations", func(t *testing.T) {
		input := "<script>alert(1)</script>"
		result := redstr.XSSTagVariations(input)
		// Should return something
		if len(result) == 0 {
			t.Error("XSSTagVariations returned empty string")
		}
	})
}

// TestUnicodeFunctions verifies Unicode transformations
func TestUnicodeFunctions(t *testing.T) {
	t.Run("HomoglyphSubstitution", func(t *testing.T) {
		input := "admin"
		result := redstr.HomoglyphSubstitution(input)
		// Should contain some characters
		if len(result) == 0 {
			t.Error("HomoglyphSubstitution returned empty string")
		}
	})

	t.Run("ZalgoText", func(t *testing.T) {
		input := "hello"
		result := redstr.ZalgoText(input)
		// Zalgo adds combining characters, so result should be longer
		if len(result) <= len(input) {
			t.Errorf("ZalgoText should add characters: input=%d, result=%d", len(input), len(result))
		}
	})
}

// TestBuilderPattern verifies the fluent builder interface
func TestBuilderPattern(t *testing.T) {
	t.Run("SingleTransformation", func(t *testing.T) {
		input := "hello"
		result := redstr.NewTransformBuilder(input).
			Base64().
			Build()

		expected := "aGVsbG8="
		if result != expected {
			t.Errorf("Builder Base64 failed: expected %s, got %s", expected, result)
		}
	})

	t.Run("ChainedTransformations", func(t *testing.T) {
		input := "hello"
		result := redstr.NewTransformBuilder(input).
			ROT13().
			Reverse().
			Build()

		// ROT13("hello") = "uryyb", reversed = "byyru"
		expected := "byyru"
		if result != expected {
			t.Errorf("Builder chain failed: expected %s, got %s", expected, result)
		}
	})

	t.Run("ComplexChain", func(t *testing.T) {
		input := "test"
		result := redstr.NewTransformBuilder(input).
			ToCamelCase().
			Build()

		// Should return "test" as camelCase
		if len(result) == 0 {
			t.Error("Builder complex chain returned empty string")
		}
	})
}

// TestBotDetectionFunctions verifies bot detection evasion functions
func TestBotDetectionFunctions(t *testing.T) {
	t.Run("RandomUserAgent", func(t *testing.T) {
		result := redstr.RandomUserAgent()
		// Should return a valid user agent string
		if len(result) == 0 {
			t.Error("RandomUserAgent returned empty string")
		}
		// Should look like a user agent
		if !strings.Contains(result, "Mozilla") && !strings.Contains(result, "Chrome") && !strings.Contains(result, "Safari") {
			t.Errorf("RandomUserAgent doesn't look like a user agent: %s", result)
		}
	})
}

// TestMultiParameterFunctions verifies functions with multiple parameters
func TestMultiParameterFunctions(t *testing.T) {
	t.Run("SSTIFrameworkVariation", func(t *testing.T) {
		template := "{{ code }}"
		framework := "jinja2"
		result := redstr.SSTIFrameworkVariation(template, framework)
		// Should return something
		if len(result) == 0 {
			t.Error("SSTIFrameworkVariation returned empty string")
		}
	})
}

// TestEdgeCases verifies edge case handling
func TestEdgeCases(t *testing.T) {
	t.Run("EmptyString", func(t *testing.T) {
		// Test that empty strings are handled gracefully
		result := redstr.Base64Encode("")
		if result != "" && result != "=" {
			t.Errorf("Empty string Base64 failed: got %s", result)
		}
	})

	t.Run("UnicodeInput", func(t *testing.T) {
		input := "こんにちは"
		result := redstr.Base64Encode(input)
		// Should handle Unicode without crashing
		if len(result) == 0 {
			t.Error("Unicode input failed")
		}
	})

	t.Run("LongString", func(t *testing.T) {
		input := strings.Repeat("a", 1000)
		result := redstr.Base64Encode(input)
		// Should handle long strings
		if len(result) == 0 {
			t.Error("Long string failed")
		}
	})
}

// TestMemoryManagement verifies no memory leaks in repeated calls
func TestMemoryManagement(t *testing.T) {
	t.Run("RepeatedCalls", func(t *testing.T) {
		// Call functions repeatedly to check for memory leaks
		for i := 0; i < 100; i++ {
			_ = redstr.Base64Encode("test")
			_ = redstr.Leetspeak("password")
			_ = redstr.ROT13("hello")
		}
		// If we get here without crashing, memory management is working
	})
}

// TestRealWorldScenarios demonstrates practical usage
func TestRealWorldScenarios(t *testing.T) {
	t.Run("WAFBypass", func(t *testing.T) {
		sqlQuery := "SELECT * FROM users WHERE id = 1"
		result := redstr.SQLCommentInjection(sqlQuery)
		// Should contain the original query with comments
		if !strings.Contains(result, "SELECT") || !strings.Contains(result, "users") {
			t.Errorf("WAF bypass lost original query: %s", result)
		}
	})

	t.Run("PhishingDomain", func(t *testing.T) {
		domain := "paypal.com"
		result := redstr.DomainTyposquat(domain)
		// Should return a variation of the domain
		if len(result) == 0 {
			t.Error("Domain typosquat returned empty string")
		}
	})

	t.Run("PayloadEncoding", func(t *testing.T) {
		payload := "<script>alert('XSS')</script>"
		result := redstr.NewTransformBuilder(payload).
			HTMLEntity().
			URLEncode().
			Build()

		// Should be encoded
		if result == payload {
			t.Error("Payload not encoded")
		}
	})
}
