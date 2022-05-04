use crate::block::xor_blocks;

use std::thread::spawn;

use super::serial_digest::serial_arb_digest;

//Parallel digest computation, made by splitting the input into chunks
//for each thread, given by the number of CPUs in your system.

#[inline(always)]
pub fn parallel_arb_digest<const RND: u64, const LEN: usize>(
    input: &[[u8; LEN]],
    output: &mut [u8; LEN],
) {
    output.iter_mut().for_each(|elem| *elem = 0);
    let workload_len = input.len() / num_cpus::get();
    let mut thread_handles: Vec<_> = input
        .chunks(workload_len)
        .enumerate()
        .map(|(i, chunk)| {
            let section_blocks = chunk.to_owned();
            let offset = workload_len * i;
            spawn(move || {
                let mut output = [0u8; LEN];
                serial_arb_digest::<RND, LEN>(&section_blocks, &mut output, offset);
                output
            })
        })
        .collect();
    while let Some(worker) = thread_handles.pop() {
        xor_blocks(output, &worker.join().unwrap());
    }
}
