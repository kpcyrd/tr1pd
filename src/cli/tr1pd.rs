use clap::{App, SubCommand, Arg, AppSettings};

#[inline]
pub fn build_cli() -> App<'static, 'static> {
    App::new("tr1pd")
        .setting(AppSettings::ColoredHelp)
        .subcommand(SubCommand::with_name("bash-completion")
            .about("Generate bash completion script for the tr1pd command.")
        )
        .arg(Arg::with_name("socket")
            .short("S")
            .long("socket")
            .takes_value(true)
        )
}
