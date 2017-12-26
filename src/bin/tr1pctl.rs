extern crate tr1pd;
extern crate clap;
extern crate env_logger;

use clap::{App, SubCommand, Arg};

use tr1pd::storage::BlockStorage;
use tr1pd::blocks::BlockPointer;
use tr1pd::crypto;

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    env_logger::init().unwrap();

    let matches = App::new("tr1pctl")
        .subcommand(SubCommand::with_name("init")
        )
        .subcommand(SubCommand::with_name("get")
            .arg(Arg::with_name("all")
                .short("a")
                .long("all")
            )
            .arg(Arg::with_name("parent")
                .short("p")
                .long("parent")
            )
            .arg(Arg::with_name("block")
                .required(true)
            )
        )
        .subcommand(SubCommand::with_name("head")
        )
        .subcommand(SubCommand::with_name("ls")
            .arg(Arg::with_name("since")
                .short("s")
                .long("since")
                .takes_value(true)
            )
        )
        .get_matches();

    let mut path = env::home_dir().unwrap();
    path.push(".tr1pd/");
    let storage = BlockStorage::new(path);

    if let Some(_matches) = matches.subcommand_matches("init") {
        let (pk, sk) = crypto::gen_keypair(); // TODO: load encryption keys

        {
            let mut file = File::create("/etc/tr1pd/lt.pk").expect("create lt.pk");
            file.write_all(&pk.0).unwrap();
        };

        {
            let mut file = File::create("/etc/tr1pd/lt.sk").expect("create lt.sk");
            file.write_all(&sk.0).unwrap();
        };
    }

    if let Some(matches) = matches.subcommand_matches("get") {
        let all = matches.occurrences_of("all") > 0;
        let parent = matches.occurrences_of("parent") > 0;

        let pointer = matches.value_of("block").unwrap();
        let pointer = BlockPointer::from_hex(pointer).unwrap();
        let block = storage.get(&pointer).unwrap();

        // TODO: verify block(?)

        if all {
            println!("{:?}", block);
        } else if parent {
            println!("{:x}", block.prev());
        } else if let Some(bytes) = block.msg() {
            println!("{}", String::from_utf8(bytes.to_vec()).unwrap());
        }
    }

    if let Some(_matches) = matches.subcommand_matches("head") {
        let head = storage.get_head().unwrap();
        println!("{:x}", head);
    }

    if let Some(matches) = matches.subcommand_matches("ls") {

        if let Some(since) = matches.value_of("since") {

            let since = BlockPointer::from_hex(since).unwrap();

            let mut backtrace = vec![];

            let mut pointer = storage.get_head().unwrap();
            loop {
                let block = storage.get(&pointer).unwrap();
                pointer = block.prev().clone();

                backtrace.push(pointer.clone());

                if pointer == since {
                    break;
                }

                if pointer == BlockPointer::from_slice(&[
                    0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0,
                    ]).unwrap() {
                    break;
                }
            }

            for pointer in backtrace.iter().rev() {
                let block = storage.get(&pointer).unwrap();
                if let Some(bytes) = block.msg() {
                    println!("{}", String::from_utf8(bytes.to_vec()).unwrap());
                }
            }

            // loop over backtrace




        } else {
            let mut pointer = storage.get_head().unwrap();

            loop {
                let block = storage.get(&pointer).unwrap();
                if let Some(bytes) = block.msg() {
                    println!("{}", String::from_utf8(bytes.to_vec()).unwrap());
                }

                pointer = block.prev().clone();

                if pointer == BlockPointer::from_slice(&[
                    0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0,
                    ]).unwrap() {
                    break;
                }
            }
        }
    }
}
