namespace Redstr;

/// <summary>
/// Static methods for string transformations used in security testing.
/// </summary>
public static class Transforms
{
    // ========================================================================
    // Case Transformations
    // ========================================================================

    /// <summary>
    /// Randomize the capitalization of each character.
    /// </summary>
    /// <param name="input">The input string.</param>
    /// <returns>The transformed string with random capitalization.</returns>
    public static string RandomizeCapitalization(string input)
        => Native.PtrToStringAndFree(Native.RandomizeCapitalization(input));

    /// <summary>
    /// Swap the case of each character.
    /// </summary>
    /// <param name="input">The input string.</param>
    /// <returns>The transformed string with swapped case.</returns>
    public static string CaseSwap(string input)
        => Native.PtrToStringAndFree(Native.CaseSwap(input));

    /// <summary>
    /// Alternate case for each character.
    /// </summary>
    /// <param name="input">The input string.</param>
    /// <returns>The transformed string with alternating case.</returns>
    public static string AlternateCase(string input)
        => Native.PtrToStringAndFree(Native.AlternateCase(input));

    /// <summary>
    /// Inverse case transformation.
    /// </summary>
    /// <param name="input">The input string.</param>
    /// <returns>The transformed string with inverted case.</returns>
    public static string InverseCase(string input)
        => Native.PtrToStringAndFree(Native.InverseCase(input));

    // ========================================================================
    // Encoding Transformations
    // ========================================================================

    /// <summary>
    /// Encode string to Base64.
    /// </summary>
    /// <param name="input">The input string.</param>
    /// <returns>The Base64 encoded string.</returns>
    public static string Base64Encode(string input)
        => Native.PtrToStringAndFree(Native.Base64Encode(input));

    /// <summary>
    /// URL encode a string.
    /// </summary>
    /// <param name="input">The input string.</param>
    /// <returns>The URL encoded string.</returns>
    public static string UrlEncode(string input)
        => Native.PtrToStringAndFree(Native.UrlEncode(input));

    /// <summary>
    /// Hex encode a string.
    /// </summary>
    /// <param name="input">The input string.</param>
    /// <returns>The hex encoded string.</returns>
    public static string HexEncode(string input)
        => Native.PtrToStringAndFree(Native.HexEncode(input));

    /// <summary>
    /// HTML entity encode a string.
    /// </summary>
    /// <param name="input">The input string.</param>
    /// <returns>The HTML entity encoded string.</returns>
    public static string HtmlEntityEncode(string input)
        => Native.PtrToStringAndFree(Native.HtmlEntityEncode(input));

    // ========================================================================
    // Obfuscation Transformations
    // ========================================================================

    /// <summary>
    /// Apply leetspeak transformation.
    /// </summary>
    /// <param name="input">The input string.</param>
    /// <returns>The leetspeak transformed string.</returns>
    public static string Leetspeak(string input)
        => Native.PtrToStringAndFree(Native.Leetspeak(input));

    /// <summary>
    /// Apply ROT13 cipher.
    /// </summary>
    /// <param name="input">The input string.</param>
    /// <returns>The ROT13 transformed string.</returns>
    public static string Rot13(string input)
        => Native.PtrToStringAndFree(Native.Rot13(input));

    /// <summary>
    /// Reverse a string.
    /// </summary>
    /// <param name="input">The input string.</param>
    /// <returns>The reversed string.</returns>
    public static string ReverseString(string input)
        => Native.PtrToStringAndFree(Native.ReverseString(input));

    /// <summary>
    /// Double each character in the string.
    /// </summary>
    /// <param name="input">The input string.</param>
    /// <returns>The string with doubled characters.</returns>
    public static string DoubleCharacters(string input)
        => Native.PtrToStringAndFree(Native.DoubleCharacters(input));

    // ========================================================================
    // Unicode Transformations
    // ========================================================================

    /// <summary>
    /// Apply homoglyph substitution.
    /// </summary>
    /// <param name="input">The input string.</param>
    /// <returns>The string with homoglyph substitutions.</returns>
    public static string HomoglyphSubstitution(string input)
        => Native.PtrToStringAndFree(Native.HomoglyphSubstitution(input));

    /// <summary>
    /// Apply zalgo text effect.
    /// </summary>
    /// <param name="input">The input string.</param>
    /// <returns>The string with zalgo effect.</returns>
    public static string ZalgoText(string input)
        => Native.PtrToStringAndFree(Native.ZalgoText(input));

    // ========================================================================
    // Phishing Transformations
    // ========================================================================

    /// <summary>
    /// Generate typosquatted domain.
    /// </summary>
    /// <param name="input">The domain to typosquat.</param>
    /// <returns>A typosquatted domain variation.</returns>
    public static string DomainTyposquat(string input)
        => Native.PtrToStringAndFree(Native.DomainTyposquat(input));

    /// <summary>
    /// Obfuscate email address.
    /// </summary>
    /// <param name="input">The email address to obfuscate.</param>
    /// <returns>The obfuscated email address.</returns>
    public static string EmailObfuscation(string input)
        => Native.PtrToStringAndFree(Native.EmailObfuscation(input));

    // ========================================================================
    // Injection Transformations
    // ========================================================================

    /// <summary>
    /// Apply XSS tag variations.
    /// </summary>
    /// <param name="input">The XSS payload.</param>
    /// <returns>The payload with tag variations.</returns>
    public static string XssTagVariations(string input)
        => Native.PtrToStringAndFree(Native.XssTagVariations(input));

    /// <summary>
    /// Apply SQL comment injection.
    /// </summary>
    /// <param name="input">The SQL payload.</param>
    /// <returns>The payload with comment injection.</returns>
    public static string SqlCommentInjection(string input)
        => Native.PtrToStringAndFree(Native.SqlCommentInjection(input));

    /// <summary>
    /// Apply command injection patterns.
    /// </summary>
    /// <param name="input">The command to inject.</param>
    /// <returns>The command with injection patterns.</returns>
    public static string CommandInjection(string input)
        => Native.PtrToStringAndFree(Native.CommandInjection(input));

    /// <summary>
    /// Apply path traversal patterns.
    /// </summary>
    /// <param name="input">The path to traverse.</param>
    /// <returns>The path with traversal patterns.</returns>
    public static string PathTraversal(string input)
        => Native.PtrToStringAndFree(Native.PathTraversal(input));

    // ========================================================================
    // Bot Detection Transformations
    // ========================================================================

    /// <summary>
    /// Generate a random user agent string.
    /// </summary>
    /// <returns>A random browser user agent string.</returns>
    public static string RandomUserAgent()
        => Native.PtrToStringAndFree(Native.RandomUserAgent());

    // ========================================================================
    // Shell Transformations
    // ========================================================================

    /// <summary>
    /// Obfuscate PowerShell command.
    /// </summary>
    /// <param name="input">The PowerShell command.</param>
    /// <returns>The obfuscated command.</returns>
    public static string PowershellObfuscate(string input)
        => Native.PtrToStringAndFree(Native.PowershellObfuscate(input));

    /// <summary>
    /// Obfuscate Bash command.
    /// </summary>
    /// <param name="input">The Bash command.</param>
    /// <returns>The obfuscated command.</returns>
    public static string BashObfuscate(string input)
        => Native.PtrToStringAndFree(Native.BashObfuscate(input));
}
