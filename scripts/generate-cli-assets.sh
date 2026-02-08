#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

MODES=(
  random leetspeak alternate inverse camel snake kebab case-swap
  unicode unicode-normalize zalgo homoglyph space-variants reverse rot13
  vowel-swap double whitespace-padding js-concat mixed-encoding base64
  url-encode hex-encode hex-mixed html-entity sql-comment xss-tags null-byte
  path-traversal command-injection mongodb-injection couchdb-injection
  dynamodb-obfuscate nosql-operator ssti-injection ssti-syntax ssti-framework
  domain-typosquat advanced-domain-spoof email-obfuscation url-shortening
  random-user-agent http2-header-order tls-fingerprint accept-language
  cf-challenge-variation cf-turnstile cf-challenge-response tls-handshake
  canvas-fingerprint webgl-obfuscate font-fingerprint http-header api-endpoint
  graphql-obfuscate graphql-variable graphql-introspection session-token
  jwt-header jwt-payload jwt-alg-confusion jwt-signature-bypass
  html-input-attr html-form-field html-input-type html-form-action
  html-input-value bash-obfuscate powershell-obfuscate env-var-obfuscate
  file-path-obfuscate
)

mkdir -p completions man

{
  echo "_redstr_completions()"
  echo "{"
  echo "    local cur prev modes"
  echo "    COMPREPLY=()"
  echo "    cur=\"\${COMP_WORDS[COMP_CWORD]}\""
  echo "    prev=\"\${COMP_WORDS[COMP_CWORD-1]}\""
  echo
  printf "    modes=\""
  printf "%s " "${MODES[@]}"
  echo "\""
  echo
  echo "    if [[ \"\${cur}\" == -* ]]; then"
  echo "        COMPREPLY=( \$(compgen -W \"--help -h --list-modes --list --json --seed\" -- \"\${cur}\") )"
  echo "        return 0"
  echo "    fi"
  echo
  echo "    if [[ \${COMP_CWORD} -eq 1 ]]; then"
  echo "        COMPREPLY=( \$(compgen -W \"\${modes}\" -- \"\${cur}\") )"
  echo "        return 0"
  echo "    fi"
  echo "}"
  echo
  echo "complete -F _redstr_completions redstr"
} > completions/redstr.bash

{
  echo "#compdef redstr"
  echo
  echo "local -a modes"
  echo "modes=("
  for mode in "${MODES[@]}"; do
    echo "  ${mode}"
  done
  echo ")"
  echo
  cat <<'EOF'
_arguments \
  '--help[Show help]' \
  '--list-modes[List available modes]' \
  '--list[List available modes]' \
  '--json[Emit JSON output]' \
  '--seed[Deterministic seed]:seed value:' \
  '1:mode:->mode' \
  '*:text:->text'

case $state in
  mode)
    _describe -t modes 'redstr mode' modes
    ;;
esac
EOF
} > completions/redstr.zsh

{
  cat <<'EOF'
complete -c redstr -l help -s h -d "Show help"
complete -c redstr -l list-modes -d "List available modes"
complete -c redstr -l list -d "List available modes"
complete -c redstr -l json -d "Emit JSON output"
complete -c redstr -l seed -r -d "Deterministic seed"

set -l modes \
EOF
  for i in "${!MODES[@]}"; do
    mode="${MODES[$i]}"
    if [[ "$i" -eq 0 ]]; then
      printf "    %s" "${mode}"
    else
      printf " %s" "${mode}"
    fi
    if [[ "$i" -lt $((${#MODES[@]} - 1)) ]]; then
      printf " \\\n"
    else
      printf "\n"
    fi
  done
  cat <<'EOF'

for mode in $modes
    complete -c redstr -n "__fish_use_subcommand" -a $mode
end
EOF
} > completions/redstr.fish

cat > man/redstr.1 <<'EOF'
.TH REDSTR 1 "" "redstr" "User Commands"
.SH NAME
redstr \- string obfuscation and transformation CLI for security testing
.SH SYNOPSIS
.B redstr
[\fIOPTIONS\fR] [\fIMODE\fR] [\fITEXT ...\fR]
.br
.B redstr
[\fIOPTIONS\fR] \fBrandom-user-agent\fR
.SH DESCRIPTION
\fBredstr\fR applies security-focused string transformations for red team, blue team,
and purple team testing workflows.
.SH OPTIONS
.TP
.B \-h, \-\-help
Show usage information.
.TP
.B \-\-list-modes, \-\-list
Print supported transformation modes and exit.
.TP
.B \-\-json
Print a JSON object with \fBmode\fR, \fBinput\fR, and \fBoutput\fR fields.
.TP
.B \-\-seed \fIu64\fR
Provide a deterministic seed for supported random modes (\fBrandom\fR and \fBcase-swap\fR).
.SH EXAMPLES
.TP
\fBredstr\fR "Hello World"
Random capitalization.
.TP
\fBredstr leetspeak password123\fR
Leetspeak transformation.
.TP
\fBredstr --json xss-tags "<script>alert(1)</script>"\fR
Output transformation result in JSON.
.TP
\fBredstr ssti-framework jinja2 "{{7*7}}"\fR
SSTI framework-specific variation.
.SH SEE ALSO
Project documentation: https://github.com/arvid-berndtsson/redstr
EOF

echo "Generated CLI assets:"
echo "  completions/redstr.bash"
echo "  completions/redstr.zsh"
echo "  completions/redstr.fish"
echo "  man/redstr.1"
