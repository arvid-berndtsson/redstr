use redstr::TransformBuilder;
use redstr::*;

fn main() {
    println!("=== Cloudflare Evasion Examples ===\n");

    // Example 1: Cloudflare Turnstile challenge variation
    let challenge = "challenge-token-abc123";
    println!("1. Cloudflare Turnstile Challenge Variation:");
    println!("   Input:  {}", challenge);
    println!("   Output: {}\n", cloudflare_turnstile_variation(challenge));

    // Example 2: Cloudflare challenge response
    let cf_clearance = "cf_clearance=abc123def456";
    println!("2. Cloudflare Challenge Response:");
    println!("   Input:  {}", cf_clearance);
    println!(
        "   Output: {}\n",
        cloudflare_challenge_response(cf_clearance)
    );

    // Example 3: TLS handshake pattern variation
    let tls_pattern = "TLS_AES_256_GCM_SHA384:TLS_CHACHA20_POLY1305_SHA256:TLS_AES_128_GCM_SHA256";
    println!("3. TLS Handshake Pattern Variation:");
    println!("   Input:  {}", tls_pattern);
    println!("   Output: {}\n", tls_handshake_pattern(tls_pattern));

    // Example 4: Canvas fingerprint variation
    let canvas_data = "canvas-fingerprint-hash-abc123";
    println!("4. Canvas Fingerprint Variation:");
    println!("   Input:  {}", canvas_data);
    println!("   Output: {}\n", canvas_fingerprint_variation(canvas_data));

    // Example 5: WebGL fingerprint obfuscation
    let webgl_renderer =
        "WebGL 2.0 Renderer: ANGLE (Intel, Intel(R) UHD Graphics 620 Direct3D11 vs_5_0 ps_5_0)";
    println!("5. WebGL Fingerprint Obfuscation:");
    println!("   Input:  {}", webgl_renderer);
    println!(
        "   Output: {}\n",
        webgl_fingerprint_obfuscate(webgl_renderer)
    );

    // Example 6: Font fingerprint consistency
    let font_list = "Arial, Helvetica, Times New Roman, Courier New";
    println!("6. Font Fingerprint Consistency:");
    println!("   Input:  {}", font_list);
    println!("   Output: {}\n", font_fingerprint_consistency(font_list));

    // Example 7: Combining multiple Cloudflare evasion techniques
    let payload = "test-payload";
    println!("7. Combined Cloudflare Evasion (using TransformBuilder):");
    println!("   Input:  {}", payload);
    let result = TransformBuilder::new(payload)
        .cloudflare_turnstile()
        .cloudflare_challenge_response()
        .build();
    println!("   Output: {}\n", result);

    // Example 8: Real-world Cloudflare bypass scenario
    let user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36";
    println!("8. Real-world Cloudflare Bypass Scenario:");
    println!("   User-Agent: {}", user_agent);
    println!("   Random UA:   {}", random_user_agent());
    println!(
        "   TLS Pattern: {}",
        tls_handshake_pattern("TLS_AES_256_GCM_SHA384")
    );
    println!(
        "   Challenge:   {}\n",
        cloudflare_turnstile_variation("turnstile-challenge")
    );

    // Example 9: Browser fingerprinting evasion chain
    println!("9. Browser Fingerprinting Evasion Chain:");
    let fingerprint_data = "browser-fingerprint-data";
    println!("   Original: {}", fingerprint_data);
    println!(
        "   Canvas:   {}",
        canvas_fingerprint_variation(fingerprint_data)
    );
    println!(
        "   WebGL:    {}",
        webgl_fingerprint_obfuscate(fingerprint_data)
    );
    println!(
        "   Fonts:    {}\n",
        font_fingerprint_consistency("Arial, Helvetica")
    );

    // Example 10: Multiple challenge variations
    println!("10. Multiple Challenge Variations:");
    let challenges = vec![
        "cf_clearance=token123",
        "__cf_bm=cookie456",
        "turnstile-challenge-789",
    ];
    for challenge in challenges {
        println!(
            "   {} -> {}",
            challenge,
            cloudflare_challenge_response(challenge)
        );
    }
}
