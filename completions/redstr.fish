complete -c redstr -l help -s h -d "Show help"
complete -c redstr -l list-modes -d "List available modes"
complete -c redstr -l list -d "List available modes"
complete -c redstr -l json -d "Emit JSON output"
complete -c redstr -l seed -r -d "Deterministic seed"

set -l modes \
    random leetspeak alternate inverse camel snake kebab case-swap \
    unicode unicode-normalize zalgo homoglyph space-variants reverse rot13 \
    vowel-swap double whitespace-padding js-concat mixed-encoding base64 \
    url-encode hex-encode hex-mixed html-entity sql-comment xss-tags null-byte \
    path-traversal command-injection mongodb-injection couchdb-injection \
    dynamodb-obfuscate nosql-operator ssti-injection ssti-syntax ssti-framework \
    domain-typosquat advanced-domain-spoof email-obfuscation url-shortening \
    random-user-agent http2-header-order tls-fingerprint accept-language \
    cf-challenge-variation cf-turnstile cf-challenge-response tls-handshake \
    canvas-fingerprint webgl-obfuscate font-fingerprint http-header api-endpoint \
    graphql-obfuscate graphql-variable graphql-introspection session-token \
    jwt-header jwt-payload jwt-alg-confusion jwt-signature-bypass \
    html-input-attr html-form-field html-input-type html-form-action html-input-value \
    bash-obfuscate powershell-obfuscate env-var-obfuscate file-path-obfuscate

for mode in $modes
    complete -c redstr -n "__fish_use_subcommand" -a $mode
end
