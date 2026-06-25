use redstr::*;
use std::env;

#[derive(Clone, Copy)]
struct ModeSpec {
    canonical: &'static str,
    aliases: &'static [&'static str],
    requires_input: bool,
}

const MODES: &[ModeSpec] = &[
    ModeSpec {
        canonical: "random",
        aliases: &["r"],
        requires_input: true,
    },
    ModeSpec {
        canonical: "leetspeak",
        aliases: &["l"],
        requires_input: true,
    },
    ModeSpec {
        canonical: "alternate",
        aliases: &["a"],
        requires_input: true,
    },
    ModeSpec {
        canonical: "inverse",
        aliases: &["i"],
        requires_input: true,
    },
    ModeSpec {
        canonical: "camel",
        aliases: &["c"],
        requires_input: true,
    },
    ModeSpec {
        canonical: "snake",
        aliases: &["s"],
        requires_input: true,
    },
    ModeSpec {
        canonical: "kebab",
        aliases: &["k"],
        requires_input: true,
    },
    ModeSpec {
        canonical: "unicode",
        aliases: &["u"],
        requires_input: true,
    },
    ModeSpec {
        canonical: "zalgo",
        aliases: &["z"],
        requires_input: true,
    },
    ModeSpec {
        canonical: "reverse",
        aliases: &["rv"],
        requires_input: true,
    },
    ModeSpec {
        canonical: "rot13",
        aliases: &[],
        requires_input: true,
    },
    ModeSpec {
        canonical: "homoglyph",
        aliases: &["h"],
        requires_input: true,
    },
    ModeSpec {
        canonical: "vowel-swap",
        aliases: &["vs"],
        requires_input: true,
    },
    ModeSpec {
        canonical: "double",
        aliases: &["d"],
        requires_input: true,
    },
    ModeSpec {
        canonical: "space-variants",
        aliases: &["sv"],
        requires_input: true,
    },
    ModeSpec {
        canonical: "mixed-encoding",
        aliases: &["me"],
        requires_input: true,
    },
    ModeSpec {
        canonical: "base64",
        aliases: &["b64"],
        requires_input: true,
    },
    ModeSpec {
        canonical: "url-encode",
        aliases: &["url"],
        requires_input: true,
    },
    ModeSpec {
        canonical: "sql-comment",
        aliases: &["sql"],
        requires_input: true,
    },
    ModeSpec {
        canonical: "xss-tags",
        aliases: &["xss"],
        requires_input: true,
    },
    ModeSpec {
        canonical: "case-swap",
        aliases: &["cs"],
        requires_input: true,
    },
    ModeSpec {
        canonical: "null-byte",
        aliases: &["nb"],
        requires_input: true,
    },
    ModeSpec {
        canonical: "path-traversal",
        aliases: &["pt"],
        requires_input: true,
    },
    ModeSpec {
        canonical: "command-injection",
        aliases: &["ci"],
        requires_input: true,
    },
    ModeSpec {
        canonical: "hex-encode",
        aliases: &["hex"],
        requires_input: true,
    },
    ModeSpec {
        canonical: "hex-mixed",
        aliases: &["hm"],
        requires_input: true,
    },
    ModeSpec {
        canonical: "random-user-agent",
        aliases: &[],
        requires_input: false,
    },
];

enum CliAction {
    Help,
    ListModes,
    Run {
        mode: &'static str,
        input: String,
        json: bool,
        seed: Option<u64>,
    },
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let action = match parse_args(&args) {
        Ok(action) => action,
        Err(message) => {
            eprintln!("{message}");
            print_usage(&args[0]);
            std::process::exit(1);
        }
    };

    match action {
        CliAction::Help => {
            print_usage(&args[0]);
        }
        CliAction::ListModes => {
            for mode in MODES {
                println!("{}", mode.canonical);
            }
        }
        CliAction::Run {
            mode,
            input,
            json,
            seed,
        } => {
            let result = run_mode(mode, &input, seed);
            if json {
                println!(
                    "{{\"mode\":\"{}\",\"input\":\"{}\",\"output\":\"{}\"}}",
                    escape_json(mode),
                    escape_json(&input),
                    escape_json(&result)
                );
            } else {
                println!("{result}");
            }
        }
    }
}

fn parse_args(args: &[String]) -> Result<CliAction, String> {
    if args.len() < 2 {
        return Err("Missing mode or input".to_string());
    }

    let mut idx = 1;
    let mut json = false;
    let mut seed = None;

    while idx < args.len() {
        let token = args[idx].as_str();
        match token {
            "--help" | "-h" => return Ok(CliAction::Help),
            "--list-modes" => return Ok(CliAction::ListModes),
            "--json" => {
                json = true;
                idx += 1;
            }
            "--seed" => {
                if idx + 1 >= args.len() {
                    return Err("Missing value for --seed".to_string());
                }
                let raw_seed = &args[idx + 1];
                let parsed_seed = raw_seed
                    .parse::<u64>()
                    .map_err(|_| format!("Invalid --seed value: {raw_seed}"))?;
                seed = Some(parsed_seed);
                idx += 2;
            }
            _ if token.starts_with('-') => return Err(format!("Unknown option: {token}")),
            _ => break,
        }
    }

    if idx >= args.len() {
        return Err("Missing mode or input".to_string());
    }

    let tail = &args[idx..];
    let (mode, input_parts) = match find_mode(tail[0].as_str()) {
        Some(spec) => (spec.canonical, &tail[1..]),
        None => ("random", tail),
    };

    let input = input_parts.join(" ");
    let requires_input = find_mode(mode).map(|m| m.requires_input).unwrap_or(true);

    if requires_input && input.is_empty() {
        return Err(format!("Mode '{mode}' requires input text"));
    }

    if !requires_input && !input.is_empty() {
        return Err(format!("Mode '{mode}' does not accept input text"));
    }

    Ok(CliAction::Run {
        mode,
        input,
        json,
        seed,
    })
}

fn find_mode(mode: &str) -> Option<ModeSpec> {
    MODES
        .iter()
        .copied()
        .find(|spec| spec.canonical == mode || spec.aliases.contains(&mode))
}

fn run_mode(mode: &str, input: &str, seed: Option<u64>) -> String {
    match mode {
        "random" => match seed {
            Some(seed_value) => randomize_capitalization_seeded(input, seed_value),
            None => randomize_capitalization(input),
        },
        "leetspeak" => leetspeak(input),
        "alternate" => alternate_case(input),
        "inverse" => inverse_case(input),
        "camel" => to_camel_case(input),
        "snake" => to_snake_case(input),
        "kebab" => to_kebab_case(input),
        "unicode" => unicode_variations(input),
        "zalgo" => zalgo_text(input),
        "reverse" => reverse_string(input),
        "rot13" => rot13(input),
        "homoglyph" => homoglyph_substitution(input),
        "vowel-swap" => vowel_swap(input),
        "double" => double_characters(input),
        "space-variants" => space_variants(input),
        "mixed-encoding" => mixed_encoding(input),
        "base64" => base64_encode(input),
        "url-encode" => url_encode(input),
        "sql-comment" => sql_comment_injection(input),
        "xss-tags" => xss_tag_variations(input),
        "case-swap" => match seed {
            Some(seed_value) => case_swap_seeded(input, seed_value),
            None => case_swap(input),
        },
        "null-byte" => null_byte_injection(input),
        "path-traversal" => path_traversal(input),
        "command-injection" => command_injection(input),
        "hex-encode" => hex_encode(input),
        "hex-mixed" => hex_encode_mixed(input),
        "random-user-agent" => random_user_agent(),
        _ => input.to_string(),
    }
}

fn randomize_capitalization_seeded(input: &str, seed: u64) -> String {
    let mut rng = SeededRng::new(seed);
    let mut result = String::with_capacity(input.len() * 2);

    for c in input.chars() {
        if c.is_alphabetic() {
            if rng.next().is_multiple_of(2) {
                for uc in c.to_uppercase() {
                    result.push(uc);
                }
            } else {
                for lc in c.to_lowercase() {
                    result.push(lc);
                }
            }
        } else {
            result.push(c);
        }
    }

    result
}

fn case_swap_seeded(input: &str, seed: u64) -> String {
    let mut rng = SeededRng::new(seed);
    let mut result = String::with_capacity(input.len() * 2);
    let mut swapped_any = false;

    for c in input.chars() {
        if c.is_alphabetic() && rng.next().is_multiple_of(2) {
            swapped_any = true;
            if c.is_uppercase() {
                for lc in c.to_lowercase() {
                    result.push(lc);
                }
            } else {
                for uc in c.to_uppercase() {
                    result.push(uc);
                }
            }
        } else {
            result.push(c);
        }
    }

    if !swapped_any && input.chars().any(|c| c.is_alphabetic()) {
        let mut guaranteed = String::with_capacity(result.len());
        let mut changed = false;

        for c in input.chars() {
            if !changed && c.is_alphabetic() {
                changed = true;
                if c.is_uppercase() {
                    for lc in c.to_lowercase() {
                        guaranteed.push(lc);
                    }
                } else {
                    for uc in c.to_uppercase() {
                        guaranteed.push(uc);
                    }
                }
            } else {
                guaranteed.push(c);
            }
        }
        return guaranteed;
    }

    result
}

fn escape_json(input: &str) -> String {
    let mut escaped = String::with_capacity(input.len());
    for c in input.chars() {
        match c {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            '\u{08}' => escaped.push_str("\\b"),
            '\u{0C}' => escaped.push_str("\\f"),
            c if c.is_control() => escaped.push_str(&format!("\\u{:04x}", c as u32)),
            _ => escaped.push(c),
        }
    }
    escaped
}

#[derive(Clone, Copy)]
struct SeededRng {
    state: u64,
}

impl SeededRng {
    fn new(seed: u64) -> Self {
        Self {
            state: seed ^ 0x9E37_79B9_7F4A_7C15,
        }
    }

    fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let mut x = self.state;
        x ^= x >> 33;
        x = x.wrapping_mul(0xff51afd7ed558ccd);
        x ^= x >> 33;
        x = x.wrapping_mul(0xc4ceb9fe1a85ec53);
        x ^ (x >> 33)
    }
}

fn print_usage(program_name: &str) {
    eprintln!("Random Cap - String Obfuscation Tool for Security Testing");
    eprintln!();
    eprintln!("Usage: {} [options] [mode] <text...>", program_name);
    eprintln!("       {} [options] random-user-agent", program_name);
    eprintln!();
    eprintln!("Options:");
    eprintln!("  --help, -h         Show this help");
    eprintln!("  --list-modes       List all available modes");
    eprintln!("  --json             Output JSON {{ mode, input, output }}");
    eprintln!("  --seed <u64>       Deterministic output for random and case-swap");
    eprintln!();
    eprintln!("Basic Transformations:");
    eprintln!("  random, r         Random capitalization (default)");
    eprintln!("  alternate, a      Alternate upper/lower case");
    eprintln!("  inverse, i        Invert the case of each letter");
    eprintln!("  reverse, rv       Reverse the string");
    eprintln!();
    eprintln!("Case Conversion:");
    eprintln!("  camel, c          Convert to camelCase");
    eprintln!("  snake, s          Convert to snake_case");
    eprintln!("  kebab, k          Convert to kebab-case");
    eprintln!();
    eprintln!("Security Testing - Red/Blue/Purple Team:");
    eprintln!("  leetspeak, l      Convert to leetspeak (filter evasion)");
    eprintln!("  homoglyph, h      Substitute with lookalike characters (phishing)");
    eprintln!("  unicode, u        Random unicode variations (normalization testing)");
    eprintln!("  zalgo, z          Add zalgo combining characters (display testing)");
    eprintln!("  rot13             Apply ROT13 cipher");
    eprintln!("  vowel-swap, vs    Swap vowels randomly (pattern matching)");
    eprintln!("  double, d         Double random characters (validation testing)");
    eprintln!("  space-variants, sv Use various space characters");
    eprintln!("  mixed-encoding, me Mix character encodings");
    eprintln!();
    eprintln!("Encoding/Obfuscation:");
    eprintln!("  base64, b64       Encode to Base64 (payload obfuscation)");
    eprintln!("  url-encode, url   URL/percent encoding (web testing)");
    eprintln!("  hex-encode, hex   Encode to hexadecimal");
    eprintln!("  hex-mixed, hm     Mixed hex formats (\\x, %, 0x, &#x)");
    eprintln!();
    eprintln!("Injection Testing:");
    eprintln!("  sql-comment, sql  SQL comment injection patterns");
    eprintln!("  xss-tags, xss     XSS tag variations (filter evasion)");
    eprintln!("  case-swap, cs     Random case swapping (WAF bypass)");
    eprintln!("  null-byte, nb     Null byte injection patterns");
    eprintln!("  path-traversal, pt Path traversal patterns (../)");
    eprintln!("  command-injection, ci OS command injection separators");
    eprintln!("  random-user-agent Generate a random user agent");
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  {} 'Hello World'", program_name);
    eprintln!("  {} leetspeak 'password123'", program_name);
    eprintln!("  {} --json base64 'hello'", program_name);
    eprintln!(
        "  {} --seed 42 case-swap 'SELECT * FROM users'",
        program_name
    );
    eprintln!("  {} --list-modes", program_name);
    eprintln!("  {} random-user-agent", program_name);
    eprintln!("  {} homoglyph 'admin@example.com'", program_name);
    eprintln!("  {} sql-comment 'SELECT * FROM users'", program_name);
    eprintln!("  {} xss-tags '<script>alert(1)</script>'", program_name);
}
