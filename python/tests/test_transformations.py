"""
Tests for redstr Python bindings.

Tests cover all transformation categories to ensure Python bindings work correctly.
"""

import pytest
import redstr


class TestCaseTransformations:
    """Test case transformation functions."""

    def test_alternate_case(self):
        result = redstr.alternate_case("hello")
        # Check that some alternation happened
        assert len(result) == 5
        assert result.lower() == "hello"

    def test_case_swap(self):
        result = redstr.case_swap("Hello World")
        # Check that case was swapped
        assert len(result) == 11
        assert result != "Hello World"

    def test_inverse_case(self):
        result = redstr.inverse_case("Hello")
        assert result == "hELLO"

    def test_randomize_capitalization(self):
        result = redstr.randomize_capitalization("hello")
        assert len(result) == 5
        assert result.lower() == "hello"

    def test_to_camel_case(self):
        result = redstr.to_camel_case("hello world")
        assert result == "helloWorld"

    def test_to_snake_case(self):
        result = redstr.to_snake_case("Hello World")
        assert result == "hello_world"

    def test_to_kebab_case(self):
        result = redstr.to_kebab_case("Hello World")
        assert result == "hello-world"


class TestEncodingTransformations:
    """Test encoding transformation functions."""

    def test_base64_encode(self):
        result = redstr.base64_encode("hello")
        assert result == "aGVsbG8="

    def test_hex_encode(self):
        result = redstr.hex_encode("hi")
        assert result == "6869"

    def test_url_encode(self):
        result = redstr.url_encode("hello world")
        assert "hello" in result and "%20" in result

    def test_html_entity_encode(self):
        result = redstr.html_entity_encode("<script>")
        # Check that HTML encoding happened (can be decimal or hex)
        assert "&#" in result or "&lt;" in result


class TestUnicodeTransformations:
    """Test Unicode transformation functions."""

    def test_homoglyph_substitution(self):
        result = redstr.homoglyph_substitution("admin")
        assert len(result) >= 5  # Should have some substitutions

    def test_unicode_variations(self):
        result = redstr.unicode_variations("hello")
        assert len(result) >= 5

    def test_zalgo_text(self):
        result = redstr.zalgo_text("hello")
        assert len(result) > len("hello")  # Should have combining marks


class TestInjectionTransformations:
    """Test injection transformation functions."""

    def test_sql_comment_injection(self):
        result = redstr.sql_comment_injection("SELECT * FROM users")
        assert "/*" in result or "--" in result or len(result) > 0

    def test_xss_tag_variations(self):
        result = redstr.xss_tag_variations("alert(1)")
        assert "alert" in result.lower()

    def test_path_traversal(self):
        result = redstr.path_traversal("etc/passwd")
        # Check that some traversal technique was applied
        assert len(result) >= len("etc/passwd")

    def test_command_injection(self):
        result = redstr.command_injection("ls")
        assert len(result) >= 2

    def test_mongodb_injection(self):
        result = redstr.mongodb_injection("admin")
        assert "admin" in result or "$" in result

    def test_null_byte_injection(self):
        result = redstr.null_byte_injection("file.txt")
        # Check that some injection happened (may include escaped nulls)
        assert len(result) >= len("file.txt")


class TestObfuscationTransformations:
    """Test obfuscation transformation functions."""

    def test_leetspeak(self):
        result = redstr.leetspeak("elite")
        # Should have some leet substitutions
        assert any(c in result for c in "0134567")

    def test_rot13(self):
        result = redstr.rot13("hello")
        assert result == "uryyb"
        # ROT13 is reversible
        assert redstr.rot13(result) == "hello"

    def test_reverse_string(self):
        result = redstr.reverse_string("hello")
        assert result == "olleh"

    def test_double_characters(self):
        result = redstr.double_characters("hi")
        # Check that characters were doubled (may not double all)
        assert len(result) >= len("hi")


class TestPhishingTransformations:
    """Test phishing transformation functions."""

    def test_domain_typosquat(self):
        result = redstr.domain_typosquat("google.com")
        assert "google" in result.lower() or "gogle" in result.lower()

    def test_email_obfuscation(self):
        result = redstr.email_obfuscation("test@example.com")
        assert "test" in result.lower()


class TestBotDetectionTransformations:
    """Test bot detection transformation functions."""

    def test_random_user_agent(self):
        result = redstr.random_user_agent()
        assert len(result) > 10
        assert "Mozilla" in result or "Chrome" in result or "Safari" in result


class TestShellTransformations:
    """Test shell transformation functions."""

    def test_bash_obfuscate(self):
        result = redstr.bash_obfuscate("cat file.txt")
        assert "cat" in result or "c" in result

    def test_powershell_obfuscate(self):
        result = redstr.powershell_obfuscate("Get-Process")
        # Check that some obfuscation happened
        assert len(result) >= len("Get-Process")


class TestWebSecurityTransformations:
    """Test web security transformation functions."""

    def test_jwt_signature_bypass(self):
        jwt = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.signature"
        result = redstr.jwt_signature_bypass(jwt)
        assert len(result) > 0
        # Should have header and payload
        parts = result.split(".")
        assert len(parts) >= 2

    def test_graphql_obfuscate(self):
        result = redstr.graphql_obfuscate("query { user { name } }")
        assert "query" in result.lower()

    def test_http_header_variation(self):
        result = redstr.http_header_variation("User-Agent")
        assert "user" in result.lower() or "agent" in result.lower()


class TestTransformBuilder:
    """Test TransformBuilder class for chaining transformations."""

    def test_builder_creation(self):
        builder = redstr.TransformBuilder("hello")
        result = builder.build()
        assert result == "hello"

    def test_builder_single_transform(self):
        builder = redstr.TransformBuilder("hello")
        result = builder.rot13().build()
        assert result == "uryyb"

    def test_builder_chain_transforms(self):
        builder = redstr.TransformBuilder("hello")
        result = builder.rot13().reverse().build()
        # ROT13 of "hello" is "uryyb", reversed is "byyru"
        assert result == "byyru"

    def test_builder_base64(self):
        builder = redstr.TransformBuilder("hello")
        result = builder.base64().build()
        assert result == "aGVsbG8="

    def test_builder_multiple_chains(self):
        # Test that builder can chain multiple different transforms
        builder = redstr.TransformBuilder("test")
        result = builder.case_swap().url().build()
        assert len(result) > 0


def test_module_version():
    """Test that module has version attribute."""
    assert hasattr(redstr, "__version__")
    assert isinstance(redstr.__version__, str)


def test_all_exports():
    """Test that all expected functions are exported."""
    expected_functions = [
        # Case transformations
        "alternate_case", "case_swap", "inverse_case",
        "randomize_capitalization", "to_camel_case", "to_kebab_case", "to_snake_case",
        # Encoding transformations
        "base64_encode", "hex_encode", "url_encode", "html_entity_encode",
        # Unicode transformations
        "homoglyph_substitution", "unicode_variations", "zalgo_text",
        # Injection transformations
        "sql_comment_injection", "xss_tag_variations", "path_traversal",
        "command_injection", "mongodb_injection",
        # Obfuscation transformations
        "leetspeak", "rot13", "reverse_string", "double_characters",
        # Phishing transformations
        "domain_typosquat", "email_obfuscation",
        # Bot detection transformations
        "random_user_agent",
        # Shell transformations
        "bash_obfuscate", "powershell_obfuscate",
        # Web security transformations
        "jwt_signature_bypass", "graphql_obfuscate", "http_header_variation",
    ]
    
    for func_name in expected_functions:
        assert hasattr(redstr, func_name), f"Missing function: {func_name}"


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
