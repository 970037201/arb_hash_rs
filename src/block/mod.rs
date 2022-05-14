use std::mem::MaybeUninit;

#[cfg(test)]
mod tests;

// Block definition, standard cryptographic chunk of data
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AHBlock<const LEN: usize> {
    pub data: [u8; LEN],
}

impl<const LEN: usize> AHBlock<LEN> {
    /// Construct a new block, initialized with all zeros, at compile time if possible
    #[inline(always)]
    pub const fn new() -> Self {
        Self { data: [0u8; LEN] }
    }
    
    /// Construct a new block, initialized from zeros and a source slice, at compile time if possible
    #[inline(always)]
    pub const fn from_slice(input: &[u8]) -> Self {
        let mut block_out = Self::new();
        let min_len = match input.len() < LEN {
            true => input.len(),
            false => LEN,
        };
        let mut i = 0;
        while i < min_len {
            block_out.data[i] = input[i];
            i += 1;
        }
        block_out
    }

    /// Construct a new block, initialized from zeros and another block, at compile time if possible
    #[inline(always)]
    pub const fn from_block<const LEN2: usize>(input: &AHBlock<LEN2>) -> Self {
        Self::from_slice(&input.data)
    }

    /// XOR block with another block
    #[inline(always)]
    pub fn xor_block_assign(&mut self, other: &Self) {
        let mut i = 0;
        while i < LEN {
            self.data[i] ^= other.data[i];
            i += 1;
        }
    }

    /// XOR block with another block, producing an output, at compile time if possible
    #[inline(always)]
    pub const fn xor_block(&self, other: &Self) -> Self {
        let mut output = Self::from_block(self);
        let mut i = 0;
        while i < LEN {
            output.data[i] ^= other.data[i];
            i += 1;
        }
        output
    }

    /// Increment block, little endian
    #[inline(always)]
    pub fn inc_block_assign(&mut self) {
        let mut i = 0;
        while i < LEN {
            self.data[i] = self.data[i].wrapping_add(1);
            if self.data[i] != 0 {
                return;
            }
            i += 1;
        }
    }

    /// Output incremented block, little endian, at compile time if possible
    #[inline(always)]
    pub const fn inc_block(&self) -> Self {
        let mut output = Self::from_block(self);
        let mut i = 0;
        while i < LEN {
            output.data[i] = output.data[i].wrapping_add(1);
            if output.data[i] != 0 {
                return output;
            }
            i += 1;
        }
        output
    }
}

/// Padding byte slice to multiples of blocks, compliant with ISO/IEC 7816-4
#[inline(always)]
pub fn pad_to_blocks<const LEN: usize>(input: &[u8]) -> Vec<AHBlock<LEN>> {
    let chunks = input.chunks_exact(LEN);
    let marked_remainder = [chunks.remainder(), &[0x80]].concat();
    let padding_slice = marked_remainder.as_slice();
    chunks
        .chain([padding_slice])
        .map(AHBlock::from_slice)
        .collect()
}
