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
            .env("TR1PD_SOCKET")
        )
        .arg(Arg::with_name("data-dir")
            .short("D")
            .long("data-dir")
            .takes_value(true)
            .env("TR1PD_DATADIR")
        )
        .arg(Arg::with_name("unprivileged")
            .help("Reserved for internal usage")
            .long("unprivileged")
        )
}
