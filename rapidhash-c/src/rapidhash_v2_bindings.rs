#[link(name = "rapidhash_v2")]
extern "C" {
    fn rapidhash_v2_extern(
        key: *const core::ffi::c_void,
        len: libc::size_t,
        seed: u64,
    ) -> u64;
}

pub fn rapidhashcc_v2(key: &[u8], seed: u64) -> u64 {
    unsafe { rapidhash_v2_extern(key.as_ptr() as *const core::ffi::c_void, key.len(), seed) }
}

#[cfg(test)]
mod tests {
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

            let hash = rapidhashcc_v2(&data, 0);
            for byte in 0..len {
                for bit in 0..8 {
                    let mut data = data.clone();
                    data[byte] ^= 1 << bit;
                    let new_hash = rapidhashcc_v2(&data, 0);
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
}
