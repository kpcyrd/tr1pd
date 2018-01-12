use blocks::{BlockPointer, BlockIdentifier, Block};
use spec;
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
            Blocks(::blocks::Error, ::blocks::ErrorKind);
        }
        foreign_links {
            Io(io::Error);
        }
    }
}
pub use self::errors::{Result, Error, ErrorKind};

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

    fn resolve_pointer(&self, spec: spec::SpecPointer) -> Result<BlockPointer> {
        use spec::SpecPointer::*;

        match spec {
            Block(pointer) => Ok(pointer),
            Head => self.get_head(),
            Parent((spec, num)) => {
                let mut pointer = self.resolve_pointer(*spec)?;

                for _ in 0..num {
                    let block = self.get(&pointer)?;
                    pointer = block.prev().clone();
                }

                Ok(pointer)
            },
            Session(spec) => {
                let mut pointer = self.resolve_pointer(*spec)?;

                loop {
                    let block = self.get(&pointer)?;

                    if block.identifier() == BlockIdentifier::Init {
                        break;
                    }

                    pointer = block.prev().clone();
                }

                Ok(pointer)
            },
        }
    }

    fn resolve_range(&self, spec: (spec::SpecPointer, spec::SpecPointer)) -> Result<(BlockPointer, BlockPointer)> {
        let (a, b) = spec;
        let a = self.resolve_pointer(a)?;
        let b = self.resolve_pointer(b)?;
        Ok((a, b))
    }

    fn expand_range(&self, range: (BlockPointer, BlockPointer)) -> Result<Vec<BlockPointer>> {
        // TODO: inefficient, especially in combination with session pointers

        let (start, stop) = range;

        let mut pointers = Vec::new();
        let mut cur = stop;

        loop {
            let block = self.get(&cur)?;

            let found = cur == start;
            pointers.push(cur);

            if found {
                break;
            }

            cur = block.prev().clone();
        }

        Ok(pointers.into_iter().rev().collect())
    }
}

pub enum StorageEngine {
    Disk(DiskStorage),
    Memory(MemoryStorage),
}

impl BlockStorage for StorageEngine {
    #[inline]
    fn write_bytes(&mut self, pointer: &BlockPointer, bytes: Vec<u8>) -> Result<()> {
        match *self {
            StorageEngine::Disk(ref mut s) => s.write_bytes(pointer, bytes),
            StorageEngine::Memory(ref mut s) => s.write_bytes(pointer, bytes),
        }
    }

    #[inline]
    fn get_bytes(&self, pointer: &BlockPointer) -> Result<Vec<u8>> {
        match *self {
            StorageEngine::Disk(ref s) => s.get_bytes(pointer),
            StorageEngine::Memory(ref s) => s.get_bytes(pointer),
        }
    }

    #[inline]
    fn get_head(&self) -> Result<BlockPointer> {
        match *self {
            StorageEngine::Disk(ref s) => s.get_head(),
            StorageEngine::Memory(ref s) => s.get_head(),
        }
    }

    #[inline]
    fn update_head(&mut self, pointer: &BlockPointer) -> Result<()> {
        match *self {
            StorageEngine::Disk(ref mut s) => s.update_head(pointer),
            StorageEngine::Memory(ref mut s) => s.update_head(pointer),
        }
    }
}
