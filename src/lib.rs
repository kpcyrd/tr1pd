extern crate sodiumoxide;
extern crate sha3;
// #[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate nom;

pub mod errors {
    error_chain! {
    }
}

pub mod blocks;
pub mod crypto;
pub mod engine;
pub mod storage;
#[allow(unused_variables)]
pub mod wire;


use blocks::BlockPointer;
pub fn backtrace(storage: &storage::BlockStorage, since: Option<&str>) -> Result<Vec<BlockPointer>, errors::Error> {
    let since = match since {
        Some(since) => BlockPointer::from_hex(since).unwrap(),
        None => BlockPointer::from_slice(&[
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
        ]).unwrap(),
    };

    let mut backtrace = vec![];

    let mut pointer = storage.get_head().unwrap();
    loop {
        let block = storage.get(&pointer).unwrap();
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
