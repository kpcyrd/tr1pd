#![warn(unused_extern_crates)]

extern crate tr1pd;
extern crate env_logger;
extern crate human_size;

use human_size::Size;

use tr1pd::storage::BlockStorage;
use tr1pd::engine::Engine;
use tr1pd::crypto::{SignRing, PublicKey, SecretKey};
use tr1pd::cli;
use tr1pd::cli::tr1pd::build_cli;

use std::env;
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

    let matches = build_cli()
        .get_matches();

    if matches.is_present("bash-completion") {
        cli::gen_completions(build_cli(), "tr1pd");
        return;
    }

    let mut path = env::home_dir().unwrap();
    path.push(".tr1pd/");
    let storage = BlockStorage::new(path);

    // let (pk, sk) = crypto::gen_keypair(); // TODO: load encryption keys
    let (pk, sk) = load_keypair("/etc/tr1pd/lt.pk", "/etc/tr1pd/lt.sk").unwrap();


    let ring = SignRing::new(pk, sk);

    let mut engine = Engine::start(storage, ring).unwrap();

    let mut source = stdin();

    let mut cb = |buf: Vec<u8>| {
        engine.info(buf).unwrap();
        engine.rekey().unwrap();
    };

    match matches.value_of("size") {
        Some(size) => {
            // TODO: this is a very strict parser, eg "512k" is invalid "512 KiB" isn't
            let size = match size.parse::<Size>() {
                Ok(size) => size.into_bytes() as usize,
                Err(_) => size.parse().unwrap(),
            };
            let mut buf = vec![0; size];
            loop {
                let i = source.read(&mut buf).unwrap();
                if i == 0 {
                    break;
                }
                cb(buf[..i].to_vec());
            }
        },
        None => {
            let stdin = BufReader::new(source);
            for line in stdin.lines() {
                // discard invalid lines
                if let Ok(line) = line {
                    cb(line.as_bytes().to_vec());
                }
            }
        },
    };
}
