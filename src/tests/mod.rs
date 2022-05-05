use crate::{
    digest::{arb_digest, arb_digest_const, arb_digest_parallel},
    hash::arb_hash,
};

#[test]
fn arb_hash_test() {
    const RND: u64 = 3;
    const LEN: usize = 8;
    const INPUT: [u8; 8] = [0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    const EXPECTED: [u8; 8] = [0xF2, 0x04, 0xF7, 0x45, 0xC2, 0x0D, 0xEA, 0xEA];
    const OUTPUT: [u8; 8] = arb_hash::<RND, LEN>(&INPUT);
    assert_eq!(OUTPUT, EXPECTED);
}

const RND: u64 = 4;
const LEN: usize = 6;
const EXPECTED: [u8; LEN] = [0x26, 0xBE, 0xAB, 0x55, 0x01, 0x17];
const INPUT: [u8; 17] = [
    0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00,
];

#[test]
fn const_digest_test() {
    const LEN2: usize = 3;
    const PARALLEL_INPUT: [[u8; LEN]; LEN2] = [
        [0x01, 0x00, 0x00, 0x00, 0x00, 0x00],
        [0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        [0x00, 0x00, 0x00, 0x00, 0x00, 0x80],
    ];
    let output: [u8; LEN] = arb_digest_const::<RND, LEN, LEN2>(&PARALLEL_INPUT);
    assert_eq!(output, EXPECTED);
}

#[test]
fn parallel_digest_test() {
    const THREADS: usize = 3;
    let output: [u8; LEN] = arb_digest_parallel::<RND, LEN>(&INPUT, THREADS);
    assert_eq!(output, EXPECTED);
}

#[test]
fn digest_test() {
    let output: [u8; LEN] = arb_digest::<RND, LEN>(&INPUT);
    assert_eq!(output, EXPECTED);
}
