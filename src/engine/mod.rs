use blocks::{Block, BlockPointer};
use crypto::SignRing;
use recipe::BlockRecipe;
use storage::{StorageEngine, BlockStorage};


mod errors {
    error_chain! {
        links {
            Blocks(::blocks::Error, ::blocks::ErrorKind);
            Storage(::storage::Error, ::storage::ErrorKind);
        }
    }
}
pub use self::errors::{Result, Error, ErrorKind};

pub struct Engine {
    storage: StorageEngine,
    ring: SignRing,
    head: BlockPointer,
}

impl Engine {
    pub fn start(storage: StorageEngine, ring: SignRing) -> Result<Engine> {
        // TODO: check if this is the first block
        // TODO: write genesis block if yes
        // TODO: build an init+alert otherwise

        // use builder pattern or something so pointer isn't Option<T>

        let head = match storage.get_head() {
            Ok(pointer) => {
                pointer
            },
            Err(_) => {
                // genesis block
                BlockPointer::empty()
            },
        };

        let mut engine = Engine {
            storage,
            ring,
            head,
        };

        engine.init()?;

        Ok(engine)
    }

    /*
    pub fn get(&self, pointer: &BlockPointer) -> Result<Block, storage::Error> {
        self.db.get(pointer)
    }
    */

    pub fn init(&mut self) -> Result<Block> {
        let block = Block::init(self.head.clone(), &mut self.ring)?;
        self.head = self.storage.push(&block)?;
        Ok(block)
    }

    pub fn rekey(&mut self) -> Result<Block> {
        let block = Block::rekey(self.head.clone(), &mut self.ring)?;
        self.head = self.storage.push(&block)?;
        Ok(block)
    }

    pub fn alert(&mut self, bytes: Vec<u8>) -> Result<Block> {
        let block = Block::alert(self.head.clone(), &mut self.ring, bytes)?;
        self.head = self.storage.push(&block)?;
        Ok(block)
    }

    pub fn info(&mut self, bytes: Vec<u8>) -> Result<Block> {
        let block = Block::info(self.head.clone(), &mut self.ring, bytes)?;
        self.head = self.storage.push(&block)?;
        Ok(block)
    }

    pub fn recipe(&mut self, recipe: BlockRecipe) -> Result<BlockPointer> {
        let block = match recipe {
            BlockRecipe::Rekey => {
                self.rekey()?
            },
            BlockRecipe::Info(info) => {
                self.info(info)?;
                self.rekey()?
            },
        };

        Ok(block.sha3())
    }

    pub fn storage(&self) -> &StorageEngine {
        &self.storage
    }
}
