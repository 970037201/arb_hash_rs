#[cfg(test)]
mod tests;

use crate::block::AHBlock;

use std::thread::{spawn, JoinHandle};

impl<const LEN: usize> AHBlock<LEN> {
    /// Compute digest of some input block slice, in parallel with some number of threads
    #[inline(always)]
    pub fn parallel_arb_block_digest<const RND: u64>(input: &[Self], threads: usize) -> Self {
        let mut output = Self::new();
        let workload_len = input.len() / threads + 1;
        let mut thread_handles: Vec<JoinHandle<Self>> = input
            .chunks(workload_len)
            .enumerate()
            .map(|(i, chunk)| {
                let section_blocks: Vec<Self> = chunk.to_owned();
                let offset = workload_len * i;
                spawn(move || Self::serial_arb_block_digest::<RND>(&section_blocks, offset))
            })
            .collect();
        while let Some(worker) = thread_handles.pop() {
            output.xor_block_assign(&worker.join().unwrap());
        }
        output
    }
}
