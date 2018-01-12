use tests::mocks::storage::MockStorage;

use blocks::BlockPointer;
use crypto;
use crypto::ring::SignRing;
use engine::Engine;
use storage::{MemoryStorage, BlockStorage};
use spec::Spec;

use pseudo::Mock;


const DEFAULT_SLICE_29D9: &[u8] = &[
    41, 217, 230, 186, 47, 116, 133, 90,
    64, 236, 173, 19, 184, 80, 108, 241,
    71, 11, 219, 246, 4, 32, 157, 45,
    102, 150, 232, 93, 152, 250, 186, 134,
];

#[test]
fn test_spec_get_head() {
    let spec = Spec::parse("HEAD").unwrap();
    let spec = spec.pointer().unwrap();

    let mut storage = MockStorage::new();
    storage.get_head = Mock::new(Ok(BlockPointer::from_slice(DEFAULT_SLICE_29D9).unwrap()));
    let pointer = storage.resolve_pointer(spec).unwrap();

    assert_eq!(pointer, BlockPointer::from_slice(DEFAULT_SLICE_29D9).unwrap());
}

#[test]
fn test_spec_get_block() {
    let spec = Spec::parse("29d9e6ba2f74855a40ecad13b8506cf1470bdbf604209d2d6696e85d98faba86").unwrap();
    let spec = spec.pointer().unwrap();

    let storage = MockStorage::new();
    let pointer = storage.resolve_pointer(spec).unwrap();

    assert_eq!(pointer, BlockPointer::from_slice(DEFAULT_SLICE_29D9).unwrap());
}

#[test]
fn test_spec_get_parent() {
    let spec = Spec::parse("HEAD^^^").unwrap();
    let spec = spec.pointer().unwrap();

    let (pk, sk) = crypto::gen_keypair();
    let ring = SignRing::new(pk, sk);
    let storage = MemoryStorage::new().to_engine();
    let mut engine = Engine::start(storage, ring).unwrap();
    let init = engine.storage().get_head().unwrap();
    engine.rekey().unwrap();
    engine.rekey().unwrap();
    engine.rekey().unwrap();

    let storage = engine.storage();
    let pointer = storage.resolve_pointer(spec).unwrap();

    assert_eq!(pointer, init);
}

#[test]
fn test_spec_get_session() {
    let spec = Spec::parse("@HEAD").unwrap();
    let spec = spec.pointer().unwrap();

    let (pk, sk) = crypto::gen_keypair();
    let ring = SignRing::new(pk, sk);
    let storage = MemoryStorage::new().to_engine();
    let mut engine = Engine::start(storage, ring).unwrap();
    let init = engine.storage().get_head().unwrap();
    engine.rekey().unwrap();
    engine.rekey().unwrap();
    engine.rekey().unwrap();
    engine.rekey().unwrap();

    let storage = engine.storage();
    let pointer = storage.resolve_pointer(spec).unwrap();

    assert_eq!(pointer, init);
}

#[test]
fn test_spec_expand_range() {
    let spec = Spec::parse("@HEAD..").unwrap();
    let spec = spec.range().unwrap();

    let (pk, sk) = crypto::gen_keypair();
    let ring = SignRing::new(pk, sk);
    let storage = MemoryStorage::new().to_engine();

    let mut pointers = Vec::new();

    let mut engine = Engine::start(storage, ring).unwrap();
    pointers.push(engine.storage().get_head().unwrap());
    engine.rekey().unwrap();
    pointers.push(engine.storage().get_head().unwrap());
    engine.rekey().unwrap();
    pointers.push(engine.storage().get_head().unwrap());
    engine.rekey().unwrap();
    pointers.push(engine.storage().get_head().unwrap());
    engine.rekey().unwrap();
    pointers.push(engine.storage().get_head().unwrap());

    let storage = engine.storage();
    let range = storage.resolve_range(spec).unwrap();
    let expanded_pointers = storage.expand_range(range).unwrap();

    assert_eq!(expanded_pointers, pointers);
}
