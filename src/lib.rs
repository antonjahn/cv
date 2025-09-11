use indicatif::{ProgressBar, ProgressStyle};
use log::debug;
use std::env;
use std::fs;
use std::io::{Read, Write};
use std::os::unix::fs::symlink;
use std::path::PathBuf;
use std::process::Command as ProcessCommand;

/// Returns a Vec of version strings for the given host from the zig releases JSON.
pub fn filter_zig_versions(json: &serde_json::Value, host: &str) -> Vec<String> {
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

pub fn detect_host_platform() -> String {
    format!("{}-{}", env::consts::ARCH, env::consts::OS)
}
pub fn zig_install(set_default: bool) -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://ziglang.org/download/index.json";
    debug!("Fetching Zig releases index: {url}");
    let resp = ureq::get(url).call()?.into_string()?;
    let json: serde_json::Value = serde_json::from_str(&resp)?;
    let host = detect_host_platform();
    debug!("Detected host platform: {host}");
    let versions = filter_zig_versions(&json, &host);
    let Some(latest) = versions.last() else {
        return Err("No Zig releases found for this platform".into());
    };
    debug!("Latest Zig version: {latest}");
    let entry = json
        .get(latest)
        .and_then(|v| v.get(&host))
        .ok_or("No entry for latest version and host")?;
    let tarball_url = entry
        .get("tarball")
        .and_then(|v| v.as_str())
        .ok_or("No tarball url")?;
    debug!("Downloading tarball: {tarball_url}");

    // Prepare install paths
    let home = std::env::var("HOME")?;
    let install_dir = PathBuf::from(format!(
        "{}/.local/share/cv/zig/zig-{}-{}",
        home, latest, host
    ));
    let bin_dir = install_dir.join("bin");
    let zig_bin = bin_dir.join("zig");
    let local_bin = PathBuf::from(format!("{}/.local/bin", home));
    let symlink_path = local_bin.join("zig");
    debug!("Install directory: {}", install_dir.display());
    debug!("Binary will be stored at: {}", zig_bin.display());
    debug!("Symlink (if --default): {}", symlink_path.display());

    // Download tarball with progress bar
    let resp = ureq::get(tarball_url).call()?;
    let len = resp
        .header("Content-Length")
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);
    let mut reader = resp.into_reader();
    fs::create_dir_all(&install_dir)?;
    let tarball_path = install_dir
        .parent()
        .unwrap_or(&install_dir)
        .join(format!("zig-{}-{}.tar.xz", latest, host));
    let mut tarball_file = fs::File::create(&tarball_path)?;
    debug!("Downloading to: {}", tarball_path.display());
    let pb = if len > 0 {
        let pb = ProgressBar::new(len);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
            .unwrap()
            .progress_chars("#>-"));
        Some(pb)
    } else {
        None
    };
    let mut buf = [0u8; 32 * 1024];
    let mut downloaded = 0u64;
    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 {
            break;
        }
        tarball_file.write_all(&buf[..n])?;
        downloaded += n as u64;
        if let Some(ref pb) = pb {
            pb.set_position(downloaded);
        }
    }
    if let Some(pb) = pb {
        pb.finish_with_message("Download complete");
    }

    // Extract tarball (requires tar and xz) directly into parent dir, then move contents up if needed
    debug!(
        "Extracting tarball: {} -> {}",
        tarball_path.display(),
        install_dir.display()
    );
    let status = ProcessCommand::new("tar")
        .arg("--strip-components=1")
        .arg("-xJf")
        .arg(&tarball_path)
        .arg("-C")
        .arg(&install_dir)
        .status()?;
    if !status.success() {
        return Err("Failed to extract Zig tarball".into());
    }

    let extracted_bin = install_dir.join("zig");
    debug!("Zig binary: {}", extracted_bin.display());

    // Optionally create symlink
    if set_default {
        debug!(
            "Creating symlink: {} -> {}",
            symlink_path.display(),
            extracted_bin.display()
        );
        fs::create_dir_all(&local_bin)?;
        let _ = fs::remove_file(&symlink_path); // Remove old symlink if exists
        symlink(&extracted_bin, &symlink_path)?;
        println!(
            "Symlinked {} -> {}",
            symlink_path.display(),
            extracted_bin.display()
        );
    }

    println!("Installed Zig {} to {}", latest, install_dir.display());
    Ok(())
}

pub fn zig_list() -> Result<(), Box<dyn std::error::Error>> {
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
