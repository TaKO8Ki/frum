mod archive;
mod commands;

use clap::{App, Arg, SubCommand};

const RUBY_BUILD_DEFAULT_MIRROR: &str = "https://cache.ruby-lang.org/pub/ruby";

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    if matches.subcommand_matches("install-list").is_some() {
        commands::install_list::install_list()?
    }
    if let Some(matches) = matches.subcommand_matches("install") {
        match commands::install::install(matches.value_of("version").unwrap().to_string()) {
            Ok(_) => (),
            Err(err) => {
                eprintln!("farm: {}", err);
                std::process::exit(1)
            }
        };
    }
    Ok(())
}
