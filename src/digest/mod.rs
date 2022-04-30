use crate::{
    block::{inc_block, pad_block, xor_blocks},
    hash::arb_hash,
};

//Returns the digest of an input byte array
// - input: some input byte array, could be the contents of a file, for example
// - length: don't use under 3 bytes, as the output is not well distributed
// - rounds: 1 round is sufficient, but 2 rounds *might* be better distributed
pub fn arb_digest(input: &[u8], length: usize, rounds: u64) -> Vec<u8> {
    let mut result_block = vec![0u8; length];
    let mut counter_block = vec![0u8; length];
    for unpadded_chunk in input.chunks(length) {
        let chunk = pad_block(unpadded_chunk, length);
        let chunk_hash = arb_hash(&chunk, rounds);
        let combined_hash = xor_blocks(&chunk_hash, &counter_block);
        let second_hash = arb_hash(&combined_hash, rounds);
        result_block = xor_blocks(&second_hash, &result_block);
        inc_block(&mut counter_block);
    }
    result_block
}
