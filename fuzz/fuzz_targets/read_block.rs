#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate tr1pd;

use tr1pd::wire;

fuzz_target!(|data: &[u8]| {
    let _ = wire::block(&data);
});
