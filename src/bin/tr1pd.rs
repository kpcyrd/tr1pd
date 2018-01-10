#![warn(unused_extern_crates)]

extern crate tr1pd;
extern crate env_logger;

use tr1pd::storage::BlockStorage;
use tr1pd::engine::Engine;
use tr1pd::crypto::{SignRing, PublicKey, SecretKey};
use tr1pd::cli;
use tr1pd::cli::tr1pd::build_cli;
use tr1pd::recipe::BlockRecipe;
use tr1pd::rpc::{Server, CtlRequest, CtlResponse};

use std::env;
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

    let matches = build_cli()
        .get_matches();

    if matches.is_present("bash-completion") {
        cli::gen_completions(build_cli(), "tr1pd");
        return;
    }

    let path = match matches.value_of("data-dir") {
        Some(path) => path.into(),
        None => {
            let mut path = env::home_dir().unwrap();
            path.push(".tr1pd/");
            path
        },
    };

    let (pk, sk) = load_keypair("/etc/tr1pd/lt.pk", "/etc/tr1pd/lt.sk").unwrap();

    let ring = SignRing::new(pk, sk);
    let storage = BlockStorage::new(path);
    let mut engine = Engine::start(storage, ring).unwrap();

    let socket = matches.value_of("socket").unwrap_or("ipc://tr1pd.sock");
    let mut server = Server::bind(socket).unwrap();

    loop {
        let msg = server.recv().unwrap();

        let reply = match msg {
            CtlRequest::Ping => CtlResponse::Pong,
            CtlRequest::Write(block) => {
                let pointer = match block {
                    BlockRecipe::Rekey => {
                        engine.rekey().unwrap()
                    },
                    BlockRecipe::Info(info) => {
                        engine.info(info).unwrap();
                        engine.rekey().unwrap()
                    },
                };

                CtlResponse::Ack(pointer.sha3())
            },
        };

        server.reply(reply).unwrap();
    }
}
