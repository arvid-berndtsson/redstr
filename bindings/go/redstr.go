// Package redstr provides Go bindings for the redstr Rust library.
// redstr is a red team string transformation library for offensive security operations,
// penetration testing, and evasion techniques.
package redstr

/*
#cgo LDFLAGS: -L${SRCDIR}/../../target/release -lredstr -lm -ldl -lpthread
#cgo CFLAGS: -I${SRCDIR}
#include "libredstr.h"
#include <stdlib.h>
*/
import "C"
import "unsafe"

// freeString frees a C string allocated by redstr
func freeString(ptr *C.char) {
	C.redstr_free_string(ptr)
}

// cString converts a Go string to a C string
func cString(s string) *C.char {
	return C.CString(s)
}

// goString converts a C string to a Go string and frees the C string
func goString(ptr *C.char) string {
	if ptr == nil {
		return ""
	}
	defer freeString(ptr)
	return C.GoString(ptr)
}

// Case Transformations

// RandomizeCapitalization randomly changes the capitalization of each character.
// Useful for bypassing case-sensitive filters while maintaining readability.
//
// Example:
//
//	result := redstr.RandomizeCapitalization("Hello World")
//	// Output: "HeLlO wOrLd" (varies each run)
func RandomizeCapitalization(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_randomize_capitalization(cInput))
}

// CaseSwap swaps the case of each character (uppercase becomes lowercase and vice versa).
//
// Example:
//
//	result := redstr.CaseSwap("Hello World")
//	// Output: "hELLO wORLD"
func CaseSwap(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_case_swap(cInput))
}

// AlternateCase alternates between uppercase and lowercase for each character.
//
// Example:
//
//	result := redstr.AlternateCase("hello")
//	// Output: "hElLo"
func AlternateCase(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_alternate_case(cInput))
}

// InverseCase inverts the case of the entire string.
//
// Example:
//
//	result := redstr.InverseCase("Hello World")
//	// Output: "hELLO wORLD"
func InverseCase(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_inverse_case(cInput))
}

// ToCamelCase converts a string to camelCase.
//
// Example:
//
//	result := redstr.ToCamelCase("hello world")
//	// Output: "helloWorld"
func ToCamelCase(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_to_camel_case(cInput))
}

// ToSnakeCase converts a string to snake_case.
//
// Example:
//
//	result := redstr.ToSnakeCase("HelloWorld")
//	// Output: "hello_world"
func ToSnakeCase(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_to_snake_case(cInput))
}

// ToKebabCase converts a string to kebab-case.
//
// Example:
//
//	result := redstr.ToKebabCase("HelloWorld")
//	// Output: "hello-world"
func ToKebabCase(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_to_kebab_case(cInput))
}

// Encoding Transformations

// Base64Encode encodes a string using base64 encoding.
//
// Example:
//
//	result := redstr.Base64Encode("hello")
//	// Output: "aGVsbG8="
func Base64Encode(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_base64_encode(cInput))
}

// URLEncode encodes a string using URL encoding (percent encoding).
//
// Example:
//
//	result := redstr.URLEncode("hello world")
//	// Output: "hello%20world"
func URLEncode(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_url_encode(cInput))
}

// HexEncode encodes a string to hexadecimal representation.
//
// Example:
//
//	result := redstr.HexEncode("hello")
//	// Output: "68656c6c6f"
func HexEncode(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_hex_encode(cInput))
}

// HexEncodeMixed encodes a string to mixed case hexadecimal.
//
// Example:
//
//	result := redstr.HexEncodeMixed("hello")
//	// Output: "68656C6C6F" (with random case variations)
func HexEncodeMixed(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_hex_encode_mixed(cInput))
}

// HTMLEntityEncode encodes special characters as HTML entities.
//
// Example:
//
//	result := redstr.HTMLEntityEncode("<script>")
//	// Output: "&lt;script&gt;"
func HTMLEntityEncode(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_html_entity_encode(cInput))
}

// MixedEncoding applies random encoding to create mixed encoded output.
//
// Example:
//
//	result := redstr.MixedEncoding("hello")
//	// Output: Mixed hex, URL encoding, etc.
func MixedEncoding(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_mixed_encoding(cInput))
}

// Unicode Transformations

// HomoglyphSubstitution replaces characters with visually similar Unicode characters.
// Useful for IDN homograph attacks and filter evasion.
//
// Example:
//
//	result := redstr.HomoglyphSubstitution("admin")
//	// Output: "аdmіn" (Cyrillic characters)
func HomoglyphSubstitution(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_homoglyph_substitution(cInput))
}

// UnicodeVariations applies Unicode variation selectors to characters.
//
// Example:
//
//	result := redstr.UnicodeVariations("text")
//	// Output: Unicode variation of "text"
func UnicodeVariations(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_unicode_variations(cInput))
}

// ZalgoText adds combining diacritical marks (zalgo effect).
//
// Example:
//
//	result := redstr.ZalgoText("hello")
//	// Output: "h̷̗̉e̵̘̊l̴̰̔l̷̜̈o̴̲̍"
func ZalgoText(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_zalgo_text(cInput))
}

// SpaceVariants replaces spaces with Unicode space variants.
//
// Example:
//
//	result := redstr.SpaceVariants("hello world")
//	// Output: "hello world" (with different Unicode spaces)
func SpaceVariants(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_space_variants(cInput))
}

// UnicodeNormalizeVariants applies different Unicode normalization forms.
//
// Example:
//
//	result := redstr.UnicodeNormalizeVariants("café")
//	// Output: Different normalization of "café"
func UnicodeNormalizeVariants(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_unicode_normalize_variants(cInput))
}

// Injection Testing Transformations

// SQLCommentInjection inserts SQL comments into the string for testing SQL injection filters.
//
// Example:
//
//	result := redstr.SQLCommentInjection("SELECT * FROM users")
//	// Output: "SELECT/**/\\*/**/FROM/**/users"
func SQLCommentInjection(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_sql_comment_injection(cInput))
}

// XSSTagVariations generates different XSS tag variations for testing.
//
// Example:
//
//	result := redstr.XSSTagVariations("<script>alert(1)</script>")
//	// Output: Various XSS tag variations
func XSSTagVariations(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_xss_tag_variations(cInput))
}

// NullByteInjection inserts null bytes for testing null byte injection vulnerabilities.
//
// Example:
//
//	result := redstr.NullByteInjection("file.txt")
//	// Output: "file.txt\x00.jpg"
func NullByteInjection(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_null_byte_injection(cInput))
}

// PathTraversal generates path traversal sequences.
//
// Example:
//
//	result := redstr.PathTraversal("etc/passwd")
//	// Output: "../../../etc/passwd"
func PathTraversal(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_path_traversal(cInput))
}

// CommandInjection generates command injection patterns.
//
// Example:
//
//	result := redstr.CommandInjection("ls")
//	// Output: "ls; cat /etc/passwd"
func CommandInjection(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_command_injection(cInput))
}

// MongoDBInjection generates MongoDB injection patterns.
//
// Example:
//
//	result := redstr.MongoDBInjection("username")
//	// Output: MongoDB NoSQL injection pattern
func MongoDBInjection(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_mongodb_injection(cInput))
}

// CouchDBInjection generates CouchDB injection patterns.
//
// Example:
//
//	result := redstr.CouchDBInjection("query")
//	// Output: CouchDB injection pattern
func CouchDBInjection(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_couchdb_injection(cInput))
}

// DynamoDBObfuscate obfuscates DynamoDB queries.
//
// Example:
//
//	result := redstr.DynamoDBObfuscate("query")
//	// Output: Obfuscated DynamoDB query
func DynamoDBObfuscate(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_dynamodb_obfuscate(cInput))
}

// NoSQLOperatorInjection generates NoSQL operator injection patterns.
//
// Example:
//
//	result := redstr.NoSQLOperatorInjection("field")
//	// Output: NoSQL operator injection pattern
func NoSQLOperatorInjection(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_nosql_operator_injection(cInput))
}

// SSTIInjection generates Server-Side Template Injection patterns.
//
// Example:
//
//	result := redstr.SSTIInjection("{{ 7*7 }}")
//	// Output: SSTI injection pattern
func SSTIInjection(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_ssti_injection(cInput))
}

// SSTIFrameworkVariation generates SSTI patterns for different template frameworks.
//
// Example:
//
//	result := redstr.SSTIFrameworkVariation("{{ code }}", "jinja2")
//	// Output: Framework-specific SSTI pattern
func SSTIFrameworkVariation(template, framework string) string {
	cTemplate := cString(template)
	cFramework := cString(framework)
	defer C.free(unsafe.Pointer(cTemplate))
	defer C.free(unsafe.Pointer(cFramework))
	return goString(C.redstr_ssti_framework_variation(cTemplate, cFramework))
}

// SSTISyntaxObfuscate obfuscates SSTI syntax.
//
// Example:
//
//	result := redstr.SSTISyntaxObfuscate("{{ 7*7 }}")
//	// Output: Obfuscated SSTI syntax
func SSTISyntaxObfuscate(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_ssti_syntax_obfuscate(cInput))
}

// Obfuscation Transformations

// Leetspeak converts text to leetspeak (1337 speak).
//
// Example:
//
//	result := redstr.Leetspeak("password")
//	// Output: "p@55w0rd"
func Leetspeak(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_leetspeak(cInput))
}

// ROT13 applies ROT13 cipher to the input.
//
// Example:
//
//	result := redstr.ROT13("Hello")
//	// Output: "Uryyb"
func ROT13(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_rot13(cInput))
}

// VowelSwap swaps vowels with similar-looking characters.
//
// Example:
//
//	result := redstr.VowelSwap("hello")
//	// Output: "h3ll0"
func VowelSwap(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_vowel_swap(cInput))
}

// DoubleCharacters doubles each character in the string.
//
// Example:
//
//	result := redstr.DoubleCharacters("abc")
//	// Output: "aabbcc"
func DoubleCharacters(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_double_characters(cInput))
}

// ReverseString reverses the input string.
//
// Example:
//
//	result := redstr.ReverseString("hello")
//	// Output: "olleh"
func ReverseString(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_reverse_string(cInput))
}

// WhitespacePadding adds random whitespace padding.
//
// Example:
//
//	result := redstr.WhitespacePadding("hello")
//	// Output: "  hello   " (with random spacing)
func WhitespacePadding(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_whitespace_padding(cInput))
}

// JSStringConcat converts a string to JavaScript string concatenation.
//
// Example:
//
//	result := redstr.JSStringConcat("alert")
//	// Output: "'al'+'ert'"
func JSStringConcat(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_js_string_concat(cInput))
}

// Phishing Transformations

// DomainTyposquat generates typosquatting variations of a domain.
//
// Example:
//
//	result := redstr.DomainTyposquat("google.com")
//	// Output: "gooogle.com" or similar typo
func DomainTyposquat(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_domain_typosquat(cInput))
}

// AdvancedDomainSpoof generates advanced domain spoofing patterns.
//
// Example:
//
//	result := redstr.AdvancedDomainSpoof("paypal.com")
//	// Output: Advanced IDN homograph attack domain
func AdvancedDomainSpoof(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_advanced_domain_spoof(cInput))
}

// EmailObfuscation obfuscates email addresses.
//
// Example:
//
//	result := redstr.EmailObfuscation("user@example.com")
//	// Output: Obfuscated email address
func EmailObfuscation(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_email_obfuscation(cInput))
}

// URLShorteningPattern generates URL shortening patterns.
//
// Example:
//
//	result := redstr.URLShorteningPattern("https://example.com/path")
//	// Output: Short URL pattern
func URLShorteningPattern(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_url_shortening_pattern(cInput))
}

// Bot Detection Transformations

// RandomUserAgent generates a random user agent string.
//
// Example:
//
//	result := redstr.RandomUserAgent()
//	// Output: Random user agent string
func RandomUserAgent() string {
	return goString(C.redstr_random_user_agent())
}

// TLSFingerprintVariation generates TLS fingerprint variations.
//
// Example:
//
//	result := redstr.TLSFingerprintVariation("ja3_hash")
//	// Output: TLS fingerprint variation
func TLSFingerprintVariation(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_tls_fingerprint_variation(cInput))
}

// AcceptLanguageVariation generates Accept-Language header variations.
//
// Example:
//
//	result := redstr.AcceptLanguageVariation("en-US")
//	// Output: Varied Accept-Language header
func AcceptLanguageVariation(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_accept_language_variation(cInput))
}

// CloudflareChallenge Variation generates Cloudflare challenge variations.
//
// Example:
//
//	result := redstr.CloudflareChallengeVariation("challenge")
//	// Output: Cloudflare challenge variation
func CloudflareChallengeVariation(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_cloudflare_challenge_variation(cInput))
}

// HTTP2HeaderOrder generates HTTP/2 header order variations.
//
// Example:
//
//	result := redstr.HTTP2HeaderOrder("headers")
//	// Output: HTTP/2 header order variation
func HTTP2HeaderOrder(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_http2_header_order(cInput))
}

// Cloudflare-specific Transformations

// CloudflareTurnstileVariation generates Cloudflare Turnstile challenge variations.
//
// Example:
//
//	result := redstr.CloudflareTurnstileVariation("token")
//	// Output: Turnstile challenge variation
func CloudflareTurnstileVariation(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_cloudflare_turnstile_variation(cInput))
}

// CloudflareChallengeResponse generates Cloudflare challenge response patterns.
//
// Example:
//
//	result := redstr.CloudflareChallengeResponse("challenge")
//	// Output: Challenge response pattern
func CloudflareChallengeResponse(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_cloudflare_challenge_response(cInput))
}

// TLSHandshakePattern generates TLS handshake patterns.
//
// Example:
//
//	result := redstr.TLSHandshakePattern("handshake")
//	// Output: TLS handshake pattern
func TLSHandshakePattern(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_tls_handshake_pattern(cInput))
}

// CanvasFingerprintVariation generates canvas fingerprint variations.
//
// Example:
//
//	result := redstr.CanvasFingerprintVariation("canvas_data")
//	// Output: Canvas fingerprint variation
func CanvasFingerprintVariation(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_canvas_fingerprint_variation(cInput))
}

// WebGLFingerprintObfuscate obfuscates WebGL fingerprints.
//
// Example:
//
//	result := redstr.WebGLFingerprintObfuscate("webgl_data")
//	// Output: Obfuscated WebGL fingerprint
func WebGLFingerprintObfuscate(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_webgl_fingerprint_obfuscate(cInput))
}

// FontFingerprintConsistency ensures font fingerprint consistency.
//
// Example:
//
//	result := redstr.FontFingerprintConsistency("font_data")
//	// Output: Consistent font fingerprint
func FontFingerprintConsistency(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_font_fingerprint_consistency(cInput))
}

// Web Security Transformations

// HTTPHeaderVariation generates HTTP header variations.
//
// Example:
//
//	result := redstr.HTTPHeaderVariation("User-Agent: Mozilla")
//	// Output: HTTP header variation
func HTTPHeaderVariation(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_http_header_variation(cInput))
}

// APIEndpointVariation generates API endpoint variations.
//
// Example:
//
//	result := redstr.APIEndpointVariation("/api/users")
//	// Output: API endpoint variation
func APIEndpointVariation(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_api_endpoint_variation(cInput))
}

// GraphQLObfuscate obfuscates GraphQL queries.
//
// Example:
//
//	result := redstr.GraphQLObfuscate("query { users }")
//	// Output: Obfuscated GraphQL query
func GraphQLObfuscate(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_graphql_obfuscate(cInput))
}

// SessionTokenVariation generates session token variations.
//
// Example:
//
//	result := redstr.SessionTokenVariation("token123")
//	// Output: Session token variation
func SessionTokenVariation(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_session_token_variation(cInput))
}

// GraphQLVariableInjection generates GraphQL variable injection patterns.
//
// Example:
//
//	result := redstr.GraphQLVariableInjection("$var")
//	// Output: GraphQL variable injection pattern
func GraphQLVariableInjection(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_graphql_variable_injection(cInput))
}

// GraphQLIntrospectionBypass generates introspection bypass patterns.
//
// Example:
//
//	result := redstr.GraphQLIntrospectionBypass("__schema")
//	// Output: Introspection bypass pattern
func GraphQLIntrospectionBypass(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_graphql_introspection_bypass(cInput))
}

// JWTHeaderManipulation manipulates JWT headers.
//
// Example:
//
//	result := redstr.JWTHeaderManipulation("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9")
//	// Output: Manipulated JWT header
func JWTHeaderManipulation(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_jwt_header_manipulation(cInput))
}

// JWTPayloadObfuscate obfuscates JWT payloads.
//
// Example:
//
//	result := redstr.JWTPayloadObfuscate("payload")
//	// Output: Obfuscated JWT payload
func JWTPayloadObfuscate(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_jwt_payload_obfuscate(cInput))
}

// JWTAlgorithmConfusion generates JWT algorithm confusion attacks.
//
// Example:
//
//	result := redstr.JWTAlgorithmConfusion("HS256")
//	// Output: Algorithm confusion pattern
func JWTAlgorithmConfusion(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_jwt_algorithm_confusion(cInput))
}

// JWTSignatureBypass generates JWT signature bypass patterns.
//
// Example:
//
//	result := redstr.JWTSignatureBypass("signature")
//	// Output: Signature bypass pattern
func JWTSignatureBypass(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_jwt_signature_bypass(cInput))
}

// Shell Transformations

// PowershellObfuscate obfuscates PowerShell commands.
//
// Example:
//
//	result := redstr.PowershellObfuscate("Get-Process")
//	// Output: Obfuscated PowerShell command
func PowershellObfuscate(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_powershell_obfuscate(cInput))
}

// BashObfuscate obfuscates Bash commands.
//
// Example:
//
//	result := redstr.BashObfuscate("ls -la")
//	// Output: Obfuscated Bash command
func BashObfuscate(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_bash_obfuscate(cInput))
}

// EnvVarObfuscate obfuscates environment variables.
//
// Example:
//
//	result := redstr.EnvVarObfuscate("PATH")
//	// Output: Obfuscated environment variable
func EnvVarObfuscate(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_env_var_obfuscate(cInput))
}

// FilePathObfuscate obfuscates file paths.
//
// Example:
//
//	result := redstr.FilePathObfuscate("/etc/passwd")
//	// Output: Obfuscated file path
func FilePathObfuscate(input string) string {
	cInput := cString(input)
	defer C.free(unsafe.Pointer(cInput))
	return goString(C.redstr_file_path_obfuscate(cInput))
}
