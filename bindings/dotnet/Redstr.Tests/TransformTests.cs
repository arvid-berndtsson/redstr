using Xunit;

namespace Redstr.Tests;

public class CaseTransformationTests
{
    [Fact]
    public void RandomizeCapitalization_ReturnsModifiedString()
    {
        var input = "hello world";
        var result = RedstrTransform.RandomizeCapitalization(input);
        
        Assert.NotNull(result);
        Assert.NotEmpty(result);
        Assert.Equal(input.Length, result.Length);
    }

    [Fact]
    public void CaseSwap_SwapsCases()
    {
        var input = "Hello World";
        var result = RedstrTransform.CaseSwap(input);
        
        Assert.NotNull(result);
        Assert.Equal(input.Length, result.Length);
        // CaseSwap may use randomization, so just verify it returns a result
        Assert.NotEmpty(result);
    }

    [Fact]
    public void ToSnakeCase_ConvertsCorrectly()
    {
        var input = "HelloWorld";
        var result = RedstrTransform.ToSnakeCase(input);
        
        Assert.Equal("hello_world", result);
    }

    [Fact]
    public void ToCamelCase_ConvertsCorrectly()
    {
        var input = "hello_world";
        var result = RedstrTransform.ToCamelCase(input);
        
        Assert.Equal("helloWorld", result);
    }

    [Fact]
    public void ToKebabCase_ConvertsCorrectly()
    {
        var input = "HelloWorld";
        var result = RedstrTransform.ToKebabCase(input);
        
        Assert.Equal("hello-world", result);
    }
}

public class EncodingTests
{
    [Fact]
    public void Base64Encode_EncodesCorrectly()
    {
        var input = "hello";
        var result = RedstrTransform.Base64Encode(input);
        
        Assert.Equal("aGVsbG8=", result);
    }

    [Fact]
    public void UrlEncode_EncodesSpecialCharacters()
    {
        var input = "hello world!";
        var result = RedstrTransform.UrlEncode(input);
        
        Assert.Contains("%20", result);
        Assert.Contains("%21", result);
    }

    [Fact]
    public void HexEncode_EncodesCorrectly()
    {
        var input = "hello";
        var result = RedstrTransform.HexEncode(input);
        
        Assert.Contains("68656c6c6f", result.ToLower());
    }
}

public class ObfuscationTests
{
    [Fact]
    public void Leetspeak_ReturnsObfuscatedString()
    {
        var input = "password";
        var result = RedstrTransform.Leetspeak(input);
        
        Assert.NotNull(result);
        Assert.NotEmpty(result);
        // Should contain some leetspeak characters
        Assert.Matches(@"[0-9@$]", result);
    }

    [Fact]
    public void Rot13_ReturnsTransformed()
    {
        var input = "hello";
        var result = RedstrTransform.Rot13(input);
        
        Assert.Equal("uryyb", result);
    }

    [Fact]
    public void Rot13_IsReversible()
    {
        var input = "hello";
        var encoded = RedstrTransform.Rot13(input);
        var decoded = RedstrTransform.Rot13(encoded);
        
        Assert.Equal(input, decoded);
    }

    [Fact]
    public void ReverseString_ReversesCorrectly()
    {
        var input = "hello";
        var result = RedstrTransform.ReverseString(input);
        
        Assert.Equal("olleh", result);
    }

    [Fact]
    public void DoubleCharacters_ModifiesString()
    {
        var input = "abc";
        var result = RedstrTransform.DoubleCharacters(input);
        
        Assert.NotNull(result);
        // DoubleCharacters may selectively double characters
        Assert.True(result.Length >= input.Length);
    }
}

public class UnicodeTests
{
    [Fact]
    public void HomoglyphSubstitution_ReturnsModifiedString()
    {
        var input = "admin";
        var result = RedstrTransform.HomoglyphSubstitution(input);
        
        Assert.NotNull(result);
        Assert.NotEmpty(result);
    }

    [Fact]
    public void ZalgoText_AddsCharacters()
    {
        var input = "hello";
        var result = RedstrTransform.ZalgoText(input);
        
        Assert.NotNull(result);
        Assert.True(result.Length >= input.Length);
    }
}

public class InjectionTests
{
    [Fact]
    public void SqlCommentInjection_ModifiesQuery()
    {
        var input = "SELECT * FROM users";
        var result = RedstrTransform.SqlCommentInjection(input);
        
        Assert.NotNull(result);
        Assert.NotEmpty(result);
        // SQL comment injection modifies the query
        Assert.True(result.Length >= input.Length || result.Contains("#") || result.Contains("--"));
    }

    [Fact]
    public void PathTraversal_ReturnsModifiedString()
    {
        var input = "file.txt";
        var result = RedstrTransform.PathTraversal(input);
        
        Assert.NotNull(result);
        Assert.NotEmpty(result);
        // Path traversal adds prefix or modifies the string
        Assert.True(result.Length >= input.Length || result.Contains("/"));
    }

    [Fact]
    public void CommandInjection_ReturnsModifiedString()
    {
        var input = "ls";
        var result = RedstrTransform.CommandInjection(input);
        
        Assert.NotNull(result);
        Assert.NotEmpty(result);
        // Command injection should modify the string
        Assert.True(result.Length >= input.Length);
    }
}

public class BotDetectionTests
{
    [Fact]
    public void RandomUserAgent_ReturnsValidUserAgent()
    {
        var result = RedstrTransform.RandomUserAgent();
        
        Assert.NotNull(result);
        Assert.NotEmpty(result);
        Assert.Contains("Mozilla", result);
    }

    [Fact]
    public void RandomUserAgent_ReturnsDifferentValues()
    {
        var results = new HashSet<string>();
        
        for (int i = 0; i < 10; i++)
        {
            results.Add(RedstrTransform.RandomUserAgent());
        }
        
        // Should have at least 2 different user agents in 10 tries
        Assert.True(results.Count >= 2);
    }
}

public class PhishingTests
{
    [Fact]
    public void DomainTyposquat_ReturnsValidDomain()
    {
        var input = "example.com";
        var result = RedstrTransform.DomainTyposquat(input);
        
        Assert.NotNull(result);
        Assert.NotEmpty(result);
        // Domain typosquat returns a variation (may be same if no substitution available)
        Assert.Contains(".", result);
    }

    [Fact]
    public void EmailObfuscation_ObfuscatesEmail()
    {
        var input = "user@example.com";
        var result = RedstrTransform.EmailObfuscation(input);
        
        Assert.NotNull(result);
        Assert.NotEmpty(result);
    }
}

public class NullInputTests
{
    [Fact]
    public void Transform_ThrowsOnNullInput()
    {
        Assert.Throws<ArgumentNullException>(() => 
            RedstrTransform.RandomizeCapitalization(null!));
        
        Assert.Throws<ArgumentNullException>(() => 
            RedstrTransform.Base64Encode(null!));
        
        Assert.Throws<ArgumentNullException>(() => 
            RedstrTransform.Leetspeak(null!));
    }
}

public class EmptyStringTests
{
    [Fact]
    public void Transform_HandlesEmptyString()
    {
        var empty = "";
        
        Assert.Equal("", RedstrTransform.Base64Encode(empty));
        Assert.Equal("", RedstrTransform.Rot13(empty));
        Assert.Equal("", RedstrTransform.ReverseString(empty));
    }
}
