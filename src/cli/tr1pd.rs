use clap::{App, SubCommand, Arg, AppSettings};

use cli::common;

#[inline]
pub fn build_cli() -> App<'static, 'static> {
    App::new("tr1pd")
        .setting(AppSettings::ColoredHelp)
        .subcommand(SubCommand::with_name("bash-completion")
            .about("Generate bash completion script for the tr1pd command.")
        )
        .arg(common::socket())
        .arg(common::data_dir())
        .arg(Arg::with_name("unprivileged")
            .help("Reserved for internal usage")
            .long("unprivileged")
        )
}
