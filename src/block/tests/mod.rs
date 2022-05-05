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

#[test]
fn xor_block_assign_test() {
    const ARRAY_A: [u8; 6] = [5, 14, 23, 32, 41, 50];
    const ARRAY_B: [u8; 6] = [71, 69, 67, 65, 63, 61];
    const EXPECTED: [u8; 6] = [66, 75, 84, 97, 22, 15];
    const XOR_BLOCK: AHBlock<6> = AHBlock::from_slice(&ARRAY_A);
    let mut input: AHBlock<6>  = AHBlock::from_slice(&ARRAY_B);
    input.xor_block_assign(&XOR_BLOCK);
    assert_eq!(input.data, EXPECTED);
}

#[test]
fn xor_block() {
    const ARRAY_A: [u8; 3] = [0x10, 0xA0, 0x7E];
    const ARRAY_B: [u8; 3] = [0x99, 0x88, 0x77];
    const EXPECTED: [u8; 3] = [137, 40, 9];
    const BLOCK_A: AHBlock<3> = AHBlock::from_slice(&ARRAY_A);
    const BLOCK_B: AHBlock<3> = AHBlock::from_slice(&ARRAY_B);
    const XOR_RESULT: AHBlock<3> = BLOCK_A.xor_block(&BLOCK_B);
    assert_eq!(XOR_RESULT.data, EXPECTED);
}

#[test]
fn inc_block_assign_test() {
    const EXPECTED: [u8; 10] = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut original: AHBlock<10> = AHBlock::new();
    original.inc_block_assign();
    assert_eq!(original.data, EXPECTED);
}

#[test]
fn inc_block_test() {
    const ORIGIN_ARR: [u8; 2] = [0xFF, 1];
    const EXPECTED_ARR: [u8; 2] = [0, 2]; 
    const ORIGIN: AHBlock<2> = AHBlock::from_slice(&ORIGIN_ARR);
    const INCREMENTED: AHBlock<2> = ORIGIN.inc_block();
    assert_eq!(INCREMENTED.data, EXPECTED_ARR);
}