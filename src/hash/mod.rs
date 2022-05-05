#[cfg(test)]
mod tests;

use crate::block::AHBlock;

impl<const LEN: usize> AHBlock<LEN> {
    /// Function for hashing some block
    #[inline(always)]
    pub fn arb_hash_assign<const RND: u64>(&mut self) {
        *self = self.arb_hash::<RND>();
    }

    /// Function for returning the hash of some block
    #[inline(always)]
    pub const fn arb_hash<const RND: u64>(&self) -> Self {
        let mut output = Self::from_similar_block(self);
        const SHIFTS: [u32; 5] = [1, 2, 3, 5, 7];
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
                    output.data[elem] = output.data[elem].wrapping_add(output.data[b_index]);
                    output.data[c_index] ^= output.data[elem];
                    output.data[c_index] = output.data[c_index].rotate_left(shift);
                    elem += 1;
                }
                b_shift += 1;
            }
            _rnd += 1;
        }
        output.xor_block(self)
    }
}

/// Creates a table of indexes offset into an array, wrapping.
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
