use crate::{
    block::{pad_block, resize_block, xor_blocks},
    hash::arb_hash,
};

#[inline(always)]
pub fn serial_arb_digest(input: &[u8], length: usize, rounds: u64) -> Vec<u8> {
    let mut result_block = vec![0u8; length];
    let padded_input = pad_block(input, length);
    for (ctr, chunk) in padded_input.chunks_exact(length).enumerate() {
        let chunk_hash = arb_hash(&chunk, rounds);
        let ctr_block = resize_block(&ctr.to_le_bytes(), length);
        let combined_hash = xor_blocks(&chunk_hash, &ctr_block);
        let second_hash = arb_hash(&combined_hash, rounds);
        result_block = xor_blocks(&second_hash, &result_block);
    }
    result_block
}
