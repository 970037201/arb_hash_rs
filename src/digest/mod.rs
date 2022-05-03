use crate::{
    block::{pad_block, resize_block, xor_blocks},
    hash::arb_hash,
};

//Returns the digest of an input byte array
// - input: some input byte array, could be the contents of a file, for example
// - length: don't use under 3 bytes, as the output is not well distributed
// - rounds: 1 round is sufficient, but 2 rounds *might* be better distributed

#[cfg(not(feature = "parallel"))]
#[inline(always)]
pub fn arb_digest(input: &[u8], length: usize, rounds: u64) -> Vec<u8> {
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

#[cfg(feature = "parallel")]
#[inline(always)]
pub fn arb_digest(input: &[u8], length: usize, rounds: u64) -> Vec<u8> {
    use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
    use std::sync::RwLock;
    
    let r_block_arc = RwLock::new(vec![0u8; length]);
    let padded_input = pad_block(input, length);
    let chunks: Vec<&[u8]> = padded_input.chunks_exact(length).collect();
    chunks.into_par_iter().enumerate().for_each(|(ctr, chunk)| {
        let chunk_hash = arb_hash(chunk, rounds);
        let ctr_block = resize_block(&ctr.to_le_bytes(), length);
        let combined_hash = xor_blocks(&chunk_hash, &ctr_block);
        let second_hash = arb_hash(&combined_hash, rounds);
        match r_block_arc.write() {
            Ok(mut lock) => *lock = xor_blocks(&second_hash, &*lock),
            _ => panic!(),
        }
    });
    r_block_arc.into_inner().unwrap()
}
