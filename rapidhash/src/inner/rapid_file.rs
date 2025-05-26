use std::io::Read;
use crate::mix::{rapid_mix, rapid_mum};
use crate::read::{read_u32, read_u64};
use crate::inner::rapid_const::{RAPID_SEED, RAPID_SECRET, rapidhash_finish, rapidhash_seed};

/// Rapidhash a file, matching the C++ implementation.
///
/// This method will check the metadata for a file length, and then stream the file with a
/// [BufReader] to compute the hash. This avoids loading the entire file into memory.
#[inline]
pub fn rapidhash_file<R: Read>(data: R) -> std::io::Result<u64> {
    rapidhash_file_inline::<R, false>(data, RAPID_SEED)
}

/// Rapidhash a file, matching the C++ implementation, with a custom seed.
///
/// This method will check the metadata for a file length, and then stream the file with a
/// [BufReader] to compute the hash. This avoids loading the entire file into memory.
#[inline]
pub fn rapidhash_file_seeded<R: Read>(data: R, seed: u64) -> std::io::Result<u64> {
    rapidhash_file_inline::<R, false>(data, seed)
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
pub fn rapidhash_file_inline<R: Read, const PROTECTED: bool>(data: R, mut seed: u64) -> std::io::Result<u64> {
    seed = rapidhash_seed(seed);
    let mut reader = ChunkedStreamReader::new(data, 16);
    let (a, b, seed) = rapidhash_file_core::<R, PROTECTED>(0, 0, seed, &mut reader)?;
    Ok(rapidhash_finish::<PROTECTED>(a, b, seed))
}

struct ChunkedStreamReader<R: Read> {
    reader: R,
    start: usize,
    end: usize,
    total_read: usize,
    buffer: Vec<u8>,
    last: Vec<u8>,
}

impl<R: Read> ChunkedStreamReader<R> {
    pub fn new(reader: R, keep_last: usize) -> Self {
        Self {
            reader,
            start: 0,
            end: 0,
            total_read: 0,
            buffer: vec![0; 8 * 1024],
            last: vec![0; keep_last],
        }
    }

    #[inline(always)]
    pub fn debug_invariants(&self) {
        debug_assert!(self.start <= self.end);
        debug_assert!(self.end <= self.buffer.len());
    }

    /// Returns the buffer size.
    pub fn fill_buffer(&mut self, chunk_size: usize) -> std::io::Result<usize> {
        self.debug_invariants();
        if chunk_size > self.buffer.len() {
            self.buffer.resize(chunk_size, 0);
        }

        let mut read_in_round = 0;

        while self.end - self.start < chunk_size {
            if self.buffer.len() - self.start < chunk_size {
                self.buffer.copy_within(self.start..self.end, 0);
                self.end -= self.start;
                self.start = 0;
            }

            let read = self.reader.read(&mut self.buffer[self.end..])?;
            if read == 0 {
                break;
            }
            read_in_round += read;
            self.end += read;
        }

        self.total_read += read_in_round;
        self.debug_invariants();
        Ok(read_in_round)
    }

    pub fn consume(&mut self, consume: usize) {
        self.debug_invariants();
        self.start += consume;
        self.debug_invariants();
        if self.start > self.end {
            self.start = self.end;
        }
    }

    /// Read a chunk of data, guaranteeing to return at least `chunk_size` unless the reader has
    /// reached the end. May return larger than `chunk_size` if available.
    pub fn read_chunk(&mut self, chunk_size: usize) -> std::io::Result<&[u8]> {
        let read = self.fill_buffer(chunk_size)?;

        if read > 0 {
            if read < self.last.len() {
                self.last.copy_within(read.., 0);
            }

            let read = read.min(self.last.len());
            let offset = self.last.len() - read;

            self.last[offset..].copy_from_slice(&self.buffer[self.end - read..self.end]);
        }

        Ok(&self.buffer[self.start..self.end])
    }

    pub fn last_read(&self) -> &[u8] {
        &self.last
    }

    pub fn total_read(&self) -> usize {
        self.total_read
    }
}

#[inline(always)]
fn rapidhash_file_core<R: Read, const PROTECTED: bool>(mut a: u64, mut b: u64, mut seed: u64, iter: &mut ChunkedStreamReader<R>) -> std::io::Result<(u64, u64, u64)> {
    let mut chunk = iter.read_chunk(300)?;
    let mut consumed = 0;

    if chunk.len() <= 16 {
        let len = chunk.len();
        if len >= 4 {
            if len >= 8 {
                let plast = len - 8;
                a = read_u64(chunk, 0);
                b = read_u64(chunk, plast);
            } else {
                let plast = len - 4;
                a = read_u32(chunk, 0) as u64;
                b = read_u32(chunk, plast) as u64;
            }
        } else if len > 0 {
            a = ((chunk[0] as u64) << 56) | chunk[chunk.len() - 1] as u64;
            b = chunk[chunk.len() >> 1] as u64;
        }
        consumed += chunk.len();
    } else if chunk.len() > 288 {
        // because we're using a buffered reader, it might be worth unrolling this loop further
        let mut see1 = seed;
        let mut see2 = seed;
        let mut see3 = seed;
        let mut see4 = seed;
        let mut see5 = seed;
        let mut see6 = seed;

        while chunk.len() >= 224 {
            seed = rapid_mix::<PROTECTED>(read_u64(chunk, 0) ^ RAPID_SECRET[0], read_u64(chunk, 8) ^ seed);
            see1 = rapid_mix::<PROTECTED>(read_u64(chunk, 16) ^ RAPID_SECRET[1], read_u64(chunk, 24) ^ see1);
            see2 = rapid_mix::<PROTECTED>(read_u64(chunk, 32) ^ RAPID_SECRET[2], read_u64(chunk, 40) ^ see2);
            see3 = rapid_mix::<PROTECTED>(read_u64(chunk, 48) ^ RAPID_SECRET[3], read_u64(chunk, 56) ^ see3);
            see4 = rapid_mix::<PROTECTED>(read_u64(chunk, 64) ^ RAPID_SECRET[4], read_u64(chunk, 72) ^ see4);
            see5 = rapid_mix::<PROTECTED>(read_u64(chunk, 80) ^ RAPID_SECRET[5], read_u64(chunk, 88) ^ see5);
            see6 = rapid_mix::<PROTECTED>(read_u64(chunk, 96) ^ RAPID_SECRET[6], read_u64(chunk, 104) ^ see6);

            seed = rapid_mix::<PROTECTED>(read_u64(chunk, 112) ^ RAPID_SECRET[0], read_u64(chunk, 120) ^ seed);
            see1 = rapid_mix::<PROTECTED>(read_u64(chunk, 128) ^ RAPID_SECRET[1], read_u64(chunk, 136) ^ see1);
            see2 = rapid_mix::<PROTECTED>(read_u64(chunk, 144) ^ RAPID_SECRET[2], read_u64(chunk, 152) ^ see2);
            see3 = rapid_mix::<PROTECTED>(read_u64(chunk, 160) ^ RAPID_SECRET[3], read_u64(chunk, 168) ^ see3);
            see4 = rapid_mix::<PROTECTED>(read_u64(chunk, 176) ^ RAPID_SECRET[4], read_u64(chunk, 184) ^ see4);
            see5 = rapid_mix::<PROTECTED>(read_u64(chunk, 192) ^ RAPID_SECRET[5], read_u64(chunk, 200) ^ see5);
            see6 = rapid_mix::<PROTECTED>(read_u64(chunk, 208) ^ RAPID_SECRET[6], read_u64(chunk, 216) ^ see6);

            iter.consume(224);
            consumed += 224;
            chunk = iter.read_chunk(224)?;
        }

        if chunk.len() >= 112 {
            seed = rapid_mix::<PROTECTED>(read_u64(chunk, 0) ^ RAPID_SECRET[0], read_u64(chunk, 8) ^ seed);
            see1 = rapid_mix::<PROTECTED>(read_u64(chunk, 16) ^ RAPID_SECRET[1], read_u64(chunk, 24) ^ see1);
            see2 = rapid_mix::<PROTECTED>(read_u64(chunk, 32) ^ RAPID_SECRET[2], read_u64(chunk, 40) ^ see2);
            see3 = rapid_mix::<PROTECTED>(read_u64(chunk, 48) ^ RAPID_SECRET[3], read_u64(chunk, 56) ^ see3);
            see4 = rapid_mix::<PROTECTED>(read_u64(chunk, 64) ^ RAPID_SECRET[4], read_u64(chunk, 72) ^ see4);
            see5 = rapid_mix::<PROTECTED>(read_u64(chunk, 80) ^ RAPID_SECRET[5], read_u64(chunk, 88) ^ see5);
            see6 = rapid_mix::<PROTECTED>(read_u64(chunk, 96) ^ RAPID_SECRET[6], read_u64(chunk, 104) ^ see6);

            consumed += 112;
            chunk = &chunk[112..chunk.len()];
        }

        if chunk.len() >= 48 {
            seed = rapid_mix::<PROTECTED>(read_u64(chunk, 0) ^ RAPID_SECRET[0], read_u64(chunk, 8) ^ seed);
            see1 = rapid_mix::<PROTECTED>(read_u64(chunk, 16) ^ RAPID_SECRET[1], read_u64(chunk, 24) ^ see1);
            see2 = rapid_mix::<PROTECTED>(read_u64(chunk, 32) ^ RAPID_SECRET[2], read_u64(chunk, 40) ^ see2);

            consumed += 48;
            chunk = &chunk[48..chunk.len()];

            if chunk.len() >= 48 {
                seed = rapid_mix::<PROTECTED>(read_u64(chunk, 0) ^ RAPID_SECRET[0], read_u64(chunk, 8) ^ seed);
                see1 = rapid_mix::<PROTECTED>(read_u64(chunk, 16) ^ RAPID_SECRET[1], read_u64(chunk, 24) ^ see1);
                see2 = rapid_mix::<PROTECTED>(read_u64(chunk, 32) ^ RAPID_SECRET[2], read_u64(chunk, 40) ^ see2);

                consumed += 48;
                chunk = &chunk[48..chunk.len()];
            }
        }

        see3 ^= see4;
        see5 ^= see6;
        seed ^= see1;
        see3 ^= see2;
        seed ^= see5;
        seed ^= see3;

        if chunk.len() > 16 {
            seed = rapid_mix::<PROTECTED>(read_u64(chunk, 0) ^ RAPID_SECRET[2], read_u64(chunk, 8) ^ seed);
            consumed += 16;
            if chunk.len() > 32 {
                seed = rapid_mix::<PROTECTED>(read_u64(chunk, 16) ^ RAPID_SECRET[2], read_u64(chunk, 24) ^ seed);
                consumed += 16;
            }
        }

        let last = iter.last_read();
        a ^= read_u64(&last, last.len() - 16);
        b ^= read_u64(&last, last.len() - 8);
    } else {
        if chunk.len() > 48 {
            // because we're using a buffered reader, it might be worth unrolling this loop further
            let mut see1 = seed;
            let mut see2 = seed;
            while chunk.len() >= 96 {
                seed = rapid_mix::<PROTECTED>(read_u64(chunk, 0) ^ RAPID_SECRET[0], read_u64(chunk, 8) ^ seed);
                see1 = rapid_mix::<PROTECTED>(read_u64(chunk, 16) ^ RAPID_SECRET[1], read_u64(chunk, 24) ^ see1);
                see2 = rapid_mix::<PROTECTED>(read_u64(chunk, 32) ^ RAPID_SECRET[2], read_u64(chunk, 40) ^ see2);
                seed = rapid_mix::<PROTECTED>(read_u64(chunk, 48) ^ RAPID_SECRET[0], read_u64(chunk, 56) ^ seed);
                see1 = rapid_mix::<PROTECTED>(read_u64(chunk, 64) ^ RAPID_SECRET[1], read_u64(chunk, 72) ^ see1);
                see2 = rapid_mix::<PROTECTED>(read_u64(chunk, 80) ^ RAPID_SECRET[2], read_u64(chunk, 88) ^ see2);

                consumed += 96;
                chunk = &chunk[96..];
            }

            if chunk.len() >= 48 {
                seed = rapid_mix::<PROTECTED>(read_u64(chunk, 0) ^ RAPID_SECRET[0], read_u64(chunk, 8) ^ seed);
                see1 = rapid_mix::<PROTECTED>(read_u64(chunk, 16) ^ RAPID_SECRET[1], read_u64(chunk, 24) ^ see1);
                see2 = rapid_mix::<PROTECTED>(read_u64(chunk, 32) ^ RAPID_SECRET[2], read_u64(chunk, 40) ^ see2);

                consumed += 48;
                chunk = &chunk[48..];
            }

            seed ^= see1 ^ see2;
        }

        if chunk.len() > 16 {
            seed = rapid_mix::<PROTECTED>(read_u64(chunk, 0) ^ RAPID_SECRET[0], read_u64(chunk, 8) ^ seed);
            consumed += 16;
            if chunk.len() > 32 {
                seed = rapid_mix::<PROTECTED>(read_u64(chunk, 16) ^ RAPID_SECRET[1], read_u64(chunk, 24) ^ seed);
                consumed += 16;
            }
        }

        let last = iter.last_read();
        a ^= read_u64(&last, last.len() - 16);
        b ^= read_u64(&last, last.len() - 8);
    }

    debug_assert!(iter.total_read() - consumed <= 16, "{consumed} bytes consumed");
    seed = seed.wrapping_add(iter.total_read() as u64);
    a ^= RAPID_SECRET[1];
    b ^= seed;

    let (a2, b2) = rapid_mum::<PROTECTED>(a, b);
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
                super::super::rapidhash(&data),
                rapidhash_file(&mut file).unwrap(),
                "Mismatch for input len: {}", &data.len()
            );
        }
    }
}
