use clap::{App, AppSettings, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    App::new("frum")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version("1.0")
        .about("A blazing fast Ruby version manager written in Rust")
        .subcommand(
            SubCommand::with_name("init").about("Sets environment variables for initializing frum"),
        )
        .subcommand(
            SubCommand::with_name("install")
                .about("Installs a specific Ruby version")
                .arg(
                    Arg::with_name("list")
                        .short("l")
                        .long("list")
                        .help("Lists Ruby versions available to install"),
                )
                .arg(
                    Arg::with_name("with-openssl-dir")
                        .short("w")
                        .long("with-openssl-dir")
                        .help("Specify a openssl directory"),
                )
                .arg(Arg::with_name("version").index(1)),
        )
        .subcommand(
            SubCommand::with_name("uninstall")
                .about("Uninstall a specific Ruby version")
                .arg(Arg::with_name("version").index(1).required(true)),
        )
        .subcommand(SubCommand::with_name("versions").about("Lists installed Ruby versions"))
        .subcommand(
            SubCommand::with_name("local")
                .about("Sets the current Ruby version")
                .arg(Arg::with_name("version").index(1))
                .arg(
                    Arg::with_name("quiet")
                        .short("q")
                        .long("quiet")
                        .takes_value(false)
                        .help("Supress messages for missing .ruby-version files"),
                ),
        )
        .subcommand(
            SubCommand::with_name("global")
                .about("Sets the global Ruby version")
                .arg(Arg::with_name("version").index(1).required(true)),
        )
        .subcommand(
            SubCommand::with_name("completions")
                .about("Print shell completions to stdout")
                .arg(
                    Arg::with_name("shell")
                        .short("s")
                        .long("shell")
                        .help("The shell syntax to use")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("list")
                        .short("l")
                        .long("list")
                        .help("Lists installed Ruby versions")
                        .hidden(true),
                ),
        )
}
