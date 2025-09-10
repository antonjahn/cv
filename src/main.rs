use clap::Command;
use serde_json;
use std::env;
use ureq;

fn main() {
    let mut cmd = Command::new("cv")
        .bin_name("cv")
        .about("A fast, minimal C/C++ toolchain manager.")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(
            Command::new("zig")
                .about("Manage zig versions and installations")
                .bin_name("cv zig")
                .subcommand(Command::new("list").about("List the available zig installations")),
        )
        .subcommand(Command::new("version").about("Display cv's version"));

    let matches = cmd.clone().get_matches();

    match matches.subcommand() {
        Some(("zig", sub_m)) => match sub_m.subcommand() {
            Some(("list", _)) => {
                if let Err(e) = zig_list() {
                    eprintln!("Error: {e}");
                    std::process::exit(1);
                }
            }
            None => {
                let _ = cmd.find_subcommand_mut("zig").unwrap().print_help();
                println!();
            }
            _ => {
                let _ = cmd.print_help();
            }
        },
        Some(("version", _)) => {
            println!("{}", env!("CARGO_PKG_VERSION"));
        }
        _ => {
            let _ = cmd.print_help();
        }
    }
}

fn zig_list() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://ziglang.org/download/index.json";
    let resp = ureq::get(url).call()?.into_string()?;
    let json: serde_json::Value = serde_json::from_str(&resp)?;

    let host = detect_host_platform();
    let versions = filter_zig_versions(&json, &host);

    if versions.is_empty() {
        println!("No Zig releases found for platform: {host}");
        return Ok(());
    }

    for ver in versions {
        println!("zig-{ver}-{host}    <download available>");
    }
    Ok(())
}

/// Returns a Vec of version strings for the given host from the zig releases JSON.
fn filter_zig_versions(json: &serde_json::Value, host: &str) -> Vec<String> {
    let mut versions = Vec::new();
    if let Some(obj) = json.as_object() {
        for (ver, entry) in obj {
            if ver == "master" {
                continue;
            }
            if !is_version_at_least(ver, 0, 10, 0) {
                continue;
            }
            if let Some(platforms) = entry.as_object() {
                if let Some(platform) = platforms.get(host) {
                    if platform.get("tarball").is_some() {
                        versions.push(ver.clone());
                    }
                }
            }
        }
    }
    versions
}

fn is_version_at_least(ver: &str, min_major: u64, min_minor: u64, min_patch: u64) -> bool {
    // Accepts versions like "0.14.1", "0.15.1", etc. Returns true if >= min version.
    let parts: Vec<&str> = ver.split('.').collect();
    if parts.len() < 2 {
        return false;
    }
    let major = parts[0].parse::<u64>().unwrap_or(0);
    let minor = parts[1].parse::<u64>().unwrap_or(0);
    let patch = if parts.len() > 2 {
        parts[2]
            .split(|c: char| !c.is_ascii_digit())
            .next()
            .unwrap_or("0")
            .parse::<u64>()
            .unwrap_or(0)
    } else {
        0
    };
    (major, minor, patch) >= (min_major, min_minor, min_patch)
}

fn detect_host_platform() -> String {
    format!("{}-{}", env::consts::ARCH, env::consts::OS)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_version_at_least() {
        // Versions >= 0.10.0
        assert!(is_version_at_least("0.10.0", 0, 10, 0));
        assert!(is_version_at_least("0.10.1", 0, 10, 0));
        assert!(is_version_at_least("0.11.0", 0, 10, 0));
        assert!(is_version_at_least("1.0.0", 0, 10, 0));
        assert!(is_version_at_least("0.10.0-alpha", 0, 10, 0));
        // Versions < 0.10.0
        assert!(!is_version_at_least("0.9.9", 0, 10, 0));
        assert!(!is_version_at_least("0.1.0", 0, 10, 0));
        assert!(!is_version_at_least("0.9.9-beta", 0, 10, 0));
        // Patch/minor edge cases
        assert!(is_version_at_least("0.10.0", 0, 9, 9));
        assert!(!is_version_at_least("0.10", 0, 10, 1));
    }

    #[test]
    fn test_zig_json_parsing() {
        let data = r#"{
            "0.15.1": {
                "x86_64-linux": { "tarball": "url" },
                "aarch64-linux": { "tarball": "url" }
            },
            "0.9.0": {
                "x86_64-linux": { "tarball": "url" }
            },
            "master": {
                "x86_64-linux": { "tarball": "url" }
            }
        }"#;
        let json: serde_json::Value = serde_json::from_str(data).unwrap();
        let host = "x86_64-linux";
        let versions = filter_zig_versions(&json, host);
        assert_eq!(versions, vec!["0.15.1"]);
    }
}
