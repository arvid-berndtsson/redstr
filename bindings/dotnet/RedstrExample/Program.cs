using Redstr;

Console.WriteLine("=== Redstr C# Bindings Example ===\n");

// Case Transformations
Console.WriteLine("1. Random Capitalization:");
string text1 = "Hello World";
string result1 = RedstrTransform.RandomizeCapitalization(text1);
Console.WriteLine($"   Input:  {text1}");
Console.WriteLine($"   Output: {result1}\n");

// Encoding
Console.WriteLine("2. Base64 Encoding:");
string text2 = "SecurePassword123";
string result2 = RedstrTransform.Base64Encode(text2);
Console.WriteLine($"   Input:  {text2}");
Console.WriteLine($"   Output: {result2}\n");

// Obfuscation
Console.WriteLine("3. Leetspeak:");
string text3 = "password";
string result3 = RedstrTransform.Leetspeak(text3);
Console.WriteLine($"   Input:  {text3}");
Console.WriteLine($"   Output: {result3}\n");

// Unicode Transformations
Console.WriteLine("4. Homoglyph Substitution:");
string text4 = "admin@example.com";
string result4 = RedstrTransform.HomoglyphSubstitution(text4);
Console.WriteLine($"   Input:  {text4}");
Console.WriteLine($"   Output: {result4}\n");

// URL Encoding
Console.WriteLine("5. URL Encoding:");
string text5 = "https://example.com/path?query=value";
string result5 = RedstrTransform.UrlEncode(text5);
Console.WriteLine($"   Input:  {text5}");
Console.WriteLine($"   Output: {result5}\n");

// Case Conversion
Console.WriteLine("6. Snake Case:");
string text6 = "HelloWorldExample";
string result6 = RedstrTransform.ToSnakeCase(text6);
Console.WriteLine($"   Input:  {text6}");
Console.WriteLine($"   Output: {result6}\n");

// Bot Detection
Console.WriteLine("7. Random User Agent:");
string ua = RedstrTransform.RandomUserAgent();
Console.WriteLine($"   Output: {ua}\n");

Console.WriteLine("=== Example Complete ===");
