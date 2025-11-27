using System;
using System.Runtime.InteropServices;
using System.Text;

namespace Redstr
{
    /// <summary>
    /// Red team string transformation library for offensive security operations,
    /// penetration testing, and evasion techniques.
    /// 
    /// This library provides 60+ string obfuscation and transformation functions
    /// for red team, blue team, and purple team security operations.
    /// </summary>
    public static class RedstrTransform
    {
        /// <summary>
        /// Helper method to call a native transformation function that takes a string input
        /// </summary>
        private static string Transform(string input, Func<IntPtr, IntPtr> nativeFunc)
        {
            if (input == null)
                throw new ArgumentNullException(nameof(input));

            IntPtr inputPtr = Marshal.StringToHGlobalAnsi(input);
            try
            {
                IntPtr resultPtr = nativeFunc(inputPtr);
                if (resultPtr == IntPtr.Zero)
                    throw new InvalidOperationException("Native transformation failed");

                try
                {
                    return Marshal.PtrToStringAnsi(resultPtr) ?? string.Empty;
                }
                finally
                {
                    NativeMethods.redstr_free_string(resultPtr);
                }
            }
            finally
            {
                Marshal.FreeHGlobal(inputPtr);
            }
        }

        // Case Transformations

        /// <summary>
        /// Randomly capitalizes characters in the input string.
        /// Useful for bypassing case-sensitive filters.
        /// </summary>
        /// <param name="input">The string to transform</param>
        /// <returns>String with random capitalization</returns>
        /// <example>
        /// <code>
        /// var result = RedstrTransform.RandomizeCapitalization("Hello World");
        /// // Output: "HeLlO wOrLd" (varies each run)
        /// </code>
        /// </example>
        public static string RandomizeCapitalization(string input)
            => Transform(input, NativeMethods.redstr_randomize_capitalization);

        /// <summary>
        /// Swaps the case of all letters in the input string.
        /// Lowercase becomes uppercase and vice versa.
        /// </summary>
        public static string CaseSwap(string input)
            => Transform(input, NativeMethods.redstr_case_swap);

        /// <summary>
        /// Alternates the case of each character.
        /// </summary>
        public static string AlternateCase(string input)
            => Transform(input, NativeMethods.redstr_alternate_case);

        /// <summary>
        /// Inverts the normal case pattern (lowercase first letter, uppercase rest).
        /// </summary>
        public static string InverseCase(string input)
            => Transform(input, NativeMethods.redstr_inverse_case);

        /// <summary>
        /// Converts string to camelCase format.
        /// </summary>
        public static string ToCamelCase(string input)
            => Transform(input, NativeMethods.redstr_to_camel_case);

        /// <summary>
        /// Converts string to snake_case format.
        /// </summary>
        public static string ToSnakeCase(string input)
            => Transform(input, NativeMethods.redstr_to_snake_case);

        /// <summary>
        /// Converts string to kebab-case format.
        /// </summary>
        public static string ToKebabCase(string input)
            => Transform(input, NativeMethods.redstr_to_kebab_case);

        // Encoding Transformations

        /// <summary>
        /// Encodes the input string as Base64.
        /// </summary>
        public static string Base64Encode(string input)
            => Transform(input, NativeMethods.redstr_base64_encode);

        /// <summary>
        /// URL encodes the input string.
        /// </summary>
        public static string UrlEncode(string input)
            => Transform(input, NativeMethods.redstr_url_encode);

        /// <summary>
        /// Encodes the input string as hexadecimal.
        /// </summary>
        public static string HexEncode(string input)
            => Transform(input, NativeMethods.redstr_hex_encode);

        /// <summary>
        /// Encodes the input string as mixed case hexadecimal.
        /// </summary>
        public static string HexEncodeMixed(string input)
            => Transform(input, NativeMethods.redstr_hex_encode_mixed);

        /// <summary>
        /// Encodes special characters as HTML entities.
        /// </summary>
        public static string HtmlEntityEncode(string input)
            => Transform(input, NativeMethods.redstr_html_entity_encode);

        /// <summary>
        /// Applies mixed encoding (combination of different encoding techniques).
        /// </summary>
        public static string MixedEncoding(string input)
            => Transform(input, NativeMethods.redstr_mixed_encoding);

        // Unicode Transformations

        /// <summary>
        /// Replaces characters with visually similar Unicode homoglyphs.
        /// Useful for phishing tests and IDN spoofing.
        /// </summary>
        /// <example>
        /// <code>
        /// var spoofed = RedstrTransform.HomoglyphSubstitution("admin@example.com");
        /// // Output: "аdmіn@еxаmple.com" (with Cyrillic characters)
        /// </code>
        /// </example>
        public static string HomoglyphSubstitution(string input)
            => Transform(input, NativeMethods.redstr_homoglyph_substitution);

        /// <summary>
        /// Creates Unicode variations of the input string.
        /// </summary>
        public static string UnicodeVariations(string input)
            => Transform(input, NativeMethods.redstr_unicode_variations);

        /// <summary>
        /// Adds combining characters to create "zalgo" text effect.
        /// </summary>
        public static string ZalgoText(string input)
            => Transform(input, NativeMethods.redstr_zalgo_text);

        /// <summary>
        /// Replaces spaces with Unicode space variants.
        /// </summary>
        public static string SpaceVariants(string input)
            => Transform(input, NativeMethods.redstr_space_variants);

        /// <summary>
        /// Creates Unicode normalization variants.
        /// </summary>
        public static string UnicodeNormalizeVariants(string input)
            => Transform(input, NativeMethods.redstr_unicode_normalize_variants);

        // Injection Transformations

        /// <summary>
        /// Inserts SQL comments into the input string.
        /// Useful for SQL injection testing.
        /// </summary>
        public static string SqlCommentInjection(string input)
            => Transform(input, NativeMethods.redstr_sql_comment_injection);

        /// <summary>
        /// Creates XSS tag variations for testing.
        /// </summary>
        public static string XssTagVariations(string input)
            => Transform(input, NativeMethods.redstr_xss_tag_variations);

        /// <summary>
        /// Creates command injection patterns.
        /// </summary>
        public static string CommandInjection(string input)
            => Transform(input, NativeMethods.redstr_command_injection);

        /// <summary>
        /// Creates path traversal patterns (e.g., ../../).
        /// </summary>
        public static string PathTraversal(string input)
            => Transform(input, NativeMethods.redstr_path_traversal);

        /// <summary>
        /// Inserts null byte characters for injection testing.
        /// </summary>
        public static string NullByteInjection(string input)
            => Transform(input, NativeMethods.redstr_null_byte_injection);

        /// <summary>
        /// Creates MongoDB injection patterns.
        /// </summary>
        public static string MongoDbInjection(string input)
            => Transform(input, NativeMethods.redstr_mongodb_injection);

        /// <summary>
        /// Creates CouchDB injection patterns.
        /// </summary>
        public static string CouchDbInjection(string input)
            => Transform(input, NativeMethods.redstr_couchdb_injection);

        /// <summary>
        /// Obfuscates DynamoDB queries.
        /// </summary>
        public static string DynamoDbObfuscate(string input)
            => Transform(input, NativeMethods.redstr_dynamodb_obfuscate);

        /// <summary>
        /// Creates NoSQL operator injection patterns.
        /// </summary>
        public static string NoSqlOperatorInjection(string input)
            => Transform(input, NativeMethods.redstr_nosql_operator_injection);

        /// <summary>
        /// Creates Server-Side Template Injection (SSTI) patterns.
        /// </summary>
        public static string SstiInjection(string input)
            => Transform(input, NativeMethods.redstr_ssti_injection);

        /// <summary>
        /// Obfuscates SSTI syntax.
        /// </summary>
        public static string SstiSyntaxObfuscate(string input)
            => Transform(input, NativeMethods.redstr_ssti_syntax_obfuscate);

        // Obfuscation Transformations

        /// <summary>
        /// Converts text to leetspeak (1337 speak).
        /// </summary>
        /// <example>
        /// <code>
        /// var result = RedstrTransform.Leetspeak("password");
        /// // Output: "p@55w0rd" or "p4$$w0rd"
        /// </code>
        /// </example>
        public static string Leetspeak(string input)
            => Transform(input, NativeMethods.redstr_leetspeak);

        /// <summary>
        /// Applies ROT13 cipher to the input.
        /// </summary>
        public static string Rot13(string input)
            => Transform(input, NativeMethods.redstr_rot13);

        /// <summary>
        /// Doubles each character in the string.
        /// </summary>
        public static string DoubleCharacters(string input)
            => Transform(input, NativeMethods.redstr_double_characters);

        /// <summary>
        /// Swaps vowels with similar vowels.
        /// </summary>
        public static string VowelSwap(string input)
            => Transform(input, NativeMethods.redstr_vowel_swap);

        /// <summary>
        /// Reverses the input string.
        /// </summary>
        public static string ReverseString(string input)
            => Transform(input, NativeMethods.redstr_reverse_string);

        /// <summary>
        /// Obfuscates string as JavaScript string concatenation.
        /// </summary>
        public static string JsStringConcat(string input)
            => Transform(input, NativeMethods.redstr_js_string_concat);

        /// <summary>
        /// Adds whitespace padding around the string.
        /// </summary>
        public static string WhitespacePadding(string input)
            => Transform(input, NativeMethods.redstr_whitespace_padding);

        // Phishing Transformations

        /// <summary>
        /// Creates typosquatting variations of a domain name.
        /// </summary>
        /// <example>
        /// <code>
        /// var spoofed = RedstrTransform.DomainTyposquat("paypal.com");
        /// // Output: variations like "paypai.com", "paypa1.com", etc.
        /// </code>
        /// </example>
        public static string DomainTyposquat(string input)
            => Transform(input, NativeMethods.redstr_domain_typosquat);

        /// <summary>
        /// Creates advanced domain spoofing patterns.
        /// </summary>
        public static string AdvancedDomainSpoof(string input)
            => Transform(input, NativeMethods.redstr_advanced_domain_spoof);

        /// <summary>
        /// Obfuscates email addresses.
        /// </summary>
        public static string EmailObfuscation(string input)
            => Transform(input, NativeMethods.redstr_email_obfuscation);

        /// <summary>
        /// Creates URL shortening patterns.
        /// </summary>
        public static string UrlShorteningPattern(string input)
            => Transform(input, NativeMethods.redstr_url_shortening_pattern);

        // Bot Detection Transformations

        /// <summary>
        /// Generates a random user agent string.
        /// </summary>
        public static string RandomUserAgent()
        {
            IntPtr resultPtr = NativeMethods.redstr_random_user_agent();
            if (resultPtr == IntPtr.Zero)
                throw new InvalidOperationException("Failed to generate user agent");

            try
            {
                return Marshal.PtrToStringAnsi(resultPtr) ?? string.Empty;
            }
            finally
            {
                NativeMethods.redstr_free_string(resultPtr);
            }
        }

        /// <summary>
        /// Creates TLS fingerprint variations.
        /// </summary>
        public static string TlsFingerprintVariation(string input)
            => Transform(input, NativeMethods.redstr_tls_fingerprint_variation);

        /// <summary>
        /// Creates HTTP/2 header order variations.
        /// </summary>
        public static string Http2HeaderOrder(string input)
            => Transform(input, NativeMethods.redstr_http2_header_order);

        /// <summary>
        /// Creates Accept-Language header variations.
        /// </summary>
        public static string AcceptLanguageVariation(string input)
            => Transform(input, NativeMethods.redstr_accept_language_variation);

        /// <summary>
        /// Creates Cloudflare challenge variations.
        /// </summary>
        public static string CloudflareChallengeVariation(string input)
            => Transform(input, NativeMethods.redstr_cloudflare_challenge_variation);

        // Cloudflare Transformations

        /// <summary>
        /// Creates Cloudflare Turnstile challenge variations.
        /// </summary>
        public static string CloudflareTurnstileVariation(string input)
            => Transform(input, NativeMethods.redstr_cloudflare_turnstile_variation);

        /// <summary>
        /// Creates Cloudflare challenge response patterns.
        /// </summary>
        public static string CloudflareChallengeResponse(string input)
            => Transform(input, NativeMethods.redstr_cloudflare_challenge_response);

        /// <summary>
        /// Creates TLS handshake patterns.
        /// </summary>
        public static string TlsHandshakePattern(string input)
            => Transform(input, NativeMethods.redstr_tls_handshake_pattern);

        /// <summary>
        /// Creates canvas fingerprint variations.
        /// </summary>
        public static string CanvasFingerprintVariation(string input)
            => Transform(input, NativeMethods.redstr_canvas_fingerprint_variation);

        /// <summary>
        /// Obfuscates WebGL fingerprints.
        /// </summary>
        public static string WebglFingerprintObfuscate(string input)
            => Transform(input, NativeMethods.redstr_webgl_fingerprint_obfuscate);

        /// <summary>
        /// Creates consistent font fingerprint patterns.
        /// </summary>
        public static string FontFingerprintConsistency(string input)
            => Transform(input, NativeMethods.redstr_font_fingerprint_consistency);

        // Web Security Transformations

        /// <summary>
        /// Creates HTTP header variations.
        /// </summary>
        public static string HttpHeaderVariation(string input)
            => Transform(input, NativeMethods.redstr_http_header_variation);

        /// <summary>
        /// Creates API endpoint variations.
        /// </summary>
        public static string ApiEndpointVariation(string input)
            => Transform(input, NativeMethods.redstr_api_endpoint_variation);

        /// <summary>
        /// Obfuscates GraphQL queries.
        /// </summary>
        public static string GraphqlObfuscate(string input)
            => Transform(input, NativeMethods.redstr_graphql_obfuscate);

        /// <summary>
        /// Creates session token variations.
        /// </summary>
        public static string SessionTokenVariation(string input)
            => Transform(input, NativeMethods.redstr_session_token_variation);

        /// <summary>
        /// Creates GraphQL variable injection patterns.
        /// </summary>
        public static string GraphqlVariableInjection(string input)
            => Transform(input, NativeMethods.redstr_graphql_variable_injection);

        /// <summary>
        /// Creates GraphQL introspection bypass patterns.
        /// </summary>
        public static string GraphqlIntrospectionBypass(string input)
            => Transform(input, NativeMethods.redstr_graphql_introspection_bypass);

        /// <summary>
        /// Manipulates JWT headers for testing.
        /// </summary>
        public static string JwtHeaderManipulation(string input)
            => Transform(input, NativeMethods.redstr_jwt_header_manipulation);

        /// <summary>
        /// Obfuscates JWT payloads.
        /// </summary>
        public static string JwtPayloadObfuscate(string input)
            => Transform(input, NativeMethods.redstr_jwt_payload_obfuscate);

        /// <summary>
        /// Creates JWT algorithm confusion patterns.
        /// </summary>
        public static string JwtAlgorithmConfusion(string input)
            => Transform(input, NativeMethods.redstr_jwt_algorithm_confusion);

        /// <summary>
        /// Creates JWT signature bypass patterns.
        /// </summary>
        public static string JwtSignatureBypass(string input)
            => Transform(input, NativeMethods.redstr_jwt_signature_bypass);

        // Shell Transformations

        /// <summary>
        /// Obfuscates PowerShell commands.
        /// </summary>
        public static string PowershellObfuscate(string input)
            => Transform(input, NativeMethods.redstr_powershell_obfuscate);

        /// <summary>
        /// Obfuscates Bash commands.
        /// </summary>
        public static string BashObfuscate(string input)
            => Transform(input, NativeMethods.redstr_bash_obfuscate);

        /// <summary>
        /// Obfuscates environment variable references.
        /// </summary>
        public static string EnvVarObfuscate(string input)
            => Transform(input, NativeMethods.redstr_env_var_obfuscate);

        /// <summary>
        /// Obfuscates file paths.
        /// </summary>
        public static string FilePathObfuscate(string input)
            => Transform(input, NativeMethods.redstr_file_path_obfuscate);
    }
}
