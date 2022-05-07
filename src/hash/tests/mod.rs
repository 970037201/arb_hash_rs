use crate::block::AHBlock;

#[test]
fn arb_hash_test() {
    const RND: u64 = 3;
    const LEN: usize = 8;
    const INPUT: AHBlock<LEN> =
        AHBlock::from_slice(&[0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    const EXPECTED: AHBlock<LEN> =
        AHBlock::from_slice(&[0xF2, 0x04, 0xF7, 0x45, 0xC2, 0x0D, 0xEA, 0xEA]);
    let output: AHBlock<LEN> = INPUT.arb_hash::<RND>();
    assert_eq!(output, EXPECTED);
}
