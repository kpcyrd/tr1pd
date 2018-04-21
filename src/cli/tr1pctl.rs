use structopt::StructOpt;
use structopt::clap::AppSettings;

use recipe;


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
    #[structopt(subcommand)]
    pub subcommand: SubCommand,
}

impl Args {
    pub fn subcommand_is_from(&self) -> bool {
        match self.subcommand {
            SubCommand::From(_) => true,
            _ => false
        }
    }
}

#[derive(StructOpt, Debug)]
pub enum SubCommand {
    #[structopt(author = "",
                name = "init",
                about = "Generate the long-term keypair")]
    Init(InitCmd),
    #[structopt(author = "",
                name = "get",
                about = "Read block")]
    Get(GetCmd),
    #[structopt(author = "",
                name = "head",
                about = "Show the current head of the chain")]
    Head,
    #[structopt(author = "",
                name = "ls",
                about = "List blocks")]
    Ls(LsCmd),
    #[structopt(author = "",
                name = "write",
                about = "Write to the ledger")]
    Write(WriteCmd),
    #[structopt(author = "",
                name = "from",
                about = "Write command output to ledger")]
    From(FromCmd),
    #[structopt(author = "",
                name = "rekey",
                about = "Explicitly write a rekey block")]
    Rekey,
    #[structopt(author = "",
                name = "fsck",
                about = "Verify ledger")]
    Fsck(FsckCmd),
    #[structopt(author = "",
                name = "ping",
                about = "Ping the daemon process")]
    Ping(PingCmd),
    #[structopt(author = "",
                name = "bash-completion",
                about = "Generate bash completion script for the tr1pd command.")]
    BashCompletion,
}

#[derive(StructOpt, Debug)]
pub struct InitCmd {
    #[structopt(long = "force",
                help = "Overwrite existing keypair")]
    pub force: bool,
}

#[derive(StructOpt, Debug)]
pub struct GetCmd {
    #[structopt(short = "a",
                long = "all",
                help = "Print all fields of the block")]
    pub all: bool,
    #[structopt(short = "p",
                long = "parent",
                help = "Print the pointer to the parent")]
    pub parent: bool,
    pub block: String,
}

#[derive(StructOpt, Debug)]
pub struct LsCmd {
    #[structopt(default_value = "..",
                help = "Specify range to verify")]
    pub spec: String,
}

#[derive(StructOpt, Debug)]
pub struct WriteCmd {
    #[structopt(short = "s",
                long = "size",
                parse(try_from_str = "recipe::parse_size"),
                help = "Use buffer size instead of lines")]
    pub size: Option<usize>,
}

#[derive(StructOpt, Debug)]
pub struct FromCmd {
    #[structopt(short = "s",
                long = "size",
                parse(try_from_str = "recipe::parse_size"),
                help = "Use buffer size instead of lines")]
    pub size: Option<usize>,
    #[structopt(help = "Program to execute")]
    pub prog: String,
    #[structopt(help = "Program arguments")]
    pub args: Vec<String>,
}

#[derive(StructOpt, Debug)]
pub struct FsckCmd {
    #[structopt(default_value = "..",
                help = "Specify range to verify")]
    pub spec: String,
    #[structopt(short = "v",
                help = "Verbose output")]
    pub verbose: bool,
    #[structopt(short = "q",
                help = "Quiet output")]
    pub quiet: bool,
    #[structopt(short = "p",
                long = "paranoid",
                help = "Consider 2nd init block within range fatal")]
    pub paranoid: bool,
}

#[derive(StructOpt, Debug)]
pub struct PingCmd {
    #[structopt(short = "q",
                help = "Quiet ping")]
    pub quiet: bool,
}

pub fn parse() -> Args {
    Args::from_args()
}
