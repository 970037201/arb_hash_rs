//Returns the digest of an input byte array
// - input: some input byte array, could be the contents of a file, for example
// - length: don't use under 3 bytes, as the output is not well distributed
// - rounds: 1 round is sufficient, but 2 rounds *might* be better distributed
// RND: The number of rounds for each hash during the digest (> 0 for a sane hash)
// LEN: The length of blocks in bytes (>= 3 for a sane hash)

mod serial_digest;
use self::serial_digest::serial_arb_digest;
use crate::block::{pad_input, AHBlock};

#[cfg(feature = "parallel")]
mod parallel_digest;
#[cfg(feature = "parallel")]
use parallel_digest::parallel_arb_digest;

// Parallel computation of digest of byte array
// - input: byte array to digest
// - threads: number of threads to use (crate num_cpus could help with this)
// RND: rounds to use in the hashing
// LEN: length of the output block in bytes
#[cfg(feature = "parallel")]
#[inline(always)]
pub fn arb_digest_parallel<const RND: u64, const LEN: usize>(
    input: &[u8],
    threads: usize,
) -> AHBlock<LEN> {
    let padded = pad_input(input);
    parallel_arb_digest::<RND, LEN>(&padded, threads)
}

// Runtime function for digest computation, running in a single thread
// - input: byte array to digest
// RND: rounds to use in the hashing
// LEN: length of the output block in bytes
#[inline(always)]
pub fn arb_digest<const RND: u64, const LEN: usize>(input: &[u8]) -> AHBlock<LEN> {
    let padded = pad_input(input);
    serial_arb_digest::<RND, LEN>(&padded, 0)
}

// Const funtion for digest computation, running during compile time
// - blocks: array of array of bytes to string together and form digest of
// RND: rounds to use in the hashing
// LEN: length of the output block in bytes, and the length of each block in the array "blocks"
// LEN2: number of blocks in the array "blocks"
#[inline(always)]
pub const fn arb_digest_const<const RND: u64, const LEN: usize, const LEN2: usize>(
    blocks: &[AHBlock<LEN>; LEN2],
) -> AHBlock<LEN> {
    serial_arb_digest::<RND, LEN>(blocks, 0)
}
