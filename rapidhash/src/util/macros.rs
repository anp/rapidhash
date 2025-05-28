macro_rules! compare_to_c {
    ($test:ident, $rust_fn:path, $compact_fn:path, $cc_fn:ident) => {
        #[test]
        fn $test() {
            use rand::Rng;
            use rapidhash_c::$cc_fn;

            for len in 0..=512 {
                let mut data = std::vec![0; len];
                rand::rng().fill(&mut data[..]);

                for byte in 0..len {
                    for bit in 0..8 {
                        let mut data = data.clone();
                        data[byte] ^= 1 << bit;

                        let rust_hash = $rust_fn(&data, RAPID_SEED);
                        let compact_hash = $compact_fn(&data, RAPID_SEED);
                        let c_hash = $cc_fn(&data, RAPID_SEED);
                        assert_eq!(rust_hash, c_hash, "Mismatch with C on input {} byte {} bit {}", len, byte, bit);
                        assert_eq!(rust_hash, compact_hash, "Mismatch with COMPACT on input {} byte {} bit {}", len, byte, bit);
                    }
                }
            }
        }
    };
}

pub(crate) use compare_to_c;