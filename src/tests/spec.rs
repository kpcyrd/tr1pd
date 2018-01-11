use spec::{Spec, SpecPointer};
use blocks::BlockPointer;


const DEFAULT_SLICE_29D9: &[u8] = &[
    41, 217, 230, 186, 47, 116, 133, 90,
    64, 236, 173, 19, 184, 80, 108, 241,
    71, 11, 219, 246, 4, 32, 157, 45,
    102, 150, 232, 93, 152, 250, 186, 134,
];
const DEFAULT_SLICE_3E15: &[u8] = &[
    62, 21, 147, 107, 102, 80, 216, 141,
    69, 141, 97, 125, 195, 216, 38, 134,
    107, 198, 222, 69, 117, 175, 191, 206,
    167, 172, 155, 211, 137, 123, 7, 35,
];


#[test]
fn test_pointer() {
    let spec = Spec::parse("29d9e6ba2f74855a40ecad13b8506cf1470bdbf604209d2d6696e85d98faba86").unwrap();
    assert_eq!(spec, Spec::Pointer(SpecPointer::Block(BlockPointer::from_slice(DEFAULT_SLICE_29D9).unwrap())));
}

#[test]
fn test_range() {
    let spec = Spec::parse("3e15936b6650d88d458d617dc3d826866bc6de4575afbfcea7ac9bd3897b0723..29d9e6ba2f74855a40ecad13b8506cf1470bdbf604209d2d6696e85d98faba86").unwrap();

    assert_eq!(spec, Spec::Range((
        SpecPointer::Block(BlockPointer::from_slice(DEFAULT_SLICE_3E15).unwrap()),
        SpecPointer::Block(BlockPointer::from_slice(DEFAULT_SLICE_29D9).unwrap()),
    )));
}

#[test]
fn test_parent() {
    let spec = Spec::parse("29d9e6ba2f74855a40ecad13b8506cf1470bdbf604209d2d6696e85d98faba86^^^").unwrap();

    assert_eq!(spec, Spec::Pointer(SpecPointer::Parent((Box::new(
        SpecPointer::Block(BlockPointer::from_slice(DEFAULT_SLICE_29D9).unwrap())
    ), 3))));
}

#[test]
fn test_session() {
    let spec = Spec::parse("@29d9e6ba2f74855a40ecad13b8506cf1470bdbf604209d2d6696e85d98faba86").unwrap();

    assert_eq!(spec, Spec::Pointer(
        SpecPointer::Session(Box::new(
            SpecPointer::Block(BlockPointer::from_slice(DEFAULT_SLICE_29D9).unwrap())
        ))
    ));
}

// TODO: session parent!?

#[test]
fn test_head() {
    let spec = Spec::parse("HEAD").unwrap();
    assert_eq!(spec, Spec::Pointer(SpecPointer::Head));
}

#[test]
fn test_range_since() {
    let spec = Spec::parse("29d9e6ba2f74855a40ecad13b8506cf1470bdbf604209d2d6696e85d98faba86..").unwrap();

    assert_eq!(spec, Spec::Range((
        SpecPointer::Block(BlockPointer::from_slice(DEFAULT_SLICE_29D9).unwrap()),
        SpecPointer::Head,
    )));
}

#[test]
fn test_pointer_this_session() {
    let spec = Spec::parse("@").unwrap();
    assert_eq!(spec, Spec::Pointer(SpecPointer::Session(Box::new(SpecPointer::Head))));
}

#[test]
fn test_range_this_session() {
    let spec = Spec::parse("@..").unwrap();
    assert_eq!(spec, Spec::Range((
        SpecPointer::Session(Box::new(SpecPointer::Head)),
        SpecPointer::Head,
    )));
}
