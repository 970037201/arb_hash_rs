use crate::block::AHBlock;

const RND: u64 = 4;
const LEN: usize = 6;
const EXPECTED: AHBlock<LEN> = AHBlock::from_slice(&[0x26, 0xBE, 0xAB, 0x55, 0x01, 0x17]);
const INPUT: [u8; 17] = [
    0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00,
];

#[test]
fn parallel_digest_test() {
    const THREADS: usize = 3;
    let output = AHBlock::arb_digest_parallel::<RND>(&INPUT, THREADS);
    assert_eq!(output, EXPECTED);
}
