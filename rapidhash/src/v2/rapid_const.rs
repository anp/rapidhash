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

/// Rapidhash V2.2 a single byte stream, matching the C++ implementation, with the default seed.
///
/// See [rapidhash_v2_inline] to compute the hash value using V2.0 or V2.2.
///
/// Fixed length inputs will greatly benefit from inlining with [rapidhash_inline] instead.
#[inline]
pub const fn rapidhash_v2_2(data: &[u8]) -> u64 {
    rapidhash_v2_inline::<2, false, false>(data, RAPID_SEED)
}

/// Rapidhash V2.2 a single byte stream, matching the C++ implementation, with a custom seed.
///
/// See [rapidhash_v2_inline] to compute the hash value using V2.0 or V2.2.
///
/// Fixed length inputs will greatly benefit from inlining with [rapidhash_inline] instead.
#[inline]
pub const fn rapidhash_v2_2_seeded(data: &[u8], seed: u64) -> u64 {
    rapidhash_v2_inline::<2, false, false>(data, seed)
}

/// Rapidhash V2 a single byte stream, matching the C++ implementation.
///
/// Is marked with `#[inline(always)]` to force the compiler to inline and optimise the method.
/// Can provide large performance uplifts for fixed-length inputs at compile time.
///
/// Compile time arguments:
/// - `COMPACT`: Generates fewer instructions at compile time with less manual loop unrolling, but
///     may be slower on some platforms. Disabled by default.
/// - `PROTECTED`: Slightly stronger hash quality and DoS resistance by performing two extra XOR
///     instructions on every mix step. Disabled by default.
/// - `MINOR`: the minor version of the rapidhash algorithm:
///     - 0: v2.0
///     - 1: v2.1
///     - 2: v2.2
#[inline(always)]
pub const fn rapidhash_v2_inline<const MINOR: u8, const COMPACT: bool, const PROTECTED: bool>(data: &[u8], mut seed: u64) -> u64 {
    seed = rapidhash_seed(seed) ^ data.len() as u64;
    let (a, b, _) = rapidhash_core::<MINOR, COMPACT, PROTECTED>(0, 0, seed, data);
    rapidhash_finish::<PROTECTED>(a, b, data.len() as u64)
}

#[inline(always)]
pub(super) const fn rapidhash_seed(seed: u64) -> u64 {
    seed ^ rapid_mix::<false>(seed ^ RAPID_SECRET[2], RAPID_SECRET[1])
}

#[inline(always)]
pub(super) const fn rapidhash_core<const MINOR: u8, const COMPACT: bool, const PROTECTED: bool>(mut a: u64, mut b: u64, mut seed: u64, data: &[u8]) -> (u64, u64, u64) {
    if MINOR > 2 {
        panic!("rapidhash_core unsupported minor version. Supported versions are 0, 1, and 2.");
    }

    if data.len() <= 16 {
        if data.len() >= 4 {
            if data.len() >= 8 {
                let plast = data.len() - 8;
                a ^= read_u64(data, 0);
                b ^= read_u64(data, plast);
            } else {
                let plast = data.len() - 4;
                a ^= read_u32(data, 0) as u64;
                b ^= read_u32(data, plast) as u64;
            }
        } else if !data.is_empty() {
            if MINOR < 2 {
                a ^= ((data[0] as u64) << 56) | ((data[data.len() >> 1] as u64) << 32) | data[data.len() - 1] as u64;
            } else {
                a ^= ((data[0] as u64) << 56) | data[data.len() - 1] as u64;
                b ^= data[data.len() >> 1] as u64;
            }
        }
    } else if (MINOR == 0 && data.len() <= 56) || (MINOR > 0 && data.len() <= 64) {
        // len is 17..=64
        (a, b, seed) = rapidhash_core_17_64::<MINOR, PROTECTED>(a, b, seed, data);
    } else {
        (a, b, seed) = rapidhash_core_cold::<COMPACT, PROTECTED>(a, b, seed, data);
    }

    a ^= RAPID_SECRET[1];
    b ^= seed;

    (a, b) = rapid_mum::<PROTECTED>(a, b);
    (a, b, seed)
}

#[inline]  // intentionally not always
const fn rapidhash_core_17_64<const MINOR: u8, const PROTECTED: bool>(mut a: u64, mut b: u64, mut seed: u64, data: &[u8]) -> (u64, u64, u64) {
    let slice = data;

    seed = rapid_mix::<PROTECTED>(read_u64(slice, 0) ^ RAPID_SECRET[0], read_u64(slice, 8) ^ seed);
    if slice.len() > 32 {
        seed = rapid_mix::<PROTECTED>(read_u64(slice, 16) ^ RAPID_SECRET[1], read_u64(slice, 24) ^ seed);
        if slice.len() > 48 {
            let index: usize = if MINOR < 2 { 0 } else { 1 };
            seed = rapid_mix::<PROTECTED>(read_u64(slice, 32) ^ RAPID_SECRET[index], read_u64(slice, 40) ^ seed);
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

    (a, b, seed)
}

#[inline(always)]
pub(super) const fn rapidhash_finish<const PROTECTED: bool>(a: u64, b: u64, len: u64) -> u64 {
    rapid_mix::<PROTECTED>(a ^ RAPID_SECRET[7] ^ len, b ^ RAPID_SECRET[1])
}
