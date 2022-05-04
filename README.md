# arb_hash_rs
Arbitrary block length hash function, potentially cryptographically secure. Passes BigCrush

`features = ["block"]`
- `resize_block` (Resizes a block, truncating or padding with zeros as needed)
- `pad_input` (Returns array of blocks, zero padded for the last block)
- `xor_blocks` (Assigning mod 2 addition on two blocks)
- `inc_block` (Increment a block, 0th element least significant)

`features = ["digest"]`
- `arb_digest` (Returns the digest of an input byte array)

`features = ["parallel", "digest"]`
- `arb_digest` (Same as above, but parts computed in parallel)

`features = ["hash"]`
- `arb_hash` (Hashes some input byte array)
