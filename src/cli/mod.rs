pub mod tr1pd;
pub mod tr1pctl;

use clap::{App, Shell};

use std::io;

#[inline]
pub fn gen_completions(mut cli: App<'static, 'static>, name: &str) {
    cli.gen_completions_to(name, Shell::Bash, &mut io::stdout())
}
