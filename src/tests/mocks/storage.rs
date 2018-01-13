use storage::{self, BlockStorage, Result};
use blocks::BlockPointer;

use pseudo::Mock;

use std::result;


#[derive(Debug, Clone)]
pub struct ClonableError;

impl Into<storage::Error> for ClonableError {
    fn into(self) -> storage::Error {
        format!("{:?}", self).into()
    }
}

pub struct MockStorage {
    pub write_bytes: Mock<(BlockPointer, Vec<u8>), result::Result<(), ClonableError>>,

    pub get_bytes: Mock<BlockPointer, result::Result<Vec<u8>, ClonableError>>,

    pub get_head: Mock<(), result::Result<BlockPointer, ClonableError>>,

    pub update_head: Mock<BlockPointer, result::Result<(), ClonableError>>,
}

impl MockStorage {
    pub fn new() -> MockStorage {
        MockStorage {
            write_bytes: Mock::new(Ok(())),
            get_bytes: Mock::new(Err(ClonableError)),
            get_head: Mock::new(Err(ClonableError)),
            update_head: Mock::new(Ok(())),
        }
    }
}

impl BlockStorage for MockStorage {
    fn write_bytes(&mut self, pointer: &BlockPointer, bytes: Vec<u8>) -> Result<()> {
        self.write_bytes.call((pointer.clone(), bytes)).map_err(|x| x.into())
    }

    fn get_bytes(&self, pointer: &BlockPointer) -> Result<Vec<u8>> {
        self.get_bytes.call(pointer.clone()).map_err(|x| x.into())
    }

    fn get_head(&self) -> Result<BlockPointer> {
        self.get_head.call(()).map_err(|x| x.into())
    }

    fn update_head(&mut self, pointer: &BlockPointer) -> Result<()> {
        self.update_head.call(pointer.clone()).map_err(|x| x.into())
    }
}
