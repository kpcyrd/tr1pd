use blocks::BlockPointer;
use crypto;
use crypto::ring::{VerifyRing, SignRing};

#[test]
fn init() {
    let (pk, sk) = crypto::gen_keypair();

    let _sr = SignRing::new(pk.clone(), sk);
    let _vr = VerifyRing::new(pk);
}

#[test]
fn longterm() {
    let (pk, sk) = crypto::gen_keypair();

    let sr = SignRing::new(pk.clone(), sk);
    let vr = VerifyRing::new(pk);

    let bytes = [0,1,2,3];
    let sig = sr.sign_longterm(&bytes);
    assert!(vr.verify_longterm(&bytes, &sig).is_ok());
}

#[test]
fn longterm_fail_pubkey() {
    let (pk, sk) = crypto::gen_keypair();
    let sr = SignRing::new(pk.clone(), sk);
    let vr = VerifyRing::new(crypto::to_pubkey(&[
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
    ]).unwrap());

    let bytes = [0,1,2,3];
    let sig = sr.sign_longterm(&bytes);
    assert!(vr.verify_longterm(&bytes, &sig).is_err());
}

#[test]
fn longterm_fail_signature() {
    let (pk, sk) = crypto::gen_keypair();
    let sr = SignRing::new(pk.clone(), sk);
    let vr = VerifyRing::new(pk);

    let bytes = [0,1,2,3];
    let sig = sr.sign_longterm(&bytes);
    assert!(vr.verify_longterm(&[9, 9, 9, 9], &sig).is_err());
}

#[test]
fn session() {
    let (pk, sk) = crypto::gen_keypair();

    let mut sr = SignRing::new(pk.clone(), sk);
    let mut vr = VerifyRing::new(pk);

    let pk = sr.init();
    vr.init(pk).unwrap();

    let bytes = [0,1,2,3];
    let sig = sr.sign_session(&bytes);
    assert!(vr.verify_session(&bytes, &sig).is_ok());
}

#[test]
fn session_fail_pubkey() {
    let (pk, sk) = crypto::gen_keypair();
    let mut sr = SignRing::new(pk.clone(), sk);
    let mut vr = VerifyRing::new(pk);

    let _ = sr.init();
    vr.init(crypto::to_pubkey(&[
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
    ]).unwrap()).unwrap();

    let bytes = [0,1,2,3];
    let sig = sr.sign_session(&bytes);
    assert!(vr.verify_session(&bytes, &sig).is_err());
}

#[test]
fn session_fail_signature() {
    let (pk, sk) = crypto::gen_keypair();
    let mut sr = SignRing::new(pk.clone(), sk);
    let mut vr = VerifyRing::new(pk);

    let pk = sr.init();
    vr.init(pk).unwrap();

    let bytes = [0,1,2,3];
    let sig = sr.sign_session(&bytes);
    assert!(vr.verify_session(&[9, 9, 9, 9], &sig).is_err());
}

#[test]
fn session_rekey() {
    let (pk, sk) = crypto::gen_keypair();
    let dummy = BlockPointer::from(None);

    let mut sr = SignRing::new(pk.clone(), sk);
    let mut vr = VerifyRing::new(pk);

    let pk = sr.init();
    vr.init(pk).unwrap();

    let bytes = [0,1,2,3];
    let sig1 = sr.sign_session(&bytes);
    assert!(vr.verify_session(&bytes, &sig1).is_ok());

    let block = sr.rekey(dummy.clone());
    assert!(vr.rekey(&block).is_ok());

    let bytes = [0,1,2,3];
    let sig2 = sr.sign_session(&bytes);
    assert!(vr.verify_session(&bytes, &sig2).is_ok());
    assert!(vr.verify_session(&bytes, &sig1).is_err());
}

#[test]
fn session_alert_rekey() {
    let (pk, sk) = crypto::gen_keypair();
    let dummy = BlockPointer::from(None);

    let mut sr = SignRing::new(pk.clone(), sk);
    let mut vr = VerifyRing::new(pk);

    let pk = sr.init();
    vr.init(pk).unwrap();

    let bytes = [0,1,2,3];
    let sig1 = sr.sign_session(&bytes);
    assert!(vr.verify_session(&bytes, &sig1).is_ok());

    let block = sr.alert(dummy.clone(), "ohai".as_bytes().to_vec());
    assert!(vr.alert_rekey(&block).is_ok());

    let bytes = [0,1,2,3];
    let sig2 = sr.sign_session(&bytes);
    assert!(vr.verify_session(&bytes, &sig2).is_ok());
    assert!(vr.verify_session(&bytes, &sig1).is_err());
}
