# arb_hash_rs
Arbitrary block length hash function, potentially cryptographically secure. Passes BigCrush

`features = ["block"]`
- `resize_block` (Runtime funtion for truncating a block with zeros)
- `const_resize_block` (Const funtion for truncating a compile-time known sized block with zeros)
- `pad_input` (Runtime function for padding bytes to multiples of blocks)
- `xor_blocks` (Const function for mod-2 addition of two blocks)
- `inc_block` (Const function for Mod-2 addition of two blocks)

`features = ["digest"]`
- `arb_digest` (Runtime function for digest computation, running in a single thread)
- `arb_digest_const` (Const funtion for digest computation, running during compile time)

`features = ["parallel", "digest"]`
- `arg_digest_parallel` (Parallel computation of digest of byte array)

`features = ["hash"]`
- `arb_hash` (Const funtion for hashing some input byte array)

Notes:
    Rust seems to be lacking compile time arithmetic with const generics,
so there are some limitations to what I can do. The funtion arb_digest_const
should really allow a byte array instead of array of blocks to be input,
however that would require me to split that into blocks, which I couldn't find
a way to do with const generics.
    Anyways, if there is a flaw or bug in this library, please let me know so I
can fix said bug immediately.
    Happy hashing!