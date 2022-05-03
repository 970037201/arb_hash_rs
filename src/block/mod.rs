//Resizes a block, truncating or padding with zeros as needed
// - block: The block to pad
// - count: The number of bytes to resize to
#[inline(always)]
pub fn resize_block(block: &[u8], count: usize) -> Vec<u8> {
    let mut result = block.to_vec();
    result.resize(count, 0);
    result
}

//Returns zero padded block to a multiple of bytes
// - block: The block to pad
// - count: The multiple of bytes to pad to
#[inline(always)]
pub fn pad_block(block: &[u8], count: usize) -> Vec<u8> {
    match count == 0 {
        true => Vec::from(block),
        false => {
            let remainder = block.len() % count;
            let extend = (count - remainder) % count;
            resize_block(block, block.len() + extend)
        }
    }
}

//Returns the mod-2 addition of two blocks
// - block_a: The first block
// - block_b: The second block
#[inline(always)]
pub fn xor_blocks(block_a: &[u8], block_b: &[u8]) -> Vec<u8> {
    let max_len = block_a.len().max(block_b.len());
    let resized_a = resize_block(block_a, max_len);
    let resized_b = resize_block(block_b, max_len);
    (0..max_len).map(|i| resized_a[i] ^ resized_b[i]).collect()
}

//Increment a block, 0th element least significant
// - block: The mutable byte array to increment
#[inline(always)]
pub fn inc_block(block: &mut [u8]) {
    for elem in block {
        *elem = elem.wrapping_add(1);
        if *elem != 0 {
            return;
        }
    }
}
