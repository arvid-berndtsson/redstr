use crate::transformations::bot_detection::cloudflare_challenge_variation;
use crate::transformations::case::{case_swap, randomize_capitalization};
use crate::transformations::cloudflare::{
    cloudflare_challenge_response, cloudflare_turnstile_variation,
};
use crate::transformations::encoding::{base64_encode, hex_encode, url_encode};
use crate::transformations::obfuscation::{leetspeak, rot13};
use crate::transformations::phishing::{advanced_domain_spoof, email_obfuscation};
use crate::transformations::shell::{bash_obfuscate, powershell_obfuscate};
use crate::transformations::unicode::homoglyph_substitution;
use crate::transformations::web_security::graphql_obfuscate;

/// Creates a transformer builder for chaining multiple transformations.
///
/// # Examples
///
/// ```
/// use redstr::TransformBuilder;
/// let result = TransformBuilder::new("test")
///     .leetspeak()
///     .base64()
///     .build();
/// assert!(result.len() > 0);
/// ```
pub struct TransformBuilder {
    text: String,
}

impl TransformBuilder {
    /// Creates a new transformer with the given input text.
    pub fn new(input: &str) -> Self {
        Self {
            text: input.to_string(),
        }
    }

    /// Applies leetspeak transformation.
    pub fn leetspeak(mut self) -> Self {
        self.text = leetspeak(&self.text);
        self
    }

    /// Applies base64 encoding.
    pub fn base64(mut self) -> Self {
        self.text = base64_encode(&self.text);
        self
    }

    /// Applies URL encoding.
    pub fn url_encode(mut self) -> Self {
        self.text = url_encode(&self.text);
        self
    }

    /// Applies random capitalization.
    pub fn randomize_capitalization(mut self) -> Self {
        self.text = randomize_capitalization(&self.text);
        self
    }

    /// Applies random capitalization.
    #[deprecated(
        since = "0.2.7",
        note = "Use randomize_capitalization() for clearer intent."
    )]
    pub fn redstrs(self) -> Self {
        self.randomize_capitalization()
    }

    /// Applies homoglyph substitution.
    pub fn homoglyphs(mut self) -> Self {
        self.text = homoglyph_substitution(&self.text);
        self
    }

    /// Applies case swapping.
    pub fn case_swap(mut self) -> Self {
        self.text = case_swap(&self.text);
        self
    }

    /// Applies hex encoding.
    pub fn hex_encode(mut self) -> Self {
        self.text = hex_encode(&self.text);
        self
    }

    /// Applies ROT13 cipher.
    pub fn rot13(mut self) -> Self {
        self.text = rot13(&self.text);
        self
    }

    /// Applies advanced domain spoofing (for EvilJinx).
    pub fn advanced_domain_spoof(mut self) -> Self {
        self.text = advanced_domain_spoof(&self.text);
        self
    }

    /// Applies email obfuscation (for EvilJinx).
    pub fn email_obfuscation(mut self) -> Self {
        self.text = email_obfuscation(&self.text);
        self
    }

    /// Applies PowerShell obfuscation (for Windows pentesting).
    pub fn powershell_obfuscate(mut self) -> Self {
        self.text = powershell_obfuscate(&self.text);
        self
    }

    /// Applies bash obfuscation (for Linux pentesting).
    pub fn bash_obfuscate(mut self) -> Self {
        self.text = bash_obfuscate(&self.text);
        self
    }

    /// Applies Cloudflare challenge variation.
    pub fn cloudflare_challenge(mut self) -> Self {
        self.text = cloudflare_challenge_variation(&self.text);
        self
    }

    /// Applies Cloudflare Turnstile challenge variation.
    pub fn cloudflare_turnstile(mut self) -> Self {
        self.text = cloudflare_turnstile_variation(&self.text);
        self
    }

    /// Applies Cloudflare challenge response pattern.
    pub fn cloudflare_challenge_response(mut self) -> Self {
        self.text = cloudflare_challenge_response(&self.text);
        self
    }

    /// Applies GraphQL obfuscation (for Caido).
    pub fn graphql_obfuscate(mut self) -> Self {
        self.text = graphql_obfuscate(&self.text);
        self
    }

    /// Returns the transformed text.
    pub fn build(self) -> String {
        self.text
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_builder() {
        let result = TransformBuilder::new("test").leetspeak().build();
        assert!(result.len() > 0);

        let result2 = TransformBuilder::new("hello")
            .randomize_capitalization()
            .base64()
            .build();
        assert!(result2.len() > 0);
    }

    #[test]
    fn test_transform_builder_new_functions() {
        let result = TransformBuilder::new("paypal.com")
            .advanced_domain_spoof()
            .build();
        assert!(result.len() > 0);

        let result2 = TransformBuilder::new("admin@example.com")
            .email_obfuscation()
            .build();
        assert!(result2.contains("@"));

        let result3 = TransformBuilder::new("Get-Process")
            .powershell_obfuscate()
            .build();
        assert!(result3.len() > 0);
    }

    #[test]
    fn test_transform_builder_cloudflare_functions() {
        let result = TransformBuilder::new("challenge-token")
            .cloudflare_turnstile()
            .build();
        assert!(result.len() > 0);
        assert!(result.contains("challenge-token"));

        let result2 = TransformBuilder::new("cf_clearance=abc123")
            .cloudflare_challenge_response()
            .build();
        assert!(result2.len() > 0);
        assert!(
            result2.to_lowercase().contains("cf_clearance")
                || result2.to_lowercase().contains("cf-clearance")
        );

        let result3 = TransformBuilder::new("test")
            .cloudflare_turnstile()
            .cloudflare_challenge_response()
            .build();
        assert!(result3.len() > 0);
    }

    #[test]
    fn test_transform_builder_preserves_chain_semantics() {
        let manual = base64_encode(&case_swap("payload"));
        let built = TransformBuilder::new("payload")
            .case_swap()
            .base64()
            .build();
        assert_eq!(built.len(), manual.len());
    }

    #[test]
    #[allow(deprecated)]
    fn test_deprecated_redstrs_alias_still_works() {
        let output = TransformBuilder::new("hello").redstrs().build();
        assert_eq!(output.len(), "hello".len());
    }
}
