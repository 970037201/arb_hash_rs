use crate::block::xor_blocks;

//Const funtion for hashing some input byte array
// - input: some input block, larger is better but slower
// RND: The number of rounds (> 0 for a sane hash)
// LEN: The length of blocks in bytes (> 2 for a sane hash)
#[inline(always)]
pub const fn arb_hash<const RND: u64, const LEN: usize>(input: &[u8; LEN]) -> [u8; LEN] {
    const SHIFTS: [u32; 5] = [1, 2, 3, 5, 7];
    // SAFETY: All elements are initialized and never read from before initialization
    let mut output = [0u8; LEN];
    let mut i = 0;
    while i < LEN {
        output[i] = input[i];
        i += 1;
    }
    let b_ind_table = index_table::<LEN, 1>();
    let c_ind_table = index_table::<LEN, 2>();
    let mut _rnd = 0;
    while _rnd < RND {
        let mut b_shift = 0;
        while b_shift < LEN {
            let mut elem = 0;
            while elem < LEN {
                let shift = SHIFTS[(b_shift + elem) % 5];
                let b_index = b_ind_table[elem];
                let c_index = c_ind_table[elem];
                output[elem] = output[elem].wrapping_add(output[b_index]);
                output[c_index] ^= output[elem];
                output[c_index] = output[c_index].rotate_left(shift);
                elem += 1;
            }
            b_shift += 1;
        }
        _rnd += 1;
    }
    xor_blocks(&output, input)
}

// Creates a table of indexes offset into an array, wrapping.
#[inline(always)]
const fn index_table<const LEN: usize, const OFF: usize>() -> [usize; LEN] {
    let mut table = [0usize; LEN];
    let mut i = 0;
    while i < LEN {
        table[i] = (OFF + i) % LEN;
        i += 1;
    }
    table
}
