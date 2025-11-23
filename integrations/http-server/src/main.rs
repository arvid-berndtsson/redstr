use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

/// Simple HTTP server for redstr transformations
/// Provides REST API for external tools to use redstr
fn main() {
    let address = "127.0.0.1:8080";
    let listener = TcpListener::bind(address)
        .expect("Failed to bind to address");
    
    println!("redstr HTTP server listening on http://{}", address);
    println!("Ready to accept transformation requests");
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    // 8KB buffer for request handling - supports most use cases
    // For larger requests, consider using a production HTTP server library
    let mut buffer = [0; 8192];
    
    match stream.read(&mut buffer) {
        Ok(size) => {
            let request = String::from_utf8_lossy(&buffer[..size]);
            
            // Parse HTTP request
            if request.starts_with("POST /transform") {
                handle_transform(&mut stream, &request);
            } else if request.starts_with("GET /health") {
                send_response(&mut stream, 200, "OK", "application/json", r#"{"status":"healthy"}"#);
            } else if request.starts_with("GET /") {
                send_response(&mut stream, 200, "OK", "application/json", r#"{"service":"redstr","version":"0.2.0","endpoints":["/transform","/health"]}"#);
            } else {
                send_response(&mut stream, 404, "Not Found", "text/plain", "Endpoint not found");
            }
        }
        Err(e) => {
            eprintln!("Failed to read from stream: {}", e);
        }
    }
}

fn handle_transform(stream: &mut TcpStream, request: &str) {
    // Extract JSON body from POST request
    let body_start = request.find("\r\n\r\n").map(|i| i + 4);
    
    if let Some(start) = body_start {
        let body = &request[start..];
        
        // Simple JSON parsing - suitable for basic use cases
        // Limitations: Does not handle nested objects, arrays, or complex escaping
        // For production use with complex JSON, consider adding serde_json dependency
        // Expected format: {"function":"method_name","input":"text"}
        let result = parse_and_transform(body);
        
        match result {
            Ok(output) => {
                let response_body = format!(r#"{{"output":"{}"}}"#, escape_json(&output));
                send_response(stream, 200, "OK", "application/json", &response_body);
            }
            Err(e) => {
                let error_body = format!(r#"{{"error":"{}"}}"#, escape_json(&e));
                send_response(stream, 400, "Bad Request", "application/json", &error_body);
            }
        }
    } else {
        send_response(stream, 400, "Bad Request", "text/plain", "No body found");
    }
}

fn parse_and_transform(json: &str) -> Result<String, String> {
    // Simple JSON parsing - extract function and input
    let function = extract_json_field(json, "function")
        .ok_or("Missing 'function' field")?;
    let input = extract_json_field(json, "input")
        .ok_or("Missing 'input' field")?;
    
    // Call appropriate redstr function
    let output = match function.as_str() {
        // Case transformations
        "randomize_capitalization" => redstr::randomize_capitalization(&input),
        "leetspeak" => redstr::leetspeak(&input),
        "alternate_case" => redstr::alternate_case(&input),
        "case_swap" => redstr::case_swap(&input),
        "to_camel_case" => redstr::to_camel_case(&input),
        "to_snake_case" => redstr::to_snake_case(&input),
        "to_kebab_case" => redstr::to_kebab_case(&input),
        
        // Encoding
        "base64_encode" => redstr::base64_encode(&input),
        "url_encode" => redstr::url_encode(&input),
        "hex_encode" => redstr::hex_encode(&input),
        "html_entity_encode" => redstr::html_entity_encode(&input),
        "mixed_encoding" => redstr::mixed_encoding(&input),
        
        // Injection
        "sql_comment_injection" => redstr::sql_comment_injection(&input),
        "xss_tag_variations" => redstr::xss_tag_variations(&input),
        "command_injection" => redstr::command_injection(&input),
        "path_traversal" => redstr::path_traversal(&input),
        "null_byte_injection" => redstr::null_byte_injection(&input),
        "mongodb_injection" => redstr::mongodb_injection(&input),
        "couchdb_injection" => redstr::couchdb_injection(&input),
        "dynamodb_obfuscate" => redstr::dynamodb_obfuscate(&input),
        "nosql_operator_injection" => redstr::nosql_operator_injection(&input),
        "ssti_injection" => redstr::ssti_injection(&input),
        "ssti_syntax_obfuscate" => redstr::ssti_syntax_obfuscate(&input),
        "ssti_framework_variation" => {
            // For this function, we expect input format: "template|framework"
            let parts: Vec<&str> = input.split('|').collect();
            if parts.len() == 2 {
                redstr::ssti_framework_variation(parts[0], parts[1])
            } else {
                // Default to jinja2 if no framework specified
                redstr::ssti_framework_variation(&input, "jinja2")
            }
        },
        
        // Phishing
        "domain_typosquat" => redstr::domain_typosquat(&input),
        "advanced_domain_spoof" => redstr::advanced_domain_spoof(&input),
        "email_obfuscation" => redstr::email_obfuscation(&input),
        "url_shortening_pattern" => redstr::url_shortening_pattern(&input),
        
        // Obfuscation
        "rot13" => redstr::rot13(&input),
        "reverse_string" => redstr::reverse_string(&input),
        "vowel_swap" => redstr::vowel_swap(&input),
        "double_characters" => redstr::double_characters(&input),
        "whitespace_padding" => redstr::whitespace_padding(&input),
        "js_string_concat" => redstr::js_string_concat(&input),
        
        // Unicode
        "homoglyph_substitution" => redstr::homoglyph_substitution(&input),
        "unicode_variations" => redstr::unicode_variations(&input),
        "zalgo_text" => redstr::zalgo_text(&input),
        "space_variants" => redstr::space_variants(&input),
        "unicode_normalize_variants" => redstr::unicode_normalize_variants(&input),
        
        // Cloudflare
        "cloudflare_turnstile_variation" => redstr::cloudflare_turnstile_variation(&input),
        "cloudflare_challenge_response" => redstr::cloudflare_challenge_response(&input),
        "tls_fingerprint_variation" => redstr::tls_fingerprint_variation(&input),
        "tls_handshake_pattern" => redstr::tls_handshake_pattern(&input),
        "canvas_fingerprint_variation" => redstr::canvas_fingerprint_variation(&input),
        "webgl_fingerprint_obfuscate" => redstr::webgl_fingerprint_obfuscate(&input),
        "font_fingerprint_consistency" => redstr::font_fingerprint_consistency(&input),
        
        // Web Security
        "http_header_variation" => redstr::http_header_variation(&input),
        "api_endpoint_variation" => redstr::api_endpoint_variation(&input),
        "graphql_obfuscate" => redstr::graphql_obfuscate(&input),
        "graphql_variable_injection" => redstr::graphql_variable_injection(&input),
        "graphql_introspection_bypass" => redstr::graphql_introspection_bypass(&input),
        "session_token_variation" => redstr::session_token_variation(&input),
        "jwt_header_manipulation" => redstr::jwt_header_manipulation(&input),
        "jwt_payload_obfuscate" => redstr::jwt_payload_obfuscate(&input),
        "jwt_algorithm_confusion" => redstr::jwt_algorithm_confusion(&input),
        "jwt_signature_bypass" => redstr::jwt_signature_bypass(&input),
        
        // Shell
        "bash_obfuscate" => redstr::bash_obfuscate(&input),
        "powershell_obfuscate" => redstr::powershell_obfuscate(&input),
        "env_var_obfuscate" => redstr::env_var_obfuscate(&input),
        "file_path_obfuscate" => redstr::file_path_obfuscate(&input),
        
        // Bot detection
        "random_user_agent" => redstr::random_user_agent(), // No input needed - generates random UA
        "http2_header_order" => redstr::http2_header_order(&input),
        "cloudflare_challenge_variation" => redstr::cloudflare_challenge_variation(&input),
        "accept_language_variation" => redstr::accept_language_variation(&input),
        
        _ => return Err(format!("Unknown function: {}", function)),
    };
    
    Ok(output)
}

fn extract_json_field(json: &str, field: &str) -> Option<String> {
    // Simple JSON field extraction - handles basic escaped quotes and backslashes
    // Limitations: Does not handle nested objects or complex JSON structures
    let pattern = format!("\"{}\":\"", field);
    let start = json.find(&pattern)? + pattern.len();
    let remaining = &json[start..];
    
    // Find the closing quote, accounting for escaped quotes
    let mut end = 0;
    let mut escaped = false;
    for (i, c) in remaining.chars().enumerate() {
        if escaped {
            escaped = false;
            continue;
        }
        if c == '\\' {
            escaped = true;
            continue;
        }
        if c == '"' {
            end = i;
            break;
        }
    }
    
    if end == 0 && !remaining.is_empty() {
        return None;
    }
    
    Some(remaining[..end].replace("\\\"", "\"").replace("\\\\", "\\")
        .replace("\\n", "\n").replace("\\r", "\r").replace("\\t", "\t"))
}

fn escape_json(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

fn send_response(stream: &mut TcpStream, status: u16, status_text: &str, content_type: &str, body: &str) {
    let response = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\n\r\n{}",
        status, status_text, content_type, body.len(), body
    );
    
    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Failed to send response: {}", e);
    }
}
