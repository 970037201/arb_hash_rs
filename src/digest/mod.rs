//Returns the digest of an input byte array
// - input: some input byte array, could be the contents of a file, for example
// - length: don't use under 3 bytes, as the output is not well distributed
// - rounds: 1 round is sufficient, but 2 rounds *might* be better distributed

#[cfg(feature = "parallel")]
mod parallel_digest;

#[cfg(feature = "parallel")]
#[inline(always)]
pub fn arb_digest(input: &[u8], length: usize, rounds: u64) -> Vec<u8> {
    use parallel_digest::parallel_arb_digest;
    parallel_arb_digest(input, length, rounds)
}

#[cfg(not(feature = "parallel"))]
mod serial_digest;

#[cfg(not(feature = "parallel"))]
#[inline(always)]
pub fn arb_digest(input: &[u8], length: usize, rounds: u64) -> Vec<u8> {
    use serial_digest::serial_arb_digest;
    serial_arb_digest(input, length, rounds)
}
