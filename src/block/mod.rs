//Resizes a block, truncating or padding with zeros as needed
// - block: The block to pad
// - count: The number of bytes to resize to
#[no_mangle]
pub fn resize_block(block: &[u8], count: usize) -> Vec<u8> {
    let mut result = block.to_vec();
    result.resize(count, 0);
    result
}

//Returns zero padded block to a multiple of bytes
// - block: The block to pad
// - count: The multiple of bytes to pad to, must NOT BE ZERO
#[no_mangle]
pub fn pad_block(block: &[u8], count: usize) -> Vec<u8> {
    let remainder = block.len() % count;
    let extend = (count - remainder) % count;
    resize_block(block, block.len() + extend)
}

//Assigning mod 2 addition on two blocks
// - block_a: The first block
// - block_b: The second block
#[no_mangle]
pub fn xor_blocks(lhs: &mut [u8], rhs: &[u8]) {
    let max_len = lhs.len().min(rhs.len());
    (0..max_len).for_each(|i| lhs[i] ^= rhs[i]);
}

//Increment a block, 0th element least significant
// - block: The mutable byte array to increment
#[no_mangle]
pub fn inc_block(block: &mut [u8]) {
    for elem in block {
        *elem = elem.wrapping_add(1);
        if *elem != 0 {
            return;
        }
    }
}
