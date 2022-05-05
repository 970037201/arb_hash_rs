use crate::block::AHBlock;

#[test]
fn parallel_digest_test() {
    const THREADS: usize = 3;
    const RND: u64 = 4;
    const LEN: usize = 6;
    const EXPECTED: AHBlock<LEN> = AHBlock::from_slice(&[0x26, 0xBE, 0xAB, 0x55, 0x01, 0x17]);
    const INPUT: [u8; 17] = [
        0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00,
    ];
    let output = AHBlock::arb_digest_parallel::<RND>(&INPUT, THREADS);
    assert_eq!(output, EXPECTED);
}

#[test]
fn empty_digest_test() {
    const THREADS: usize = 2;
    const RND: u64 = 3;
    const INPUT: [u8; 0] = [];
    const EXPECTED: [u8; 7] = [84, 116, 49, 0, 37, 201, 27];
    let output = AHBlock::<7>::arb_digest_parallel::<RND>(&INPUT, THREADS);
    assert_eq!(output.data, EXPECTED);
}
