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
        let mut input = Self::from_block(self);
        const SHIFTS: [u32; 5] = [1, 2, 3, 5, 7];
        let mut _rnd = 0;
        while _rnd < RND {
            let mut b_shift = 0;
            while b_shift < LEN {
                let mut elem = 0;
                while elem < LEN {
                    let shift = SHIFTS[(b_shift + elem) % 5];
                    let c_index = (elem + 2) % LEN;
                    input.data[elem] = input.data[elem].wrapping_add(input.data[(elem + 1) % LEN]);
                    input.data[c_index] ^= input.data[elem];
                    input.data[c_index] = input.data[c_index].rotate_left(shift);
                    elem += 1;
                }
                b_shift += 1;
            }
            _rnd += 1;
        }
        input.xor_block(self)
    }
}
