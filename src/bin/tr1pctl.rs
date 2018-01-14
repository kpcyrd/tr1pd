#![warn(unused_extern_crates)]

extern crate tr1pd;
extern crate env_logger;
extern crate nom;
extern crate colored;

use colored::Colorize;

use tr1pd::blocks::InnerBlock;
use tr1pd::cli;
use tr1pd::cli::tr1pctl::build_cli;
use tr1pd::config;
use tr1pd::crypto::{self, PublicKey};
use tr1pd::spec::{Spec, SpecPointer};
use tr1pd::storage::{DiskStorage, BlockStorage};
use tr1pd::recipe::{self, BlockRecipe, InfoBlockPipe};
use tr1pd::rpc::{ClientBuilder, CtlRequest};
use tr1pd::wire;

use nom::IResult;

use std::io;
use std::io::stdin;
use std::io::prelude::*;
use std::path::Path;
use std::str;
use std::process;
use std::fs::File;
use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;

fn load_pubkey(pk: &str) -> Result<PublicKey, ()> {
    let mut file = File::open(pk).expect("create lt.pk");
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    let pk = PublicKey::from_slice(&buf).unwrap();
    Ok(pk)
}

fn main() {
    env_logger::init().unwrap();

    let matches = build_cli()
        .get_matches();

    let config = config::load_config();

    let path = matches.value_of("data-dir").unwrap_or(config.datadir());
    let storage = DiskStorage::new(path);

    let socket = matches.value_of("socket").unwrap_or(config.socket());
    let client = ClientBuilder::new(socket);


    if let Some(matches) = matches.subcommand_matches("init") {
        let force = matches.occurrences_of("force") > 0;

        let (pk, sk) = crypto::gen_keypair();
        let pk_path = Path::new(config.pub_key());
        let sk_path = Path::new(config.sec_key());

        // TODO: create folder with correct permissions

        if force || !pk_path.exists() {
            let mut file = OpenOptions::new()
                            .write(true)
                            .create(true)
                            .create_new(!force)
                            .mode(0o640)
                            .open(pk_path).expect("create lt.pk");
            file.write_all(&pk.0).unwrap();
            println!("[+] wrote public key to {:?}", pk_path);
        }

        if force || !sk_path.exists() {
            let mut file = OpenOptions::new()
                            .write(true)
                            .create(true)
                            .create_new(!force)
                            .mode(0o600)
                            .open(sk_path).expect("create lt.sk");
            file.write_all(&sk.0).unwrap();
            println!("[+] wrote secret key to {:?}", sk_path);
        }
    }

    if let Some(matches) = matches.subcommand_matches("get") {
        let all = matches.occurrences_of("all") > 0;
        let parent = matches.occurrences_of("parent") > 0;

        let longterm_pk = load_pubkey(config.pub_key()).unwrap();

        let spec = matches.value_of("block").unwrap();
        let spec = SpecPointer::parse(spec).expect("failed to parse spec");
        let pointer = storage.resolve_pointer(spec).expect("failed to resolve pointer");
        let block = storage.get(&pointer).expect("failed to load block");

        block.verify_longterm(&longterm_pk).expect("verify_longterm");

        if all {
            println!("{:?}", block);
        } else if parent {
            println!("{:x}", block.prev());
        } else if let Some(bytes) = block.msg() {
            print!("{}", str::from_utf8(bytes).unwrap());
        }
    }

    if let Some(_matches) = matches.subcommand_matches("head") {
        let head = storage.get_head().unwrap();
        // XXX: verify signature before printing this?
        println!("{:x}", head);
    }

    if let Some(matches) = matches.subcommand_matches("ls") {
        let longterm_pk = load_pubkey(config.pub_key()).unwrap();

        let spec = matches.value_of("spec").unwrap_or("..");

        let spec = Spec::parse_range(spec).expect("failed to parse spec");
        let range = storage.resolve_range(spec).expect("failed to expand range");

        for pointer in storage.expand_range(range).unwrap() {
            let block = storage.get(&pointer).unwrap();

            // TODO: verify session as well
            block.verify_longterm(&longterm_pk).expect("verify_longterm");

            if let Some(bytes) = block.msg() {
                print!("{}", str::from_utf8(bytes).unwrap());
            }
        }
    }

    if let Some(matches) = matches.subcommand_matches("write") {
        let mut client = client.connect().unwrap();

        let mut pipe = InfoBlockPipe::new(client, stdin());

        let size = matches.value_of("size")
            .map(|size| recipe::parse_size(size).expect("failed to parse size"));

        match size {
            Some(size) => pipe.start_bytes(size),
            None       => pipe.start_lines(),
        };
    }

    if let Some(_matches) = matches.subcommand_matches("rekey") {
        let mut client = client.connect().unwrap();

        let block = BlockRecipe::Rekey;
        let pointer = client.write_block(block).expect("write block");
        // if not quiet
        println!("{:x}", pointer);
    }

    if let Some(matches) = matches.subcommand_matches("fsck") {
        let longterm_pk = load_pubkey(config.pub_key()).unwrap();

        let spec = matches.value_of("spec").unwrap_or("..");
        let _verbose = matches.occurrences_of("verbose");
        let paranoid = matches.occurrences_of("paranoid") > 0;

        let spec = Spec::parse_range(spec).expect("failed to parse spec");
        let range = storage.resolve_range(spec).expect("failed to expand range");

        let mut session = None;

        // The first block in the spec parameter is trusted
        // If this is an init block this is non-fatal in paranoid mode
        let mut first_block = true;

        for pointer in storage.expand_range(range).unwrap() {
            print!("{:x} ... ", pointer);
            io::stdout().flush().unwrap();

            let buf = storage.get_bytes(&pointer).unwrap();

            // TODO: do a 2-stage decode to avoid reencoding for verification

            if let IResult::Done(_, block) = wire::block(&buf) {
                block.verify_longterm(&longterm_pk).expect("verify_longterm");

                match *block.inner() {
                    InnerBlock::Init(ref init) => {
                        print!("{}  ... ", "init".yellow());
                        io::stdout().flush().unwrap();

                        if paranoid && !first_block {
                            panic!("2nd init block is not allowed in paranoid mode");
                        }

                        session = Some(init.pubkey().clone());
                        // println!("ALERT: init: {:?}", session);
                    },
                    InnerBlock::Rekey(ref rekey) => {
                        print!("rekey ... ");
                        io::stdout().flush().unwrap();

                        rekey.verify_session(&session.unwrap()).expect("verify_session");

                        session = Some(rekey.pubkey().clone());
                        // println!("rekey: {:?}", session);
                    },
                    InnerBlock::Alert(ref alert) => {
                        print!("alert ... ");
                        io::stdout().flush().unwrap();

                        alert.verify_session(&session.unwrap()).expect("verify_session");

                        session = Some(alert.pubkey().clone());
                        // println!("alert: {:?}", session);
                    },
                    InnerBlock::Info(ref info) => {
                        print!("info  ... ");
                        io::stdout().flush().unwrap();

                        info.verify_session(&session.unwrap()).expect("verify_session");
                        // println!("info");
                    },
                };
            } else {
                panic!("corrupted entry");
            }

            println!("{}", "ok".green());
            first_block = false;
        }
    }

    if let Some(matches) = matches.subcommand_matches("ping") {
        let mut client = client.connect().unwrap();

        let quiet = matches.occurrences_of("quiet") > 0;

        let req = CtlRequest::Ping;
        match client.send(&req) {
            Ok(_) if quiet => (),
            Ok(_) => println!("pong"),
            Err(err) => {
                eprintln!("{:?}", err);
                process::exit(1);
            },
        }
    }

    if matches.is_present("bash-completion") {
        cli::gen_completions(build_cli(), "tr1pctl");
    }
}
