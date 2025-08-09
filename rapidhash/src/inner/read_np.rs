//! Internal module for reading unaligned bytes from a slice into `u64` and `u32` values.
//!
//! This is a non-portable implementation specifically designed for `RapidHasher`.

/// Hacky const-friendly memory-safe unaligned bytes to u64. Compiler can't seem to remove the
/// bounds check, and so we have an unsafe version behind the `unsafe` feature flag.
#[cfg(not(feature = "unsafe"))]
#[inline(always)]
pub(crate) const fn read_u64_np(slice: &[u8], offset: usize) -> u64 {
    // equivalent to slice[offset..offset+8].try_into().unwrap(), but const-friendly
    let maybe_buf = slice.split_at(offset).1.first_chunk::<8>();
    let buf = match maybe_buf {
        Some(buf) => *buf,
        None => panic!("read_u64: slice too short"),
    };
    u64::from_ne_bytes(buf)
}

/// Hacky const-friendly memory-safe unaligned bytes to u64. Compiler can't seem to remove the
/// bounds check, and so we have an unsafe version behind the `unsafe` feature flag.
#[cfg(not(feature = "unsafe"))]
#[inline(always)]
pub(crate) const fn read_u32_np(slice: &[u8], offset: usize) -> u32 {
    // equivalent to slice[offset..offset+4].try_into().unwrap(), but const-friendly
    let maybe_buf = slice.split_at(offset).1.first_chunk::<4>();
    let buf = match maybe_buf {
        Some(buf) => *buf,
        None => panic!("read_u32: slice too short"),
    };
    u32::from_ne_bytes(buf)
}

/// Unsafe but const-friendly unaligned bytes to u64. The compiler can't seem to remove the bounds
/// checks for small integers because we do some funky bit shifting in the indexing.
///
/// SAFETY: `slice` must be at least `offset+8` bytes long, which we guarantee in this rapidhash
/// implementation.
#[cfg(feature = "unsafe")]
#[inline(always)]
pub(crate) const fn read_u64_np(slice: &[u8], offset: usize) -> u64 {
    debug_assert!(offset as isize >= 0);
    debug_assert!(slice.len() >= 8 + offset);
    unsafe { core::ptr::read_unaligned(slice.as_ptr().offset(offset as isize) as *const u64) }
}

/// Unsafe but const-friendly unaligned bytes to u32. The compiler can't seem to remove the bounds
/// checks for small integers because we do some funky bit shifting in the indexing.
///
/// SAFETY: `slice` must be at least `offset+8` bytes long, which we guarantee in this rapidhash
/// implementation.
#[cfg(feature = "unsafe")]
#[inline(always)]
pub(crate) const fn read_u32_np(slice: &[u8], offset: usize) -> u32 {
    debug_assert!(offset as isize >= 0);
    debug_assert!(slice.len() >= 4 + offset);
    unsafe { core::ptr::read_unaligned(slice.as_ptr().offset(offset as isize) as *const u32) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_endian = "little")]
    #[test]
    fn test_read_u32_np() {
        let bytes = &[23, 145, 3, 34];

        let split_result = bytes.split_at(0).1;
        assert_eq!(split_result.len(), 4);
        let maybe_buf = split_result.first_chunk::<4>();
        assert_eq!(maybe_buf, Some(&[23, 145, 3, 34]));

        assert_eq!(read_u32_np(bytes, 0), 570659095);

        let bytes = &[24, 54, 3, 23, 145, 3, 34];
        assert_eq!(read_u32_np(bytes, 3), 570659095);

        assert_eq!(read_u32_np(&[0, 0, 0, 0], 0), 0);
        assert_eq!(read_u32_np(&[1, 0, 0, 0], 0), 1);
        assert_eq!(read_u32_np(&[12, 0, 0, 0], 0), 12);
        assert_eq!(read_u32_np(&[0, 10, 0, 0], 0), 2560);
    }

    #[cfg(target_endian = "little")]
    #[test]
    fn test_read_u64_np() {
        let bytes = [23, 145, 3, 34, 0, 0, 0, 0, 0, 0, 0].as_slice();
        assert_eq!(read_u64_np(bytes, 0), 570659095);

        let bytes = [1, 2, 3, 23, 145, 3, 34, 0, 0, 0, 0, 0, 0, 0].as_slice();
        assert_eq!(read_u64_np(bytes, 3), 570659095);

        let bytes = [0, 0, 0, 0, 0, 0, 0, 0].as_slice();
        assert_eq!(read_u64_np(bytes, 0), 0);
    }

    #[cfg(target_endian = "little")]
    #[cfg(feature = "std")]
    #[test]
    fn test_u32_to_u128_delta() {
        fn formula(len: u64) -> u64 {
            (len & 24) >> (len >> 3)
        }

        fn formula2(len: u64) -> u64 {
            match len {
                8.. => 4,
                _ => 0,
            }
        }

        let inputs: std::vec::Vec<u64> = (4..=16).collect();
        let outputs: std::vec::Vec<u64> = inputs.iter().map(|&x| formula(x)).collect();
        let expected = std::vec![0, 0, 0, 0, 4, 4, 4, 4, 4, 4, 4, 4, 4];
        assert_eq!(outputs, expected);
        assert_eq!(outputs, inputs.iter().map(|&x| formula2(x)).collect::<Vec<u64>>());
    }

    #[test]
    #[should_panic]
    #[cfg(any(test, not(feature = "unsafe")))]
    fn test_read_u32_np_to_short_panics() {
        let bytes = [23, 145, 0].as_slice();
        assert_eq!(read_u32_np(bytes, 0), 0);
    }

    #[test]
    #[should_panic]
    #[cfg(any(test, not(feature = "unsafe")))]
    fn test_read_u64_np_to_short_panics() {
        let bytes = [23, 145, 0].as_slice();
        assert_eq!(read_u64_np(bytes, 0), 0);
    }
}
