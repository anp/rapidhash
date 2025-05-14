#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(docsrs, feature(doc_cfg_hide))]
#![cfg_attr(docsrs, doc(cfg_hide(docsrs)))]

#[deny(missing_docs)]
#[deny(unused_must_use)]

/// Rapidhash V1 algorithm implementations. Only exposed with the `versioned` feature.
#[cfg(feature = "v1")]
pub mod v1;
/// Rapidhash V2 algorithm implementations (default). Only exposed with the `versioned` feature.
#[cfg(feature = "v2")]
pub mod v2;

/// Privately load v2 when exporting default aliases.
#[cfg(all(not(feature = "v2"), feature = "vlatest"))]
mod v2;

#[cfg(feature = "vlatest")]
mod latest;

#[cfg(feature = "vlatest")]
pub use latest::*;
