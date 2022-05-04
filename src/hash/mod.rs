use crate::block::xor_blocks;

//Returns the hash of an input byte array
// - input: some input block, don't use one less than 3 bytes, larger is better but slower
// - output: some output block, MUST be the same length as the input block
// - rounds: 1 round is sufficient, but 2 rounds *might* be better distributed
#[no_mangle]
pub fn arb_hash(input: &[u8], output: &mut [u8], rounds: u64) {
    const SHIFTS: [u32; 5] = [1, 2, 3, 5, 7];
    let output_len = output.len();
    (0..output_len).for_each(|i| output[i] = input[i]);
    let shift_table: Vec<u32> = (0..output_len * 2).map(|i| SHIFTS[i % 5]).collect();
    let b_ind_table: Vec<usize> = (0..output_len).map(|i| (i + 1) % output_len).collect();
    let c_ind_table: Vec<usize> = (0..output_len).map(|i| (i + 2) % output_len).collect();
    for _rnd in 0..rounds {
        for b_shift in 0..output_len {
            for elem in 0..output_len {
                let shift = shift_table[b_shift + elem];
                let b_index = b_ind_table[elem];
                let c_index = c_ind_table[elem];
                output[elem] = output[elem].wrapping_add(output[b_index]);
                output[c_index] ^= output[elem];
                output[c_index] = output[c_index].rotate_left(shift);
            }
        }
    }
    xor_blocks(output, input)
}
