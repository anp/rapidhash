//! Aliases to export the latest version of rapidhash.

pub use crate::v2::rapidhash;
pub use crate::v2::rapidhash_inline;
pub use crate::v2::rapidhash_seeded;

#[cfg(any(feature = "std", docsrs))]
pub use crate::v2::rapidhash_file;
#[cfg(any(feature = "std", docsrs))]
pub use crate::v2::rapidhash_file_inline;
#[cfg(any(feature = "std", docsrs))]
pub use crate::v2::rapidhash_file_seeded;

pub use crate::v2::RAPID_SEED;

pub use crate::v2::RapidHasher;
pub use crate::v2::RapidBuildHasher;
pub use crate::v2::RapidHashMap;
pub use crate::v2::RapidHashSet;

pub use crate::v2::RapidInlineHasher;
pub use crate::v2::RapidInlineBuildHasher;
pub use crate::v2::RapidInlineHashMap;
pub use crate::v2::RapidInlineHashSet;

#[cfg(any(feature = "std", feature = "rand", docsrs))]
pub use crate::v2::RapidRandomState;
pub use crate::v2::RapidRng;
pub use crate::v2::rapidrng_fast;
#[cfg(any(feature = "std", docsrs))]
pub use crate::v2::rapidrng_time;
