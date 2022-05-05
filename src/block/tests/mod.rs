use crate::block::AHBlock;

#[test]
fn new_test() {
    const NEW_BLOCK: AHBlock<4> = AHBlock::new();
    assert_eq!(NEW_BLOCK.data, [0, 0, 0, 0]);
}

#[test]
fn from_slice_test() {
    const SLICE: &[u8; 5] = &[1, 2, 3, 4, 5];
    const NEW_BLOCK: AHBlock<5> = AHBlock::from_slice(SLICE);
    const EXPECTED: [u8; 5] = [1, 2, 3, 4, 5];
    assert_eq!(NEW_BLOCK.data, EXPECTED);
}

#[test]
fn from_block_test() {
    const SLICE: &[u8; 7] = &[0, 6, 1, 5, 2, 4, 3];
    const BLOCK: AHBlock<7> = AHBlock::from_slice(SLICE);
    const NEW_BLOCK: AHBlock<7> = AHBlock::from_block(&BLOCK);
    assert_eq!(NEW_BLOCK, BLOCK);
}