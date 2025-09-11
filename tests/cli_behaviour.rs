use assert_cmd::Command;

#[test]
pub(crate) fn cli_zig_list_prints_versions() {
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

#[test]
pub(crate) fn cli_zig_install() {
    let mut cmd = Command::cargo_bin("cv").unwrap();
    cmd.args(["zig", "install"]);
    let output = cmd.assert().success().get_output().stdout.clone();
    let out = String::from_utf8_lossy(&output);
    assert!(
        out.contains("Installed Zig 0.15.1"),
        "Expected installation message, got: {}",
        out
    );
}
