#![warn(unused_extern_crates)]

extern crate tr1pd;
extern crate env_logger;

use tr1pd::storage::DiskStorage;
use tr1pd::engine::Engine;
use tr1pd::cli;
use tr1pd::cli::tr1pd::build_cli;
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
    env_logger::init().unwrap();

    sandbox::activate_stage1().expect("sandbox stage1");

    let matches = build_cli()
        .get_matches();

    if matches.is_present("bash-completion") {
        cli::gen_completions(build_cli(), "tr1pd");
        return;
    }

    let config = config::load_config();

    let (pk, sk) = (config.pub_key(), config.sec_key());
    let (pk, sk) = load_keypair(pk, sk).unwrap();

    let ring = SignRing::new(pk, sk);
    let path = matches.value_of("data-dir").unwrap_or(config.datadir());
    let storage = DiskStorage::new(path).to_engine();
    let mut engine = Engine::start(storage, ring).unwrap();

    let socket = matches.value_of("socket").unwrap_or(config.socket());
    let mut server = Server::bind(socket).unwrap();

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
