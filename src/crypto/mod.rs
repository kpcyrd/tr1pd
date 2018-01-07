use std::ops::Deref;

pub use sodiumoxide::crypto::sign::{self, Signature, SecretKey, PublicKey};

pub mod ring;

pub use self::ring::{VerifyRing, SignRing};

pub mod errors {
    error_chain! {
        errors {
            InvalidSignature
            CorruptedKey
        }
    }
}
use self::errors::{Result, ErrorKind};


pub mod prelude {
    pub use super::{Unverified, Signable, Signed};
    pub use super::{PublicKey, SecretKey, Signature};
}


pub fn gen_keypair() -> (PublicKey, SecretKey) {
    sign::gen_keypair()
}

pub fn sign(m: &[u8], sk: &SecretKey) -> Signature {
    sign::sign_detached(m, sk)
}

pub fn verify(sig: &Signature, m: &[u8], pk: &PublicKey) -> Result<()> {
    if sign::verify_detached(sig, m, pk) {
        Ok(())
    } else {
        Err(ErrorKind::InvalidSignature.into())
    }
}

pub fn to_pubkey(pk: &[u8]) -> Result<PublicKey> {
    match PublicKey::from_slice(pk) {
        Some(pk) => Ok(pk),
        None => Err(ErrorKind::CorruptedKey.into()),
    }
}

pub fn to_privkey(sk: &[u8]) -> Result<SecretKey> {
    match SecretKey::from_slice(sk) {
        Some(sk) => Ok(sk),
        None => Err(ErrorKind::CorruptedKey.into()),
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unverified<T>(pub T);

impl<T> Deref for Unverified<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.0
    }
}

pub trait Signable {
    fn encode(&self) -> Vec<u8>;
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Signed<T: Signable>(pub T, pub Signature);

impl<T: Signable> Deref for Signed<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T: Signable> Signed<T> {
    pub fn new(inner: T, signature: Signature) -> Signed<T> {
        Signed(inner, signature)
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buf = self.encode_inner();
        buf.extend(self.signature().0.iter());
        buf
    }

    fn encode_inner(&self) -> Vec<u8> {
        self.0.encode()
    }

    pub fn inner(&self) -> &T {
        &self.0
    }

    pub fn signature(&self) -> &Signature {
        &self.1
    }

    pub fn verify_session(&self, pubkey: &PublicKey) -> Result<()> {
        verify(&self.1, &self.0.encode(), pubkey)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Verified<T>(pub T);

impl<T> Deref for Verified<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.0
    }
}
