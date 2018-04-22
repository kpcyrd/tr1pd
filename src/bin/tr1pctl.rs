#![warn(unused_extern_crates)]

extern crate tr1pd;
extern crate env_logger;
extern crate nom;
extern crate colored;
extern crate error_chain;
#[macro_use] extern crate log;

use colored::Colorize;

use tr1pd::{Result, ResultExt};
use tr1pd::blocks::InnerBlock;
use tr1pd::cli;
use tr1pd::config;
use tr1pd::crypto::{self, PublicKey};
use tr1pd::sandbox;
use tr1pd::storage::{DiskStorage, BlockStorage};
use tr1pd::recipe::{BlockRecipe, InfoBlockPipe};
use tr1pd::rpc::{ClientBuilder, CtlRequest};
use tr1pd::wire;

use nom::IResult;

use std::io;
use std::io::stdin;
use std::io::prelude::*;
use std::path::Path;
use std::str;
use std::fs::File;
use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::process::{Command, Stdio};


fn load_pubkey(pk: &str) -> Result<PublicKey> {
    let mut file = File::open(pk)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    let pk = PublicKey::from_slice(&buf)
                .chain_err(|| "invalid public key")?;
    Ok(pk)
}

fn run() -> Result<()> {
    env_logger::init();

    let args = cli::tr1pctl::parse();

    if !args.subcommand_is_from() {
        // `tr1pctl from` can't setup seccomp properly
        // since the new process would inherit our filter.
        // if that problem has been resolved, activate_stage1
        // should be moved below env_logger::init
        sandbox::activate_stage1()
            .chain_err(|| "sandbox stage1")?;
    }

    let config = config::load_config();

    let path = args.data_dir.unwrap_or(config.datadir().to_string());
    let storage = DiskStorage::new(path);

    let socket = args.socket.unwrap_or(config.socket().to_string());
    let client = ClientBuilder::new(socket);

    use cli::tr1pctl::SubCommand;
    match args.subcommand {
        SubCommand::Init(matches) => {
            let (pk, sk) = crypto::gen_keypair();
            let pk_path = Path::new(config.pub_key());
            let sk_path = Path::new(config.sec_key());

            // TODO: create folder with correct permissions

            if matches.force || !pk_path.exists() {
                let mut file = OpenOptions::new()
                                .write(true)
                                .create(true)
                                .create_new(!matches.force)
                                .mode(0o640)
                                .open(pk_path)?;
                file.write_all(&pk.0)?;
                println!("[+] wrote public key to {:?}", pk_path);
            }

            if matches.force || !sk_path.exists() {
                let mut file = OpenOptions::new()
                                .write(true)
                                .create(true)
                                .create_new(!matches.force)
                                .mode(0o600)
                                .open(sk_path)?;
                file.write_all(&sk.0)?;
                println!("[+] wrote secret key to {:?}", sk_path);
            }
        },
        SubCommand::Get(matches) => {
            let longterm_pk = load_pubkey(config.pub_key())?;

            let pointer = storage.resolve_pointer(matches.block).expect("failed to resolve pointer");
            let block = storage.get(&pointer).expect("failed to load block");

            block.verify_longterm(&longterm_pk).expect("verify_longterm");

            if matches.all {
                println!("{:?}", block);
            } else if matches.parent {
                println!("{:x}", block.prev());
            } else if let Some(bytes) = block.msg() {
                let mut stdout = io::stdout();
                stdout.write(&bytes)?;
            }
        },

        SubCommand::Head => {
            let head = storage.get_head()?;
            // XXX: verify signature before printing this?
            println!("{:x}", head);
        },

        SubCommand::Ls(matches) => {
            let longterm_pk = load_pubkey(config.pub_key())?;

            let range = storage.resolve_range(matches.spec).expect("failed to expand range");

            let mut stdout = io::stdout();
            for pointer in storage.expand_range(range)? {
                let block = storage.get(&pointer)?;

                // TODO: verify session as well
                block.verify_longterm(&longterm_pk).expect("verify_longterm");

                if let Some(bytes) = block.msg() {
                    stdout.write(&bytes)?;
                }
            }
        },

        SubCommand::Write(matches) => {
            let client = client.connect()?;

            let mut pipe = InfoBlockPipe::new(client, stdin());

            match matches.size {
                Some(size) => pipe.start_bytes(size),
                None       => pipe.start_lines(),
            };
        },

        SubCommand::From(matches) => {
            let client = client.connect()?;

            let size = matches.size;

            let prog = matches.prog;
            let args = matches.args;

            info!("executing: {:?} {:?}", prog, args);

            let mut child = Command::new(prog)
                .args(args)
                .stdout(Stdio::piped())
                .spawn().expect("failed to start");

            let stdout = child.stdout.take().unwrap();
            let mut pipe = InfoBlockPipe::new(client, stdout);

            match size {
                Some(size) => pipe.start_bytes(size),
                None       => pipe.start_lines(),
            };

            let _status = child.wait().expect("failed to wait on child");
        },

        SubCommand::Rekey => {
            let mut client = client.connect()?;

            let block = BlockRecipe::Rekey;
            let pointer = client.write_block(block)?;
            // if not quiet
            println!("{:x}", pointer);
        },

        SubCommand::Fsck(matches) => {
            let longterm_pk = load_pubkey(config.pub_key())?;

            let paranoid = matches.paranoid;

            let range = storage.resolve_range(matches.spec).expect("failed to expand range");

            let mut session = None;

            // The first block in the spec parameter is trusted
            // If this is an init block this is non-fatal in paranoid mode
            let mut first_block = true;

            for pointer in storage.expand_range(range)? {
                print!("{:x} ... ", pointer);
                io::stdout().flush()?;

                let buf = storage.get_bytes(&pointer)?;

                // TODO: do a 2-stage decode to avoid reencoding for verification

                if let IResult::Done(_, block) = wire::block(&buf) {
                    block.verify_longterm(&longterm_pk)?;

                    match *block.inner() {
                        InnerBlock::Init(ref init) => {
                            print!("{}  ... ", "init".yellow());
                            io::stdout().flush()?;

                            if paranoid && !first_block {
                                panic!("2nd init block is not allowed in paranoid mode");
                            }

                            session = Some(init.pubkey().clone());
                            // println!("ALERT: init: {:?}", session);
                        },
                        InnerBlock::Rekey(ref rekey) => {
                            print!("rekey ... ");
                            io::stdout().flush()?;

                            rekey.verify_session(&session.unwrap())?;

                            session = Some(rekey.pubkey().clone());
                            // println!("rekey: {:?}", session);
                        },
                        InnerBlock::Alert(ref alert) => {
                            print!("alert ... ");
                            io::stdout().flush()?;

                            alert.verify_session(&session.unwrap())?;

                            session = Some(alert.pubkey().clone());
                            // println!("alert: {:?}", session);
                        },
                        InnerBlock::Info(ref info) => {
                            print!("info  ... ");
                            io::stdout().flush()?;

                            info.verify_session(&session.unwrap())?;
                            // println!("info");
                        },
                    };
                } else {
                    return Err(format!("corrupted entry: {:?}", buf).into());
                }

                println!("{}", "ok".green());
                first_block = false;
            }
        },

        SubCommand::Ping(matches) => {
            let mut client = client.connect()?;

            let req = CtlRequest::Ping;
            client.send(&req)?;

            if !matches.quiet {
                println!("pong");
            }
        },

        SubCommand::BashCompletion => {
            cli::gen_completions::<cli::tr1pctl::Args>("tr1pctl");
        }
    }

    Ok(())
}

fn main() {
    if let Err(ref e) = run() {
        use error_chain::ChainedError; // trait which holds `display_chain`

        eprintln!("{}", e.display_chain());
        ::std::process::exit(1);
    }
}
