_redstr_completions()
{
    local cur prev modes
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"

    modes="random leetspeak alternate inverse camel snake kebab case-swap unicode unicode-normalize zalgo homoglyph space-variants reverse rot13 vowel-swap double whitespace-padding js-concat mixed-encoding base64 url-encode hex-encode hex-mixed html-entity sql-comment xss-tags null-byte path-traversal command-injection mongodb-injection couchdb-injection dynamodb-obfuscate nosql-operator ssti-injection ssti-syntax ssti-framework domain-typosquat advanced-domain-spoof email-obfuscation url-shortening random-user-agent http2-header-order tls-fingerprint accept-language cf-challenge-variation cf-turnstile cf-challenge-response tls-handshake canvas-fingerprint webgl-obfuscate font-fingerprint http-header api-endpoint graphql-obfuscate graphql-variable graphql-introspection session-token jwt-header jwt-payload jwt-alg-confusion jwt-signature-bypass html-input-attr html-form-field html-input-type html-form-action html-input-value bash-obfuscate powershell-obfuscate env-var-obfuscate file-path-obfuscate"

    if [[ "${cur}" == -* ]]; then
        COMPREPLY=( $(compgen -W "--help -h --list-modes --list --json --seed" -- "${cur}") )
        return 0
    fi

    if [[ ${COMP_CWORD} -eq 1 ]]; then
        COMPREPLY=( $(compgen -W "${modes}" -- "${cur}") )
        return 0
    fi
}

complete -F _redstr_completions redstr
