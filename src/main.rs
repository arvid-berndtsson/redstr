use redstr::*;
use std::env;

#[derive(Debug, Default)]
struct CliOptions {
    json: bool,
    list_modes: bool,
    show_help: bool,
    seed: Option<u64>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program_name = &args[0];

    let (options, positional) = match parse_options(&args[1..]) {
        Ok(parsed) => parsed,
        Err(err) => {
            eprintln!("{err}");
            print_usage(program_name);
            std::process::exit(1);
        }
    };

    if options.list_modes {
        print_modes();
        return;
    }
    if options.show_help {
        print_usage(program_name);
        return;
    }

    let (mode, input_args) = parse_mode_and_input(&positional);
    let input = input_args.join(" ");

    let result = match apply_mode(mode, &input, &input_args, options.seed) {
        Ok(result) => result,
        Err(err) => {
            eprintln!("{err}");
            print_usage(program_name);
            std::process::exit(1);
        }
    };

    if options.json {
        print_json_output(mode, &input, &result);
    } else {
        println!("{result}");
    }
}

fn parse_options(args: &[String]) -> Result<(CliOptions, Vec<String>), String> {
    let mut options = CliOptions::default();
    let mut positional = Vec::new();
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "--help" | "-h" => options.show_help = true,
            "--json" => options.json = true,
            "--list-modes" | "--list" => options.list_modes = true,
            "--seed" => {
                let Some(value) = args.get(i + 1) else {
                    return Err("Missing value for --seed".to_string());
                };
                let parsed = value
                    .parse::<u64>()
                    .map_err(|_| format!("Invalid --seed value: {value}"))?;
                options.seed = Some(parsed);
                i += 1;
            }
            value if value.starts_with('-') => return Err(format!("Unknown option: {value}")),
            _ => positional.push(args[i].clone()),
        }
        i += 1;
    }

    if positional.is_empty() && !options.list_modes && !options.show_help {
        return Err("Missing input text or mode".to_string());
    }

    Ok((options, positional))
}

fn parse_mode_and_input(positional: &[String]) -> (&str, Vec<String>) {
    match positional {
        [] => ("random", vec![]),
        [single] => {
            if is_no_input_mode(single) {
                (single, vec![])
            } else {
                ("random", vec![single.clone()])
            }
        }
        _ => (positional[0].as_str(), positional[1..].to_vec()),
    }
}

fn is_no_input_mode(mode: &str) -> bool {
    matches!(mode, "random-user-agent" | "rua" | "ua")
}

fn apply_mode(
    mode: &str,
    input: &str,
    input_args: &[String],
    seed: Option<u64>,
) -> Result<String, String> {
    let output = match mode {
        // Case / core
        "random" | "r" => match seed {
            Some(seed) => randomize_capitalization_with_seed(input, seed),
            None => randomize_capitalization(input),
        },
        "leetspeak" | "l" => leetspeak(input),
        "alternate" | "a" => alternate_case(input),
        "inverse" | "i" => inverse_case(input),
        "camel" | "c" => to_camel_case(input),
        "snake" | "s" => to_snake_case(input),
        "kebab" | "k" => to_kebab_case(input),
        "case-swap" | "cs" => match seed {
            Some(seed) => case_swap_with_seed(input, seed),
            None => case_swap(input),
        },

        // Unicode / obfuscation
        "unicode" | "u" => unicode_variations(input),
        "unicode-normalize" | "unorm" => unicode_normalize_variants(input),
        "zalgo" | "z" => zalgo_text(input),
        "homoglyph" | "h" => homoglyph_substitution(input),
        "space-variants" | "sv" => space_variants(input),
        "reverse" | "rv" => reverse_string(input),
        "rot13" => rot13(input),
        "vowel-swap" | "vs" => vowel_swap(input),
        "double" | "d" => double_characters(input),
        "whitespace-padding" | "wp" => whitespace_padding(input),
        "js-concat" | "js" => js_string_concat(input),

        // Encoding
        "mixed-encoding" | "me" => mixed_encoding(input),
        "base64" | "b64" => base64_encode(input),
        "url-encode" | "url" => url_encode(input),
        "hex-encode" | "hex" => hex_encode(input),
        "hex-mixed" | "hm" => hex_encode_mixed(input),
        "html-entity" | "html" => html_entity_encode(input),

        // Injection
        "sql-comment" | "sql" => sql_comment_injection(input),
        "xss-tags" | "xss" => xss_tag_variations(input),
        "null-byte" | "nb" => null_byte_injection(input),
        "path-traversal" | "pt" => path_traversal(input),
        "command-injection" | "ci" => command_injection(input),
        "mongodb-injection" | "mongo" => mongodb_injection(input),
        "couchdb-injection" | "couch" => couchdb_injection(input),
        "dynamodb-obfuscate" | "dynamo" => dynamodb_obfuscate(input),
        "nosql-operator" | "nosql" => nosql_operator_injection(input),
        "ssti-injection" | "ssti" => ssti_injection(input),
        "ssti-syntax" | "ssti-syntax-obf" => ssti_syntax_obfuscate(input),
        "ssti-framework" | "ssti-fw" => {
            if input_args.len() < 2 {
                return Err(
                    "ssti-framework requires two arguments: <framework> <template>".to_string(),
                );
            }
            let framework = &input_args[0];
            let template = input_args[1..].join(" ");
            ssti_framework_variation(&template, framework)
        }

        // Phishing
        "domain-typosquat" | "typo" => domain_typosquat(input),
        "advanced-domain-spoof" | "spoof" => advanced_domain_spoof(input),
        "email-obfuscation" | "email-obf" => email_obfuscation(input),
        "url-shortening" | "short-url" => url_shortening_pattern(input),

        // Bot detection / Cloudflare
        "random-user-agent" | "rua" | "ua" => random_user_agent(),
        "http2-header-order" | "h2-order" => http2_header_order(input),
        "tls-fingerprint" | "tls-fp" => tls_fingerprint_variation(input),
        "accept-language" | "alv" => accept_language_variation(input),
        "cf-challenge-variation" | "cfcv" => cloudflare_challenge_variation(input),
        "cf-turnstile" | "cft" => cloudflare_turnstile_variation(input),
        "cf-challenge-response" | "cfr" => cloudflare_challenge_response(input),
        "tls-handshake" | "tlsh" => tls_handshake_pattern(input),
        "canvas-fingerprint" | "canvas" => canvas_fingerprint_variation(input),
        "webgl-obfuscate" | "webgl" => webgl_fingerprint_obfuscate(input),
        "font-fingerprint" | "fontfp" => font_fingerprint_consistency(input),

        // Web security
        "http-header" | "hhv" => http_header_variation(input),
        "api-endpoint" | "api" => api_endpoint_variation(input),
        "graphql-obfuscate" | "gql" => graphql_obfuscate(input),
        "graphql-variable" | "gql-var" => graphql_variable_injection(input),
        "graphql-introspection" | "gql-intro" => graphql_introspection_bypass(input),
        "session-token" | "sess" => session_token_variation(input),
        "jwt-header" | "jwt-h" => jwt_header_manipulation(input),
        "jwt-payload" | "jwt-p" => jwt_payload_obfuscate(input),
        "jwt-alg-confusion" | "jwt-alg" => jwt_algorithm_confusion(input),
        "jwt-signature-bypass" | "jwt-sig" => jwt_signature_bypass(input),
        "html-input-attr" | "html-attr" => html_input_attribute_variation(input),
        "html-form-field" | "html-field" => html_form_field_obfuscate(input),
        "html-input-type" | "html-type" => html_input_type_variation(input),
        "html-form-action" | "html-action" => html_form_action_variation(input),
        "html-input-value" | "html-value" => html_input_value_obfuscate(input),

        // Shell
        "bash-obfuscate" | "bash" => bash_obfuscate(input),
        "powershell-obfuscate" | "psh" => powershell_obfuscate(input),
        "env-var-obfuscate" | "env" => env_var_obfuscate(input),
        "file-path-obfuscate" | "path-obf" => file_path_obfuscate(input),

        _ => return Err(format!("Unknown mode: {mode}")),
    };

    Ok(output)
}

fn print_usage(program_name: &str) {
    eprintln!("redstr - String Obfuscation Tool for Security Testing");
    eprintln!();
    eprintln!("Usage:");
    eprintln!("  {program_name} [options] [mode] <text...>");
    eprintln!("  {program_name} [options] random-user-agent");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  -h, --help        Show this help");
    eprintln!("  --list-modes      List all available modes");
    eprintln!("  --json            Output JSON");
    eprintln!("  --seed <u64>      Deterministic seed (random/case-swap modes)");
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  {program_name} 'Hello World'");
    eprintln!("  {program_name} leetspeak password123");
    eprintln!("  {program_name} ssti-framework jinja2 '{{{{7*7}}}}'");
    eprintln!("  {program_name} --json xss-tags '<script>alert(1)</script>'");
}

fn print_modes() {
    let modes = [
        "random",
        "leetspeak",
        "alternate",
        "inverse",
        "camel",
        "snake",
        "kebab",
        "case-swap",
        "unicode",
        "unicode-normalize",
        "zalgo",
        "homoglyph",
        "space-variants",
        "reverse",
        "rot13",
        "vowel-swap",
        "double",
        "whitespace-padding",
        "js-concat",
        "mixed-encoding",
        "base64",
        "url-encode",
        "hex-encode",
        "hex-mixed",
        "html-entity",
        "sql-comment",
        "xss-tags",
        "null-byte",
        "path-traversal",
        "command-injection",
        "mongodb-injection",
        "couchdb-injection",
        "dynamodb-obfuscate",
        "nosql-operator",
        "ssti-injection",
        "ssti-syntax",
        "ssti-framework",
        "domain-typosquat",
        "advanced-domain-spoof",
        "email-obfuscation",
        "url-shortening",
        "random-user-agent",
        "http2-header-order",
        "tls-fingerprint",
        "accept-language",
        "cf-challenge-variation",
        "cf-turnstile",
        "cf-challenge-response",
        "tls-handshake",
        "canvas-fingerprint",
        "webgl-obfuscate",
        "font-fingerprint",
        "http-header",
        "api-endpoint",
        "graphql-obfuscate",
        "graphql-variable",
        "graphql-introspection",
        "session-token",
        "jwt-header",
        "jwt-payload",
        "jwt-alg-confusion",
        "jwt-signature-bypass",
        "html-input-attr",
        "html-form-field",
        "html-input-type",
        "html-form-action",
        "html-input-value",
        "bash-obfuscate",
        "powershell-obfuscate",
        "env-var-obfuscate",
        "file-path-obfuscate",
    ];

    for mode in modes {
        println!("{mode}");
    }
}

fn print_json_output(mode: &str, input: &str, output: &str) {
    println!(
        "{{\"mode\":\"{}\",\"input\":\"{}\",\"output\":\"{}\"}}",
        json_escape(mode),
        json_escape(input),
        json_escape(output)
    );
}

fn json_escape(input: &str) -> String {
    let mut escaped = String::with_capacity(input.len());
    for c in input.chars() {
        match c {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            c if c.is_control() => escaped.push_str(&format!("\\u{:04x}", c as u32)),
            _ => escaped.push(c),
        }
    }
    escaped
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mode_and_input_single_defaults_to_random() {
        let positional = vec!["Hello World".to_string()];
        let (mode, input) = parse_mode_and_input(&positional);
        assert_eq!(mode, "random");
        assert_eq!(input, vec!["Hello World".to_string()]);
    }

    #[test]
    fn test_parse_options_seed() {
        let args = vec![
            "--json".to_string(),
            "--seed".to_string(),
            "42".to_string(),
            "random".to_string(),
            "hello".to_string(),
        ];
        let (options, positional) = parse_options(&args).unwrap();
        assert!(options.json);
        assert_eq!(options.seed, Some(42));
        assert_eq!(positional, vec!["random".to_string(), "hello".to_string()]);
    }

    #[test]
    fn test_seeded_mode_is_deterministic() {
        let one = apply_mode("random", "hello", &["hello".to_string()], Some(123)).unwrap();
        let two = apply_mode("random", "hello", &["hello".to_string()], Some(123)).unwrap();
        assert_eq!(one, two);
    }

    #[test]
    fn test_json_escape() {
        let escaped = json_escape("line1\n\"line2\"");
        assert_eq!(escaped, "line1\\n\\\"line2\\\"");
    }
}
