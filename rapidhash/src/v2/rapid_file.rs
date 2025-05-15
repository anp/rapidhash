use std::fs::File;
use std::io::{BufReader, Read};
use crate::v2::rapid_const::{RAPID_SEED, RAPID_SECRET, rapid_mix, rapid_mum, rapidhash_finish, rapidhash_seed, read_u64, read_u32};

/// Rapidhash a file, matching the C++ implementation.
///
/// This method will check the metadata for a file length, and then stream the file with a
/// [BufReader] to compute the hash. This avoids loading the entire file into memory.
#[inline]
pub fn rapidhash_file(data: &mut File) -> std::io::Result<u64> {
    rapidhash_file_inline(data, RAPID_SEED)
}

/// Rapidhash a file, matching the C++ implementation, with a custom seed.
///
/// This method will check the metadata for a file length, and then stream the file with a
/// [BufReader] to compute the hash. This avoids loading the entire file into memory.
#[inline]
pub fn rapidhash_file_seeded(data: &mut File, seed: u64) -> std::io::Result<u64> {
    rapidhash_file_inline(data, seed)
}

/// Rapidhash a file, matching the C++ implementation.
///
/// This method will check the metadata for a file length, and then stream the file with a
/// [BufReader] to compute the hash. This avoids loading the entire file into memory.
///
/// We could easily add more ways to read other streams that can be converted to a [BufReader],
/// but the length must be known at the start of the stream due to how rapidhash is seeded using
/// the data length. Raise a [GitHub](https://github.com/hoxxep/rapidhash) issue if you have a
/// use case to support other stream types.
///
/// Is marked with `#[inline(always)]` to force the compiler to inline and optimise the method.
/// Can provide large performance uplifts for inputs where the length is known at compile time.
#[inline(always)]
pub fn rapidhash_file_inline(data: &mut File, mut seed: u64) -> std::io::Result<u64> {
    let len = data.metadata()?.len();
    let mut reader = BufReader::new(data);
    seed = rapidhash_seed(seed, len);
    let (a, b, _) = rapidhash_file_core(0, 0, seed, len as usize, &mut reader)?;
    Ok(rapidhash_finish(a, b, len))
}

#[inline(always)]
fn rapidhash_file_core(mut a: u64, mut b: u64, mut seed: u64, len: usize, iter: &mut BufReader<&mut File>) -> std::io::Result<(u64, u64, u64)> {
    if len <= 16 {
        let mut buf = [0u8; 16];
        iter.read_exact(&mut buf[0..len])?;
        let data = &buf[..len];

        if data.len() >= 4 {
            if data.len() >= 8 {
                let plast = data.len() - 8;
                a = read_u64(&data, 0);
                b = read_u64(&data, plast);
            } else {
                let plast = data.len() - 4;
                a = read_u32(&data, 0) as u64;
                b = read_u32(&data, plast) as u64;
            }
        } else if data.len() > 0 {
            a = ((data[0] as u64) << 56) | ((data[data.len() >> 1] as u64) << 32) | data[data.len() - 1] as u64;
        }
    } else if len > 56 {
        let mut remaining = len;
        let mut buf = [0u8; 448];

        // slice is a view on the buffer that we use for reading into, and reading from, depending
        // on the stage of the loop.
        let mut slice = &mut buf[..224];

        // because we're using a buffered reader, it might be worth unrolling this loop further
        let mut see1 = seed;
        let mut see2 = seed;
        let mut see3 = seed;
        let mut see4 = seed;
        let mut see5 = seed;
        let mut see6 = seed;

        while remaining >= 224 {
            // read into and process using the first half of the buffer
            iter.read_exact(&mut slice)?;

            seed = rapid_mix(read_u64(slice, 0) ^ RAPID_SECRET[0], read_u64(slice, 8) ^ seed);
            see1 = rapid_mix(read_u64(slice, 16) ^ RAPID_SECRET[1], read_u64(slice, 24) ^ see1);
            see2 = rapid_mix(read_u64(slice, 32) ^ RAPID_SECRET[2], read_u64(slice, 40) ^ see2);
            see3 = rapid_mix(read_u64(slice, 48) ^ RAPID_SECRET[3], read_u64(slice, 56) ^ see3);
            see4 = rapid_mix(read_u64(slice, 64) ^ RAPID_SECRET[4], read_u64(slice, 72) ^ see4);
            see5 = rapid_mix(read_u64(slice, 80) ^ RAPID_SECRET[5], read_u64(slice, 88) ^ see5);
            see6 = rapid_mix(read_u64(slice, 96) ^ RAPID_SECRET[6], read_u64(slice, 104) ^ see6);

            seed = rapid_mix(read_u64(slice, 112) ^ RAPID_SECRET[0], read_u64(slice, 120) ^ seed);
            see1 = rapid_mix(read_u64(slice, 128) ^ RAPID_SECRET[1], read_u64(slice, 136) ^ see1);
            see2 = rapid_mix(read_u64(slice, 144) ^ RAPID_SECRET[2], read_u64(slice, 152) ^ see2);
            see3 = rapid_mix(read_u64(slice, 160) ^ RAPID_SECRET[3], read_u64(slice, 168) ^ see3);
            see4 = rapid_mix(read_u64(slice, 176) ^ RAPID_SECRET[4], read_u64(slice, 184) ^ see4);
            see5 = rapid_mix(read_u64(slice, 192) ^ RAPID_SECRET[5], read_u64(slice, 200) ^ see5);
            see6 = rapid_mix(read_u64(slice, 208) ^ RAPID_SECRET[6], read_u64(slice, 216) ^ see6);

            remaining -= 224;
        }

        // remaining might be up to 224 bytes, so we read into the second half of the buffer,
        // which allows us to negative index safely in the final a and b xor using `end`.
        slice = &mut buf[224..224 + remaining];
        iter.read_exact(&mut slice)?;
        let end = 224 + remaining;

        if slice.len() >= 112 {
            seed = rapid_mix(read_u64(slice, 0) ^ RAPID_SECRET[0], read_u64(slice, 8) ^ seed);
            see1 = rapid_mix(read_u64(slice, 16) ^ RAPID_SECRET[1], read_u64(slice, 24) ^ see1);
            see2 = rapid_mix(read_u64(slice, 32) ^ RAPID_SECRET[2], read_u64(slice, 40) ^ see2);
            see3 = rapid_mix(read_u64(slice, 48) ^ RAPID_SECRET[3], read_u64(slice, 56) ^ see3);
            see4 = rapid_mix(read_u64(slice, 64) ^ RAPID_SECRET[4], read_u64(slice, 72) ^ see4);
            see5 = rapid_mix(read_u64(slice, 80) ^ RAPID_SECRET[5], read_u64(slice, 88) ^ see5);
            see6 = rapid_mix(read_u64(slice, 96) ^ RAPID_SECRET[6], read_u64(slice, 104) ^ see6);
            slice = &mut slice[112..remaining];
            remaining -= 112;
        }

        if remaining >= 48 {
            seed = rapid_mix(read_u64(slice, 0) ^ RAPID_SECRET[0], read_u64(slice, 8) ^ seed);
            see1 = rapid_mix(read_u64(slice, 16) ^ RAPID_SECRET[1], read_u64(slice, 24) ^ see1);
            see2 = rapid_mix(read_u64(slice, 32) ^ RAPID_SECRET[2], read_u64(slice, 40) ^ see2);
            slice = &mut slice[48..remaining];
            remaining -= 48;

            if remaining >= 48 {
                seed = rapid_mix(read_u64(slice, 0) ^ RAPID_SECRET[0], read_u64(slice, 8) ^ seed);
                see1 = rapid_mix(read_u64(slice, 16) ^ RAPID_SECRET[1], read_u64(slice, 24) ^ see1);
                see2 = rapid_mix(read_u64(slice, 32) ^ RAPID_SECRET[2], read_u64(slice, 40) ^ see2);
                slice = &mut slice[48..remaining];
                remaining -= 48;
            }
        }

        see3 ^= see4;
        see5 ^= see6;
        seed ^= see1;
        see3 ^= see2;
        seed ^= see5;
        seed ^= see3;

        if remaining > 16 {
            seed = rapid_mix(read_u64(slice, 0) ^ RAPID_SECRET[2], read_u64(slice, 8) ^ seed);
            if remaining > 32 {
                seed = rapid_mix(read_u64(slice, 16) ^ RAPID_SECRET[2], read_u64(slice, 24) ^ seed);
            }
        }

        a ^= read_u64(&buf, end - 16);
        b ^= read_u64(&buf, end - 8);
    } else {
        let data = &mut [0u8; 56];
        iter.read_exact(&mut data[0..len])?;
        let slice = &data[..len];

        seed = rapid_mix(read_u64(slice, 0) ^ RAPID_SECRET[0], read_u64(slice, 8) ^ seed);
        if slice.len() > 32 {
            seed = rapid_mix(read_u64(slice, 16) ^ RAPID_SECRET[1], read_u64(slice, 24) ^ seed);
            if slice.len() > 48 {
                seed = rapid_mix(read_u64(slice, 32) ^ RAPID_SECRET[0], read_u64(slice, 40) ^ seed);
            }
        }

        a = read_u64(slice, slice.len() - 16);
        b = read_u64(slice, slice.len() - 8);
    }

    a ^= RAPID_SECRET[1];
    b ^= seed;

    let (a2, b2) = rapid_mum(a, b);
    a = a2;
    b = b2;
    Ok((a, b, seed))
}

#[cfg(test)]
mod tests {
    use std::io::{Seek, SeekFrom, Write};
    use super::*;

    #[test]
    fn test_compare_rapidhash_file() {
        use rand::RngCore;

        const LENGTH: usize = 1024;
        for len in 1..=LENGTH {
            let mut data = vec![0u8; len];
            rand::rng().fill_bytes(&mut data);

            let mut file = tempfile::tempfile().unwrap();
            file.write(&data).unwrap();
            file.seek(SeekFrom::Start(0)).unwrap();

            assert_eq!(
                crate::v2::rapidhash(&data),
                rapidhash_file(&mut file).unwrap(),
                "Mismatch for input len: {}", &data.len()
            );
        }
    }
}
