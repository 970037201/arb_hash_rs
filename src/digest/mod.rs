//Returns the digest of an input byte array
// - input: some input byte array, could be the contents of a file, for example
// - length: don't use under 3 bytes, as the output is not well distributed
// - rounds: 1 round is sufficient, but 2 rounds *might* be better distributed
// RND: The number of rounds for each hash during the digest (> 0 for a sane hash)
// LEN: The length of blocks in bytes (>= 3 for a sane hash)

mod serial_digest;

#[cfg(feature = "parallel")]
mod parallel_digest;

use crate::block::pad_input;

#[cfg(not(feature = "parallel"))]
use serial_digest::serial_arb_digest;

#[cfg(feature = "parallel")]
use parallel_digest::parallel_arb_digest;

#[inline(always)]
pub fn arb_digest<const RND: u64, const LEN: usize>(input: &[u8]) -> [u8; LEN] {
    let padded = pad_input(input);
    let mut output = [0u8; LEN];

    #[cfg(feature = "parallel")]
    parallel_arb_digest::<RND, LEN>(&padded, &mut output);
    #[cfg(not(feature = "parallel"))]
    serial_arb_digest::<RND, LEN>(&padded, &mut output, 0);

    output
}
