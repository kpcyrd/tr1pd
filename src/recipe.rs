use blocks;
use rpc::Client;

use std::io::{Read, BufReader, BufRead};

use human_size::Size;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlockRecipe {
    Rekey,
    Info(Vec<u8>),
}

impl BlockRecipe {
    pub fn info(buf: Vec<u8>) -> Result<BlockRecipe, blocks::Error> {
        blocks::validate_block_size(buf.len())?;
        Ok(BlockRecipe::Info(buf))
    }
}


pub fn parse_size(size: &str) -> Result<usize, String> {

    // TODO: this is a very strict parser, eg "512k" is invalid "512 KiB" isn't
    let mut size = match size.parse::<Size>() {
        Ok(size) => size.into_bytes() as usize,
        Err(_) => match size.parse() {
            Ok(size) => size,
            Err(_) => return Err("failed to parse size".to_string()),
        },
    };

    if size >= 65536 {
        eprintln!("WARN: --size exceeds maximum block size, caping to 65535");
        size = 65535;
    }

    Ok(size)
}

pub struct InfoBlockPipe<R: Read> {
    pub quiet: bool,
    client: Client,
    src: Option<R>,
}

impl<R: Read> InfoBlockPipe<R> {
    #[inline]
    pub fn new(client: Client, src: R) -> InfoBlockPipe<R> {
        InfoBlockPipe {
            quiet: false,
            client,
            src: Some(src),
        }
    }

    #[inline]
    pub fn write(&mut self, buf: Vec<u8>) -> Result<(), ()> {
        // TODO: panics
        let block = BlockRecipe::info(buf).expect("couldn't build block recipe");
        let pointer = self.client.write_block(block).expect("write block");

        if !self.quiet {
            println!("{:x}", pointer);
        }

        Ok(())
    }

    #[inline]
    pub fn start_lines(&mut self) {
        let src = BufReader::new(self.src.take().unwrap());
        for line in src.lines() {
            // discard invalid lines
            if let Ok(mut line) = line {
                line.push('\n');
                self.write(line.as_bytes().to_vec()).expect("write failed");
            }
        }
    }

    #[inline]
    pub fn start_bytes(&mut self, size: usize) {
        let mut src = self.src.take().unwrap();

        let mut buf = vec![0; size];
        loop {
            let i = src.read(&mut buf).unwrap();
            if i == 0 {
                break;
            }
            self.write(buf[..i].to_vec()).expect("write failed");
        }
    }
}
