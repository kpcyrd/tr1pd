#![warn(unused_extern_crates)]

extern crate sodiumoxide;
extern crate sha3;
#[macro_use] extern crate structopt;
extern crate zmq;
extern crate toml;
extern crate human_size;
extern crate libc;
#[cfg(not(target_os="linux"))]
extern crate users;
#[cfg(target_os="linux")]
extern crate seccomp_sys;
#[cfg(target_os="linux")]
extern crate caps;
#[cfg(target_os="openbsd")]
#[macro_use] extern crate pledge;

#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate nom;

#[cfg(test)]
extern crate pseudo;

mod errors {
    error_chain! {
        links {
            Blocks(::blocks::Error, ::blocks::ErrorKind);
            Storage(::storage::Error, ::storage::ErrorKind);
        }
    }
}
pub use self::errors::{Result, Error, ErrorKind};

pub mod blocks;
pub mod cli;
pub mod config;
pub mod crypto;
pub mod engine;
pub mod recipe;
pub mod rpc;
pub mod sandbox;
pub mod spec;
pub mod storage;
#[allow(unused_variables)]
pub mod wire;

#[cfg(test)]
mod tests;
