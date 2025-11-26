# Fish completion script for redstr
# Install: sudo cp redstr.fish /usr/share/fish/vendor_completions.d/
# Or: cp redstr.fish ~/.config/fish/completions/

# Basic transformations
complete -c redstr -n "__fish_use_subcommand" -a "random" -d "Random capitalization (default)"
complete -c redstr -n "__fish_use_subcommand" -a "r" -d "Random capitalization (short)"
complete -c redstr -n "__fish_use_subcommand" -a "alternate" -d "Alternate upper/lower case"
complete -c redstr -n "__fish_use_subcommand" -a "a" -d "Alternate case (short)"
complete -c redstr -n "__fish_use_subcommand" -a "inverse" -d "Invert case of each letter"
complete -c redstr -n "__fish_use_subcommand" -a "i" -d "Invert case (short)"
complete -c redstr -n "__fish_use_subcommand" -a "reverse" -d "Reverse the string"
complete -c redstr -n "__fish_use_subcommand" -a "rv" -d "Reverse string (short)"

# Case conversion
complete -c redstr -n "__fish_use_subcommand" -a "camel" -d "Convert to camelCase"
complete -c redstr -n "__fish_use_subcommand" -a "c" -d "camelCase (short)"
complete -c redstr -n "__fish_use_subcommand" -a "snake" -d "Convert to snake_case"
complete -c redstr -n "__fish_use_subcommand" -a "s" -d "snake_case (short)"
complete -c redstr -n "__fish_use_subcommand" -a "kebab" -d "Convert to kebab-case"
complete -c redstr -n "__fish_use_subcommand" -a "k" -d "kebab-case (short)"

# Security testing
complete -c redstr -n "__fish_use_subcommand" -a "leetspeak" -d "Convert to leetspeak"
complete -c redstr -n "__fish_use_subcommand" -a "l" -d "leetspeak (short)"
complete -c redstr -n "__fish_use_subcommand" -a "homoglyph" -d "Substitute with lookalike characters"
complete -c redstr -n "__fish_use_subcommand" -a "h" -d "homoglyph (short)"
complete -c redstr -n "__fish_use_subcommand" -a "unicode" -d "Random unicode variations"
complete -c redstr -n "__fish_use_subcommand" -a "u" -d "unicode (short)"
complete -c redstr -n "__fish_use_subcommand" -a "zalgo" -d "Add zalgo combining characters"
complete -c redstr -n "__fish_use_subcommand" -a "z" -d "zalgo (short)"
complete -c redstr -n "__fish_use_subcommand" -a "rot13" -d "Apply ROT13 cipher"
complete -c redstr -n "__fish_use_subcommand" -a "vowel-swap" -d "Swap vowels randomly"
complete -c redstr -n "__fish_use_subcommand" -a "vs" -d "vowel-swap (short)"
complete -c redstr -n "__fish_use_subcommand" -a "double" -d "Double random characters"
complete -c redstr -n "__fish_use_subcommand" -a "d" -d "double (short)"
complete -c redstr -n "__fish_use_subcommand" -a "space-variants" -d "Use various space characters"
complete -c redstr -n "__fish_use_subcommand" -a "sv" -d "space-variants (short)"
complete -c redstr -n "__fish_use_subcommand" -a "mixed-encoding" -d "Mix character encodings"
complete -c redstr -n "__fish_use_subcommand" -a "me" -d "mixed-encoding (short)"

# Encoding/Obfuscation
complete -c redstr -n "__fish_use_subcommand" -a "base64" -d "Encode to Base64"
complete -c redstr -n "__fish_use_subcommand" -a "b64" -d "base64 (short)"
complete -c redstr -n "__fish_use_subcommand" -a "url-encode" -d "URL/percent encoding"
complete -c redstr -n "__fish_use_subcommand" -a "url" -d "url-encode (short)"
complete -c redstr -n "__fish_use_subcommand" -a "hex-encode" -d "Encode to hexadecimal"
complete -c redstr -n "__fish_use_subcommand" -a "hex" -d "hex-encode (short)"
complete -c redstr -n "__fish_use_subcommand" -a "hex-mixed" -d "Mixed hex formats"
complete -c redstr -n "__fish_use_subcommand" -a "hm" -d "hex-mixed (short)"

# Injection testing
complete -c redstr -n "__fish_use_subcommand" -a "sql-comment" -d "SQL comment injection patterns"
complete -c redstr -n "__fish_use_subcommand" -a "sql" -d "sql-comment (short)"
complete -c redstr -n "__fish_use_subcommand" -a "xss-tags" -d "XSS tag variations"
complete -c redstr -n "__fish_use_subcommand" -a "xss" -d "xss-tags (short)"
complete -c redstr -n "__fish_use_subcommand" -a "case-swap" -d "Random case swapping"
complete -c redstr -n "__fish_use_subcommand" -a "cs" -d "case-swap (short)"
complete -c redstr -n "__fish_use_subcommand" -a "null-byte" -d "Null byte injection"
complete -c redstr -n "__fish_use_subcommand" -a "nb" -d "null-byte (short)"
complete -c redstr -n "__fish_use_subcommand" -a "path-traversal" -d "Path traversal patterns"
complete -c redstr -n "__fish_use_subcommand" -a "pt" -d "path-traversal (short)"
complete -c redstr -n "__fish_use_subcommand" -a "command-injection" -d "Command injection separators"
complete -c redstr -n "__fish_use_subcommand" -a "ci" -d "command-injection (short)"

# Help
complete -c redstr -s h -l help -d "Show help message"
