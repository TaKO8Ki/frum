mod install_list;

use clap::{App, SubCommand};

const RUBY_BUILD_DEFAULT_MIRROR: &str = "https://cache.ruby-lang.org/pub/ruby";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("farm")
        .version("1.0")
        .author("Takayuki Maeda <takoyaki0316@gmail.com>")
        .about("A blazing fast Ruby version manager written in Rust")
        .subcommand(
            SubCommand::with_name("install-list")
                .about("Lists the Ruby versions available to install.")
                // or "myapp help"
                .version("0.1")
                .author("Kevin K."),
        )
        .get_matches();

    if matches.subcommand_matches("install-list").is_some() {
        crate::install_list::install_list().unwrap()
    }
    Ok(())
}
