# arb_hash_rs
Arbitrary block length hash function, potentially cryptographically secure. Passes BigCrush

`features = ["block"]`
- `pad_block` (Returns zero padded block to a multiple of bytes)
- `xor_blocks` (Returns the mod-2 addition of two blocks)
- `inc_block` (Increment a block, 0th element least significant)

`features = ["digest"]`
- `arb_digest` (Returns the digest of an input byte array)

`features = ["hash"]`
- `arb_hash` (Returns the hash of an input byte array)