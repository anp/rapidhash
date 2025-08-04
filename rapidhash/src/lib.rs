#![cfg_attr(docsrs, doc = include_str!("../README.md"))]
#![cfg_attr(not(docsrs), doc = "# Rapidhash")]
#![cfg_attr(not(feature = "std"), no_std)]

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(docsrs, feature(doc_cfg_hide))]
#![cfg_attr(docsrs, doc(cfg_hide(docsrs)))]

// note: we don't #![deny(unsafe_code)] as seeding.rs uses unsafe code to initialise the static
// secrets, but we're confident about the safety of that code (and it cannot interact with
// user-generated values).
#![deny(missing_docs)]
#![deny(unused_must_use)]
#![allow(clippy::manual_hash_one)]

pub(crate) mod util;

pub mod v1;
pub mod v2;
pub mod v3;

pub mod inner;
pub mod fast;
pub mod quality;

pub mod rng;
