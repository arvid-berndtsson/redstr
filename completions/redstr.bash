# Bash completion script for redstr
# Install: sudo cp redstr.bash /etc/bash_completion.d/
# Or: source this file in your ~/.bashrc

_redstr_completions() {
    local cur prev opts
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    
    # Available modes
    opts="random r leetspeak l alternate a inverse i camel c snake s kebab k \
          unicode u zalgo z reverse rv rot13 homoglyph h vowel-swap vs double d \
          space-variants sv mixed-encoding me base64 b64 url-encode url \
          sql-comment sql xss-tags xss case-swap cs null-byte nb path-traversal pt \
          command-injection ci hex-encode hex hex-mixed hm --help -h"
    
    # If we're completing the first argument (mode), show all options
    if [ $COMP_CWORD -eq 1 ]; then
        COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
        return 0
    fi
    
    # If we're completing the second argument (text), don't autocomplete
    if [ $COMP_CWORD -eq 2 ]; then
        return 0
    fi
}

complete -F _redstr_completions redstr
