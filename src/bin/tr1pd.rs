#![warn(unused_extern_crates)]

extern crate tr1pd;
extern crate env_logger;
extern crate error_chain;
#[macro_use] extern crate log;

use tr1pd::Result;
use tr1pd::storage::DiskStorage;
use tr1pd::engine::Engine;
use tr1pd::cli;
use tr1pd::config;
use tr1pd::crypto::{SignRing, PublicKey, SecretKey};
use tr1pd::sandbox::{self, ResultExt};
use tr1pd::rpc::{Server, CtlRequest, CtlResponse};

use std::fs::File;
use std::io::prelude::*;


fn load_keypair(pk: &str, sk: &str) -> Result<(PublicKey, SecretKey)> {
    let pk = {
        let mut file = File::open(pk)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        PublicKey::from_slice(&buf)
            .chain_err(|| "failed to decode public key")?
    };

    let sk = {
        let mut file = File::open(sk)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        SecretKey::from_slice(&buf)
            .chain_err(|| "failed to decode secret key")?
    };

    Ok((pk, sk))
}

fn run() -> Result<()> {
    env_logger::init();

    sandbox::activate_stage1()
        .chain_err(|| "sandbox stage1")?;

    let args = cli::tr1pd::parse();

    match args.subcommand {
        Some(cli::tr1pd::SubCommand::BashCompletion) => {
            cli::gen_completions::<cli::tr1pd::Args>("tr1pd");
            return Ok(());
        }
        _ => (),
    }

    let mut config = config::load_config();

    config.set_socket(args.socket);
    config.set_datadir(args.data_dir);

    let (pk, sk) = load_keypair(&config.pub_key(), &config.sec_key())?;

    let mut server = Server::bind(config.socket())?;

    sandbox::activate_stage2(&mut config)
        .chain_err(|| "sandbox stage2")?;

    let ring = SignRing::new(pk, sk);
    let storage = DiskStorage::new(config.datadir()).to_engine();
    let mut engine = Engine::start(storage, ring)?;

    loop {
        let msg = server.recv()?;

        let reply = match msg {
            CtlRequest::Ping => CtlResponse::Pong,
            CtlRequest::Write(block) => match engine.recipe(block) {
                Ok(pointer) => CtlResponse::Ack(pointer),
                Err(err) => {
                    error!("Write fail: {:?}", err);
                    CtlResponse::Nack
                }
            },
        };

        server.reply(reply)?;
    }
}

fn main() {
    if let Err(ref e) = run() {
        use error_chain::ChainedError; // trait which holds `display_chain`

        eprintln!("{}", e.display_chain());
        ::std::process::exit(1);
    }
}
