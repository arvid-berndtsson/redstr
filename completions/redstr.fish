complete -c redstr -l help -s h -d "Show help"
complete -c redstr -l list-modes -d "List available modes"
complete -c redstr -l list -d "List available modes"
complete -c redstr -l json -d "Emit JSON output"
complete -c redstr -l seed -r -d "Deterministic seed"

set -l modes \
    random \n leetspeak \n alternate \n inverse \n camel \n snake \n kebab \n case-swap \n unicode \n unicode-normalize \n zalgo \n homoglyph \n space-variants \n reverse \n rot13 \n vowel-swap \n double \n whitespace-padding \n js-concat \n mixed-encoding \n base64 \n url-encode \n hex-encode \n hex-mixed \n html-entity \n sql-comment \n xss-tags \n null-byte \n path-traversal \n command-injection \n mongodb-injection \n couchdb-injection \n dynamodb-obfuscate \n nosql-operator \n ssti-injection \n ssti-syntax \n ssti-framework \n domain-typosquat \n advanced-domain-spoof \n email-obfuscation \n url-shortening \n random-user-agent \n http2-header-order \n tls-fingerprint \n accept-language \n cf-challenge-variation \n cf-turnstile \n cf-challenge-response \n tls-handshake \n canvas-fingerprint \n webgl-obfuscate \n font-fingerprint \n http-header \n api-endpoint \n graphql-obfuscate \n graphql-variable \n graphql-introspection \n session-token \n jwt-header \n jwt-payload \n jwt-alg-confusion \n jwt-signature-bypass \n html-input-attr \n html-form-field \n html-input-type \n html-form-action \n html-input-value \n bash-obfuscate \n powershell-obfuscate \n env-var-obfuscate \n file-path-obfuscate

for mode in $modes
    complete -c redstr -n "__fish_use_subcommand" -a $mode
end
