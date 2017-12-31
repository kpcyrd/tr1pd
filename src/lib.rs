extern crate sodiumoxide;
extern crate sha3;
// #[macro_use] extern crate log;
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
pub mod crypto;
pub mod engine;
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
