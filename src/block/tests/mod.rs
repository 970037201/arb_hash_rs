use crate::block::AHBlock;

#[test]
fn new_test() {
    const NEW_BLOCK: AHBlock<4> = AHBlock::new();
    assert_eq!(NEW_BLOCK.data, [0, 0, 0, 0]);
}
