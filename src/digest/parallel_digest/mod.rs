use crate::block::xor_blocks;

use std::thread::spawn;

use super::serial_digest::serial_arb_digest;

//Parallel digest computation, made by splitting the input into chunks
//for each thread, given by the number of CPUs in your system.

#[inline(always)]
pub fn parallel_arb_digest<const RND: u64, const LEN: usize>(
    input: &[[u8; LEN]],
    threads: usize,
) -> [u8; LEN] {
    // SAFETY: All elements are initialized and never read from before initialization
    let mut output = [0u8; LEN];
    let workload_len = input.len() / threads;
    let mut thread_handles: Vec<_> = input
        .chunks(workload_len)
        .enumerate()
        .map(|(i, chunk)| {
            let section_blocks = chunk.to_owned();
            let offset = workload_len * i;
            spawn(move || serial_arb_digest::<RND, LEN>(&section_blocks, offset))
        })
        .collect();
    while let Some(worker) = thread_handles.pop() {
        output = xor_blocks(&output, &worker.join().unwrap());
    }
    output
}
