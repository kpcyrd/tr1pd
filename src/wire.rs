use nom::{IResult, be_u8, be_u16};
use blocks::{BlockPointer, InnerBlock, Block};
use blocks::{InitBlock, RekeyBlock, AlertBlock, InfoBlock};
use crypto::{PublicKey, Signature};


mod errors {
    error_chain! {
        errors {
            BlockTooLarge
        }
    }
}
pub use self::errors::{Result, Error, ErrorKind};


/// Convert the length to a byte array (big endian).
///
/// ```
/// use tr1pd::wire::len_to_u16_vec;
///
/// // regular block
/// len_to_u16_vec(25).ok().unwrap();
/// // maximum block size
/// len_to_u16_vec(65535).ok().unwrap();
/// // too large
/// len_to_u16_vec(65536).err().unwrap();
/// // way too large
/// len_to_u16_vec(52_428_800).err().unwrap();
/// ```
#[inline]
pub fn len_to_u16_vec(i: usize) -> Result<[u8; 2]> {
    if i >= 2usize.pow(16) {
        // overflow
        return Err(ErrorKind::BlockTooLarge.into());
    }

    let length = i as u16;
    let msb = ((length & 0b1111_1111_0000_0000) >> 8) as u8;
    let lsb = (length & 0b1111_1111) as u8;

    let mut bytes = [0; 2];
    bytes[0] = msb;
    bytes[1] = lsb;
    Ok(bytes)
}

named!(pub pointer<&[u8], BlockPointer>, map_res!(take!(32), BlockPointer::from_slice));
named!(pub pubkey<&[u8], PublicKey>, map_opt!(take!(32), PublicKey::from_slice));
named!(pub signature<&[u8], Signature>, map_opt!(take!(64), Signature::from_slice));

fn inner(input: &[u8]) -> IResult<&[u8], InnerBlock> {
    do_parse!(input,
        prev: pointer           >>
        inner: switch!(be_u8,
            0x00 => apply!(init, prev) |
            0x01 => apply!(rekey, prev) |
            0x02 => apply!(alert, prev) |
            0x03 => apply!(info, prev)
        ) >>
        (inner)
    )
}

fn init(input: &[u8], prev: BlockPointer) -> IResult<&[u8], InnerBlock> {
    do_parse!(input,
        pubkey: pubkey  >>
        ({
            InitBlock::from_network(
                prev,
                pubkey,
            )
        })
    )
}

fn rekey(input: &[u8], prev: BlockPointer) -> IResult<&[u8], InnerBlock> {
    do_parse!(input,
        pubkey: pubkey          >>
        signature: signature    >>
        ({
            RekeyBlock::from_network(
                prev,
                pubkey,
                signature,
            )
        })
    )
}

fn alert(input: &[u8], prev: BlockPointer) -> IResult<&[u8], InnerBlock> {
    do_parse!(input,
        pubkey: pubkey          >>
        length: be_u16          >>
        bytes: take!(length)    >>
        signature: signature    >>
        ({
            AlertBlock::from_network(
                prev,
                pubkey,
                bytes.to_vec(),
                signature,
            )
        })
    )
}

fn info(input: &[u8], prev: BlockPointer) -> IResult<&[u8], InnerBlock> {
    do_parse!(input,
        length: be_u16          >>
        bytes: take!(length)    >>
        signature: signature    >>
        ({
            InfoBlock::from_network(
                prev,
                bytes.to_vec(),
                signature,
            )
        })
    )
}


pub fn block(input: &[u8]) -> IResult<&[u8], Block> {
    do_parse!(input,
        inner: inner            >>
        signature: signature    >>
        ({
            Block::new(inner, signature)
        })
    )
}


/*
pub mod tokio {
    use nom::{IResult, be_u8};

    use super::{block, pointer};
    use blocks::prelude::*;

    #[derive(Debug)]
    pub enum BlockPacket {
        Block(Block),
        Head(BlockPointer),
        /*
        Req(BlockReq),
        Resp(BlockResp),
        */
    }

    impl From<Block> for BlockPacket {
        fn from(block: Block) -> BlockPacket {
            BlockPacket::Block(block)
        }
    }

    impl From<BlockPointer> for BlockPacket {
        fn from(pointer: BlockPointer) -> BlockPacket {
            BlockPacket::Head(pointer)
        }
    }

    /*
    fn pkt_block(input: &[u8]) -> IResult<&[u8], BlockPacket> {
        match block(input) {
            IResult::Done(remaining, block) => {
                IResult::Done(remaining, block.0.into())
            },
            IResult::Incomplete(remaining) => IResult::Incomplete(remaining),
            IResult::Error(e) => IResult::Error(e),
        }
    }

    fn pkt_pointer(input: &[u8]) -> IResult<&[u8], BlockPacket> {
        match pointer(input) {
            IResult::Done(remaining, pointer) => {
                IResult::Done(remaining, pointer.into())
            },
            IResult::Incomplete(remaining) => IResult::Incomplete(remaining),
            IResult::Error(e) => IResult::Error(e),
        }
    }

    /// used for tokio-proto
    pub fn packet(input: &[u8]) -> IResult<&[u8], BlockPacket> {
        do_parse!(input,
            inner: switch!(be_u8,
                0x00 => call!(pkt_block) |
                0x01 => call!(pkt_pointer)
            ) >>
            (inner)
        )
    }
    */
}
*/
