use sodiumoxide::utils;

use crypto;
use crypto::prelude::*;
use blocks::prelude::*;


mod errors {
    error_chain! {
        errors {
            ProtocolViolation
        }
        links {
            Crypto(::crypto::errors::Error, ::crypto::errors::ErrorKind);
        }
    }
}
pub use self::errors::{Result, Error, ErrorKind};


pub struct VerifyRing {
    longterm_key: PublicKey,
    session_key: Option<PublicKey>,
}

impl VerifyRing {
    pub fn new(longterm: PublicKey) -> VerifyRing {
        VerifyRing {
            longterm_key: longterm,
            session_key: None,
        }
    }

    pub fn verify_longterm(&self, m: &[u8], sig: &Signature) -> Result<()> {
        crypto::verify(sig, m, &self.longterm_key)?;
        Ok(())
    }

    pub fn verify_session(&self, m: &[u8], sig: &Signature) -> Result<()> {
        match self.session_key {
            Some(ref pk) => {
                crypto::verify(sig, &m, pk)?;
                Ok(())
            },
            None => {
                Err(ErrorKind::ProtocolViolation.into())
            },
        }
    }

    // TODO: this might be obsolete
    pub fn init(&mut self, pubkey: PublicKey) -> Result<()> {
        if self.session_key.is_none() {
            self.session_key = Some(pubkey);
            Ok(())
        } else {
            Err(ErrorKind::ProtocolViolation.into())
        }
    }

    pub fn unclean_rekey(&mut self, pubkey: PublicKey) {
        self.session_key = Some(pubkey);
    }

    fn internal_rekey(&mut self, pubkey: PublicKey, buf: &[u8], sig: &Signature) -> Result<()> {
        match self.verify_session(&buf, sig) {
            Ok(_) => {
                self.session_key = Some(pubkey);
                Ok(())
            },
            Err(err) => Err(err),
        }
    }

    pub fn rekey(&mut self, block: &Signed<RekeyBlock>) -> Result<()> {
        self.internal_rekey(block.pubkey().clone(),
                            &block.encode_inner(),
                            block.signature())
    }

    pub fn alert_rekey(&mut self, block: &Signed<AlertBlock>) -> Result<()> {
        self.internal_rekey(block.pubkey().clone(),
                            &block.encode_inner(),
                            block.signature())
    }

    pub fn verify_block_session<T: Signable>(&mut self, block: &Signed<T>) -> Result<()> {
        self.verify_session(&block.encode_inner(),
                            block.signature())
    }
}


pub struct SignRing {
    #[allow(dead_code)]
    longterm_pk: PublicKey,
    longterm_sk: SecretKey,

    session_pk: Option<PublicKey>,
    session_sk: Option<SecretKey>,

    delayed_keypair: Option<(PublicKey, SecretKey)>,
}

impl SignRing {
    pub fn new(pk: PublicKey, sk: SecretKey) -> SignRing {
        SignRing {
            longterm_pk: pk,
            longterm_sk: sk,

            session_pk: None,
            session_sk: None,

            delayed_keypair: None,
        }
    }

    /*
    pub fn longterm_key(&self) -> &PublicKey {
        &self.longterm_pk
    }

    pub fn session_key(&self) -> &Option<PublicKey> {
        &self.session_pk
    }
    */

    pub fn sign_longterm(&self, m: &[u8]) -> Signature {
        crypto::sign(m, &self.longterm_sk)
    }

    pub fn sign_session(&self, m: &[u8]) -> Signature {
        match self.session_sk {
            Some(ref sk) => crypto::sign(&m, sk),
            None => panic!("session key is None"),
        }
    }

    pub fn init(&mut self) -> PublicKey {
        let (pk, sk) = crypto::gen_keypair();
        self.session_pk = Some(pk.clone());
        self.session_sk = Some(sk);
        pk
    }

    fn start_rekey(&mut self) -> PublicKey {
        let (pk, sk) = crypto::gen_keypair();

        // old key is still active to sign off the current block
        self.delayed_keypair = Some((pk.clone(), sk));

        pk
    }

    pub fn rekey(&mut self, prev: BlockPointer) -> Signed<RekeyBlock> {
        let pubkey = self.start_rekey();
        let block = RekeyBlock::new(prev, pubkey);

        let signature = self.finalize_rekey(&block.encode())
                                .expect("start_rekey hasn't been called");

        Signed::new(block, signature)
    }

    pub fn alert(&mut self, prev: BlockPointer, bytes: Vec<u8>) -> Signed<AlertBlock> {
        let pubkey = self.start_rekey();
        let block = AlertBlock::new(prev, pubkey, bytes);

        let signature = self.finalize_rekey(&block.encode())
                                .expect("start_rekey hasn't been called");

        Signed::new(block, signature)
    }

    fn memzero(x: &mut [u8]) {
        utils::memzero(x)
    }

    fn finalize_rekey(&mut self, buf: &[u8]) -> Result<Signature> {
        let signature = self.sign_session(&buf);

        match self.delayed_keypair.take() {
            Some((pk, sk)) => {
                match self.session_sk {
                    Some(ref mut sk) => SignRing::memzero(&mut sk.0),
                    _ => (),
                };

                self.session_pk = Some(pk);
                self.session_sk = Some(sk);

                Ok(signature)
            },
            None => Err(ErrorKind::ProtocolViolation.into()),
        }
    }
}
