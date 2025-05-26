#![cfg_attr(docsrs, doc = include_str!("../../README.md"))]
#![cfg_attr(not(feature = "std"), no_std)]

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(docsrs, feature(doc_cfg_hide))]
#![cfg_attr(docsrs, doc(cfg_hide(docsrs)))]

#[deny(missing_docs)]
#[deny(unused_must_use)]

mod mix;
mod read;

/// Rapidhash V1 algorithm implementations.
pub mod v1;
/// Rapidhash V2 algorithm implementations.
pub mod v2;

/// Rapidhash rust trait implementations.
pub mod inner;

pub mod fast;
pub mod quality;
pub mod rng;
