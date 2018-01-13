pub mod common;
pub mod tr1pd;
pub mod tr1pctl;

use clap::{App, Shell};

use std::io;

pub const TR1PD_SOCKET: &'static str = "ipc:///run/tr1pd/tr1pd.sock";
pub const TR1PD_DATADIR: &'static str = "/var/lib/tr1pd";

#[inline]
pub fn gen_completions(mut cli: App<'static, 'static>, name: &str) {
    cli.gen_completions_to(name, Shell::Bash, &mut io::stdout())
}
