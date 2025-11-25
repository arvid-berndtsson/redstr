#!/bin/bash
# redstr-wrapper.sh - Convenience wrapper for common redstr workflows
# 
# This script provides shortcuts for common security testing workflows
# integrating redstr with other Kali/ParrotOS tools.

set -e

REDSTR_BIN="${REDSTR_BIN:-redstr}"

# Colors for terminal output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_banner() {
    echo -e "${BLUE}"
    echo "╔═══════════════════════════════════════════════════╗"
    echo "║         redstr - Red Team String Tool            ║"
    echo "║    Security Testing Workflow Wrapper             ║"
    echo "╚═══════════════════════════════════════════════════╝"
    echo -e "${NC}"
}

print_usage() {
    cat << EOF
Usage: $(basename "$0") [workflow] [options]

Common Workflows:
  waf-bypass <payload>      Generate WAF bypass variations
  phishing <domain>         Generate phishing domain variations
  xss-test <payload>        Generate XSS payload variations
  sql-test <query>          Generate SQL injection variations
  encode-all <text>         Encode text in multiple formats
  batch-transform <file>    Transform all lines in a file

Options:
  -h, --help               Show this help message
  -o, --output <file>      Write output to file
  -v, --verbose            Verbose output

Examples:
  $(basename "$0") waf-bypass "SELECT * FROM users"
  $(basename "$0") phishing paypal.com
  $(basename "$0") xss-test "<script>alert(1)</script>"
  $(basename "$0") batch-transform payloads.txt -o results.txt

EOF
}

# WAF bypass workflow
workflow_waf_bypass() {
    local payload="$1"
    echo -e "${GREEN}[*] Generating WAF bypass variations...${NC}"
    echo
    
    echo -e "${YELLOW}Case Variations:${NC}"
    echo "Random Case: $($REDSTR_BIN random "$payload")"
    echo "Case Swap: $($REDSTR_BIN case-swap "$payload")"
    echo
    
    echo -e "${YELLOW}Encoding Variations:${NC}"
    echo "URL Encoded: $($REDSTR_BIN url-encode "$payload")"
    echo "Hex Encoded: $($REDSTR_BIN hex-encode "$payload")"
    echo "Base64: $($REDSTR_BIN base64 "$payload")"
    echo
    
    echo -e "${YELLOW}SQL-Specific:${NC}"
    echo "SQL Comments: $($REDSTR_BIN sql-comment "$payload")"
    echo
}

# Phishing workflow
workflow_phishing() {
    local domain="$1"
    echo -e "${GREEN}[*] Generating phishing domain variations...${NC}"
    echo
    
    echo -e "${YELLOW}Homoglyph Variations:${NC}"
    for i in {1..5}; do
        echo "$($REDSTR_BIN homoglyph "$domain")"
    done
    echo
    
    echo -e "${YELLOW}Unicode Variations:${NC}"
    for i in {1..3}; do
        echo "$($REDSTR_BIN unicode "$domain")"
    done
    echo
}

# XSS testing workflow
workflow_xss_test() {
    local payload="$1"
    echo -e "${GREEN}[*] Generating XSS payload variations...${NC}"
    echo
    
    echo -e "${YELLOW}Tag Variations:${NC}"
    echo "$($REDSTR_BIN xss-tags "$payload")"
    echo
    
    echo -e "${YELLOW}Encoding Variations:${NC}"
    echo "URL Encoded: $($REDSTR_BIN url-encode "$payload")"
    echo "HTML Entities: $($REDSTR_BIN mixed-encoding "$payload")"
    echo
    
    echo -e "${YELLOW}Case Obfuscation:${NC}"
    echo "Case Swap: $($REDSTR_BIN case-swap "$payload")"
    echo
}

# SQL injection testing workflow
workflow_sql_test() {
    local query="$1"
    echo -e "${GREEN}[*] Generating SQL injection variations...${NC}"
    echo
    
    echo -e "${YELLOW}Comment Injection:${NC}"
    echo "$($REDSTR_BIN sql-comment "$query")"
    echo
    
    echo -e "${YELLOW}Case Variations:${NC}"
    echo "Random Case: $($REDSTR_BIN random "$query")"
    echo "Case Swap: $($REDSTR_BIN case-swap "$query")"
    echo
    
    echo -e "${YELLOW}Encoding:${NC}"
    echo "URL Encoded: $($REDSTR_BIN url-encode "$query")"
    echo
}

# Encode in all available formats
workflow_encode_all() {
    local text="$1"
    echo -e "${GREEN}[*] Encoding in all formats...${NC}"
    echo
    
    echo "Original: $text"
    echo "Base64: $($REDSTR_BIN base64 "$text")"
    echo "URL Encode: $($REDSTR_BIN url-encode "$text")"
    echo "Hex: $($REDSTR_BIN hex-encode "$text")"
    echo "Hex Mixed: $($REDSTR_BIN hex-mixed "$text")"
    echo "ROT13: $($REDSTR_BIN rot13 "$text")"
    echo
}

# Batch transform file
workflow_batch_transform() {
    local file="$1"
    local output="$2"
    local mode="${3:-random}"
    
    if [[ ! -f "$file" ]]; then
        echo -e "${RED}[!] Error: File not found: $file${NC}"
        exit 1
    fi
    
    echo -e "${GREEN}[*] Processing file: $file${NC}"
    echo -e "${GREEN}[*] Mode: $mode${NC}"
    
    if [[ -n "$output" ]]; then
        echo -e "${GREEN}[*] Output: $output${NC}"
        > "$output"  # Clear output file
    fi
    
    local count=0
    while IFS= read -r line; do
        if [[ -n "$line" ]]; then
            result=$($REDSTR_BIN "$mode" "$line")
            if [[ -n "$output" ]]; then
                echo "$result" >> "$output"
            else
                echo "$result"
            fi
            ((count++))
        fi
    done < "$file"
    
    echo -e "${GREEN}[+] Processed $count lines${NC}"
}

# Main script logic
main() {
    if [[ $# -eq 0 ]]; then
        print_banner
        print_usage
        exit 0
    fi
    
    case "$1" in
        -h|--help)
            print_banner
            print_usage
            exit 0
            ;;
        waf-bypass)
            print_banner
            if [[ -z "$2" ]]; then
                echo -e "${RED}[!] Error: Payload required${NC}"
                exit 1
            fi
            workflow_waf_bypass "$2"
            ;;
        phishing)
            print_banner
            if [[ -z "$2" ]]; then
                echo -e "${RED}[!] Error: Domain required${NC}"
                exit 1
            fi
            workflow_phishing "$2"
            ;;
        xss-test)
            print_banner
            if [[ -z "$2" ]]; then
                echo -e "${RED}[!] Error: Payload required${NC}"
                exit 1
            fi
            workflow_xss_test "$2"
            ;;
        sql-test)
            print_banner
            if [[ -z "$2" ]]; then
                echo -e "${RED}[!] Error: Query required${NC}"
                exit 1
            fi
            workflow_sql_test "$2"
            ;;
        encode-all)
            print_banner
            if [[ -z "$2" ]]; then
                echo -e "${RED}[!] Error: Text required${NC}"
                exit 1
            fi
            workflow_encode_all "$2"
            ;;
        batch-transform)
            print_banner
            if [[ -z "$2" ]]; then
                echo -e "${RED}[!] Error: Input file required${NC}"
                exit 1
            fi
            
            output=""
            mode="random"
            shift 2
            
            while [[ $# -gt 0 ]]; do
                case "$1" in
                    -o|--output)
                        output="$2"
                        shift 2
                        ;;
                    -m|--mode)
                        mode="$2"
                        shift 2
                        ;;
                    *)
                        echo -e "${RED}[!] Unknown option: $1${NC}"
                        exit 1
                        ;;
                esac
            done
            
            workflow_batch_transform "$2" "$output" "$mode"
            ;;
        *)
            echo -e "${RED}[!] Unknown workflow: $1${NC}"
            echo
            print_usage
            exit 1
            ;;
    esac
}

main "$@"
