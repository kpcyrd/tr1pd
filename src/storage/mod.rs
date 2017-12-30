use std::fs;
use std::fs::File;
use std::os::unix;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use nom::IResult;

use blocks::{BlockPointer, Block};
use wire;

pub mod errors {
    use std::io;

    error_chain! {
        foreign_links {
            Io(io::Error);
        }
    }
}
use self::errors::{Result};

#[derive(Debug)]
pub struct BlockStorage {
    path: PathBuf,
}

impl BlockStorage {
    pub fn new<I: Into<PathBuf>>(path: I) -> BlockStorage {
        BlockStorage {
            path: path.into(),
        }
    }

    pub fn pointer_to_path(&self, pointer: &BlockPointer) -> PathBuf {
        let (prefix, hash) = pointer.slice();

        let mut path = self.path.clone();
        path.push("blocks");
        path.push(prefix);
        path.push(hash);

        path
    }

    #[inline]
    fn ensure_parent_folder(&self, path: &Path) -> Result<()> {
        let parent = path.parent().expect("path has no parent folder");
        fs::create_dir_all(parent)?;
        Ok(())
    }

    pub fn push(&self, block: &Block) -> Result<BlockPointer> {
        let pointer = block.sha3();
        let path = self.pointer_to_path(&pointer);

        self.ensure_parent_folder(&path)?;

        let mut file = File::create(&path)?;
        file.write_all(&block.encode())?;

        println!("wrote {:x} to {:?}", pointer, path);
        self.update_head(&pointer).unwrap();

        Ok(pointer)
    }

    pub fn get(&self, pointer: &BlockPointer) -> Result<Block> {
        let buf = self.get_raw(pointer)?;

        if let IResult::Done(_, block) = wire::block(&buf) {
            let block = block.0;

            // println!("[block] decoded: {:?}", block);

            Ok(block)
        } else {
            panic!("Error::CorruptedEntry")
        }
    }

    pub fn get_raw(&self, pointer: &BlockPointer) -> Result<Vec<u8>> {
        let path = self.pointer_to_path(&pointer);
        let mut file = File::open(path)?;

        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;

        // verify block
        pointer.verify(&buf).expect("fsck block");

        Ok(buf)
    }

    pub fn get_head(&self) -> Result<BlockPointer> {
        let mut path = self.path.clone();
        path.push("HEAD");
        let mut head = fs::read_link(path)?;

        let hash = {
            let x = head.file_name().unwrap();
            x.to_str().unwrap().to_owned()
        };
        head.pop();
        let prefix = {
            let x = head.file_name().unwrap();
            x.to_str().unwrap().to_owned()
        };

        let hex = format!("{}{}", prefix, hash);

        let pointer = BlockPointer::from_hex(&hex).unwrap();
        Ok(pointer)
    }

    pub fn update_head(&self, pointer: &BlockPointer) -> Result<()> {
        let (prefix, hash) = pointer.slice();

        let mut src = PathBuf::from("blocks");
        src.push(prefix);
        src.push(hash);

        let mut dest = self.path.clone();
        dest.push("HEAD");

        // TODO: atomic replace
        if let Ok(_) = fs::symlink_metadata(&dest) {
            fs::remove_file(&dest)?;
        }

        unix::fs::symlink(src, dest)?;
        Ok(())
    }
}
