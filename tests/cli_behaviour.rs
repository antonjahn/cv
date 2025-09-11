use assert_cmd::Command;

#[test]
#[ignore] // Ignored because it depends on network access and external data
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
#[ignore] // Ignored because it depends on network access and external data
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

#[test]
#[ignore] // Ignored because it depends on file access
pub(crate) fn cli_init() {
    // Create a temp directory and go there
    let tmp_dir = tempfile::tempdir().unwrap();
    std::env::set_current_dir(&tmp_dir).unwrap();
    // Run the init command
    let mut cmd = Command::cargo_bin("cv").unwrap();
    cmd.args(["init"]);
    cmd.assert().success();
    // Check that cvproject.toml and src/main.c files exist
    assert!(std::path::Path::new("cvproject.toml").exists());
    assert!(std::path::Path::new("src/main.c").exists());
}
