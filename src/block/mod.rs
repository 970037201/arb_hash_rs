//Returns zero padded block to a multiple of bytes
// - block: The block to pad
// - count: The multiple of bytes to pad to
pub fn pad_block(block: &[u8], count: usize) -> Vec<u8> {
    match count == 0 {
        true => Vec::from(block),
        false => {
            let remainder = block.len() % count;
            let extend = (count - remainder) % count;
            [block, &vec![0u8; extend]].concat()
        }        
    }
}

//Returns the mod-2 addition of two blocks
// - block_a: The first block
// - block_b: The second block
pub fn xor_blocks(block_a: &[u8], block_b: &[u8]) -> Vec<u8> {
    let (a, b) = match block_a.len() > block_b.len() {
        true => (block_a, block_b),
        false => (block_b, block_a),
    };
    (0..b.len())
        .map(|i| a[i] ^ b[i])
        .chain(a[b.len()..a.len()].to_owned())
        .collect()
}

//Increment a block, 0th element least significant
// - block: The mutable byte array to imcrement
pub fn inc_block(block: &mut [u8]) {
    for elem in block {
        *elem = elem.wrapping_add(1);
        if *elem != 0 {
            return;
        }
    }
}
