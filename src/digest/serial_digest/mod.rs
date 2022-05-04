use crate::{
    block::{inc_block, resize_block, xor_blocks},
    hash::arb_hash,
};

#[no_mangle]
pub fn serial_arb_digest(
    padded_input: &[u8],
    offset: usize,
    length: usize,
    rounds: u64,
) -> Vec<u8> {
    let mut result_block = vec![0u8; length];
    let mut chunk_hash_temp = vec![0u8; length];
    let mut second_hash_temp = vec![0u8; length];
    let mut ctr_block = resize_block(&offset.to_le_bytes(), length);
    for chunk in padded_input.chunks_exact(length) {
        arb_hash(chunk, &mut chunk_hash_temp, rounds);
        xor_blocks(&mut chunk_hash_temp, &ctr_block);
        arb_hash(&chunk_hash_temp, &mut second_hash_temp, rounds);
        xor_blocks(&mut result_block, &second_hash_temp);
        inc_block(&mut ctr_block);
    }
    result_block
}
