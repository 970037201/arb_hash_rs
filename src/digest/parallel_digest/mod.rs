use crate::block::{pad_block, xor_blocks};

use std::{
    sync::{Arc, RwLock},
    thread::{spawn, JoinHandle},
};

use super::serial_digest::serial_arb_digest;

#[no_mangle]
pub fn parallel_arb_digest(input: &[u8], length: usize, rounds: u64) -> Vec<u8> {
    let result_block = Arc::new(RwLock::new(vec![0u8; length]));
    let padded_input = pad_block(input, length);
    let chunks: Vec<&[u8]> = padded_input.chunks_exact(length).collect();
    let workload_len = chunks.len() / 4; //num_cpus::get();
    let mut workers: Vec<JoinHandle<()>> = chunks
        .chunks(workload_len)
        .enumerate()
        .map(|(i, chunk)| {
            let offset = workload_len * i;
            let part_input = chunk.concat();
            let result_clone = result_block.clone();
            spawn(move || {
                let part_block = serial_arb_digest(&part_input, offset, length, rounds);
                match result_clone.write() {
                    Ok(mut lock) => xor_blocks(&mut lock, &part_block),
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
