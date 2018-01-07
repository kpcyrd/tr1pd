mod ring;

use crypto;

#[test]
fn simple() {
    let (pk, sk) = crypto::gen_keypair();

    let msg = "ohai".as_bytes();
    let sig = crypto::sign(&msg, &sk);

    assert!(crypto::verify(&sig, &msg, &pk).is_ok());
}

#[test]
fn simple_fail_signature() {
    let (pk, sk) = crypto::gen_keypair();

    let msg = "ohai".as_bytes();
    let sig = crypto::sign(&msg, &sk);

    assert!(crypto::verify(&sig, "oh noes".as_bytes(), &pk).is_err());
}

#[test]
fn simple_fail_pubkey() {
    let (_, sk) = crypto::gen_keypair();

    let msg = "ohai".as_bytes();
    let sig = crypto::sign(&msg, &sk);

    let (pk, _) = crypto::gen_keypair();
    assert!(crypto::verify(&sig, &msg, &pk).is_err());
}
