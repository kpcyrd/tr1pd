use std::fs;
use std::fs::File;
use std::os::unix;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;

use storage::{BlockStorage, Result};
use blocks::BlockPointer;


#[derive(Debug)]
pub struct DiskStorage {
    path: PathBuf,
}

impl DiskStorage {
    pub fn new<I: Into<PathBuf>>(path: I) -> DiskStorage {
        DiskStorage {
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
        // TODO: set permissions
        let parent = path.parent().expect("path has no parent folder");
        fs::create_dir_all(parent)?;
        Ok(())
    }
}

impl BlockStorage for DiskStorage {
    fn write_bytes(&mut self, pointer: &BlockPointer, bytes: Vec<u8>) -> Result<()> {
        let path = self.pointer_to_path(&pointer);

        self.ensure_parent_folder(&path)?;

        let mut file = OpenOptions::new()
                        .write(true)
                        .create(true)
                        .create_new(true)
                        .mode(0o640)
                        .open(&path)?;
        file.write_all(&bytes)?;

        println!("wrote {:x} to {:?}", pointer, path);

        Ok(())
    }

    fn get_bytes(&self, pointer: &BlockPointer) -> Result<Vec<u8>> {
        let path = self.pointer_to_path(&pointer);
        let mut file = File::open(path)?;

        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;

        // verify block
        pointer.verify(&buf)?;

        Ok(buf)
    }

    fn get_head(&self) -> Result<BlockPointer> {
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

        let pointer = BlockPointer::from_hex(&hex)?;
        Ok(pointer)
    }

    fn update_head(&mut self, pointer: &BlockPointer) -> Result<()> {
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
