use crate::block::AHBlock;

const RND: u64 = 4;
const LEN: usize = 6;
const EXPECTED: AHBlock<LEN> = AHBlock::from_slice(&[38, 190, 171, 85, 1, 23]);

#[test]
fn const_digest_test() {
    const LEN2: usize = 3;
    const PARALLEL_INPUT: [AHBlock<LEN>; LEN2] = [
        AHBlock::from_slice(&[0x01, 0x00, 0x00, 0x00, 0x00, 0x00]),
        AHBlock::from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
        AHBlock::from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x80]),
    ];
    let output = AHBlock::serial_arb_block_digest::<RND>(&PARALLEL_INPUT, 0);
    assert_eq!(output, EXPECTED);
}
