use crate::{
    block::{const_resize_block, inc_block, xor_blocks},
    hash::arb_hash,
};

// Compile time constant, serial method of computing a digest of some list of blocks
// basically hashes the input, XORs on the block number,
// hashes that, then XORs it onto the result (which is initalized to 0)
#[inline(always)]
pub const fn serial_arb_digest<const RND: u64, const LEN: usize>(
    blocks: &[[u8; LEN]],
    offset: usize,
) -> [u8; LEN] {
    let mut output = [0u8; LEN];
    let mut ctr_block = const_resize_block(&offset.to_le_bytes());
    let mut i = 0;
    let block_count = blocks.len();
    while i < block_count {
        let chunk_hash = arb_hash::<RND, LEN>(&blocks[i]);
        let ctr_mix_hash = xor_blocks(&chunk_hash, &ctr_block);
        let second_hash = arb_hash::<RND, LEN>(&ctr_mix_hash);
        output = xor_blocks(&output, &second_hash);
        ctr_block = inc_block(&ctr_block);
        i += 1;
    }
    output
}
