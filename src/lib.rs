extern crate sodiumoxide;
extern crate sha3;
extern crate clap;
extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_service;
extern crate tokio_proto;
extern crate tokio_uds_proto;
extern crate serde;
extern crate serde_json;
extern crate mrsc;
extern crate bytes;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate nom;

pub mod errors {
    error_chain! {
        links {
            Blocks(::blocks::errors::Error, ::blocks::errors::ErrorKind);
            Storage(::storage::errors::Error, ::storage::errors::ErrorKind);
        }
    }
}
use self::errors::Result;

pub mod blocks;
pub mod cli;
pub mod crypto;
pub mod engine;
pub mod recipe;
pub mod rpc;
pub mod storage;
#[allow(unused_variables)]
pub mod wire;


use blocks::BlockPointer;
pub fn backtrace(storage: &storage::BlockStorage, since: Option<&str>, to: Option<&str>) -> Result<Vec<BlockPointer>> {
    let since = match since {
        Some(since) => BlockPointer::from_hex(since)?,
        None => BlockPointer::from_slice(&[
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
        ]).unwrap(),
    };

    let mut pointer = match to {
        Some(to) => BlockPointer::from_hex(to)?,
        None => storage.get_head()?,
    };

    let mut backtrace = vec![pointer.clone()];

    loop {
        let block = storage.get(&pointer)?;
        pointer = block.prev().clone();

        if pointer == BlockPointer::from_slice(&[
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            ]).unwrap() {
            break;
        }

        backtrace.push(pointer.clone());

        if pointer == since {
            break;
        }
    }

    Ok(backtrace)
}
