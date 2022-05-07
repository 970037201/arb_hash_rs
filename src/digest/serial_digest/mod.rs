#[cfg(test)]
mod tests;

use crate::block::AHBlock;

impl<const LEN: usize> AHBlock<LEN> {
    /// Compute digest of array of blocks, at compile time if possible
    #[inline(always)]
    pub fn serial_arb_block_digest<const RND: u64>(blocks: &[Self], offset: usize) -> Self {
        let mut output = Self::new();
        let ctr_bytes = &offset.to_le_bytes();
        let mut ctr_block = Self::from_slice(ctr_bytes);
        let (mut i, block_count) = (0, blocks.len());
        let mut result;
        while i < block_count {
            result = blocks[i];
            result.arb_hash_assign::<RND>();
            result.xor_block_assign(&ctr_block);
            result.arb_hash_assign::<RND>();
            output.xor_block_assign(&result);
            ctr_block.inc_block_assign();
            i += 1;
        }
        output
    }
}
