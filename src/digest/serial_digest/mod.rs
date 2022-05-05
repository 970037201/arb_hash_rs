#[cfg(test)]
mod tests;

use crate::block::AHBlock;

impl<const LEN: usize> AHBlock<LEN> {
    /// Compute digest of array of blocks, at compile time if possible
    #[inline(always)]
    pub const fn serial_arb_block_digest<const RND: u64>(blocks: &[Self], offset: usize) -> Self {
        let mut output = Self::new();
        let ctr_bytes = &offset.to_le_bytes();
        let mut ctr_block = Self::from_slice(ctr_bytes);
        let (mut i, block_count) = (0, blocks.len());
        while i < block_count {
            let mut chunk_hash = blocks[i].arb_hash::<RND>();
            chunk_hash = chunk_hash.xor_block(&ctr_block);
            chunk_hash = chunk_hash.arb_hash::<RND>();
            output = output.xor_block(&chunk_hash);
            ctr_block = ctr_block.inc_block();
            i += 1;
        }
        output
    }
}
