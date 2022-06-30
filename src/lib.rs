pub fn arb_hash<const LEN: usize, const RND: u64>(mut input: [u8; LEN]) -> [u8; LEN] {
    const SHIFTS: [u32; 5] = [1, 2, 3, 5, 7];
    let original = input;
    for _rnd in 0..RND {
        for b_shift in 0..LEN {
            for elem in 0..LEN {
                let shift = SHIFTS[(b_shift + elem) % 5];
                let c_index = (elem + 2) % LEN;
                input[elem] = input[elem].wrapping_add(input[(elem + 1) % LEN]);
                input[c_index] ^= input[elem];
                input[c_index] = input[c_index].rotate_left(shift);
            }
        }
    }
    for elem in 0..LEN {
        input[elem] ^= original[elem];
    }
    input
}

#[test]
fn arb_hash_test() {
    assert_eq!(
        arb_hash::<8, 3>([0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
        [0xF2, 0x04, 0xF7, 0x45, 0xC2, 0x0D, 0xEA, 0xEA]
    );
}
