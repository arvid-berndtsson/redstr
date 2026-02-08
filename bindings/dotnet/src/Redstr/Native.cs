using System.Runtime.InteropServices;

namespace Redstr;

/// <summary>
/// P/Invoke declarations for the native redstr library.
/// </summary>
internal static partial class Native
{
    private const string LibName = "redstr_ffi";

    // ========================================================================
    // Memory Management
    // ========================================================================

    /// <summary>
    /// Free a string that was allocated by a redstr function.
    /// </summary>
    [LibraryImport(LibName, EntryPoint = "redstr_free_string")]
    internal static partial void FreeString(IntPtr s);

    // ========================================================================
    // Case Transformations
    // ========================================================================

    [LibraryImport(LibName, EntryPoint = "redstr_randomize_capitalization", StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr RandomizeCapitalization(string input);

    [LibraryImport(LibName, EntryPoint = "redstr_case_swap", StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr CaseSwap(string input);

    [LibraryImport(LibName, EntryPoint = "redstr_alternate_case", StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr AlternateCase(string input);

    [LibraryImport(LibName, EntryPoint = "redstr_inverse_case", StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr InverseCase(string input);

    // ========================================================================
    // Encoding Transformations
    // ========================================================================

    [LibraryImport(LibName, EntryPoint = "redstr_base64_encode", StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr Base64Encode(string input);

    [LibraryImport(LibName, EntryPoint = "redstr_url_encode", StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr UrlEncode(string input);

    [LibraryImport(LibName, EntryPoint = "redstr_hex_encode", StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr HexEncode(string input);

    [LibraryImport(LibName, EntryPoint = "redstr_html_entity_encode", StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr HtmlEntityEncode(string input);

    // ========================================================================
    // Obfuscation Transformations
    // ========================================================================

    [LibraryImport(LibName, EntryPoint = "redstr_leetspeak", StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr Leetspeak(string input);

    [LibraryImport(LibName, EntryPoint = "redstr_rot13", StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr Rot13(string input);

    [LibraryImport(LibName, EntryPoint = "redstr_reverse_string", StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr ReverseString(string input);

    [LibraryImport(LibName, EntryPoint = "redstr_double_characters", StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr DoubleCharacters(string input);

    // ========================================================================
    // Unicode Transformations
    // ========================================================================

    [LibraryImport(LibName, EntryPoint = "redstr_homoglyph_substitution", StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr HomoglyphSubstitution(string input);

    [LibraryImport(LibName, EntryPoint = "redstr_zalgo_text", StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr ZalgoText(string input);

    // ========================================================================
    // Phishing Transformations
    // ========================================================================

    [LibraryImport(LibName, EntryPoint = "redstr_domain_typosquat", StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr DomainTyposquat(string input);

    [LibraryImport(LibName, EntryPoint = "redstr_email_obfuscation", StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr EmailObfuscation(string input);

    // ========================================================================
    // Injection Transformations
    // ========================================================================

    [LibraryImport(LibName, EntryPoint = "redstr_xss_tag_variations", StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr XssTagVariations(string input);

    [LibraryImport(LibName, EntryPoint = "redstr_sql_comment_injection", StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr SqlCommentInjection(string input);

    [LibraryImport(LibName, EntryPoint = "redstr_command_injection", StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr CommandInjection(string input);

    [LibraryImport(LibName, EntryPoint = "redstr_path_traversal", StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr PathTraversal(string input);

    // ========================================================================
    // Bot Detection Transformations
    // ========================================================================

    [LibraryImport(LibName, EntryPoint = "redstr_random_user_agent")]
    internal static partial IntPtr RandomUserAgent();

    // ========================================================================
    // Shell Transformations
    // ========================================================================

    [LibraryImport(LibName, EntryPoint = "redstr_powershell_obfuscate", StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr PowershellObfuscate(string input);

    [LibraryImport(LibName, EntryPoint = "redstr_bash_obfuscate", StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr BashObfuscate(string input);

    // ========================================================================
    // Helper Methods
    // ========================================================================

    /// <summary>
    /// Convert a native string pointer to a managed string and free the native memory.
    /// </summary>
    internal static string PtrToStringAndFree(IntPtr ptr)
    {
        if (ptr == IntPtr.Zero)
        {
            return string.Empty;
        }

        try
        {
            return Marshal.PtrToStringUTF8(ptr) ?? string.Empty;
        }
        finally
        {
            FreeString(ptr);
        }
    }
}
