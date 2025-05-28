use crate::util::mix::{rapid_mix, rapid_mum};
use crate::util::read::{read_u32, read_u64};

/// The rapidhash default seed.
pub const RAPID_SEED: u64 = 0;

/// Rapidhash secret parameters.
pub(super) const RAPID_SECRET: [u64; 8] = [
    0x2d358dccaa6c78a5,
    0x8bb84b93962eacc9,
    0x4b33a62ed433d4a3,
    0x4d5a2da51de1aa47,
    0xa0761d6478bd642f,
    0xe7037ed1a0b428db,
    0x90ed1765281c388c,
    0xaaaaaaaaaaaaaaaa,
];

/// Rapidhash a single byte stream, matching the C++ implementation, with the default seed.
///
/// Fixed length inputs will greatly benefit from inlining with [rapidhash_rs_inline] instead.
#[inline]
pub(crate) const fn rapidhash_rs(data: &[u8]) -> u64 {
    rapidhash_rs_inline::<false, false>(data, RAPID_SEED)
}

/// Rapidhash a single byte stream, matching the C++ implementation, with a custom seed.
///
/// Fixed length inputs will greatly benefit from inlining with [rapidhash_rs_inline] instead.
#[inline]
pub(crate) const fn rapidhash_rs_seeded(data: &[u8], seed: u64) -> u64 {
    rapidhash_rs_inline::<false, false>(data, seed)
}

/// Rapidhash a single byte stream, matching the C++ implementation.
///
/// Is marked with `#[inline(always)]` to force the compiler to inline and optimise the method.
/// Can provide large performance uplifts for fixed-length inputs at compile time.
#[inline(always)]
pub(crate) const fn rapidhash_rs_inline<const COMPACT: bool, const PROTECTED: bool>(data: &[u8], mut seed: u64) -> u64 {
    seed = rapidhash_seed(seed);  // .wrapping_add(data.len() as u64);
    let (a, b, seed) = rapidhash_core::<COMPACT, PROTECTED>(0, 0, seed, data);
    rapidhash_finish::<PROTECTED>(a, b, seed)
}

#[inline(always)]
#[must_use]
pub(super) const fn rapidhash_seed(seed: u64) -> u64 {
    seed ^ rapid_mix::<false>(seed ^ RAPID_SECRET[2], RAPID_SECRET[1])
}

#[inline(always)]
#[must_use]
pub(super) const fn rapidhash_core<const COMPACT: bool, const PROTECTED: bool>(mut a: u64, mut b: u64, mut seed: u64, data: &[u8]) -> (u64, u64, u64) {
    // TODO: benchmark without the a,b XOR -- eg. a oneshot
    if data.len() <= 16 {
        if data.len() >= 4 {
            if data.len() >= 8 {
                a ^= read_u64(data, 0);
                b ^= read_u64(data, data.len() - 8);
            } else {
                a ^= read_u32(data, 0) as u64;
                b ^= read_u32(data, data.len() - 4) as u64;
            }
        } else if data.len() > 0 {
            a ^= ((data[0] as u64) << 45) | data[data.len() - 1] as u64;
            b ^= data[data.len() >> 1] as u64;
        }
    } else if data.len() <= 288 {
        // len is 16..=288
        (a, b, seed) = rapidhash_core_16_288::<COMPACT, PROTECTED>(a, b, seed, data);
    } else {
        // len is >288
        return rapidhash_core_cold::<COMPACT, PROTECTED>(a, b, seed, data);
    }

    seed = seed.wrapping_add(data.len() as u64);
    a ^= RAPID_SECRET[1];
    b ^= seed;

    (a, b) = rapid_mum::<PROTECTED>(a, b);
    (a, b, seed)
}

#[must_use]
const fn rapidhash_core_16_288<const COMPACT: bool, const PROTECTED: bool>(mut a: u64, mut b: u64, mut seed: u64, data: &[u8]) -> (u64, u64, u64) {
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
        seed = rapid_mix::<PROTECTED>(read_u64(slice, 0) ^ RAPID_SECRET[0], read_u64(slice, 8) ^ seed);
        if slice.len() > 32 {
            seed = rapid_mix::<PROTECTED>(read_u64(slice, 16) ^ RAPID_SECRET[1], read_u64(slice, 24) ^ seed);
        }
    }

    a ^= read_u64(data, data.len() - 16);
    b ^= read_u64(data, data.len() - 8);

    (a, b, seed)
}

/// The long path, intentionally kept cold because at this length of data the function call is
/// minor, but the complexity of this function — if it were inlined — could prevent x.hash() from
/// being inlined which would have a much higher penalty and prevent other optimisations.
#[cold]
#[inline(never)]
#[must_use]
const fn rapidhash_core_cold<const COMPACT: bool, const PROTECTED: bool>(mut a: u64, mut b: u64, mut seed: u64, data: &[u8]) -> (u64, u64, u64) {
    let mut slice = data;

    // most CPUs appear to benefit from this unrolled loop
    let mut see1 = seed;
    let mut see2 = seed;
    let mut see3 = seed;
    let mut see4 = seed;
    let mut see5 = seed;
    let mut see6 = seed;

    if !COMPACT {
        while slice.len() >= 224 {
            seed = rapid_mix::<PROTECTED>(read_u64(slice, 0) ^ RAPID_SECRET[0], read_u64(slice, 8) ^ seed);
            see1 = rapid_mix::<PROTECTED>(read_u64(slice, 16) ^ RAPID_SECRET[1], read_u64(slice, 24) ^ see1);
            see2 = rapid_mix::<PROTECTED>(read_u64(slice, 32) ^ RAPID_SECRET[2], read_u64(slice, 40) ^ see2);
            see3 = rapid_mix::<PROTECTED>(read_u64(slice, 48) ^ RAPID_SECRET[3], read_u64(slice, 56) ^ see3);
            see4 = rapid_mix::<PROTECTED>(read_u64(slice, 64) ^ RAPID_SECRET[4], read_u64(slice, 72) ^ see4);
            see5 = rapid_mix::<PROTECTED>(read_u64(slice, 80) ^ RAPID_SECRET[5], read_u64(slice, 88) ^ see5);
            see6 = rapid_mix::<PROTECTED>(read_u64(slice, 96) ^ RAPID_SECRET[6], read_u64(slice, 104) ^ see6);

            seed = rapid_mix::<PROTECTED>(read_u64(slice, 112) ^ RAPID_SECRET[0], read_u64(slice, 120) ^ seed);
            see1 = rapid_mix::<PROTECTED>(read_u64(slice, 128) ^ RAPID_SECRET[1], read_u64(slice, 136) ^ see1);
            see2 = rapid_mix::<PROTECTED>(read_u64(slice, 144) ^ RAPID_SECRET[2], read_u64(slice, 152) ^ see2);
            see3 = rapid_mix::<PROTECTED>(read_u64(slice, 160) ^ RAPID_SECRET[3], read_u64(slice, 168) ^ see3);
            see4 = rapid_mix::<PROTECTED>(read_u64(slice, 176) ^ RAPID_SECRET[4], read_u64(slice, 184) ^ see4);
            see5 = rapid_mix::<PROTECTED>(read_u64(slice, 192) ^ RAPID_SECRET[5], read_u64(slice, 200) ^ see5);
            see6 = rapid_mix::<PROTECTED>(read_u64(slice, 208) ^ RAPID_SECRET[6], read_u64(slice, 216) ^ see6);

            let (_, split) = slice.split_at(224);
            slice = split;
        }

        if slice.len() >= 112 {
            seed = rapid_mix::<PROTECTED>(read_u64(slice, 0) ^ RAPID_SECRET[0], read_u64(slice, 8) ^ seed);
            see1 = rapid_mix::<PROTECTED>(read_u64(slice, 16) ^ RAPID_SECRET[1], read_u64(slice, 24) ^ see1);
            see2 = rapid_mix::<PROTECTED>(read_u64(slice, 32) ^ RAPID_SECRET[2], read_u64(slice, 40) ^ see2);
            see3 = rapid_mix::<PROTECTED>(read_u64(slice, 48) ^ RAPID_SECRET[3], read_u64(slice, 56) ^ see3);
            see4 = rapid_mix::<PROTECTED>(read_u64(slice, 64) ^ RAPID_SECRET[4], read_u64(slice, 72) ^ see4);
            see5 = rapid_mix::<PROTECTED>(read_u64(slice, 80) ^ RAPID_SECRET[5], read_u64(slice, 88) ^ see5);
            see6 = rapid_mix::<PROTECTED>(read_u64(slice, 96) ^ RAPID_SECRET[6], read_u64(slice, 104) ^ see6);
            let (_, split) = slice.split_at(112);
            slice = split;
        }
    } else {
        while slice.len() >= 112 {
            seed = rapid_mix::<PROTECTED>(read_u64(slice, 0) ^ RAPID_SECRET[0], read_u64(slice, 8) ^ seed);
            see1 = rapid_mix::<PROTECTED>(read_u64(slice, 16) ^ RAPID_SECRET[1], read_u64(slice, 24) ^ see1);
            see2 = rapid_mix::<PROTECTED>(read_u64(slice, 32) ^ RAPID_SECRET[2], read_u64(slice, 40) ^ see2);
            see3 = rapid_mix::<PROTECTED>(read_u64(slice, 48) ^ RAPID_SECRET[3], read_u64(slice, 56) ^ see3);
            see4 = rapid_mix::<PROTECTED>(read_u64(slice, 64) ^ RAPID_SECRET[4], read_u64(slice, 72) ^ see4);
            see5 = rapid_mix::<PROTECTED>(read_u64(slice, 80) ^ RAPID_SECRET[5], read_u64(slice, 88) ^ see5);
            see6 = rapid_mix::<PROTECTED>(read_u64(slice, 96) ^ RAPID_SECRET[6], read_u64(slice, 104) ^ see6);
            let (_, split) = slice.split_at(112);
            slice = split;
        }
    }

    if !COMPACT {
        if slice.len() >= 48 {
            seed = rapid_mix::<PROTECTED>(read_u64(slice, 0) ^ RAPID_SECRET[0], read_u64(slice, 8) ^ seed);
            see1 = rapid_mix::<PROTECTED>(read_u64(slice, 16) ^ RAPID_SECRET[1], read_u64(slice, 24) ^ see1);
            see2 = rapid_mix::<PROTECTED>(read_u64(slice, 32) ^ RAPID_SECRET[2], read_u64(slice, 40) ^ see2);
            let (_, split) = slice.split_at(48);
            slice = split;

            if slice.len() >= 48 {
                seed = rapid_mix::<PROTECTED>(read_u64(slice, 0) ^ RAPID_SECRET[0], read_u64(slice, 8) ^ seed);
                see1 = rapid_mix::<PROTECTED>(read_u64(slice, 16) ^ RAPID_SECRET[1], read_u64(slice, 24) ^ see1);
                see2 = rapid_mix::<PROTECTED>(read_u64(slice, 32) ^ RAPID_SECRET[2], read_u64(slice, 40) ^ see2);
                let (_, split) = slice.split_at(48);
                slice = split;
            }
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

    see3 ^= see4;
    see5 ^= see6;
    seed ^= see1;
    see3 ^= see2;
    seed ^= see5;
    seed ^= see3;

    if slice.len() > 16 {
        seed = rapid_mix::<PROTECTED>(read_u64(slice, 0) ^ RAPID_SECRET[2], read_u64(slice, 8) ^ seed);
        if slice.len() > 32 {
            seed = rapid_mix::<PROTECTED>(read_u64(slice, 16) ^ RAPID_SECRET[2], read_u64(slice, 24) ^ seed);
        }
    }

    a ^= read_u64(data, data.len() - 16);
    b ^= read_u64(data, data.len() - 8);

    seed = seed.wrapping_add(data.len() as u64);

    a ^= RAPID_SECRET[1];
    b ^= seed;

    (a, b) = rapid_mum::<PROTECTED>(a, b);
    (a, b, seed)
}

#[inline(always)]
#[must_use]
pub(super) const fn rapidhash_finish<const PROTECTED: bool>(a: u64, b: u64, seed: u64) -> u64 {
    rapid_mix::<PROTECTED>(a ^ RAPID_SECRET[7] ^ seed, b ^ RAPID_SECRET[1])
}
