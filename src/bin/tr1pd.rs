extern crate tr1pd;
extern crate clap;
extern crate env_logger;

// use clap::{App, SubCommand, Arg};
use clap::{App};

use tr1pd::storage::BlockStorage;
use tr1pd::engine::Engine;
use tr1pd::crypto::{SignRing, PublicKey, SecretKey};

use std::env;
// use std::path::Path;
use std::fs::File;
use std::io::stdin;
use std::io::BufReader;
use std::io::prelude::*;

fn load_keypair(pk: &str, sk: &str) -> Option<(PublicKey, SecretKey)> {
    let pk = {
        let mut file = File::open(pk).expect("create lt.pk");
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();
        PublicKey::from_slice(&buf).unwrap()
    };

    let sk = {
        let mut file = File::open(sk).expect("create lt.sk");
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();
        SecretKey::from_slice(&buf).unwrap()
    };

    Some((pk, sk))
}

fn main() {
    env_logger::init().unwrap();

    let _matches = App::new("tr1pd")
        // .subcommand(SubCommand::with_name("foo"))
        .get_matches();

    let mut path = env::home_dir().unwrap();
    path.push(".tr1pd/");
    let storage = BlockStorage::new(path);

    // let (pk, sk) = crypto::gen_keypair(); // TODO: load encryption keys
    let (pk, sk) = load_keypair("/etc/tr1pd/lt.pk", "/etc/tr1pd/lt.sk").unwrap();


    let ring = SignRing::new(pk, sk);

    let mut engine = Engine::start(storage, ring).unwrap();

    let stdin = BufReader::new(stdin());

    for line in stdin.lines() {
        // discard invalid lines
        if let Ok(line) = line {
            // println!("{:?}", line);
            // storage.push(line).unwrap();
            engine.info(line.as_bytes().to_vec()).unwrap();
            engine.rekey().unwrap();
        }
    }
}
