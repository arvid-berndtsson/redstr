use std::env;
use random_cap::*;

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
    eprintln!("Modes:");
    eprintln!("  random, r         Random capitalization (default)");
    eprintln!("  leetspeak, l      Convert to leetspeak");
    eprintln!("  alternate, a      Alternate upper/lower case");
    eprintln!("  inverse, i        Invert the case of each letter");
    eprintln!("  camel, c          Convert to camelCase");
    eprintln!("  snake, s          Convert to snake_case");
    eprintln!("  kebab, k          Convert to kebab-case");
    eprintln!("  unicode, u        Random unicode variations");
    eprintln!("  zalgo, z          Add zalgo combining characters");
    eprintln!("  reverse, rv       Reverse the string");
    eprintln!("  rot13             Apply ROT13 cipher");
    eprintln!("  homoglyph, h      Substitute with similar-looking characters");
    eprintln!("  vowel-swap, vs    Swap vowels randomly");
    eprintln!("  double, d         Double random characters");
    eprintln!("  space-variants, sv Use various space characters");
    eprintln!("  mixed-encoding, me Mix character encodings");
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  {} 'Hello World'", program_name);
    eprintln!("  {} leetspeak 'password123'", program_name);
    eprintln!("  {} homoglyph 'admin@example.com'", program_name);
}


