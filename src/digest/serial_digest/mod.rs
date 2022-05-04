use crate::{
    block::{inc_block, resize_block, xor_blocks},
    hash::arb_hash,
};

// Serial method of computing a digest of some list of blocks
// basically hashes the input, XORs on the block number,
// hashes that, then XORs it onto the result (which is initalized to 0)
#[inline(always)]
pub fn serial_arb_digest<const RND: u64, const LEN: usize>(
    blocks: &[[u8; LEN]],
    output: &mut [u8; LEN],
    offset: usize,
) {
    output.iter_mut().for_each(|elem| *elem = 0);
    let mut chunk_hash_temp = [0u8; LEN];
    let mut second_hash_temp = [0u8; LEN];
    let mut ctr_block = resize_block(&offset.to_le_bytes());
    blocks.iter().for_each(|block| {
        arb_hash::<RND, LEN>(block, &mut chunk_hash_temp);
        xor_blocks(&mut chunk_hash_temp, &ctr_block);
        arb_hash::<RND, LEN>(&chunk_hash_temp, &mut second_hash_temp);
        xor_blocks(output, &second_hash_temp);
        inc_block(&mut ctr_block);
    });
}
