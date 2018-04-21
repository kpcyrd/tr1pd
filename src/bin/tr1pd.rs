#![warn(unused_extern_crates)]

extern crate tr1pd;
extern crate env_logger;

use tr1pd::storage::DiskStorage;
use tr1pd::engine::Engine;
use tr1pd::cli;
use tr1pd::config;
use tr1pd::crypto::{SignRing, PublicKey, SecretKey};
use tr1pd::sandbox;
use tr1pd::rpc::{Server, CtlRequest, CtlResponse};

use std::fs::File;
use std::io::prelude::*;


fn load_keypair(pk: &str, sk: &str) -> Option<(PublicKey, SecretKey)> {
    let pk = {
        let mut file = File::open(pk).expect("load lt.pk");
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();
        PublicKey::from_slice(&buf).unwrap()
    };

    let sk = {
        let mut file = File::open(sk).expect("load lt.sk");
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();
        SecretKey::from_slice(&buf).unwrap()
    };

    Some((pk, sk))
}

fn main() {
    env_logger::init();

    sandbox::activate_stage1().expect("sandbox stage1");

    let args = cli::tr1pd::parse();

    match args.subcommand {
        Some(cli::tr1pd::SubCommand::BashCompletion) => {
            cli::gen_completions::<cli::tr1pd::Args>("tr1pd");
            return;
        }
        _ => (),
    }

    let mut config = config::load_config();

    config.set_socket(args.socket);
    config.set_datadir(args.data_dir);

    let (pk, sk) = {
        let (pk, sk) = (config.pub_key(), config.sec_key());
        load_keypair(&pk, &sk).expect("load keypair")
    };

    let mut server = Server::bind(config.socket()).unwrap();

    sandbox::activate_stage2(&mut config).expect("sandbox stage2");

    let ring = SignRing::new(pk, sk);
    let storage = DiskStorage::new(config.datadir()).to_engine();
    let mut engine = Engine::start(storage, ring).unwrap();

    loop {
        let msg = server.recv().unwrap();

        let reply = match msg {
            CtlRequest::Ping => CtlResponse::Pong,
            CtlRequest::Write(block) => match engine.recipe(block) {
                Ok(pointer) => CtlResponse::Ack(pointer),
                Err(err) => {
                    eprintln!("Write fail: {:?}", err);
                    CtlResponse::Nack
                }
            },
        };

        server.reply(reply).unwrap();
    }
}
