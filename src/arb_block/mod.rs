pub fn xor_blocks(a: &[u8], b: &[u8]) -> Vec<u8> {
    let len = a.len().min(b.len());
    (0..len).map(|i| a[i] ^ b[i]).collect()
}

pub fn inc_block(a: &mut [u8]) {
    for elem in a {
        *elem = elem.wrapping_add(1);
        if *elem != 0 {
            return;
        }
    }
}
