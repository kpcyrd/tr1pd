use clap::{App, SubCommand, Arg, AppSettings};

pub fn build_cli() -> App<'static, 'static> {
    App::new("tr1pd")
        .setting(AppSettings::ColoredHelp)
        .arg(Arg::with_name("size")
            .help("Use buffer size instead of lines")
            .short("s")
            .long("size")
            .takes_value(true)
        )
        .subcommand(SubCommand::with_name("bash-completion")
            .about("Generate bash completion script for the tr1pd command.")
        )
}
