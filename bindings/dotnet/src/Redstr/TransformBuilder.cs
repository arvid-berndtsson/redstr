namespace Redstr;

/// <summary>
/// Builder for chaining multiple transformations.
/// </summary>
/// <example>
/// <code>
/// var result = new TransformBuilder("payload")
///     .Leetspeak()
///     .Base64()
///     .UrlEncode()
///     .Build();
/// </code>
/// </example>
public class TransformBuilder
{
    private string _text;

    /// <summary>
    /// Create a new TransformBuilder with the given input.
    /// </summary>
    /// <param name="input">The initial string to transform.</param>
    public TransformBuilder(string input)
    {
        _text = input ?? throw new ArgumentNullException(nameof(input));
    }

    /// <summary>
    /// Apply leetspeak transformation.
    /// </summary>
    /// <returns>This builder for chaining.</returns>
    public TransformBuilder Leetspeak()
    {
        _text = Transforms.Leetspeak(_text);
        return this;
    }

    /// <summary>
    /// Apply Base64 encoding.
    /// </summary>
    /// <returns>This builder for chaining.</returns>
    public TransformBuilder Base64()
    {
        _text = Transforms.Base64Encode(_text);
        return this;
    }

    /// <summary>
    /// Apply URL encoding.
    /// </summary>
    /// <returns>This builder for chaining.</returns>
    public TransformBuilder UrlEncode()
    {
        _text = Transforms.UrlEncode(_text);
        return this;
    }

    /// <summary>
    /// Apply case swap.
    /// </summary>
    /// <returns>This builder for chaining.</returns>
    public TransformBuilder CaseSwap()
    {
        _text = Transforms.CaseSwap(_text);
        return this;
    }

    /// <summary>
    /// Apply ROT13.
    /// </summary>
    /// <returns>This builder for chaining.</returns>
    public TransformBuilder Rot13()
    {
        _text = Transforms.Rot13(_text);
        return this;
    }

    /// <summary>
    /// Apply hex encoding.
    /// </summary>
    /// <returns>This builder for chaining.</returns>
    public TransformBuilder HexEncode()
    {
        _text = Transforms.HexEncode(_text);
        return this;
    }

    /// <summary>
    /// Apply homoglyph substitution.
    /// </summary>
    /// <returns>This builder for chaining.</returns>
    public TransformBuilder Homoglyphs()
    {
        _text = Transforms.HomoglyphSubstitution(_text);
        return this;
    }

    /// <summary>
    /// Reverse the string.
    /// </summary>
    /// <returns>This builder for chaining.</returns>
    public TransformBuilder Reverse()
    {
        _text = Transforms.ReverseString(_text);
        return this;
    }

    /// <summary>
    /// Apply HTML entity encoding.
    /// </summary>
    /// <returns>This builder for chaining.</returns>
    public TransformBuilder HtmlEncode()
    {
        _text = Transforms.HtmlEntityEncode(_text);
        return this;
    }

    /// <summary>
    /// Apply XSS tag variations.
    /// </summary>
    /// <returns>This builder for chaining.</returns>
    public TransformBuilder XssVariations()
    {
        _text = Transforms.XssTagVariations(_text);
        return this;
    }

    /// <summary>
    /// Apply SQL comment injection.
    /// </summary>
    /// <returns>This builder for chaining.</returns>
    public TransformBuilder SqlComments()
    {
        _text = Transforms.SqlCommentInjection(_text);
        return this;
    }

    /// <summary>
    /// Build and return the final transformed string.
    /// </summary>
    /// <returns>The transformed string.</returns>
    public string Build() => _text;

    /// <summary>
    /// Implicit conversion to string.
    /// </summary>
    public static implicit operator string(TransformBuilder builder) => builder.Build();

    /// <inheritdoc/>
    public override string ToString() => _text;
}
