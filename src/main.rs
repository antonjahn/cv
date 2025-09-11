use clap::Command;
use clap_cargo::style::CLAP_STYLING;
use cv::*;
use log::error;
use std::env;

fn main() {
    let mut cmd = Command::new("cv")
        .bin_name("cv")
        .styles(CLAP_STYLING)
        .about("A fast, minimal C/C++ toolchain manager.")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            clap::Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Show detailed log output globally")
                .global(true)
                .action(clap::ArgAction::SetTrue),
        )
        .subcommand(
            Command::new("zig")
                .about("Manage zig versions and installations")
                .bin_name("cv zig")
                .styles(CLAP_STYLING)
                .subcommand(Command::new("list").about("List the available zig installations"))
                .subcommand(
                    Command::new("install")
                        .about("Download and install zig versions")
                        .arg(
                            clap::Arg::new("default")
                                .long("default")
                                .help("Use as the default zig version")
                                .action(clap::ArgAction::SetTrue),
                        ),
                ),
        )
        .subcommand(Command::new("version").about("Display cv's version"));

    let matches = cmd.clone().get_matches();

    // Set up logging after parsing, using the global flag
    let verbose = matches.get_flag("verbose");
    let mut builder = env_logger::Builder::new();
    builder.filter_level(if verbose {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    });
    builder.format_timestamp(None);
    let _ = builder.try_init();

    match matches.subcommand() {
        Some(("zig", sub_m)) => match sub_m.subcommand() {
            Some(("list", _)) => {
                if let Err(e) = zig_list() {
                    eprintln!("Error: {e}");
                    std::process::exit(1);
                }
            }
            Some(("install", install_m)) => {
                let set_default = install_m.get_flag("default");
                if let Err(e) = zig_install(set_default) {
                    error!("Error: {e}");
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
