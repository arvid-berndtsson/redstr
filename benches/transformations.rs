use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use redstr::*;

const SHORT_INPUT: &str = "test";
const MEDIUM_INPUT: &str = "This is a medium length test string for benchmarking";
const LONG_INPUT: &str = "This is a much longer test string that we'll use for benchmarking performance of various transformation functions. It contains multiple sentences and should give us a good idea of how the functions perform with realistic input sizes.";

fn bench_case_transformations(c: &mut Criterion) {
    let mut group = c.benchmark_group("case_transformations");
    
    for input in [SHORT_INPUT, MEDIUM_INPUT, LONG_INPUT] {
        let size = input.len();
        
        group.bench_with_input(BenchmarkId::new("randomize_capitalization", size), input, |b, i| {
            b.iter(|| randomize_capitalization(black_box(i)))
        });
        
        group.bench_with_input(BenchmarkId::new("alternate_case", size), input, |b, i| {
            b.iter(|| alternate_case(black_box(i)))
        });
        
        group.bench_with_input(BenchmarkId::new("inverse_case", size), input, |b, i| {
            b.iter(|| inverse_case(black_box(i)))
        });
        
        group.bench_with_input(BenchmarkId::new("case_swap", size), input, |b, i| {
            b.iter(|| case_swap(black_box(i)))
        });
        
        group.bench_with_input(BenchmarkId::new("to_camel_case", size), input, |b, i| {
            b.iter(|| to_camel_case(black_box(i)))
        });
        
        group.bench_with_input(BenchmarkId::new("to_snake_case", size), input, |b, i| {
            b.iter(|| to_snake_case(black_box(i)))
        });
        
        group.bench_with_input(BenchmarkId::new("to_kebab_case", size), input, |b, i| {
            b.iter(|| to_kebab_case(black_box(i)))
        });
    }
    
    group.finish();
}

fn bench_encoding_transformations(c: &mut Criterion) {
    let mut group = c.benchmark_group("encoding_transformations");
    
    for input in [SHORT_INPUT, MEDIUM_INPUT, LONG_INPUT] {
        let size = input.len();
        
        group.bench_with_input(BenchmarkId::new("base64_encode", size), input, |b, i| {
            b.iter(|| base64_encode(black_box(i)))
        });
        
        group.bench_with_input(BenchmarkId::new("url_encode", size), input, |b, i| {
            b.iter(|| url_encode(black_box(i)))
        });
        
        group.bench_with_input(BenchmarkId::new("hex_encode", size), input, |b, i| {
            b.iter(|| hex_encode(black_box(i)))
        });
        
        group.bench_with_input(BenchmarkId::new("hex_encode_mixed", size), input, |b, i| {
            b.iter(|| hex_encode_mixed(black_box(i)))
        });
        
        group.bench_with_input(BenchmarkId::new("html_entity_encode", size), input, |b, i| {
            b.iter(|| html_entity_encode(black_box(i)))
        });
        
        group.bench_with_input(BenchmarkId::new("mixed_encoding", size), input, |b, i| {
            b.iter(|| mixed_encoding(black_box(i)))
        });
    }
    
    group.finish();
}

fn bench_obfuscation_transformations(c: &mut Criterion) {
    let mut group = c.benchmark_group("obfuscation_transformations");
    
    for input in [SHORT_INPUT, MEDIUM_INPUT, LONG_INPUT] {
        let size = input.len();
        
        group.bench_with_input(BenchmarkId::new("leetspeak", size), input, |b, i| {
            b.iter(|| leetspeak(black_box(i)))
        });
        
        group.bench_with_input(BenchmarkId::new("rot13", size), input, |b, i| {
            b.iter(|| rot13(black_box(i)))
        });
        
        group.bench_with_input(BenchmarkId::new("reverse_string", size), input, |b, i| {
            b.iter(|| reverse_string(black_box(i)))
        });
        
        group.bench_with_input(BenchmarkId::new("vowel_swap", size), input, |b, i| {
            b.iter(|| vowel_swap(black_box(i)))
        });
        
        group.bench_with_input(BenchmarkId::new("double_characters", size), input, |b, i| {
            b.iter(|| double_characters(black_box(i)))
        });
        
        group.bench_with_input(BenchmarkId::new("js_string_concat", size), input, |b, i| {
            b.iter(|| js_string_concat(black_box(i)))
        });
        
        group.bench_with_input(BenchmarkId::new("whitespace_padding", size), input, |b, i| {
            b.iter(|| whitespace_padding(black_box(i)))
        });
    }
    
    group.finish();
}

fn bench_unicode_transformations(c: &mut Criterion) {
    let mut group = c.benchmark_group("unicode_transformations");
    
    for input in [SHORT_INPUT, MEDIUM_INPUT, LONG_INPUT] {
        let size = input.len();
        
        group.bench_with_input(BenchmarkId::new("homoglyph_substitution", size), input, |b, i| {
            b.iter(|| homoglyph_substitution(black_box(i)))
        });
        
        group.bench_with_input(BenchmarkId::new("unicode_variations", size), input, |b, i| {
            b.iter(|| unicode_variations(black_box(i)))
        });
        
        group.bench_with_input(BenchmarkId::new("zalgo_text", size), input, |b, i| {
            b.iter(|| zalgo_text(black_box(i)))
        });
        
        group.bench_with_input(BenchmarkId::new("space_variants", size), input, |b, i| {
            b.iter(|| space_variants(black_box(i)))
        });
        
        group.bench_with_input(BenchmarkId::new("unicode_normalize_variants", size), input, |b, i| {
            b.iter(|| unicode_normalize_variants(black_box(i)))
        });
    }
    
    group.finish();
}

fn bench_injection_transformations(c: &mut Criterion) {
    let mut group = c.benchmark_group("injection_transformations");
    
    let sql_input = "SELECT * FROM users WHERE id = 1";
    let xss_input = "<script>alert('test')</script>";
    let path_input = "/etc/passwd";
    
    group.bench_function("sql_comment_injection", |b| {
        b.iter(|| sql_comment_injection(black_box(sql_input)))
    });
    
    group.bench_function("xss_tag_variations", |b| {
        b.iter(|| xss_tag_variations(black_box(xss_input)))
    });
    
    group.bench_function("command_injection", |b| {
        b.iter(|| command_injection(black_box("ls -la")))
    });
    
    group.bench_function("path_traversal", |b| {
        b.iter(|| path_traversal(black_box(path_input)))
    });
    
    group.bench_function("null_byte_injection", |b| {
        b.iter(|| null_byte_injection(black_box("file.txt")))
    });
    
    group.finish();
}

fn bench_phishing_transformations(c: &mut Criterion) {
    let mut group = c.benchmark_group("phishing_transformations");
    
    group.bench_function("domain_typosquat", |b| {
        b.iter(|| domain_typosquat(black_box("example.com")))
    });
    
    group.bench_function("advanced_domain_spoof", |b| {
        b.iter(|| advanced_domain_spoof(black_box("paypal.com")))
    });
    
    group.bench_function("email_obfuscation", |b| {
        b.iter(|| email_obfuscation(black_box("admin@example.com")))
    });
    
    group.bench_function("url_shortening_pattern", |b| {
        b.iter(|| url_shortening_pattern(black_box("https://example.com/very/long/path")))
    });
    
    group.finish();
}

fn bench_web_security_transformations(c: &mut Criterion) {
    let mut group = c.benchmark_group("web_security_transformations");
    
    group.bench_function("http_header_variation", |b| {
        b.iter(|| http_header_variation(black_box("User-Agent")))
    });
    
    group.bench_function("jwt_header_manipulation", |b| {
        b.iter(|| jwt_header_manipulation(black_box("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9")))
    });
    
    group.bench_function("graphql_obfuscate", |b| {
        b.iter(|| graphql_obfuscate(black_box("query { users { id name } }")))
    });
    
    group.bench_function("api_endpoint_variation", |b| {
        b.iter(|| api_endpoint_variation(black_box("/api/v1/users")))
    });
    
    group.finish();
}

fn bench_bot_detection_transformations(c: &mut Criterion) {
    let mut group = c.benchmark_group("bot_detection_transformations");
    
    group.bench_function("random_user_agent", |b| {
        b.iter(|| random_user_agent())
    });
    
    group.bench_function("cloudflare_challenge_variation", |b| {
        b.iter(|| cloudflare_challenge_variation(black_box("challenge_token")))
    });
    
    group.bench_function("tls_fingerprint_variation", |b| {
        b.iter(|| tls_fingerprint_variation(black_box("ja3_fingerprint")))
    });
    
    group.bench_function("http2_header_order", |b| {
        b.iter(|| http2_header_order(black_box("header_order")))
    });
    
    group.bench_function("accept_language_variation", |b| {
        b.iter(|| accept_language_variation(black_box("en-US")))
    });
    
    group.finish();
}

fn bench_builder_pattern(c: &mut Criterion) {
    let mut group = c.benchmark_group("builder_pattern");
    
    group.bench_function("simple_chain", |b| {
        b.iter(|| {
            TransformBuilder::new(black_box("test input"))
                .leetspeak()
                .base64()
                .build()
        })
    });
    
    group.bench_function("complex_chain", |b| {
        b.iter(|| {
            TransformBuilder::new(black_box("test input"))
                .leetspeak()
                .base64()
                .url_encode()
                .case_swap()
                .build()
        })
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_case_transformations,
    bench_encoding_transformations,
    bench_obfuscation_transformations,
    bench_unicode_transformations,
    bench_injection_transformations,
    bench_phishing_transformations,
    bench_web_security_transformations,
    bench_bot_detection_transformations,
    bench_builder_pattern,
);

criterion_main!(benches);
