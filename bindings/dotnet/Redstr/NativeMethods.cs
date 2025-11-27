using System;
using System.IO;
using System.Reflection;
using System.Runtime.InteropServices;

namespace Redstr
{
    /// <summary>
    /// Low-level P/Invoke declarations for the native redstr library.
    /// Most users should use the high-level API in Redstr class instead.
    /// </summary>
    internal static class NativeMethods
    {
        private const string LibraryName = "redstr";

        static NativeMethods()
        {
            NativeLibrary.SetDllImportResolver(typeof(NativeMethods).Assembly, DllImportResolver);
        }

        private static IntPtr DllImportResolver(string libraryName, Assembly assembly, DllImportSearchPath? searchPath)
        {
            if (libraryName != LibraryName)
                return IntPtr.Zero;

            // Determine platform-specific library name and RID
            string libFileName;
            string rid;

            if (RuntimeInformation.IsOSPlatform(OSPlatform.Windows))
            {
                libFileName = "redstr.dll";
                rid = "win-x64";
            }
            else if (RuntimeInformation.IsOSPlatform(OSPlatform.Linux))
            {
                libFileName = "libredstr.so";
                rid = "linux-x64";
            }
            else if (RuntimeInformation.IsOSPlatform(OSPlatform.OSX))
            {
                libFileName = "libredstr.dylib";
                rid = RuntimeInformation.ProcessArchitecture == Architecture.Arm64 ? "osx-arm64" : "osx-x64";
            }
            else
            {
                return IntPtr.Zero;
            }

            // Try to load from runtimes directory structure
            var assemblyLocation = Path.GetDirectoryName(assembly.Location);
            if (assemblyLocation != null)
            {
                var runtimesPath = Path.Combine(assemblyLocation, "runtimes", rid, "native", libFileName);
                if (File.Exists(runtimesPath))
                {
                    return NativeLibrary.Load(runtimesPath);
                }

                // Also try directly in the assembly directory
                var directPath = Path.Combine(assemblyLocation, libFileName);
                if (File.Exists(directPath))
                {
                    return NativeLibrary.Load(directPath);
                }
            }

            // Fall back to default search
            return NativeLibrary.TryLoad(libraryName, assembly, searchPath, out var handle) ? handle : IntPtr.Zero;
        }

        /// <summary>
        /// Free a string allocated by redstr
        /// </summary>
        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void redstr_free_string(IntPtr ptr);

        // Case transformations
        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_randomize_capitalization(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_case_swap(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_alternate_case(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_inverse_case(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_to_camel_case(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_to_snake_case(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_to_kebab_case(IntPtr input);

        // Encoding transformations
        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_base64_encode(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_url_encode(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_hex_encode(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_hex_encode_mixed(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_html_entity_encode(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_mixed_encoding(IntPtr input);

        // Unicode transformations
        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_homoglyph_substitution(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_unicode_variations(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_zalgo_text(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_space_variants(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_unicode_normalize_variants(IntPtr input);

        // Injection transformations
        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_sql_comment_injection(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_xss_tag_variations(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_command_injection(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_path_traversal(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_null_byte_injection(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_mongodb_injection(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_couchdb_injection(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_dynamodb_obfuscate(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_nosql_operator_injection(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_ssti_injection(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_ssti_syntax_obfuscate(IntPtr input);

        // Obfuscation transformations
        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_leetspeak(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_rot13(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_double_characters(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_vowel_swap(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_reverse_string(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_js_string_concat(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_whitespace_padding(IntPtr input);

        // Phishing transformations
        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_domain_typosquat(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_advanced_domain_spoof(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_email_obfuscation(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_url_shortening_pattern(IntPtr input);

        // Bot detection transformations
        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_random_user_agent();

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_tls_fingerprint_variation(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_http2_header_order(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_accept_language_variation(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_cloudflare_challenge_variation(IntPtr input);

        // Cloudflare transformations
        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_cloudflare_turnstile_variation(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_cloudflare_challenge_response(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_tls_handshake_pattern(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_canvas_fingerprint_variation(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_webgl_fingerprint_obfuscate(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_font_fingerprint_consistency(IntPtr input);

        // Web security transformations
        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_http_header_variation(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_api_endpoint_variation(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_graphql_obfuscate(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_session_token_variation(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_graphql_variable_injection(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_graphql_introspection_bypass(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_jwt_header_manipulation(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_jwt_payload_obfuscate(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_jwt_algorithm_confusion(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_jwt_signature_bypass(IntPtr input);

        // Shell transformations
        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_powershell_obfuscate(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_bash_obfuscate(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_env_var_obfuscate(IntPtr input);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr redstr_file_path_obfuscate(IntPtr input);
    }
}
