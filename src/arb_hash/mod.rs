use crate::arb_block::xor_blocks;

pub fn arb_hash(input: &[u8], rounds: u128) -> Vec<u8> {
    const SHIFTS: [u32; 5] = [1, 2, 3, 5, 7];
    let mut output = Vec::from(input);
    let len: usize = input.len();
    for _rnd in 0..rounds {
        for b_shift in 0..len {
            for elem in 0..len {
                let shift = SHIFTS[(b_shift + elem) % 5];
                output[elem] = output[elem].wrapping_add(output[(elem + 1) % len]);
                output[(elem + 2) % len] ^= output[elem];
                output[(elem + 2) % len] = output[(elem + 2) % len].rotate_left(shift);
            }
        }
    }
    xor_blocks(&output, input)
}
