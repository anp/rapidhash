//! Portable hashing: rapidhash V1 algorithm.

mod rapid_const;
#[cfg(any(feature = "std", docsrs))]
mod rapid_file;

#[doc(inline)]
pub use rapid_const::*;
#[doc(inline)]
#[cfg(any(feature = "std", docsrs))]
pub use rapid_file::*;

#[cfg(test)]
mod tests {
    extern crate std;

    use crate::util::macros::{compare_to_c, flip_bit_trial};
    use super::*;

    flip_bit_trial!(flip_bit_trial_v1, rapidhash_v1_inline::<false, false, false>);
    compare_to_c!(compare_to_c_v1, rapidhash_v1_inline::<false, false, false>, rapidhash_v1_inline::<true, false, false>, rapidhashcc_v1);

    #[test]
    fn test_hardcoded_v1() {
        assert_eq!(6516417773221693515, rapidhash_v1(&[]));
        assert_eq!(5006746792674864303, rapidhash_v1("\n".as_bytes()));
        assert_eq!(15965596575264898037, rapidhash_v1("something\n".as_bytes()));
        // below is 47 bytes, an extra character would hit the V1 bug
        assert_eq!(10644405912457645442, rapidhash_v1("abcdefghijklmnopqrstuvwxyz01234567890123456789\n".as_bytes()));
        assert_eq!(7545813847373533788, rapidhash_v1("abcdefghijklmnopqrstuvwxyz012345678901234567890abcdefghijklmnopqrstuvwxyz012345678901234567890abcdefghijklmnopqrstuvwxyz012345678901234567890\n".as_bytes()));
    }
}
