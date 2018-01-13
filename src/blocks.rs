use sha3::{Digest, Sha3_256};

use crypto::{self, PublicKey, Signable, Signed, Signature};
use crypto::ring::SignRing;
use wire::len_to_u16_vec;

use std::fmt;


mod errors {
    error_chain! {
        errors {
            CorruptedBlock
            InvalidBlockPointer
            BlockTooLarge
            InvalidBlockIdentifier(b: u8) {
                description("invalid block type identifier")
                display("invalid block type identifier: {:x}", b)
            }
        }
        links {
            Crypto(::crypto::Error, ::crypto::ErrorKind);
        }
    }
}
pub use self::errors::{Result, Error, ErrorKind};


/// Validate the message doesn't exceed the maximum length of (2**16)-1.
///
/// ```
/// use tr1pd::blocks::validate_block_size;
///
/// // regular block
/// validate_block_size(25).ok().unwrap();
/// // maximum block size
/// validate_block_size(65535).ok().unwrap();
/// // too large
/// validate_block_size(65536).err().unwrap();
/// // way too large
/// validate_block_size(52_428_800).err().unwrap();
/// ```
pub fn validate_block_size(len: usize) -> Result<()> {
    if len >= 2usize.pow(16) {
        Err(ErrorKind::BlockTooLarge.into())
    } else {
        Ok(())
    }
}


/// Pointer to a [`Block`]
///
/// [`Block`]: struct.Block.html
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlockPointer(pub [u8; 32]);

impl BlockPointer {
    pub fn from_slice(bytes: &[u8]) -> Result<BlockPointer> {
        if bytes.len() == 32 {
            let mut pointer = [0; 32];
            pointer.copy_from_slice(bytes);
            Ok(BlockPointer(pointer))
        } else {
            Err(ErrorKind::InvalidBlockPointer.into())
        }
    }

    #[inline]
    pub fn empty() -> BlockPointer {
        BlockPointer([0; 32])
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0 == [0; 32]
    }

    pub fn verify(&self, buf: &[u8]) -> Result<()> {
        let result = Sha3_256::digest(&buf);
        let calculated = BlockPointer::from_slice(result.as_slice()).unwrap();

        if calculated == *self {
            Ok(())
        } else {
            Err(ErrorKind::CorruptedBlock.into())
        }
    }

    pub fn slice(&self) -> (String, String) {
        // TODO: somewhat inefficient
        let hex = format!("{:x}", self);
        let (prefix, hash) = hex.split_at(4);
        (prefix.into(), hash.into())
    }

    pub fn from_hex(mut hex: &str) -> Result<BlockPointer> {
        use std::result;

        let result: result::Result<Vec<u8>, _> = (0..32)
            .map(|_| {
                let (chunk, remain) = hex.split_at(2);
                hex = remain;
                chunk
            })
            .map(|x| {
                u8::from_str_radix(x, 16)
            })
            .collect();

        match result {
            Ok(result) => BlockPointer::from_slice(&result),
            Err(_) => Err(ErrorKind::InvalidBlockPointer.into()),
        }
    }
}

impl From<Option<[u8; 32]>> for BlockPointer {
    fn from(pointer: Option<[u8; 32]>) -> BlockPointer {
        match pointer {
            Some(pointer) => BlockPointer(pointer),
            None => BlockPointer([0; 32]),
        }
    }
}

impl fmt::LowerHex for BlockPointer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for x in self.0.iter() {
            write!(f, "{:02x}", x)?
        }
        Ok(())
    }
}

/// The outer block
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Block {
    inner: InnerBlock,
    signature: Signature,
}

impl Block {
    /// Build a new block.
    #[inline]
    pub fn new(inner: InnerBlock, signature: Signature) -> Block {
        Block {
            inner: inner,
            signature: signature,
        }
    }

    /// Sign an inner block with the long-term key.
    #[inline]
    fn sign(inner: InnerBlock, keyring: &SignRing) -> Result<Block> {
        let buf = inner.encode();
        let signature = keyring.sign_longterm(&buf);

        Ok(Block {
            inner,
            signature,
        })
    }

    /// Verify the long-term signature of the outer block.
    #[inline]
    pub fn verify_longterm(&self, pubkey: &PublicKey) -> Result<()> {
        crypto::verify(&self.signature, &self.inner.encode(), pubkey)?;
        Ok(())
    }

    /// Returns the [`BlockPointer`] of a block.
    ///
    /// [`BlockPointer`]: struct.BlockPointer.html
    #[inline]
    pub fn sha3(&self) -> BlockPointer {
        let (pointer, _) = self.sha3_encode();
        pointer
    }

    /// Same as [`Block::sha3`], but also returns the encoded block.
    ///
    /// [`Block::sha3`]: #method.sha3
    #[inline]
    pub fn sha3_encode(&self) -> (BlockPointer, Vec<u8>) {
        let bytes = self.encode();
        let sha3 = Sha3_256::digest(&bytes);
        let pointer = BlockPointer::from_slice(sha3.as_slice()).unwrap();
        (pointer, bytes)
    }

    /// Return the encoded message of the block, if there's any.
    #[inline]
    pub fn msg(&self) -> Option<&Vec<u8>> {
        match self.inner {
            InnerBlock::Init(_)  => None,
            InnerBlock::Rekey(_) => None,
            InnerBlock::Alert(ref block) => Some(block.bytes()),
            InnerBlock::Info(ref block)  => Some(block.bytes()),
        }
    }

    /// Build a new init block.
    #[inline]
    pub fn init(prev: BlockPointer, mut keyring: &mut SignRing) -> Result<Block> {
        let inner = InitBlock::new(prev, &mut keyring);
        Block::sign(InnerBlock::Init(inner), &keyring)
    }

    /// Build a new rekey block.
    #[inline]
    pub fn rekey(prev: BlockPointer, keyring: &mut SignRing) -> Result<Block> {
        let inner = keyring.rekey(prev);
        Block::sign(InnerBlock::Rekey(inner), &keyring)
    }

    /// Build a new alert block.
    #[inline]
    pub fn alert(prev: BlockPointer, keyring: &mut SignRing, bytes: Vec<u8>) -> Result<Block> {
        validate_block_size(bytes.len())?;
        let inner = keyring.alert(prev, bytes);
        Block::sign(InnerBlock::Alert(inner), &keyring)
    }

    /// Build a new info block.
    #[inline]
    pub fn info(prev: BlockPointer, mut keyring: &mut SignRing, bytes: Vec<u8>) -> Result<Block> {
        validate_block_size(bytes.len())?;
        let inner = InfoBlock::new(prev, &mut keyring, bytes);
        Block::sign(InnerBlock::Info(inner), &keyring)
    }

    /// Encode the block to it's binary format. See [`Block::sha3_encode`] if
    /// you also need the [`BlockPointer`] of the block.
    ///
    /// [`Block::sha3_encode`]: #method.sha3_encode
    /// [`BlockPointer`]: struct.BlockPointer.html
    #[inline]
    pub fn encode(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        buf.extend(self.inner.encode());
        buf.extend(self.signature.0.iter());
        buf
    }

    /// Return the pointer to the parent block.
    #[inline]
    pub fn prev(&self) -> &BlockPointer {
        self.inner.prev()
    }

    /// Return the inner block.
    #[inline]
    pub fn inner(&self) -> &InnerBlock {
        &self.inner
    }

    /// Return the long-term signature of the block.
    #[inline]
    pub fn signature(&self) -> &Signature {
        &self.signature
    }

    /// Return the [`BlockIdentifier`] for the inner block.
    ///
    /// [`BlockIdentifier`]: enum.BlockIdentifier.html
    #[inline]
    pub fn identifier(&self) -> BlockIdentifier {
        match self.inner {
            InnerBlock::Init(_)  => BlockIdentifier::Init,
            InnerBlock::Rekey(_) => BlockIdentifier::Rekey,
            InnerBlock::Alert(_) => BlockIdentifier::Alert,
            InnerBlock::Info(_)  => BlockIdentifier::Info,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum BlockIdentifier {
    Init,
    Rekey,
    Alert,
    Info,
}

impl BlockIdentifier {
    pub fn to_block_identifier(x: u8) -> Result<BlockIdentifier> {
        match x {
            0x00 => Ok(BlockIdentifier::Init),
            0x01 => Ok(BlockIdentifier::Rekey),
            0x02 => Ok(BlockIdentifier::Alert),
            0x03 => Ok(BlockIdentifier::Info),
            _ => Err(ErrorKind::InvalidBlockIdentifier(x).into()),
        }
    }

    pub fn to_byte(&self) -> u8 {
        match *self {
            BlockIdentifier::Init  => 0x00,
            BlockIdentifier::Rekey => 0x01,
            BlockIdentifier::Alert => 0x02,
            BlockIdentifier::Info  => 0x03,
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        vec![self.to_byte()]
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum InnerBlock {
    Init(InitBlock),
    Rekey(Signed<RekeyBlock>),
    Alert(Signed<AlertBlock>),
    Info(Signed<InfoBlock>),
}

impl InnerBlock {
    fn prev(&self) -> &BlockPointer {
        match *self {
            InnerBlock::Init(ref inner)  => inner.prev(),
            InnerBlock::Rekey(ref inner) => inner.prev(),
            InnerBlock::Alert(ref inner) => inner.prev(),
            InnerBlock::Info(ref inner)  => inner.prev(),
        }
    }

    fn encode(&self) -> Vec<u8> {
        match *self {
            InnerBlock::Init(ref inner)  => inner.encode(),
            InnerBlock::Rekey(ref inner) => inner.encode(),
            InnerBlock::Alert(ref inner) => inner.encode(),
            InnerBlock::Info(ref inner)  => inner.encode(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InitBlock {
    prev: BlockPointer,
    pubkey: PublicKey,
}

impl InitBlock {
    pub fn new(prev: BlockPointer, keyring: &mut SignRing) -> InitBlock {
        let pubkey = keyring.init();
        InitBlock {
            prev,
            pubkey,
        }
    }

    pub fn from_network(prev: BlockPointer, pubkey: PublicKey) -> InnerBlock {
        InnerBlock::Init(InitBlock {
            prev,
            pubkey,
        })
    }

    #[inline]
    pub fn prev(&self) -> &BlockPointer {
        &self.prev
    }

    pub fn pubkey(&self) -> &PublicKey {
        &self.pubkey
    }
}

impl Signable for InitBlock {
    fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend(self.prev.0.iter());
        buf.extend(BlockIdentifier::Init.to_vec());
        buf.extend(self.pubkey.0.iter());
        buf
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RekeyBlock {
    prev: BlockPointer,
    /// New session key
    pubkey: PublicKey,
}

impl RekeyBlock {
    pub fn new(prev: BlockPointer, pubkey: PublicKey) -> RekeyBlock {
        RekeyBlock {
            prev,
            pubkey,
        }
    }

    pub fn from_network(prev: BlockPointer, pubkey: PublicKey, signature: Signature) -> InnerBlock {
        InnerBlock::Rekey(Signed(RekeyBlock {
            prev,
            pubkey,
        }, signature))
    }

    #[inline]
    pub fn prev(&self) -> &BlockPointer {
        &self.prev
    }

    pub fn pubkey(&self) -> &PublicKey {
        &self.pubkey
    }
}

impl Signable for RekeyBlock {
    fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend(self.prev.0.iter());
        buf.extend(BlockIdentifier::Rekey.to_vec());
        buf.extend(self.pubkey.0.iter());
        buf
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AlertBlock {
    prev: BlockPointer,
    /// New session key
    pubkey: PublicKey,
    bytes: Vec<u8>,
}

impl AlertBlock {
    pub fn new(prev: BlockPointer, pubkey: PublicKey, bytes: Vec<u8>) -> AlertBlock {
        AlertBlock {
            prev,
            pubkey,
            bytes,
        }
    }

    pub fn from_network(prev: BlockPointer, pubkey: PublicKey, bytes: Vec<u8>, signature: Signature) -> InnerBlock {
        InnerBlock::Alert(Signed(AlertBlock {
            prev,
            pubkey,
            bytes,
        }, signature))
    }

    #[inline]
    pub fn prev(&self) -> &BlockPointer {
        &self.prev
    }

    pub fn pubkey(&self) -> &PublicKey {
        &self.pubkey
    }

    pub fn bytes(&self) -> &Vec<u8> {
        &self.bytes
    }

    pub fn clone_bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }
}

impl Signable for AlertBlock {
    fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend(self.prev.0.iter());
        buf.extend(BlockIdentifier::Alert.to_vec());
        buf.extend(self.pubkey.0.iter());
        buf.extend(len_to_u16_vec(self.bytes.len()).expect("block len overflow").iter());
        buf.extend(&self.bytes);
        buf
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InfoBlock {
    prev: BlockPointer,
    bytes: Vec<u8>,
}

impl InfoBlock {
    pub fn new(prev: BlockPointer, keyring: &mut SignRing, bytes: Vec<u8>) -> Signed<InfoBlock> {
        let block = InfoBlock {
            prev,
            bytes,
        };
        let signature = keyring.sign_session(&block.encode());

        Signed(block, signature)
    }

    pub fn from_network(prev: BlockPointer, bytes: Vec<u8>, signature: Signature) -> InnerBlock {
        InnerBlock::Info(Signed(InfoBlock {
            prev,
            bytes,
        }, signature))
    }

    #[inline]
    pub fn prev(&self) -> &BlockPointer {
        &self.prev
    }

    pub fn bytes(&self) -> &Vec<u8> {
        &self.bytes
    }

    pub fn clone_bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }
}

impl Signable for InfoBlock {
    fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend(self.prev.0.iter());
        buf.extend(BlockIdentifier::Info.to_vec());
        buf.extend(len_to_u16_vec(self.bytes.len()).expect("block len overflow").iter());
        buf.extend(&self.bytes);
        buf
    }
}
