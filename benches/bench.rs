#![feature(test)]
extern crate test;
extern crate tr1pd;

#[cfg(test)]
mod tests {
    use test::Bencher;
    use tr1pd::blocks::Block;
    use tr1pd::crypto;
    use tr1pd::crypto::ring::SignRing;

    #[bench]
    fn bench_empty(b: &mut Bencher) {
        let (pk, sk) = crypto::gen_keypair();
        let mut sm = SignRing::new(pk, sk);

        let pointer = None;
        b.iter(|| {
            let _dummy = Block::init(pointer.into(), &mut sm).unwrap();
        });
    }

    #[bench]
    fn bench_info_1k(b: &mut Bencher) {
        let (pk, sk) = crypto::gen_keypair();
        let mut sm = SignRing::new(pk, sk);

        let pointer = None;
        let dummy = Block::init(pointer.into(), &mut sm).unwrap();

        let bytes = vec![0; 1024];
        b.iter(|| {
            let _block = Block::info(dummy.prev().clone(), &mut sm, bytes.clone()).unwrap();
        });
    }

    #[bench]
    fn bench_info_1m(b: &mut Bencher) {
        let (pk, sk) = crypto::gen_keypair();
        let mut sm = SignRing::new(pk, sk);

        let pointer = None;
        let dummy = Block::init(pointer.into(), &mut sm).unwrap();

        let bytes = vec![0; 1024*1024];
        b.iter(|| {
            let _block = Block::info(dummy.prev().clone(), &mut sm, bytes.clone()).unwrap();
        });
    }
}
