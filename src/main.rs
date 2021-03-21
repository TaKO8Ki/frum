mod alias;
mod archive;
mod command;
mod commands;
mod config;
mod input_version;
mod shell;
mod symlink;
mod version;

#[macro_use]
mod log;

use clap::{App, Arg, SubCommand};
use command::Command;
use std::str::FromStr;

fn main() {
    env_logger::init();
    let matches = App::new("farm")
        .version("1.0")
        .author("Takayuki Maeda <takoyaki0316@gmail.com>")
        .about("A blazing fast Ruby version manager written in Rust")
        .subcommand(
            SubCommand::with_name("init")
                .about("Sets environment variables for initializing farm."),
        )
        .subcommand(
            SubCommand::with_name("install")
                .about("Installs `[VERSION]`.")
                .arg(
                    Arg::with_name("list")
                        .short("l")
                        .long("list")
                        .help("Lists the Ruby versions available to install."),
                )
                .arg(Arg::with_name("version").index(1).required_unless("list")),
        )
        .subcommand(
            SubCommand::with_name("install-list")
                .about("Lists the Ruby versions available to install."),
        )
        .subcommand(SubCommand::with_name("versions").about("Lists installed Ruby versions."))
        .get_matches();

    let config = config::FarmConfig::default();
    match matches.subcommand() {
        ("init", _) => commands::init::Init {}.call(&config),
        ("versions", _) => commands::versions::Versions {}.call(&config),
        ("install", Some(matches)) => {
            if matches.is_present("list") {
                commands::install_list::InstallList {}.call(&config);
                return;
            }
            commands::install::Install {
                version: input_version::InputVersion::from_str(
                    matches.value_of("version").unwrap(),
                )
                .expect("invalid version"),
            }
            .call(&config);
        }
        _ => (),
    };
}
