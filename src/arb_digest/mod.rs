use crate::{
    arb_block::{inc_block, xor_blocks},
    arb_hash::arb_hash,
};

pub fn arb_digest(input: &[u8], length: usize, rounds: u128) -> Vec<u8> {
    let mut result_block = vec![0u8; length];
    let mut counter_block = vec![0u8; length];
    for unpadded_chunk in input.chunks(length) {
        let mut chunk = Vec::from(unpadded_chunk);
        chunk.append(&mut vec![0u8; length - chunk.len()]);
        let chunk_hash = arb_hash(&chunk, rounds);
        let combined_hash = xor_blocks(&chunk_hash, &counter_block);
        let second_hash = arb_hash(&combined_hash, rounds);
        result_block = xor_blocks(&second_hash, &result_block);
        inc_block(&mut counter_block);
    }
    result_block
}
