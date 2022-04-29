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
