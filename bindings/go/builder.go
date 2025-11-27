package redstr

// TransformBuilder provides a fluent interface for chaining multiple transformations.
type TransformBuilder struct {
	text string
}

// NewTransformBuilder creates a new transform builder with the given input text.
//
// Example:
//
//	builder := redstr.NewTransformBuilder("hello world")
//	result := builder.Leetspeak().Base64().Build()
func NewTransformBuilder(input string) *TransformBuilder {
	return &TransformBuilder{text: input}
}

// Build returns the final transformed string.
func (tb *TransformBuilder) Build() string {
	return tb.text
}

// Case Transformations

// RandomizeCapitalization applies random capitalization.
func (tb *TransformBuilder) RandomizeCapitalization() *TransformBuilder {
	tb.text = RandomizeCapitalization(tb.text)
	return tb
}

// CaseSwap swaps the case of characters.
func (tb *TransformBuilder) CaseSwap() *TransformBuilder {
	tb.text = CaseSwap(tb.text)
	return tb
}

// AlternateCase alternates between uppercase and lowercase.
func (tb *TransformBuilder) AlternateCase() *TransformBuilder {
	tb.text = AlternateCase(tb.text)
	return tb
}

// InverseCase inverts the case.
func (tb *TransformBuilder) InverseCase() *TransformBuilder {
	tb.text = InverseCase(tb.text)
	return tb
}

// ToCamelCase converts to camelCase.
func (tb *TransformBuilder) ToCamelCase() *TransformBuilder {
	tb.text = ToCamelCase(tb.text)
	return tb
}

// ToSnakeCase converts to snake_case.
func (tb *TransformBuilder) ToSnakeCase() *TransformBuilder {
	tb.text = ToSnakeCase(tb.text)
	return tb
}

// ToKebabCase converts to kebab-case.
func (tb *TransformBuilder) ToKebabCase() *TransformBuilder {
	tb.text = ToKebabCase(tb.text)
	return tb
}

// Encoding Transformations

// Base64 encodes the text using base64.
func (tb *TransformBuilder) Base64() *TransformBuilder {
	tb.text = Base64Encode(tb.text)
	return tb
}

// URLEncode encodes the text using URL encoding.
func (tb *TransformBuilder) URLEncode() *TransformBuilder {
	tb.text = URLEncode(tb.text)
	return tb
}

// HexEncode encodes the text to hexadecimal.
func (tb *TransformBuilder) HexEncode() *TransformBuilder {
	tb.text = HexEncode(tb.text)
	return tb
}

// HexEncodeMixed encodes the text to mixed case hexadecimal.
func (tb *TransformBuilder) HexEncodeMixed() *TransformBuilder {
	tb.text = HexEncodeMixed(tb.text)
	return tb
}

// HTMLEntity encodes special characters as HTML entities.
func (tb *TransformBuilder) HTMLEntity() *TransformBuilder {
	tb.text = HTMLEntityEncode(tb.text)
	return tb
}

// MixedEncoding applies mixed encoding.
func (tb *TransformBuilder) MixedEncoding() *TransformBuilder {
	tb.text = MixedEncoding(tb.text)
	return tb
}

// Unicode Transformations

// Homoglyphs replaces characters with visually similar Unicode characters.
func (tb *TransformBuilder) Homoglyphs() *TransformBuilder {
	tb.text = HomoglyphSubstitution(tb.text)
	return tb
}

// UnicodeVariations applies Unicode variation selectors.
func (tb *TransformBuilder) UnicodeVariations() *TransformBuilder {
	tb.text = UnicodeVariations(tb.text)
	return tb
}

// Zalgo adds combining diacritical marks.
func (tb *TransformBuilder) Zalgo() *TransformBuilder {
	tb.text = ZalgoText(tb.text)
	return tb
}

// SpaceVariants replaces spaces with Unicode space variants.
func (tb *TransformBuilder) SpaceVariants() *TransformBuilder {
	tb.text = SpaceVariants(tb.text)
	return tb
}

// UnicodeNormalize applies different Unicode normalization forms.
func (tb *TransformBuilder) UnicodeNormalize() *TransformBuilder {
	tb.text = UnicodeNormalizeVariants(tb.text)
	return tb
}

// Injection Testing Transformations

// SQLComment inserts SQL comments.
func (tb *TransformBuilder) SQLComment() *TransformBuilder {
	tb.text = SQLCommentInjection(tb.text)
	return tb
}

// XSSTag generates XSS tag variations.
func (tb *TransformBuilder) XSSTag() *TransformBuilder {
	tb.text = XSSTagVariations(tb.text)
	return tb
}

// NullByte inserts null bytes.
func (tb *TransformBuilder) NullByte() *TransformBuilder {
	tb.text = NullByteInjection(tb.text)
	return tb
}

// PathTraversal generates path traversal sequences.
func (tb *TransformBuilder) PathTraversal() *TransformBuilder {
	tb.text = PathTraversal(tb.text)
	return tb
}

// CommandInjection generates command injection patterns.
func (tb *TransformBuilder) CommandInjection() *TransformBuilder {
	tb.text = CommandInjection(tb.text)
	return tb
}

// Obfuscation Transformations

// Leetspeak converts to leetspeak.
func (tb *TransformBuilder) Leetspeak() *TransformBuilder {
	tb.text = Leetspeak(tb.text)
	return tb
}

// ROT13 applies ROT13 cipher.
func (tb *TransformBuilder) ROT13() *TransformBuilder {
	tb.text = ROT13(tb.text)
	return tb
}

// VowelSwap swaps vowels with similar-looking characters.
func (tb *TransformBuilder) VowelSwap() *TransformBuilder {
	tb.text = VowelSwap(tb.text)
	return tb
}

// DoubleChars doubles each character.
func (tb *TransformBuilder) DoubleChars() *TransformBuilder {
	tb.text = DoubleCharacters(tb.text)
	return tb
}

// Reverse reverses the string.
func (tb *TransformBuilder) Reverse() *TransformBuilder {
	tb.text = ReverseString(tb.text)
	return tb
}

// WhitespacePadding adds random whitespace padding.
func (tb *TransformBuilder) WhitespacePadding() *TransformBuilder {
	tb.text = WhitespacePadding(tb.text)
	return tb
}

// JSConcat converts to JavaScript string concatenation.
func (tb *TransformBuilder) JSConcat() *TransformBuilder {
	tb.text = JSStringConcat(tb.text)
	return tb
}

// Web Security Transformations

// GraphQL obfuscates GraphQL queries.
func (tb *TransformBuilder) GraphQL() *TransformBuilder {
	tb.text = GraphQLObfuscate(tb.text)
	return tb
}

// JWTHeader manipulates JWT headers.
func (tb *TransformBuilder) JWTHeader() *TransformBuilder {
	tb.text = JWTHeaderManipulation(tb.text)
	return tb
}

// JWTPayload obfuscates JWT payloads.
func (tb *TransformBuilder) JWTPayload() *TransformBuilder {
	tb.text = JWTPayloadObfuscate(tb.text)
	return tb
}

// Shell Transformations

// Powershell obfuscates PowerShell commands.
func (tb *TransformBuilder) Powershell() *TransformBuilder {
	tb.text = PowershellObfuscate(tb.text)
	return tb
}

// Bash obfuscates Bash commands.
func (tb *TransformBuilder) Bash() *TransformBuilder {
	tb.text = BashObfuscate(tb.text)
	return tb
}
