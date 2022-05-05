use crate::{
    block::AHBlock,
    digest::{arb_digest, arb_digest_const, arb_digest_parallel},
    hash::arb_hash,
};

#[test]
fn arb_hash_test() {
    const RND: u64 = 3;
    const LEN: usize = 8;
    const INPUT: AHBlock<LEN> = [0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    const EXPECTED: AHBlock<LEN> = [0xF2, 0x04, 0xF7, 0x45, 0xC2, 0x0D, 0xEA, 0xEA];
    const OUTPUT: AHBlock<LEN> = arb_hash::<RND, LEN>(&INPUT);
    assert_eq!(OUTPUT, EXPECTED);
}

const RND: u64 = 4;
const LEN: usize = 6;
const EXPECTED: AHBlock<LEN> = [0x26, 0xBE, 0xAB, 0x55, 0x01, 0x17];
const INPUT: AHBlock<17> = [
    0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00,
];

#[test]
fn const_digest_test() {
    const LEN2: usize = 3;
    const PARALLEL_INPUT: [AHBlock<LEN>; LEN2] = [
        [0x01, 0x00, 0x00, 0x00, 0x00, 0x00],
        [0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        [0x00, 0x00, 0x00, 0x00, 0x00, 0x80],
    ];
    let output: AHBlock<LEN> = arb_digest_const::<RND, LEN, LEN2>(&PARALLEL_INPUT);
    assert_eq!(output, EXPECTED);
}

#[test]
fn parallel_digest_test() {
    const THREADS: usize = 3;
    let output: AHBlock<LEN> = arb_digest_parallel::<RND, LEN>(&INPUT, THREADS);
    assert_eq!(output, EXPECTED);
}

#[test]
fn digest_test() {
    let output: AHBlock<LEN> = arb_digest::<RND, LEN>(&INPUT);
    assert_eq!(output, EXPECTED);
}
