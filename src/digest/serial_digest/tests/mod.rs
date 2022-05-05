use crate::block::AHBlock;

const RND: u64 = 4;
const LEN: usize = 6;
const EXPECTED: AHBlock<LEN> = AHBlock::from_slice(&[0x26, 0xBE, 0xAB, 0x55, 0x01, 0x17]);

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
