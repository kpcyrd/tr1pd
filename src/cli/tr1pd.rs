use structopt::StructOpt;
use structopt::clap::AppSettings;


#[derive(StructOpt, Debug)]
#[structopt(author = "",
            raw(global_settings = "&[AppSettings::ColoredHelp, AppSettings::VersionlessSubcommands]"))]
pub struct Args {
    #[structopt(short = "S",
                long = "socket",
                env = "TR1PD_SOCKET")]
    pub socket: Option<String>,
    #[structopt(short = "D",
                long = "data-dir",
                env = "TR1PD_DATADIR")]
    pub data_dir: Option<String>,
    #[structopt(long = "unprivileged",
                help = "Reserved for internal usage")]
    pub unprivileged: bool,
    #[structopt(subcommand)]
    pub subcommand: Option<SubCommand>,
}

#[derive(StructOpt, Debug)]
pub enum SubCommand {
    #[structopt(author = "",
                name = "bash-completion",
                about = "Generate bash completion script for the tr1pd command.")]
    BashCompletion,
}

pub fn parse() -> Args {
    Args::from_args()
}
