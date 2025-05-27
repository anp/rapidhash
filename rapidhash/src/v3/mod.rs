//! The rapidhash V3 algorithm.

mod rapid_const;
#[cfg(any(feature = "std", docsrs))]
mod rapid_file;

#[doc(inline)]
pub use rapid_const::{rapidhash, rapidhash_inline, rapidhash_seeded, rapidhash_micro_inline, rapidhash_nano_inline, RAPID_SEED};

#[doc(inline)]
#[cfg(any(feature = "std", docsrs))]
pub use rapid_file::*;

#[cfg(test)]
mod tests {
    extern crate std;

    use super::*;

    /// Ensure that changing a single bit flips at least 10 bits in the resulting hash, and on
    /// average flips half of the bits.
    ///
    /// These tests are not deterministic, but should fail with a very low probability.
    #[test]
    fn flip_bit_trial() {
        use rand::Rng;

        let mut flips = std::vec![];

        for len in 1..=256 {
            let mut data = std::vec![0; len];
            rand::rng().fill(&mut data[..]);

            let hash = rapidhash(&data);
            for byte in 0..len {
                for bit in 0..8 {
                    let mut data = data.clone();
                    data[byte] ^= 1 << bit;
                    let new_hash = rapidhash(&data);
                    assert_ne!(hash, new_hash, "Flipping byte {} bit {} did not change hash for input len {}", byte, bit, len);
                    let xor = hash ^ new_hash;
                    let flipped = xor.count_ones() as u64;
                    assert!(xor.count_ones() >= 10, "Flipping bit {byte}:{bit} changed only {flipped} bits");

                    flips.push(flipped);
                }
            }
        }

        let average = flips.iter().sum::<u64>() as f64 / flips.len() as f64;
        assert!(average > 31.95 && average < 32.05, "Did not flip an average of half the bits. average: {average}, expected: 32.0");
    }

    /// Compare to the C rapidhash implementation to ensure we match perfectly.
    #[test]
    fn compare_to_c() {
        use rand::Rng;
        use rapidhash_c::{rapidhashcc_v3, rapidhashcc_v3_micro, rapidhashcc_v3_nano};

        for len in 0..=512 {
            let mut data = std::vec![0; len];
            rand::rng().fill(&mut data[..]);

            for byte in 0..len {
                for bit in 0..8 {
                    let mut data = data.clone();
                    data[byte] ^= 1 << bit;

                    let rust_hash = rapidhash_seeded(&data, RAPID_SEED);
                    let compact_hash = rapidhash_inline::<true, false>(&data, RAPID_SEED);
                    let c_hash = rapidhashcc_v3(&data, RAPID_SEED);
                    assert_eq!(rust_hash, c_hash, "Mismatch with C input {} byte {} bit {}", len, byte, bit);
                    assert_eq!(rust_hash, compact_hash, "Mismatch with COMPACT on input {} byte {} bit {}", len, byte, bit);

                    let rust_hash = rapidhash_micro_inline::<false>(&data, RAPID_SEED);
                    let c_hash = rapidhashcc_v3_micro(&data, RAPID_SEED);
                    assert_eq!(rust_hash, c_hash, "Mismatch MICRO with C input {} byte {} bit {}", len, byte, bit);

                    let rust_hash = rapidhash_nano_inline::<false>(&data, RAPID_SEED);
                    let c_hash = rapidhashcc_v3_nano(&data, RAPID_SEED);
                    assert_eq!(rust_hash, c_hash, "Mismatch NANO with C input {} byte {} bit {}", len, byte, bit);
                }
            }
        }
    }
}
