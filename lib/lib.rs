//! #xp3
//!
//! Decoder and extractor of .xp3 archives.

pub(crate) mod cxdec;
pub(crate) mod utils;

pub(crate) mod file;
pub(crate) mod header;
pub(crate) mod info;
pub(crate) mod segment;
pub(crate) mod solve;

pub use solve::Xp3;
