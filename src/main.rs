use std::env;
use polystr::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage(&args[0]);
        std::process::exit(1);
    }
    
    let mode = if args.len() >= 3 { &args[1] } else { "random" };
    let input = if args.len() >= 3 { &args[2] } else { &args[1] };
    
    let result = match mode {
        "random" | "r" => randomize_capitalization(input),
        "leetspeak" | "l" => leetspeak(input),
        "alternate" | "a" => alternate_case(input),
        "inverse" | "i" => inverse_case(input),
        "camel" | "c" => to_camel_case(input),
        "snake" | "s" => to_snake_case(input),
        "kebab" | "k" => to_kebab_case(input),
        "unicode" | "u" => unicode_variations(input),
        "zalgo" | "z" => zalgo_text(input),
        "reverse" | "rv" => reverse_string(input),
        "rot13" => rot13(input),
        "homoglyph" | "h" => homoglyph_substitution(input),
        "vowel-swap" | "vs" => vowel_swap(input),
        "double" | "d" => double_characters(input),
        "space-variants" | "sv" => space_variants(input),
        "mixed-encoding" | "me" => mixed_encoding(input),
        "base64" | "b64" => base64_encode(input),
        "url-encode" | "url" => url_encode(input),
        "sql-comment" | "sql" => sql_comment_injection(input),
        "xss-tags" | "xss" => xss_tag_variations(input),
        "case-swap" | "cs" => case_swap(input),
        "null-byte" | "nb" => null_byte_injection(input),
        "path-traversal" | "pt" => path_traversal(input),
        "command-injection" | "ci" => command_injection(input),
        "hex-encode" | "hex" => hex_encode(input),
        "hex-mixed" | "hm" => hex_encode_mixed(input),
        "--help" | "-h" => {
            print_usage(&args[0]);
            std::process::exit(0);
        }
        _ => {
            eprintln!("Unknown mode: {}", mode);
            print_usage(&args[0]);
            std::process::exit(1);
        }
    };
    
    println!("{}", result);
}

fn print_usage(program_name: &str) {
    eprintln!("Random Cap - String Obfuscation Tool for Security Testing");
    eprintln!();
    eprintln!("Usage: {} [mode] <text>", program_name);
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
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  {} 'Hello World'", program_name);
    eprintln!("  {} leetspeak 'password123'", program_name);
    eprintln!("  {} homoglyph 'admin@example.com'", program_name);
    eprintln!("  {} sql-comment 'SELECT * FROM users'", program_name);
    eprintln!("  {} xss-tags '<script>alert(1)</script>'", program_name);
}


