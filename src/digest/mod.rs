mod serial_digest;
use crate::block::{pad_to_blocks, AHBlock};

#[cfg(feature = "parallel")]
mod parallel_digest;

impl<const LEN: usize> AHBlock<LEN> {
    /// Compute digest of some input byte slice, in parallel with some number of threads
    #[cfg(feature = "parallel")]
    #[inline(always)]
    pub fn arb_digest_parallel<const RND: u64>(input: &[u8], threads: usize) -> Self {
        let padded_input = pad_to_blocks(input);
        AHBlock::parallel_arb_block_digest::<RND>(&padded_input, threads)
    }

    /// Single threaded computation of digest of byte slice
    #[inline(always)]
    pub fn arb_digest<const RND: u64>(input: &[u8]) -> Self {
        let padded = pad_to_blocks(input);
        AHBlock::serial_arb_block_digest::<RND>(&padded, 0)
    }
}
