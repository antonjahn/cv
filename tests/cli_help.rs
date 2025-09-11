//! CLI integration tests for help printout using insta

use assert_cmd::Command;
use insta::assert_snapshot;

#[test]
fn cli() {
    let mut cmd = Command::cargo_bin("cv").unwrap();
    let output = cmd.assert().success().get_output().stdout.clone();
    let help = String::from_utf8_lossy(&output);
    assert_snapshot!(help, @r"
    A fast, minimal C/C++ toolchain manager.

    Usage: cv [OPTIONS] [COMMAND]

    Commands:
      init     Initialize a new C/C++ project in the current directory
      zig      Manage zig versions and installations
      version  Display cv's version
      help     Print this message or the help of the given subcommand(s)

    Options:
      -v, --verbose  Show detailed log output globally
      -h, --help     Print help
      -V, --version  Print version
    ");
}

#[test]
fn cli_help() {
    let mut cmd = Command::cargo_bin("cv").unwrap();
    cmd.arg("--help");
    let output = cmd.assert().success().get_output().stdout.clone();
    let help = String::from_utf8_lossy(&output);
    assert_snapshot!(help, @r"
    A fast, minimal C/C++ toolchain manager.

    Usage: cv [OPTIONS] [COMMAND]

    Commands:
      init     Initialize a new C/C++ project in the current directory
      zig      Manage zig versions and installations
      version  Display cv's version
      help     Print this message or the help of the given subcommand(s)

    Options:
      -v, --verbose  Show detailed log output globally
      -h, --help     Print help
      -V, --version  Print version
    ");
}

#[test]
fn cli_init_help() {
    let mut cmd = Command::cargo_bin("cv").unwrap();
    cmd.args(["init", "--help"]);
    let output = cmd.assert().success().get_output().stdout.clone();
    let help = String::from_utf8_lossy(&output);
    assert_snapshot!(help, @r"
    Initialize a new C/C++ project in the current directory

    Usage: cv init [OPTIONS]

    Options:
      -v, --verbose  Show detailed log output globally
      -h, --help     Print help
    ");
}

#[test]
fn cli_zig_help() {
    let mut cmd = Command::cargo_bin("cv").unwrap();
    cmd.args(["zig", "--help"]);
    let output = cmd.assert().success().get_output().stdout.clone();
    let help = String::from_utf8_lossy(&output);
    assert_snapshot!(help, @r"
    Manage zig versions and installations

    Usage: cv zig [OPTIONS] [COMMAND]

    Commands:
      list     List the available zig installations
      install  Download and install zig versions
      help     Print this message or the help of the given subcommand(s)

    Options:
      -v, --verbose  Show detailed log output globally
      -h, --help     Print help
    ");
}

#[test]
fn cli_zig_list_help() {
    let mut cmd = Command::cargo_bin("cv").unwrap();
    cmd.args(["zig", "list", "--help"]);
    let output = cmd.assert().success().get_output().stdout.clone();
    let help = String::from_utf8_lossy(&output);
    assert_snapshot!(help, @r"
    List the available zig installations

    Usage: cv zig list [OPTIONS]

    Options:
      -v, --verbose  Show detailed log output globally
      -h, --help     Print help
    ");
}

#[test]
fn cli_zig_install_help() {
    let mut cmd = Command::cargo_bin("cv").unwrap();
    cmd.args(["zig", "install", "--help"]);
    let output = cmd.assert().success().get_output().stdout.clone();
    let help = String::from_utf8_lossy(&output);
    assert_snapshot!(help, @r"
    Download and install zig versions

    Usage: cv zig install [OPTIONS]

    Options:
          --default  Use as the default zig version
      -v, --verbose  Show detailed log output globally
      -h, --help     Print help
    ");
}
