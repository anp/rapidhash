#![cfg_attr(docsrs, doc = include_str!("../../README.md"))]
#![cfg_attr(not(feature = "std"), no_std)]

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(docsrs, feature(doc_cfg_hide))]
#![cfg_attr(docsrs, doc(cfg_hide(docsrs)))]

#[deny(missing_docs)]
#[deny(unused_must_use)]

mod mix;
mod read;

/// Rapidhash V1 algorithm implementations. Only exposed with the `versioned` feature.
#[cfg(any(feature = "v1", docsrs))]
pub mod v1;
/// Rapidhash V2 algorithm implementations (default). Only exposed with the `versioned` feature.
#[cfg(any(feature = "v2", docsrs))]
pub mod v2;
/// Rapidhash V3 algorithm implementations (default). Only exposed with the `versioned` feature.
#[cfg(any(feature = "v3", docsrs))]
pub mod v3;

/// Privately load v2 when exporting default aliases.
#[cfg(all(feature = "vlatest", not(any(feature = "v2", docsrs))))]
pub(crate) mod v2;

// #[cfg(any(feature = "vlatest", docsrs))]
// mod latest;
//
// #[cfg(any(feature = "vlatest", docsrs))]
// pub use latest::*;

pub mod fast;
pub mod quality;
