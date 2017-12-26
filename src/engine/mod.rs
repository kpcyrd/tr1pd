use blocks::{Block, BlockPointer};
use storage::BlockStorage;
use crypto::SignRing;


pub mod errors {
    error_chain! {
        links {
            Storage(::storage::errors::Error, ::storage::errors::ErrorKind);
        }
    }

    impl From<()> for Error {
        fn from(_x: ()) -> Error {
            // dummy to get the prototype to work
            unimplemented!()
        }
    }
}
use self::errors::{Result};

pub struct Engine {
    storage: BlockStorage,
    ring: SignRing,
    head: BlockPointer,
}

impl Engine {
    pub fn start(storage: BlockStorage, ring: SignRing) -> Result<Engine> {
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
                BlockPointer::from_slice(&[
                    0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0,
                ]).unwrap()
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
}
