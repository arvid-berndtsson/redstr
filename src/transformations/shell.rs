
use crate::rng::SimpleRng;

/// Generates PowerShell command obfuscation for Windows penetration testing.
///
/// Useful for red team operations on Windows targets and blue team detection testing.
///
/// # Examples
///
/// ```
/// use redstr::powershell_obfuscate;
/// let cmd = "Get-Process";
/// let result = powershell_obfuscate(cmd);
/// assert!(result.len() > 0);
/// ```
pub fn powershell_obfuscate(cmd: &str) -> String {
    let mut rng = SimpleRng::new();
    let mut result = String::new();

    for c in cmd.chars() {
        match c {
            '-' => {
                // PowerShell allows various dash alternatives
                match rng.next() % 3 {
                    0 => result.push('-'),
                    1 => result.push_str("'-'"), // Quoted dash
                    _ => result.push(' '),       // Space (sometimes works)
                }
            }
            ' ' => {
                // Vary whitespace
                if rng.next() % 2 == 0 {
                    result.push(' ');
                } else {
                    result.push_str("  ");
                }
            }
            _ => {
                // Case variation
                if c.is_alphabetic() && rng.next() % 3 == 0 {
                    if c.is_uppercase() {
                        result.push_str(&c.to_lowercase().to_string());
                    } else {
                        result.push_str(&c.to_uppercase().to_string());
                    }
                } else {
                    result.push(c);
                }
            }
        }
    }

    result
}

/// Generates bash command obfuscation for Linux penetration testing.
///
/// Useful for red team operations on Linux/Unix targets (Parrot, Kali) and blue team detection.
///
/// # Examples
///
/// ```
/// use redstr::bash_obfuscate;
/// let cmd = "cat /etc/passwd";
/// let result = bash_obfuscate(cmd);
/// assert!(result.contains("cat"));
/// ```
pub fn bash_obfuscate(cmd: &str) -> String {
    let mut rng = SimpleRng::new();
    let mut result = String::new();

    for c in cmd.chars() {
        match c {
            ' ' => {
                // Bash allows various space alternatives
                match rng.next() % 4 {
                    0 => result.push(' '),
                    1 => result.push_str("${IFS}"), // Internal Field Separator
                    2 => result.push('\t'),         // Tab
                    _ => result.push_str("$' '"),   // Quoted space
                }
            }
            '/' => {
                // Path separator
                result.push('/');
            }
            _ => {
                result.push(c);
            }
        }
    }

    result
}

/// Obfuscates environment variable references for shell command evasion.
///
/// Useful for penetration testing on Parrot and Kali Linux systems.
///
/// # Examples
///
/// ```
/// use redstr::env_var_obfuscate;
/// let var = "$HOME";
/// let result = env_var_obfuscate(var);
/// assert!(result.len() > 0);
/// ```
pub fn env_var_obfuscate(input: &str) -> String {
    let mut rng = SimpleRng::new();

    if !input.contains('$') {
        return input.to_string();
    }

    input
        .chars()
        .map(|c| {
            if c == '$' {
                match rng.next() % 3 {
                    0 => "$".to_string(),
                    1 => "${".to_string(), // Start brace expansion
                    2 => "$(".to_string(), // Command substitution start
                    _ => "$".to_string(),
                }
            } else if c.is_alphabetic() && rng.next() % 4 == 0 {
                // Case variation
                if c.is_uppercase() {
                    c.to_lowercase().to_string()
                } else {
                    c.to_uppercase().to_string()
                }
            } else {
                c.to_string()
            }
        })
        .collect()
}

/// Generates file path obfuscation for path traversal and file inclusion testing.
///
/// Useful for penetration testing on Parrot and Kali Linux systems.
///
/// # Examples
///
/// ```
/// use redstr::file_path_obfuscate;
/// let path = "/etc/passwd";
/// let result = file_path_obfuscate(path);
/// assert!(result.len() > 0);
/// ```
pub fn file_path_obfuscate(path: &str) -> String {
    let mut rng = SimpleRng::new();
    let mut result = String::new();

    for c in path.chars() {
        match c {
            '/' => {
                // Path separator variations
                match rng.next() % 4 {
                    0 => result.push('/'),
                    1 => {
                        // Add path traversal
                        if rng.next() % 2 == 0 {
                            result.push_str("../");
                        } else {
                            result.push('/');
                        }
                    }
                    2 => result.push('\\'), // Windows-style (sometimes works on Linux)
                    _ => result.push('/'),
                }
            }
            '.' => {
                // Dot variations
                match rng.next() % 3 {
                    0 => result.push('.'),
                    1 => result.push_str("%2e"), // URL encoded
                    _ => result.push('.'),
                }
            }
            _ => {
                // Case variation for filenames
                if c.is_alphabetic() && rng.next() % 5 == 0 {
                    if c.is_uppercase() {
                        result.push_str(&c.to_lowercase().to_string());
                    } else {
                        result.push_str(&c.to_uppercase().to_string());
                    }
                } else {
                    result.push(c);
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_powershell_obfuscate() {
        let cmd = "Get-Process";
        let result = powershell_obfuscate(cmd);
        assert!(result.len() > 0);
        assert!(result.to_lowercase().contains("get"));
    }

    #[test]
    fn test_bash_obfuscate() {
        let cmd = "cat /etc/passwd";
        let result = bash_obfuscate(cmd);
        assert!(result.contains("cat"));
    }

    #[test]
    fn test_env_var_obfuscate() {
        let var = "$HOME";
        let result = env_var_obfuscate(var);
        assert!(result.to_lowercase().contains("home"));
    }

    #[test]
    fn test_file_path_obfuscate() {
        let path = "/etc/passwd";
        let result = file_path_obfuscate(path);
        assert!(result.len() > 0);
        // Path should be obfuscated but still contain some original elements
        assert!(result.contains("etc") || result.contains("passwd") || result.contains("/"));
    }
}
