#compdef redstr
# Zsh completion script for redstr
# Install: sudo cp redstr.zsh /usr/share/zsh/site-functions/_redstr
# Or: Copy to a directory in your $fpath

_redstr() {
    local -a modes
    modes=(
        'random:Random capitalization (default)'
        'r:Random capitalization (short)'
        'leetspeak:Convert to leetspeak'
        'l:Convert to leetspeak (short)'
        'alternate:Alternate upper/lower case'
        'a:Alternate upper/lower case (short)'
        'inverse:Invert case of each letter'
        'i:Invert case (short)'
        'camel:Convert to camelCase'
        'c:Convert to camelCase (short)'
        'snake:Convert to snake_case'
        's:Convert to snake_case (short)'
        'kebab:Convert to kebab-case'
        'k:Convert to kebab-case (short)'
        'unicode:Random unicode variations'
        'u:Random unicode variations (short)'
        'zalgo:Add zalgo combining characters'
        'z:Add zalgo characters (short)'
        'reverse:Reverse the string'
        'rv:Reverse the string (short)'
        'rot13:Apply ROT13 cipher'
        'homoglyph:Substitute with lookalike characters'
        'h:Substitute with lookalikes (short)'
        'vowel-swap:Swap vowels randomly'
        'vs:Swap vowels (short)'
        'double:Double random characters'
        'd:Double random characters (short)'
        'space-variants:Use various space characters'
        'sv:Use various spaces (short)'
        'mixed-encoding:Mix character encodings'
        'me:Mix encodings (short)'
        'base64:Encode to Base64'
        'b64:Encode to Base64 (short)'
        'url-encode:URL/percent encoding'
        'url:URL encoding (short)'
        'sql-comment:SQL comment injection patterns'
        'sql:SQL injection (short)'
        'xss-tags:XSS tag variations'
        'xss:XSS tags (short)'
        'case-swap:Random case swapping'
        'cs:Case swap (short)'
        'null-byte:Null byte injection'
        'nb:Null byte (short)'
        'path-traversal:Path traversal patterns'
        'pt:Path traversal (short)'
        'command-injection:Command injection separators'
        'ci:Command injection (short)'
        'hex-encode:Encode to hexadecimal'
        'hex:Hex encode (short)'
        'hex-mixed:Mixed hex formats'
        'hm:Mixed hex (short)'
        '--help:Show help message'
        '-h:Show help (short)'
    )
    
    _arguments \
        '1: :->mode' \
        '2: :->text' \
        && return 0
    
    case $state in
        mode)
            _describe 'redstr modes' modes
            ;;
        text)
            _message 'text to transform'
            ;;
    esac
    
    return 0
}

_redstr "$@"
