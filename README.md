# arb_hash_rs
Arbitrary block length hash function, potentially cryptographically secure. Passes BigCrush

Usage:
- Pass an input buffer to the arb_hash function, it will return the hashed result.
- The number of rounds supplied to the function must be at least 1.
- The number of bytes in the input buffer should be at least 3.
