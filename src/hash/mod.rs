use crate::block::xor_blocks;

//Hashes some input byte array
// - input: some input block, don't use one less than 3 bytes, larger is better but slower
// - output: some block to store the output
// LEN: The length of blocks in bytes
// RND: The number of rounds (> 0 for a good hash)
#[inline(always)]
pub fn arb_hash<const LEN: usize, const RND: u64>(input: &[u8; LEN], output: &mut [u8; LEN]) {
    (0..LEN).for_each(|i| output[i] = input[i]);
    let shift_table = create_shift_table(LEN * 2);
    let b_ind_table = index_table::<LEN, 1>();
    let c_ind_table = index_table::<LEN, 2>();
    (0..RND).for_each(|_rnd| {
        (0..LEN).for_each(|b_shift| {
            (0..LEN).for_each(|elem| {
                let shift = shift_table[b_shift + elem];
                let b_index = b_ind_table[elem];
                let c_index = c_ind_table[elem];
                output[elem] = output[elem].wrapping_add(output[b_index]);
                output[c_index] ^= output[elem];
                output[c_index] = output[c_index].rotate_left(shift);
            });
        });
    });
    xor_blocks(output, input)
}

// Creates a table of the shifts needed for different lengths
#[inline(always)]
fn create_shift_table(len: usize) -> Vec<u32> {
    const SHIFTS: [u32; 5] = [1, 2, 3, 5, 7];
    let mut table = vec![0u32; len];
    table.iter_mut().enumerate().for_each(|(i, elem)| {
        *elem = SHIFTS[i % 5];
    });
    table
}

// Creates a table of indexes offset into an array, wrapping.
#[inline(always)]
const fn index_table<const LEN: usize, const OFF: usize>() -> [usize; LEN] {
    let mut table = [0usize; LEN];
    let mut i = 0;
    loop {
        if i == LEN {
            return table;
        }
        table[i] = (OFF + i) % LEN;
        i += 1;
    }
}
