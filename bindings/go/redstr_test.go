package redstr

import (
	"strings"
	"testing"
)

func TestBase64Encode(t *testing.T) {
	result := Base64Encode("hello")
	expected := "aGVsbG8="
	if result != expected {
		t.Errorf("Base64Encode failed: expected %s, got %s", expected, result)
	}
}

func TestLeetspeak(t *testing.T) {
	result := Leetspeak("password")
	// Leetspeak is non-deterministic, check for common substitutions
	if !strings.Contains(result, "4") && !strings.Contains(result, "@") &&
		!strings.Contains(result, "5") && !strings.Contains(result, "0") {
		t.Errorf("Leetspeak failed: expected substitutions in %s", result)
	}
}

func TestROT13(t *testing.T) {
	result := ROT13("Hello")
	expected := "Uryyb"
	if result != expected {
		t.Errorf("ROT13 failed: expected %s, got %s", expected, result)
	}
	
	// ROT13 is its own inverse
	result2 := ROT13(result)
	if result2 != "Hello" {
		t.Errorf("ROT13 inverse failed: expected Hello, got %s", result2)
	}
}

func TestCaseSwap(t *testing.T) {
	result := CaseSwap("Hello")
	// CaseSwap is non-deterministic (uses RNG), just check it returns something
	if len(result) != 5 {
		t.Errorf("CaseSwap failed: expected length 5, got %d", len(result))
	}
	// Should contain some mix of cases
	hasUpper := false
	hasLower := false
	for _, c := range result {
		if c >= 'A' && c <= 'Z' {
			hasUpper = true
		}
		if c >= 'a' && c <= 'z' {
			hasLower = true
		}
	}
	if !hasUpper && !hasLower {
		t.Errorf("CaseSwap should contain mixed cases: %s", result)
	}
}

func TestURLEncode(t *testing.T) {
	result := URLEncode("hello world")
	if !strings.Contains(result, "%20") {
		t.Errorf("URLEncode failed: expected %%20 in %s", result)
	}
}

func TestHexEncode(t *testing.T) {
	result := HexEncode("hi")
	expected := "6869"
	if result != expected {
		t.Errorf("HexEncode failed: expected %s, got %s", expected, result)
	}
}

func TestReverseString(t *testing.T) {
	result := ReverseString("hello")
	expected := "olleh"
	if result != expected {
		t.Errorf("ReverseString failed: expected %s, got %s", expected, result)
	}
}

func TestToCamelCase(t *testing.T) {
	result := ToCamelCase("hello world")
	expected := "helloWorld"
	if result != expected {
		t.Errorf("ToCamelCase failed: expected %s, got %s", expected, result)
	}
}

func TestToSnakeCase(t *testing.T) {
	result := ToSnakeCase("HelloWorld")
	expected := "hello_world"
	if result != expected {
		t.Errorf("ToSnakeCase failed: expected %s, got %s", expected, result)
	}
}

func TestToKebabCase(t *testing.T) {
	result := ToKebabCase("HelloWorld")
	expected := "hello-world"
	if result != expected {
		t.Errorf("ToKebabCase failed: expected %s, got %s", expected, result)
	}
}

func TestRandomUserAgent(t *testing.T) {
	result := RandomUserAgent()
	if len(result) == 0 {
		t.Error("RandomUserAgent returned empty string")
	}
	if !strings.Contains(result, "Mozilla") && !strings.Contains(result, "Chrome") {
		t.Errorf("RandomUserAgent doesn't look like a user agent: %s", result)
	}
}

func TestSSTIFrameworkVariation(t *testing.T) {
	result := SSTIFrameworkVariation("{{ code }}", "jinja2")
	if len(result) == 0 {
		t.Error("SSTIFrameworkVariation returned empty string")
	}
}

func TestHomoglyphSubstitution(t *testing.T) {
	result := HomoglyphSubstitution("admin")
	// Should contain some characters (may be substituted with homoglyphs)
	if len(result) == 0 {
		t.Error("HomoglyphSubstitution returned empty string")
	}
}

func TestDoubleCharacters(t *testing.T) {
	result := DoubleCharacters("abc")
	// DoubleCharacters is non-deterministic (randomly doubles characters)
	// Just check the result contains the original characters
	if len(result) < 3 {
		t.Errorf("DoubleCharacters result too short: %s", result)
	}
	// Should contain a, b, c
	if !strings.Contains(result, "a") || !strings.Contains(result, "b") || !strings.Contains(result, "c") {
		t.Errorf("DoubleCharacters should contain abc: %s", result)
	}
}

func TestPathTraversal(t *testing.T) {
	result := PathTraversal("etc/passwd")
	if !strings.Contains(result, "..") {
		t.Errorf("PathTraversal should contain ..: %s", result)
	}
}

func TestEmptyStringHandling(t *testing.T) {
	// Test that empty strings are handled gracefully
	tests := []func(string) string{
		Base64Encode,
		Leetspeak,
		ROT13,
		CaseSwap,
		URLEncode,
		HexEncode,
		ReverseString,
	}
	
	for i, fn := range tests {
		result := fn("")
		if result != "" && result != "=" {  // Base64 of empty is "="
			t.Errorf("Function %d failed to handle empty string: got %s", i, result)
		}
	}
}

func TestTransformBuilder(t *testing.T) {
	result := NewTransformBuilder("hello").
		Leetspeak().
		Base64().
		Build()
	
	if len(result) == 0 {
		t.Error("TransformBuilder chain returned empty string")
	}
	
	// Should be base64 encoded (contains only valid base64 chars)
	for _, c := range result {
		if !((c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z') || 
			(c >= '0' && c <= '9') || c == '+' || c == '/' || c == '=') {
			t.Errorf("TransformBuilder result not base64: %s", result)
			break
		}
	}
}

func TestTransformBuilderCaseConversions(t *testing.T) {
	result := NewTransformBuilder("hello world").
		ToCamelCase().
		Build()
	
	expected := "helloWorld"
	if result != expected {
		t.Errorf("Builder ToCamelCase failed: expected %s, got %s", expected, result)
	}
}

func TestTransformBuilderMultipleTransforms(t *testing.T) {
	// Test chaining multiple transformations with deterministic functions
	result := NewTransformBuilder("hello").
		ROT13().
		Reverse().
		Build()
	
	// ROT13("hello") = "uryyb", reversed = "byyru"
	expected := "byyru"
	if result != expected {
		t.Errorf("Builder multiple transforms failed: expected %s, got %s", expected, result)
	}
}
