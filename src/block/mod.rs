//Resizes a block, truncating or padding with zeros as needed
// - block: The block to pad
// LEN: the length of the new block in bytes
#[inline(always)]
pub fn resize_block<const LEN: usize>(block: &[u8]) -> [u8; LEN] {
    let mut output = [0u8; LEN];
    (0..block.len().min(LEN)).for_each(|i| output[i] = block[i]);
    output
}

//Returns array of blocks, zero padded for the last block
// - input: The input to pad
// LEN: the multiple of bytes to pad to, must NOT BE ZERO
#[inline(always)]
pub fn pad_input<const LEN: usize>(input: &[u8]) -> Vec<[u8; LEN]> {
    let block_cnt = (input.len() + LEN - 1) / LEN;
    let chunks = input.chunks_exact(LEN);
    let mut result_vec = Vec::with_capacity(block_cnt);
    let remainder = chunks.remainder();
    chunks.for_each(|chunk| result_vec.push(chunk.try_into().unwrap()));
    result_vec.push(resize_block(remainder));
    result_vec
}

//Assigning mod 2 addition on two blocks
// - block_a: The first block, which is the result
// - block_b: The second block
// LEN: the number of bytes in a block
#[inline(always)]
pub fn xor_blocks<const LEN: usize>(lhs: &mut [u8; LEN], rhs: &[u8; LEN]) {
    (0..LEN).for_each(|i| lhs[i] ^= rhs[i])
}

//Increment a block, 0th element least significant
// - block: The mutable byte array to increment
// LEN: the length of a block in bytes
#[inline(always)]
pub fn inc_block<const LEN: usize>(block: &mut [u8; LEN]) {
    for elem in block.iter_mut() {
        *elem = elem.wrapping_add(1);
        if *elem != 0 {
            return;
        }
    }
}
