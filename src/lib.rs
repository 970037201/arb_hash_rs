// Helper functions
#[cfg(feature = "block")]
pub mod block;
// Fixed size digest of byte arrays
#[cfg(feature = "digest")]
pub mod digest;
// Same-length hash of byte arrays
#[cfg(feature = "hash")]
pub mod hash;

#[cfg(test)]
mod tests {
    use crate::hash::arb_hash;

    #[test]
    fn arb_hash_test() {
        const RND: u64 = 3;
        const LEN: usize = 8;
        const INPUT: [u8; 8] = [0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        const EXPECTED: [u8; 8] = [0xF2, 0x04, 0xF7, 0x45, 0xC2, 0x0D, 0xEA, 0xEA];
        let output = arb_hash::<RND, LEN>(&INPUT);
        assert_eq!(output, EXPECTED);
    }
}
