use crate::{
    block::{pad_block, resize_block, xor_blocks},
    hash::arb_hash,
};

use std::{
    sync::{Arc, RwLock},
    thread::{spawn, JoinHandle},
};

#[inline(always)]
pub fn parallel_arb_digest(input: &[u8], length: usize, rounds: u64) -> Vec<u8> {
    let result_block = Arc::new(RwLock::new(vec![0u8; length]));
    let padded_input = pad_block(input, length);
    let chunks: Vec<&[u8]> = padded_input.chunks_exact(length).collect();
    let workload_len = chunks.len() / num_cpus::get();
    let mut workers: Vec<JoinHandle<()>> = chunks
        .chunks(workload_len)
        .enumerate()
        .map(|(i, chunk)| {
            let offset = workload_len * i;
            let part_input = chunk.concat();
            let result_clone = result_block.clone();
            spawn(move || {
                let part_block = parallel_arb_digest_part(&part_input, offset, length, rounds);
                match result_clone.write() {
                    Ok(mut lock) => *lock = xor_blocks(&part_block, &lock),
                    _ => panic!(),
                }
            })
        })
        .collect();
    while let Some(worker) = workers.pop() {
        worker.join().unwrap();
    }
    let result = result_block.read().unwrap();
    result.clone()
}

pub fn parallel_arb_digest_part(
    chunks: &[u8],
    offset: usize,
    length: usize,
    rounds: u64,
) -> Vec<u8> {
    let mut result_block = vec![0u8; length];
    for (chunk_ctr, chunk) in chunks.chunks_exact(length).enumerate() {
        let chunk_hash = arb_hash(chunk, rounds);
        let ctr = (chunk_ctr + offset).to_le_bytes();
        let ctr_block = resize_block(&ctr, length);
        let combined_hash = xor_blocks(&chunk_hash, &ctr_block);
        let second_hash = arb_hash(&combined_hash, rounds);
        result_block = xor_blocks(&second_hash, &result_block);
    }
    result_block
}
