#[cfg(test)]
mod tests;

use crate::block::AHBlock;

impl<const LEN: usize> AHBlock<LEN> {
    /// Creates a table of indexes offset into an array, wrapping.
    #[inline(always)]
    const fn index_table(offset: usize) -> [usize; LEN] {
        let mut table = [0usize; LEN];
        let mut i = 0;
        while i < LEN {
            table[i] = (offset + i) % LEN;
            i += 1;
        }
        table
    }

    /// Function for hashing some block
    #[inline(always)]
    pub fn arb_hash_assign<const RND: u64>(&mut self) {
        let input = Self::from_block(self);
        const SHIFTS: [u32; 5] = [1, 2, 3, 5, 7];
        let shifts: Vec<u32> = (0..LEN * 2).map(|i| SHIFTS[i % 5]).collect();
        let b_ind_table: [usize; LEN] = Self::index_table(1);
        let c_ind_table: [usize; LEN] = Self::index_table(2);
        for _rnd in 0..RND {
            for b_shift in 0..LEN {
                for elem in 0..LEN {
                    let shift = shifts[b_shift + elem];
                    let b_index = b_ind_table[elem];
                    let c_index = c_ind_table[elem];
                    self.data[elem] = self.data[elem].wrapping_add(self.data[b_index]);
                    self.data[c_index] ^= self.data[elem];
                    self.data[c_index] = self.data[c_index].rotate_left(shift);
                }
            }
        }
        self.xor_block_assign(&input);
    }

    /// Function for returning the hash of some block
    #[inline(always)]
    pub fn arb_hash<const RND: u64>(&self) -> Self {
        let mut input = Self::from_block(self);
        const SHIFTS: [u32; 5] = [1, 2, 3, 5, 7];
        let shifts: Vec<u32> = (0..LEN * 2).map(|i| SHIFTS[i % 5]).collect();
        let b_ind_table: [usize; LEN] = Self::index_table(1);
        let c_ind_table: [usize; LEN] = Self::index_table(2);
        for _rnd in 0..RND {
            for b_shift in 0..LEN {
                for elem in 0..LEN {
                    let shift = shifts[b_shift + elem];
                    let b_index = b_ind_table[elem];
                    let c_index = c_ind_table[elem];
                    input.data[elem] = input.data[elem].wrapping_add(input.data[b_index]);
                    input.data[c_index] ^= input.data[elem];
                    input.data[c_index] = input.data[c_index].rotate_left(shift);
                }
            }
        }
        input.xor_block_assign(self);
        input
    }
}
