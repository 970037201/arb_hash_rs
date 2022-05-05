// Block definition
pub type AHBlock<const LEN: usize> = [u8; LEN];

//Runtime funtion for truncating a block with zeros
// - block: The block to pad
// LEN: the length of the new block in bytes
#[inline(always)]
pub fn resize_block<const LEN: usize>(block: &[u8]) -> AHBlock<LEN> {
    let mut block_out = [0u8; LEN];
    let min_len = block.len().min(LEN);
    (0..min_len).for_each(|i| block_out[i] = block[i]);
    (min_len..LEN).for_each(|i| block_out[i] = 0);
    block_out
}

//Const funtion for truncating a compile-time known sized block with zeros
// - block: The block to pad
// LEN: the length of the new block in bytes
// LEN2: the size of the input block in bytes
#[inline(always)]
pub const fn const_resize_block<const LEN: usize, const LEN2: usize>(
    block: &AHBlock<LEN2>,
) -> AHBlock<LEN> {
    let mut block_out = [0u8; LEN];
    let min_len = match LEN2 < LEN {
        true => LEN2,
        false => LEN,
    };
    let mut i = 0;
    while i < min_len {
        block_out[i] = block[i];
        i += 1;
    }
    let mut i = min_len;
    while i < LEN {
        block_out[i] = 0;
        i += 1;
    }
    block_out
}

//Runtime function for padding bytes to multiples of blocks
// - input: The input to pad
// LEN: the multiple of bytes to pad to, must NOT BE ZERO

// WARNING: CHANGED SINCE 0.1.11: (and now 0.1.12)
// Now will pad compliant to ISO/IEC 7816-4
#[inline(always)]
pub fn pad_input<const LEN: usize>(input: &[u8]) -> Vec<AHBlock<LEN>> {
    let block_cnt = input.len() / LEN + 1;
    let chunks = input.chunks_exact(LEN);
    let mut result_vec = Vec::with_capacity(block_cnt);
    let remainder = chunks.remainder();
    chunks.for_each(|chunk| result_vec.push(chunk.try_into().unwrap()));
    let marked_remainder = [remainder, &[0x80]].concat();
    result_vec.push(resize_block(&marked_remainder));
    result_vec
}

//Const function for mod-2 addition of two blocks
// - block_a: The first block
// - block_b: The second block
// LEN: the number of bytes in a block
#[inline(always)]
pub const fn xor_blocks<const LEN: usize>(lhs: &AHBlock<LEN>, rhs: &AHBlock<LEN>) -> AHBlock<LEN> {
    let mut block_out = [0u8; LEN];
    let mut i = 0;
    while i < LEN {
        block_out[i] = lhs[i] ^ rhs[i];
        i += 1;
    }
    block_out
}

//Const function for Mod-2 addition of two blocks
// - block: The byte array to increment
// LEN: the length of a block in bytes
#[inline(always)]
pub const fn inc_block<const LEN: usize>(block: &AHBlock<LEN>) -> AHBlock<LEN> {
    let mut block_out = [0u8; LEN];
    let mut i = 0;
    while i < LEN {
        block_out[i] = block[i];
        i += 1;
    }
    let mut i = 0;
    while i < LEN {
        block_out[i] = block_out[i].wrapping_add(1);
        if block_out[i] != 0 {
            return block_out;
        }
        i += 1;
    }
    block_out
}
