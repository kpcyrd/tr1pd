use blocks::{BlockPointer, Block};
use wire;

use nom::IResult;

pub mod disk;
pub mod memory;

pub use self::disk::DiskStorage;
pub use self::memory::MemoryStorage;

pub mod errors {
    use std::io;

    error_chain! {
        errors {
            CorruptedEntry(bytes: Vec<u8>) {
                description("corrupted entry")
                display("corrupted entry: {:?}", bytes)
            }
        }
        links {
            Blocks(::blocks::errors::Error, ::blocks::errors::ErrorKind);
        }
        foreign_links {
            Io(io::Error);
        }
    }
}
use self::errors::{Result, ErrorKind};

pub trait BlockStorage {
    fn write_bytes(&mut self, pointer: &BlockPointer, bytes: Vec<u8>) -> Result<()>;

    fn get_bytes(&self, pointer: &BlockPointer) -> Result<Vec<u8>>;

    fn get_head(&self) -> Result<BlockPointer>;

    fn update_head(&mut self, pointer: &BlockPointer) -> Result<()>;

    #[inline]
    fn push(&mut self, block: &Block) -> Result<BlockPointer> {
        let (pointer, bytes) = block.sha3_encode();
        self.write_bytes(&pointer, bytes)?;
        self.update_head(&pointer)?;
        Ok(pointer)
    }

    fn get(&self, pointer: &BlockPointer) -> Result<Block> {
        let buf = self.get_bytes(pointer)?;

        if let IResult::Done(_, block) = wire::block(&buf) {
            debug!("[block] decoded: {:?}", block);
            Ok(block)
        } else {
            Err(ErrorKind::CorruptedEntry(buf.to_vec()).into())
        }
    }
}
