extern crate tr1pd;
extern crate clap;
extern crate env_logger;
extern crate nom;

use clap::{App, SubCommand, Arg, AppSettings};

use tr1pd::storage::BlockStorage;
use tr1pd::blocks::BlockPointer;
use tr1pd::crypto;
use tr1pd::crypto::PublicKey;

use std::env;
use std::str;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn load_pubkey(pk: &str) -> Result<PublicKey, ()> {
    let mut file = File::open(pk).expect("create lt.pk");
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    let pk = PublicKey::from_slice(&buf).unwrap();
    Ok(pk)
}

fn main() {
    env_logger::init().unwrap();

    let matches = App::new("tr1pctl")
        .settings(&[AppSettings::SubcommandRequiredElseHelp, AppSettings::ColoredHelp])
        .subcommand(SubCommand::with_name("init")
            .setting(AppSettings::ColoredHelp)
            .about("Generate the long-term keypair")
            .arg(Arg::with_name("force")
                .help("Overwrite existing keypair")
                .long("force")
            )
        )
        .subcommand(SubCommand::with_name("get")
            .setting(AppSettings::ColoredHelp)
            .about("Read block")
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
            .setting(AppSettings::ColoredHelp)
            .about("Show the current head of the chain")
        )
        .subcommand(SubCommand::with_name("ls")
            .setting(AppSettings::ColoredHelp)
            .about("List blocks")
            .arg(Arg::with_name("since")
                .short("s")
                .long("since")
                .takes_value(true)
            )
        )
        .subcommand(SubCommand::with_name("fsck")
            .setting(AppSettings::ColoredHelp)
            .about("Verify a chain")
            .arg(Arg::with_name("since")
                .help("Start verifying from this trusted block")
                .short("s")
                .long("since")
                .takes_value(true)
            )
        )
        .get_matches();

    let mut path = env::home_dir().unwrap();
    path.push(".tr1pd/");
    let storage = BlockStorage::new(path);

    if let Some(matches) = matches.subcommand_matches("init") {
        let force = matches.occurrences_of("force") > 0;

        let (pk, sk) = crypto::gen_keypair();

        {
            let mut file = OpenOptions::new()
                            .write(true)
                            .create(true)
                            .create_new(!force)
                            .open("/etc/tr1pd/lt.pk").expect("create lt.pk");
            file.write_all(&pk.0).unwrap();
        };

        {
            let mut file = OpenOptions::new()
                            .write(true)
                            .create(true)
                            .create_new(!force)
                            .open("/etc/tr1pd/lt.sk").expect("create lt.sk");
            file.write_all(&sk.0).unwrap();
        };
    }

    if let Some(matches) = matches.subcommand_matches("get") {
        let all = matches.occurrences_of("all") > 0;
        let parent = matches.occurrences_of("parent") > 0;

        let longterm_pk = load_pubkey("/etc/tr1pd/lt.pk").unwrap();

        let pointer = matches.value_of("block").unwrap();
        let pointer = BlockPointer::from_hex(pointer).unwrap();
        let block = storage.get(&pointer).unwrap();

        block.verify_longterm(&longterm_pk).expect("verify_longterm");

        if all {
            println!("{:?}", block);
        } else if parent {
            println!("{:x}", block.prev());
        } else if let Some(bytes) = block.msg() {
            println!("{}", str::from_utf8(bytes).unwrap());
        }
    }

    if let Some(_matches) = matches.subcommand_matches("head") {
        let head = storage.get_head().unwrap();
        // XXX: verify signature before printing this?
        println!("{:x}", head);
    }

    if let Some(matches) = matches.subcommand_matches("ls") {
        let longterm_pk = load_pubkey("/etc/tr1pd/lt.pk").unwrap();

        let backtrace = tr1pd::backtrace(&storage, matches.value_of("since")).unwrap();

        for pointer in backtrace.iter().rev() {
            let block = storage.get(&pointer).unwrap();

            // TODO: verify session as well
            block.verify_longterm(&longterm_pk).expect("verify_longterm");

            if let Some(bytes) = block.msg() {
                println!("{}", str::from_utf8(bytes).unwrap());
            }
        }
    }

    if let Some(matches) = matches.subcommand_matches("fsck") {
        let longterm_pk = load_pubkey("/etc/tr1pd/lt.pk").unwrap();

        let backtrace = tr1pd::backtrace(&storage, matches.value_of("since")).unwrap();

        let mut session = None;

        for pointer in backtrace.iter().rev() {
            let buf = storage.get_raw(&pointer).unwrap();

            // TODO: do a 2-stage decode to avoid reencoding for verification

            use tr1pd::wire;
            use nom::IResult;
            if let IResult::Done(_, block) = wire::block(&buf) {
                let block = block.0;

                block.verify_longterm(&longterm_pk).expect("verify_longterm");

                use tr1pd::blocks::BlockType;
                match *block.inner() {
                    BlockType::Init(ref init) => {
                        // TODO: verify new pubkey

                        session = Some(init.pubkey().clone());
                        println!("ALERT: init: {:?}", session);
                    },
                    BlockType::Rekey(ref rekey) => {
                        rekey.verify_session(&session.unwrap()).expect("verify_session");

                        session = Some(rekey.pubkey().clone());
                        println!("rekey: {:?}", session);
                    },
                    BlockType::Alert(ref alert) => {
                        alert.verify_session(&session.unwrap()).expect("verify_session");

                        session = Some(alert.pubkey().clone());
                        println!("alert: {:?}", session);
                    },
                    BlockType::Info(ref info) => {
                        info.verify_session(&session.unwrap()).expect("verify_session");
                        println!("info");
                    },
                };
            } else {
                panic!("corrupted entry");
            }
        }
    }
}
