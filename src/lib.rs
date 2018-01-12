#![warn(unused_extern_crates)]

extern crate sodiumoxide;
extern crate sha3;
extern crate clap;
extern crate serde_json;
extern crate scaproust;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate nom;

#[cfg(test)]
extern crate pseudo;

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
pub mod spec;
pub mod storage;
#[allow(unused_variables)]
pub mod wire;

#[cfg(test)]
mod tests;


use blocks::BlockPointer;

pub fn backtrace<S: storage::BlockStorage>(storage: &S, since: Option<&str>, to: Option<&str>) -> Result<Vec<BlockPointer>> {
    let since = match since {
        Some(since) => BlockPointer::from_hex(since)?,
        None => BlockPointer::empty(),
    };

    let mut pointer = match to {
        Some(to) => BlockPointer::from_hex(to)?,
        None => storage.get_head()?,
    };

    let mut backtrace = vec![pointer.clone()];

    loop {
        let block = storage.get(&pointer)?;
        pointer = block.prev().clone();

        if pointer.is_empty() {
            break;
        }

        backtrace.push(pointer.clone());

        if pointer == since {
            break;
        }
    }

    Ok(backtrace)
}
