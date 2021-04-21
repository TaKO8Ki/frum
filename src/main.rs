mod alias;
mod archive;
mod cli;
mod command;
mod commands;
mod config;
mod input_version;
mod remote_ruby_index;
mod shell;
mod symlink;
mod version;
mod version_file;

#[macro_use]
mod log;

use command::Command;
use std::str::FromStr;

fn main() {
    env_logger::init();
    let matches = cli::build_cli().get_matches();

    let config = config::FrumConfig::default();
    // println!("{:?}", config);
    match matches.subcommand() {
        ("init", _) => commands::init::Init {}.call(&config),
        ("versions", _) => commands::versions::Versions {}.call(&config),
        ("global", Some(sub_matches)) => commands::global::Global {
            version: input_version::InputVersion::from_str(
                sub_matches.value_of("version").unwrap(),
            )
            .expect("invalid version"),
        }
        .call(&config),
        ("local", Some(sub_matches)) => commands::local::Local {
            version: match sub_matches.value_of("version") {
                Some(version) => {
                    Some(input_version::InputVersion::from_str(version).expect("invalid version"))
                }
                None => None,
            },
        }
        .call(&config),
        ("install", Some(sub_matches)) => {
            if sub_matches.is_present("list") {
                commands::install_list::InstallList {}.call(&config);
                return;
            }
            commands::install::Install {
                version: match sub_matches.value_of("version") {
                    Some(version) => Some(
                        input_version::InputVersion::from_str(version).expect("invalid version"),
                    ),
                    None => None,
                },
            }
            .call(&config);
        }
        ("uninstall", Some(sub_matches)) => {
            commands::uninstall::Uninstall {
                version: input_version::InputVersion::from_str(
                    sub_matches.value_of("version").unwrap(),
                )
                .expect("invalid version"),
            }
            .call(&config);
        }
        ("completions", Some(sub_matches)) => {
            commands::completions::Completions {
                shell: match sub_matches.value_of("shell") {
                    Some(shell) => Some(clap::Shell::from_str(shell).expect("invalid shell")),
                    None => None,
                },
                list: sub_matches.is_present("list"),
            }
            .call(&config);
        }
        _ => (),
    };
}
