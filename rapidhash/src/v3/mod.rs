//! Portable hashing: rapidhash V3 algorithm.

mod rapid_const;
#[cfg(any(feature = "std", docsrs))]
mod rapid_file;
mod seed;

#[doc(inline)]
pub use rapid_const::*;

#[doc(inline)]
#[cfg(any(feature = "std", docsrs))]
pub use rapid_file::*;

#[doc(inline)]
pub use seed::*;

#[cfg(test)]
mod tests {
    extern crate std;

    use crate::util::macros::{compare_to_c, flip_bit_trial};
    use super::*;

    flip_bit_trial!(flip_bit_trial_v3, rapidhash_v3_inline::<true, false, false>);
    flip_bit_trial!(flip_bit_trial_v3_micro, rapidhash_v3_micro_inline::<true, false>);
    flip_bit_trial!(flip_bit_trial_v3_nano, rapidhash_v3_nano_inline::<true, false>);
    compare_to_c!(compare_to_c_v3, rapidhash_v3_inline::<true, false, false>, rapidhash_v3_inline::<true, true, false>, rapidhashcc_v3);
    compare_to_c!(compare_to_c_v3_micro, rapidhash_v3_micro_inline::<true, false>, rapidhash_v3_micro_inline::<true, false>, rapidhashcc_v3_micro);
    compare_to_c!(compare_to_c_v3_nano, rapidhash_v3_nano_inline::<true, false>, rapidhash_v3_nano_inline::<true, false>, rapidhashcc_v3_nano);
}
