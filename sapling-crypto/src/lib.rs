#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc))]

extern crate pairing;
extern crate bellman;
extern crate blake2_rfc;
extern crate digest;
extern crate rand;
extern crate byteorder;

#[cfg(test)]
#[macro_use]
extern crate hex_literal;

#[cfg(test)]
extern crate crypto;

#[macro_use]
extern crate parity_codec_derive;
extern crate parity_codec as codec;
// #[cfg(feature = "std")]
// extern crate serde;
// #[cfg(feature = "std")]
// #[macro_use]
// extern crate serde_derive;
#[macro_use]
#[cfg(not(feature = "std"))]
extern crate core;
extern crate sr_std as rstd;
extern crate sr_io as runtime_io;

pub mod jubjub;
pub mod group_hash;
pub mod circuit;
pub mod pedersen_hash;
pub mod primitives;
pub mod constants;
pub mod redjubjub;
pub mod util;
