//! CLI integration tests for help printout using insta

use assert_cmd::Command;
use insta::assert_snapshot;

#[test]
fn cli_help_printout() {
    let mut cmd = Command::cargo_bin("cv").unwrap();
    cmd.arg("--help");
    let output = cmd.assert().success().get_output().stdout.clone();
    let help = String::from_utf8_lossy(&output);
    assert_snapshot!(help);
}

#[test]
fn cli_zig_help_printout() {
    let mut cmd = Command::cargo_bin("cv").unwrap();
    cmd.args(["zig", "--help"]);
    let output = cmd.assert().success().get_output().stdout.clone();
    let help = String::from_utf8_lossy(&output);
    assert_snapshot!(help);
}

#[test]
fn cli_zig_list_prints_versions() {
    let mut cmd = Command::cargo_bin("cv").unwrap();
    cmd.args(["zig", "list"]);
    let output = cmd.assert().success().get_output().stdout.clone();
    let out = String::from_utf8_lossy(&output);
    // Count lines that look like version output
    let version_lines = out.lines().filter(|l| l.contains("zig-")).count();
    assert!(
        version_lines >= 3,
        "Expected at least 3 zig versions, got {}. Output: {}",
        version_lines,
        out
    );
}
