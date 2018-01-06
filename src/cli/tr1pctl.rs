use clap::{App, SubCommand, Arg, AppSettings};

#[inline]
pub fn build_cli() -> App<'static, 'static> {
    App::new("tr1pctl")
        .settings(&[AppSettings::SubcommandRequiredElseHelp, AppSettings::ColoredHelp])
        .setting(AppSettings::VersionlessSubcommands)
        .arg(Arg::with_name("socket")
            .short("S")
            .long("socket")
            .takes_value(true)
        )
        .subcommand(SubCommand::with_name("init")
            .setting(AppSettings::ColoredHelp)
            .about("Generate the long-term keypair")
            .arg(Arg::with_name("force")
                .help("Overwrite existing keypair")
                .long("force")
            )
        )
        .subcommand(SubCommand::with_name("get")
            .setting(AppSettings::ColoredHelp)
            .about("Read block")
            .arg(Arg::with_name("all")
                .short("a")
                .long("all")
            )
            .arg(Arg::with_name("parent")
                .short("p")
                .long("parent")
            )
            .arg(Arg::with_name("block")
                .required(true)
            )
        )
        .subcommand(SubCommand::with_name("head")
            .setting(AppSettings::ColoredHelp)
            .about("Show the current head of the chain")
        )
        .subcommand(SubCommand::with_name("ls")
            .setting(AppSettings::ColoredHelp)
            .about("List blocks")
            .arg(Arg::with_name("since")
                .short("s")
                .long("since")
                .takes_value(true)
            )
        )
        .subcommand(SubCommand::with_name("write")
            .setting(AppSettings::ColoredHelp)
            .about("Write to the blockchain")
            .arg(Arg::with_name("size")
                .help("Use buffer size instead of lines")
                .short("s")
                .long("size")
                .takes_value(true)
            )
        )
        .subcommand(SubCommand::with_name("fsck")
            .setting(AppSettings::ColoredHelp)
            .about("Verify blockchain")
            .arg(Arg::with_name("since")
                .help("Start verifying from this trusted block")
                .short("s")
                .long("since")
                .takes_value(true)
            )
            .arg(Arg::with_name("to")
                .help("Verify to this untrusted block")
                .short("t")
                .long("to")
                .takes_value(true)
            )
            .arg(Arg::with_name("verbose")
                .help("Verbose output")
                .short("v")
            )
            .arg(Arg::with_name("quiet")
                .help("Quiet output")
                .short("q")
            )
            .arg(Arg::with_name("paranoid")
                .help("Consider 2nd init block within range fatal")
                .short("p")
                .long("paranoid")
            )
        )
        .subcommand(SubCommand::with_name("ping")
            .setting(AppSettings::ColoredHelp)
            .about("Ping the daemon process")
            .arg(Arg::with_name("quiet")
                .help("Quiet ping")
                .short("q")
            )
        )
        .subcommand(SubCommand::with_name("bash-completion")
            .about("Generate bash completion script for the tr1pctl command.")
        )
}

