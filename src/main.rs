mod archive;
mod command;
mod commands;
mod config;

use clap::{App, Arg, SubCommand};
use command::Command;

fn main() {
    env_logger::init();
    let matches = App::new("farm")
        .version("1.0")
        .author("Takayuki Maeda <takoyaki0316@gmail.com>")
        .about("A blazing fast Ruby version manager written in Rust")
        .subcommand(
            SubCommand::with_name("install")
                .about("Installs `[VERSION]`.")
                .arg(Arg::with_name("version").index(1).required(true)),
        )
        .subcommand(
            SubCommand::with_name("install-list")
                .about("Lists the Ruby versions available to install."),
        )
        .get_matches();

    let config = config::FarmConfig::default();
    match matches.subcommand() {
        ("install-list", _) => commands::install_list::InstallList {}.call(config),
        ("install", Some(matches)) => {
            commands::install::Install {
                version: matches
                    .value_of("version")
                    .expect("missing version")
                    .to_string(),
            }
            .call(config);
        }
        _ => (),
    };
}
