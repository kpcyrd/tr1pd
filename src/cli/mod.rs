pub mod tr1pd;
pub mod tr1pctl;

use structopt::StructOpt;
use structopt::clap::Shell;

use std::io;

pub const TR1PD_SOCKET: &'static str = "ipc:///run/tr1pd/tr1pd.sock";
pub const TR1PD_DATADIR: &'static str = "/var/lib/tr1pd";

#[inline]
pub fn gen_completions<T: StructOpt>(bin_name: &str) {
    T::clap()
        .gen_completions_to(bin_name, Shell::Bash, &mut io::stdout());
}
