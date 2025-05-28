use crate::util::mix::{rapid_mix, rapid_mum};
use crate::util::read::{read_u32_combined, read_u64};

/// The rapidhash default seed.
pub const RAPID_SEED: u64 = 0xbdd89aa982704029;
pub(super) const RAPID_SECRET: [u64; 3] = [0x2d358dccaa6c78a5, 0x8bb84b93962eacc9, 0x4b33a62ed433d4a3];

/// Rapidhash a single byte stream, matching the C++ implementation.
#[inline]
pub const fn rapidhash_v1(data: &[u8]) -> u64 {
    rapidhash_v1_inline::<false, false>(data, RAPID_SEED)
}

/// Rapidhash a single byte stream, matching the C++ implementation, with a custom seed.
#[inline]
pub const fn rapidhash_v1_seeded(data: &[u8], seed: u64) -> u64 {
    rapidhash_v1_inline::<false, false>(data, seed)
}

/// Rapidhash a single byte stream, matching the C++ implementation.
///
/// Is marked with `#[inline(always)]` to force the compiler to inline and optimise the method.
/// Can provide large performance uplifts for inputs where the length is known at compile time.
#[inline(always)]
pub const fn rapidhash_v1_inline<const COMPACT: bool, const PROTECTED: bool>(data: &[u8], mut seed: u64) -> u64 {
    seed = rapidhash_seed(seed) ^ data.len() as u64;
    let (a, b, _) = rapidhash_core::<COMPACT, PROTECTED>(0, 0, seed, data);
    rapidhash_finish::<PROTECTED>(a, b, data.len() as u64)
}

#[inline(always)]
pub(super) const fn rapidhash_seed(seed: u64) -> u64 {
    seed ^ rapid_mix::<false>(seed ^ RAPID_SECRET[0], RAPID_SECRET[1])
}

#[inline(always)]
pub(super) const fn rapidhash_core<const COMPACT: bool, const PROTECTED: bool>(mut a: u64, mut b: u64, mut seed: u64, data: &[u8]) -> (u64, u64, u64) {
    if data.len() <= 16 {
        // deviation from the C++ impl computes delta as follows
        // let delta = (data.len() & 24) >> (data.len() >> 3);
        // this is equivalent to "match {..8=>0, 8..=>4}"
        // and so using the extra if-else statement is equivalent and allows the compiler to skip
        // some unnecessary bounds checks while still being safe rust.
        if data.len() >= 8 {
            // len is 4..=16
            let plast = data.len() - 4;
            let delta = 4;
            a ^= read_u32_combined(data, 0, plast);
            b ^= read_u32_combined(data, delta, plast - delta);
        } else if data.len() >= 4 {
            let plast = data.len() - 4;
            let delta = 0;
            a ^= read_u32_combined(data, 0, plast);
            b ^= read_u32_combined(data, delta, plast - delta);
        } else if data.len() > 0 {
            // len is 1..=3
            let len = data.len();
            a ^= ((data[0] as u64) << 56) | ((data[len >> 1] as u64) << 32) | data[len - 1] as u64;
            // b = 0;
        }
    } else {
        let mut slice = data;

        if slice.len() > 48 {
            // most CPUs appear to benefit from this unrolled loop
            let mut see1 = seed;
            let mut see2 = seed;

            if !COMPACT {
                while slice.len() >= 96 {
                    seed = rapid_mix::<PROTECTED>(read_u64(slice, 0) ^ RAPID_SECRET[0], read_u64(slice, 8) ^ seed);
                    see1 = rapid_mix::<PROTECTED>(read_u64(slice, 16) ^ RAPID_SECRET[1], read_u64(slice, 24) ^ see1);
                    see2 = rapid_mix::<PROTECTED>(read_u64(slice, 32) ^ RAPID_SECRET[2], read_u64(slice, 40) ^ see2);
                    seed = rapid_mix::<PROTECTED>(read_u64(slice, 48) ^ RAPID_SECRET[0], read_u64(slice, 56) ^ seed);
                    see1 = rapid_mix::<PROTECTED>(read_u64(slice, 64) ^ RAPID_SECRET[1], read_u64(slice, 72) ^ see1);
                    see2 = rapid_mix::<PROTECTED>(read_u64(slice, 80) ^ RAPID_SECRET[2], read_u64(slice, 88) ^ see2);
                    let (_, split) = slice.split_at(96);
                    slice = split;
                }
                if slice.len() >= 48 {
                    seed = rapid_mix::<PROTECTED>(read_u64(slice, 0) ^ RAPID_SECRET[0], read_u64(slice, 8) ^ seed);
                    see1 = rapid_mix::<PROTECTED>(read_u64(slice, 16) ^ RAPID_SECRET[1], read_u64(slice, 24) ^ see1);
                    see2 = rapid_mix::<PROTECTED>(read_u64(slice, 32) ^ RAPID_SECRET[2], read_u64(slice, 40) ^ see2);
                    let (_, split) = slice.split_at(48);
                    slice = split;
                }
            } else {
                while slice.len() >= 48 {
                    seed = rapid_mix::<PROTECTED>(read_u64(slice, 0) ^ RAPID_SECRET[0], read_u64(slice, 8) ^ seed);
                    see1 = rapid_mix::<PROTECTED>(read_u64(slice, 16) ^ RAPID_SECRET[1], read_u64(slice, 24) ^ see1);
                    see2 = rapid_mix::<PROTECTED>(read_u64(slice, 32) ^ RAPID_SECRET[2], read_u64(slice, 40) ^ see2);
                    let (_, split) = slice.split_at(48);
                    slice = split;
                }
            }
            seed ^= see1 ^ see2;
        }

        if slice.len() > 16 {
            seed = rapid_mix::<PROTECTED>(read_u64(slice, 0) ^ RAPID_SECRET[2], read_u64(slice, 8) ^ seed ^ RAPID_SECRET[1]);
            if slice.len() > 32 {
                seed = rapid_mix::<PROTECTED>(read_u64(slice, 16) ^ RAPID_SECRET[2], read_u64(slice, 24) ^ seed);
            }
        }

        a ^= read_u64(data, data.len() - 16);
        b ^= read_u64(data, data.len() - 8);
    }

    a ^= RAPID_SECRET[1];
    b ^= seed;

    let (a2, b2) = rapid_mum::<PROTECTED>(a, b);
    a = a2;
    b = b2;
    (a, b, seed)
}

#[inline(always)]
pub(super) const fn rapidhash_finish<const PROTECTED: bool>(a: u64, b: u64, len: u64) -> u64 {
    rapid_mix::<PROTECTED>(a ^ RAPID_SECRET[0] ^ len, b ^ RAPID_SECRET[1])
}
