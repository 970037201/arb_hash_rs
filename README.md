# arb_hash_rs
Arbitrary block length hash function, potentially cryptographically secure. Passes BigCrush

`features = ["block"]`

* xor_blocks 
Returns the mod-2 addition of two blocks
 - block_a: The first block
 - block_b: The second block
* inc_block
Increment a block, 0th element least significant
- block: The mutable byte array to imcrement

`features = ["digest"]`

* arb_digest 
Returns the digest of an input string
 - input: some input byte array, could be the contents of a file, for example
 - length: don't use under 3 bytes, as the output is not well distributed
 - rounds: 1 round is sufficient, but 2 rounds *might* be better distributed

`features = ["hash"]`

* arb_hash 
Returns the hash of an input byte array
 - input: some input block, don't use one less than 3 bytes, larger is better but slower
 - rounds: 1 round is sufficient, but 2 rounds *might* be better distributed