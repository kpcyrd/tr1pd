extern crate tr1pd;
extern crate clap;
extern crate env_logger;

// use clap::{App, SubCommand, Arg};
use clap::{App};

use tr1pd::storage::BlockStorage;
use tr1pd::engine::Engine;
use tr1pd::crypto::{self, SignRing};

use std::env;
// use std::path::Path;
use std::io::stdin;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    env_logger::init().unwrap();

    let _matches = App::new("tr1pd")
        // .subcommand(SubCommand::with_name("foo"))
        .get_matches();

    let mut path = env::home_dir().unwrap();
    path.push(".tr1pd/");
    let storage = BlockStorage::new(path);

    let (pk, sk) = crypto::gen_keypair(); // TODO: load encryption keys
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
