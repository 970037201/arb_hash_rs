//Runtime funtion for truncating a block with zeros
// - block: The block to pad
// LEN: the length of the new block in bytes
#[inline(always)]
pub fn resize_block<const LEN: usize>(block: &[u8]) -> [u8; LEN] {
    let mut uninit_output = [0u8; LEN];
    let min_len = block.len().min(LEN);
    (0..min_len).for_each(|i| uninit_output[i] = block[i]);
    (min_len..LEN).for_each(|i| uninit_output[i] = 0);
    // uninit_output is now initialized
    uninit_output
}

//Const funtion for truncating a compile-time known sized block with zeros
// - block: The block to pad
// LEN: the length of the new block in bytes
// LEN2: the size of the input block in bytes
#[inline(always)]
pub const fn const_resize_block<const LEN: usize, const LEN2: usize>(
    block: &[u8; LEN2],
) -> [u8; LEN] {
    let mut uninit_output = [0u8; LEN];
    let min_len = match LEN2 < LEN {
        true => LEN2,
        false => LEN,
    };
    let mut i = 0;
    while i < min_len {
        uninit_output[i] = block[i];
        i += 1;
    }
    let mut i = min_len;
    while i < LEN {
        uninit_output[i] = 0;
        i += 1;
    }
    uninit_output
}

//Runtime function for padding bytes to multiples of blocks
// - input: The input to pad
// LEN: the multiple of bytes to pad to, must NOT BE ZERO

// WARNING: CHANGED SINCE 0.1.11:
// As extending input with 0s could result in the same hash within LEN bytes,
// The padding function now leads with 0xFF on padding.
#[inline(always)]
pub fn pad_input<const LEN: usize>(input: &[u8]) -> Vec<[u8; LEN]> {
    let block_cnt = (input.len() + LEN - 1) / LEN;
    let chunks = input.chunks_exact(LEN);
    let mut result_vec = Vec::with_capacity(block_cnt);
    let remainder = chunks.remainder();
    chunks.for_each(|chunk| result_vec.push(chunk.try_into().unwrap()));
    if !remainder.is_empty() {
        let marked_remainder = [remainder, &[0xFF]].concat();
        result_vec.push(resize_block(&marked_remainder));
    }
    result_vec
}

//Const function for mod-2 addition of two blocks
// - block_a: The first block
// - block_b: The second block
// LEN: the number of bytes in a block
#[inline(always)]
pub const fn xor_blocks<const LEN: usize>(lhs: &[u8; LEN], rhs: &[u8; LEN]) -> [u8; LEN] {
    let mut uninit_output = [0u8; LEN];
    let mut i = 0;
    while i < LEN {
        uninit_output[i] = lhs[i] ^ rhs[i];
        i += 1;
    }
    uninit_output
}

//Const function for Mod-2 addition of two blocks
// - block: The byte array to increment
// LEN: the length of a block in bytes
#[inline(always)]
pub const fn inc_block<const LEN: usize>(block: &[u8; LEN]) -> [u8; LEN] {
    let mut uninit_output = [0u8; LEN];
    let mut i = 0;
    while i < LEN {
        uninit_output[i] = block[i];
        i += 1;
    }
    let mut i = 0;
    while i < LEN {
        uninit_output[i] = uninit_output[i].wrapping_add(1);
        if uninit_output[i] != 0 {
            return uninit_output;
        }
        i += 1;
    }
    uninit_output
}
