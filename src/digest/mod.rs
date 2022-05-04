//Returns the digest of an input byte array
// - input: some input byte array, could be the contents of a file, for example
// - length: don't use under 3 bytes, as the output is not well distributed
// - rounds: 1 round is sufficient, but 2 rounds *might* be better distributed

mod serial_digest;

#[cfg(feature = "parallel")]
mod parallel_digest;

use crate::block::pad_input;

#[cfg(not(feature = "parallel"))]
use serial_digest::serial_arb_digest;

#[cfg(feature = "parallel")]
use parallel_digest::parallel_arb_digest;

#[inline(always)]
pub fn arb_digest<const LEN: usize, const RND: u64>(input: &[u8]) -> [u8; LEN] {
    let padded = pad_input(input);
    let mut output = [0u8; LEN];

    #[cfg(feature = "parallel")]
    parallel_arb_digest::<LEN, RND>(&padded, &mut output);
    #[cfg(not(feature = "parallel"))]
    serial_arb_digest::<LEN, RND>(&padded, &mut output, 0);

    output
}
