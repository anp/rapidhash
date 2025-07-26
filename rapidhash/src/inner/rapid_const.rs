use crate::util::mix::{rapid_mix, rapid_mum};
use crate::util::read::{read_u32, read_u64};

/// The rapidhash default seed.
pub const RAPID_SEED: u64 = 0;

/// The rapidhash default secret parameters.
pub(super) const RAPID_SECRET: [u64; 7] = [
    0x2d358dccaa6c78a5,
    0x8bb84b93962eacc9,
    0x4b33a62ed433d4a3,
    0x4d5a2da51de1aa47,
    0xa0761d6478bd642f,
    0xe7037ed1a0b428db,
    0x90ed1765281c388c,
];

/// A fixed constant used in the rapidhash algorithm that in some instruction sets can be in the
/// assembly as a single instruction.
pub(super) const RAPID_CONST: u64 = 0xaaaaaaaaaaaaaaaa;

/// Rapidhash a single byte stream, matching the C++ implementation, with the default seed.
///
/// Fixed length inputs will greatly benefit from inlining with [rapidhash_rs_inline] instead.
#[inline]
#[cfg(test)]
pub(crate) const fn rapidhash_rs(data: &[u8]) -> u64 {
    rapidhash_rs_inline::<false, false>(data, RAPID_SEED)
}

/// Rapidhash a single byte stream, matching the C++ implementation, with a custom seed.
///
/// Fixed length inputs will greatly benefit from inlining with [rapidhash_rs_inline] instead.
#[inline]
#[cfg(test)]
pub(crate) const fn rapidhash_rs_seeded(data: &[u8], seed: u64) -> u64 {
    rapidhash_rs_inline::<false, false>(data, seed)
}

/// Rapidhash a single byte stream, matching the C++ implementation.
///
/// Is marked with `#[inline(always)]` to force the compiler to inline and optimise the method.
/// Can provide large performance uplifts for fixed-length inputs at compile time.
#[inline(always)]
#[cfg(test)]
pub(crate) const fn rapidhash_rs_inline<const COMPACT: bool, const PROTECTED: bool>(data: &[u8], mut seed: u64) -> u64 {
    seed = rapidhash_seed(seed);
    let secrets = &RAPID_SECRET;
    rapidhash_core::<true, COMPACT, PROTECTED>(seed, secrets, data)
}

#[inline(always)]
#[must_use]
pub(super) const fn rapidhash_seed(seed: u64) -> u64 {
    seed ^ rapid_mix::<false>(seed ^ RAPID_SECRET[2], RAPID_SECRET[1])
}

#[inline(always)]
#[must_use]
pub(super) const fn rapidhash_core<const AVALANCHE: bool, const COMPACT: bool, const PROTECTED: bool>(mut seed: u64, secrets: &[u64; 7], data: &[u8]) -> u64 {
    // TODO: benchmark without the a,b XOR -- eg. a oneshot
    if data.len() <= 16 {
        let mut a = 0;
        let mut b = 0;
        if data.len() >= 4 {
            if data.len() >= 8 {
                a ^= read_u64(data, 0);
                b ^= read_u64(data, data.len() - 8);
            } else {
                a ^= read_u32(data, 0) as u64;
                b ^= read_u32(data, data.len() - 4) as u64;
            }
        } else if !data.is_empty() {
            a ^= ((data[0] as u64) << 45) | data[data.len() - 1] as u64;
            b ^= data[data.len() >> 1] as u64;
        }

        seed = seed.wrapping_add(data.len() as u64);
        rapidhash_finish::<AVALANCHE, PROTECTED>(a, b , seed, secrets)
    } else {
        if data.len() <= 288 {
            // This can cause other code to not be inlined, and slow everything down. So at the cost of
            // marginally slower (-10%) 16..288 hashing,
            // NOT COMPACT: len is 16..=288
            rapidhash_core_16_288::<AVALANCHE, COMPACT, PROTECTED>(seed, secrets, data)
        } else {
            // len is >288, on a cold path to avoid inlining as this doesn't impact large strings, but
            // can otherwise prevent
            rapidhash_core_cold::<AVALANCHE, COMPACT, PROTECTED>(seed, secrets, data)
        }
    }
}

// TODO: review cold/inline(never)
// #[cold]
#[must_use]
// #[inline(never)]
const fn rapidhash_core_16_288<const AVALANCHE: bool, const COMPACT: bool, const PROTECTED: bool>(mut seed: u64, secrets: &[u64; 7], data: &[u8]) -> u64 {
    let mut a = 0;
    let mut b = 0;
    let mut slice = data;

    if slice.len() > 48 {
        // most CPUs appear to benefit from this unrolled loop
        let mut see1 = seed;
        let mut see2 = seed;

        while slice.len() >= 48 {
            seed = rapid_mix::<PROTECTED>(read_u64(slice, 0) ^ secrets[0], read_u64(slice, 8) ^ seed);
            see1 = rapid_mix::<PROTECTED>(read_u64(slice, 16) ^ secrets[1], read_u64(slice, 24) ^ see1);
            see2 = rapid_mix::<PROTECTED>(read_u64(slice, 32) ^ secrets[2], read_u64(slice, 40) ^ see2);
            let (_, split) = slice.split_at(48);
            slice = split;
        }

        seed ^= see1 ^ see2;
    }

    if slice.len() > 16 {
        seed = rapid_mix::<PROTECTED>(read_u64(slice, 0) ^ secrets[0], read_u64(slice, 8) ^ seed);
        if slice.len() > 32 {
            seed = rapid_mix::<PROTECTED>(read_u64(slice, 16) ^ secrets[1], read_u64(slice, 24) ^ seed);
        }
    }

    a ^= read_u64(data, data.len() - 16);
    b ^= read_u64(data, data.len() - 8);

    seed = seed.wrapping_add(data.len() as u64);
    rapidhash_finish::<AVALANCHE, PROTECTED>(a, b , seed, secrets)
}

/// The long path, intentionally kept cold because at this length of data the function call is
/// minor, but the complexity of this function — if it were inlined — could prevent x.hash() from
/// being inlined which would have a much higher penalty and prevent other optimisations.
#[cold]
#[inline(never)]
#[must_use]
const fn rapidhash_core_cold<const AVALANCHE: bool, const COMPACT: bool, const PROTECTED: bool>(mut seed: u64, secrets: &[u64; 7], data: &[u8]) -> u64 {
    let mut a = 0;
    let mut b = 0;
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
            seed = rapid_mix::<PROTECTED>(read_u64(slice, 0) ^ secrets[0], read_u64(slice, 8) ^ seed);
            see1 = rapid_mix::<PROTECTED>(read_u64(slice, 16) ^ secrets[1], read_u64(slice, 24) ^ see1);
            see2 = rapid_mix::<PROTECTED>(read_u64(slice, 32) ^ secrets[2], read_u64(slice, 40) ^ see2);
            see3 = rapid_mix::<PROTECTED>(read_u64(slice, 48) ^ secrets[3], read_u64(slice, 56) ^ see3);
            see4 = rapid_mix::<PROTECTED>(read_u64(slice, 64) ^ secrets[4], read_u64(slice, 72) ^ see4);
            see5 = rapid_mix::<PROTECTED>(read_u64(slice, 80) ^ secrets[5], read_u64(slice, 88) ^ see5);
            see6 = rapid_mix::<PROTECTED>(read_u64(slice, 96) ^ secrets[6], read_u64(slice, 104) ^ see6);

            seed = rapid_mix::<PROTECTED>(read_u64(slice, 112) ^ secrets[0], read_u64(slice, 120) ^ seed);
            see1 = rapid_mix::<PROTECTED>(read_u64(slice, 128) ^ secrets[1], read_u64(slice, 136) ^ see1);
            see2 = rapid_mix::<PROTECTED>(read_u64(slice, 144) ^ secrets[2], read_u64(slice, 152) ^ see2);
            see3 = rapid_mix::<PROTECTED>(read_u64(slice, 160) ^ secrets[3], read_u64(slice, 168) ^ see3);
            see4 = rapid_mix::<PROTECTED>(read_u64(slice, 176) ^ secrets[4], read_u64(slice, 184) ^ see4);
            see5 = rapid_mix::<PROTECTED>(read_u64(slice, 192) ^ secrets[5], read_u64(slice, 200) ^ see5);
            see6 = rapid_mix::<PROTECTED>(read_u64(slice, 208) ^ secrets[6], read_u64(slice, 216) ^ see6);

            let (_, split) = slice.split_at(224);
            slice = split;
        }

        if slice.len() >= 112 {
            seed = rapid_mix::<PROTECTED>(read_u64(slice, 0) ^ secrets[0], read_u64(slice, 8) ^ seed);
            see1 = rapid_mix::<PROTECTED>(read_u64(slice, 16) ^ secrets[1], read_u64(slice, 24) ^ see1);
            see2 = rapid_mix::<PROTECTED>(read_u64(slice, 32) ^ secrets[2], read_u64(slice, 40) ^ see2);
            see3 = rapid_mix::<PROTECTED>(read_u64(slice, 48) ^ secrets[3], read_u64(slice, 56) ^ see3);
            see4 = rapid_mix::<PROTECTED>(read_u64(slice, 64) ^ secrets[4], read_u64(slice, 72) ^ see4);
            see5 = rapid_mix::<PROTECTED>(read_u64(slice, 80) ^ secrets[5], read_u64(slice, 88) ^ see5);
            see6 = rapid_mix::<PROTECTED>(read_u64(slice, 96) ^ secrets[6], read_u64(slice, 104) ^ see6);
            let (_, split) = slice.split_at(112);
            slice = split;
        }
    } else {
        while slice.len() >= 112 {
            seed = rapid_mix::<PROTECTED>(read_u64(slice, 0) ^ secrets[0], read_u64(slice, 8) ^ seed);
            see1 = rapid_mix::<PROTECTED>(read_u64(slice, 16) ^ secrets[1], read_u64(slice, 24) ^ see1);
            see2 = rapid_mix::<PROTECTED>(read_u64(slice, 32) ^ secrets[2], read_u64(slice, 40) ^ see2);
            see3 = rapid_mix::<PROTECTED>(read_u64(slice, 48) ^ secrets[3], read_u64(slice, 56) ^ see3);
            see4 = rapid_mix::<PROTECTED>(read_u64(slice, 64) ^ secrets[4], read_u64(slice, 72) ^ see4);
            see5 = rapid_mix::<PROTECTED>(read_u64(slice, 80) ^ secrets[5], read_u64(slice, 88) ^ see5);
            see6 = rapid_mix::<PROTECTED>(read_u64(slice, 96) ^ secrets[6], read_u64(slice, 104) ^ see6);
            let (_, split) = slice.split_at(112);
            slice = split;
        }
    }

    if !COMPACT {
        if slice.len() >= 48 {
            seed = rapid_mix::<PROTECTED>(read_u64(slice, 0) ^ secrets[0], read_u64(slice, 8) ^ seed);
            see1 = rapid_mix::<PROTECTED>(read_u64(slice, 16) ^ secrets[1], read_u64(slice, 24) ^ see1);
            see2 = rapid_mix::<PROTECTED>(read_u64(slice, 32) ^ secrets[2], read_u64(slice, 40) ^ see2);
            let (_, split) = slice.split_at(48);
            slice = split;

            if slice.len() >= 48 {
                seed = rapid_mix::<PROTECTED>(read_u64(slice, 0) ^ secrets[0], read_u64(slice, 8) ^ seed);
                see1 = rapid_mix::<PROTECTED>(read_u64(slice, 16) ^ secrets[1], read_u64(slice, 24) ^ see1);
                see2 = rapid_mix::<PROTECTED>(read_u64(slice, 32) ^ secrets[2], read_u64(slice, 40) ^ see2);
                let (_, split) = slice.split_at(48);
                slice = split;
            }
        }
    } else {
        while slice.len() >= 48 {
            seed = rapid_mix::<PROTECTED>(read_u64(slice, 0) ^ secrets[0], read_u64(slice, 8) ^ seed);
            see1 = rapid_mix::<PROTECTED>(read_u64(slice, 16) ^ secrets[1], read_u64(slice, 24) ^ see1);
            see2 = rapid_mix::<PROTECTED>(read_u64(slice, 32) ^ secrets[2], read_u64(slice, 40) ^ see2);
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
        seed = rapid_mix::<PROTECTED>(read_u64(slice, 0) ^ secrets[2], read_u64(slice, 8) ^ seed);
        if slice.len() > 32 {
            seed = rapid_mix::<PROTECTED>(read_u64(slice, 16) ^ secrets[2], read_u64(slice, 24) ^ seed);
        }
    }

    a ^= read_u64(data, data.len() - 16);
    b ^= read_u64(data, data.len() - 8);

    seed = seed.wrapping_add(data.len() as u64);
    rapidhash_finish::<AVALANCHE, PROTECTED>(a, b , seed, secrets)
}

#[inline(always)]
#[must_use]
const fn rapidhash_finish<const AVALANCHE: bool, const PROTECTED: bool>(mut a: u64, mut b: u64, seed: u64, secrets: &[u64; 7]) -> u64 {
    a ^= secrets[1];
    b ^= seed;

    (a, b) = rapid_mum::<PROTECTED>(a, b);

    if AVALANCHE {
        // we keep RAPID_CONST constant as the XOR 0xaa can be done in a single instruction on some
        // platforms, whereas it would require an additional load for a random secret.
        rapid_mix::<PROTECTED>(a ^ RAPID_CONST ^ seed, b ^ secrets[1])
    } else {
        a ^ b
    }
}
