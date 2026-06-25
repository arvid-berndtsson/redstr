use std::process::{Command, Output};

fn run_redstr(args: &[&str]) -> Output {
    let bin = std::env::var("CARGO_BIN_EXE_redstr")
        .expect("CARGO_BIN_EXE_redstr must be set for integration tests");
    Command::new(bin)
        .args(args)
        .output()
        .expect("failed to run redstr")
}

fn combined_output(output: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

#[test]
fn help_flag_prints_usage() {
    let output = run_redstr(&["--help"]);
    assert!(output.status.success());
    let combined = combined_output(&output);
    assert!(combined.contains("Usage:"), "{combined}");
    assert!(!combined.contains("--hELP"), "{combined}");
}

#[test]
fn list_modes_flag_prints_modes() {
    let output = run_redstr(&["--list-modes"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("random"), "{stdout}");
    assert!(stdout.contains("leetspeak"), "{stdout}");
    assert!(stdout.contains("random-user-agent"), "{stdout}");
}

#[test]
fn random_user_agent_mode_works_without_input() {
    let output = run_redstr(&["random-user-agent"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Mozilla/5.0"), "{stdout}");
}

#[test]
fn json_flag_outputs_mode_input_and_output() {
    let output = run_redstr(&["--json", "base64", "hello"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"mode\":\"base64\""), "{stdout}");
    assert!(stdout.contains("\"input\":\"hello\""), "{stdout}");
    assert!(stdout.contains("\"output\":\"aGVsbG8=\""), "{stdout}");
}

#[test]
fn seed_makes_random_mode_deterministic() {
    let one = run_redstr(&["--seed", "42", "random", "hello"]);
    let two = run_redstr(&["--seed", "42", "random", "hello"]);

    assert!(one.status.success());
    assert!(two.status.success());

    let one_out = String::from_utf8_lossy(&one.stdout).trim().to_string();
    let two_out = String::from_utf8_lossy(&two.stdout).trim().to_string();
    assert_eq!(one_out, two_out);
}

#[test]
fn seed_makes_case_swap_mode_deterministic() {
    let one = run_redstr(&["--seed", "99", "case-swap", "SELECT"]);
    let two = run_redstr(&["--seed", "99", "case-swap", "SELECT"]);

    assert!(one.status.success());
    assert!(two.status.success());

    let one_out = String::from_utf8_lossy(&one.stdout).trim().to_string();
    let two_out = String::from_utf8_lossy(&two.stdout).trim().to_string();
    assert_eq!(one_out, two_out);
}
