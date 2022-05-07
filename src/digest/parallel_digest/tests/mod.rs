use crate::block::AHBlock;

#[test]
fn parallel_digest_test() {
    const THREADS: usize = 3;
    const RND: u64 = 4;
    const LEN: usize = 6;
    const EXPECTED: AHBlock<LEN> = AHBlock::from_slice(&[179, 94, 129, 41, 94, 28]);
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
    const EXPECTED: [u8; 7] = [188, 65, 203, 193, 109, 177, 233];
    let output = AHBlock::<7>::arb_digest_parallel::<RND>(&INPUT, THREADS);
    assert_eq!(output.data, EXPECTED);
}
