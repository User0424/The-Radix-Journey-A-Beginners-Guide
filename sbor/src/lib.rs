#![cfg_attr(not(feature = "std"), no_std)]

/// SBOR constants.
pub mod constants;
/// SBOR type system.
pub mod types;
/// SBOR value system.
pub mod values;

mod decode;
mod describe;
mod encode;
mod parse;

extern crate alloc;
use alloc::vec::Vec;

pub use decode::{Decode, DecodeError, Decoder};
pub use describe::Describe;
pub use encode::{Encode, Encoder};
pub use parse::sbor_parse;

/// Encode a `T` into byte array.
pub fn sbor_encode<T: Encode>(v: &T) -> Vec<u8> {
    let mut enc = Encoder::with_metadata();
    v.encode(&mut enc);
    enc.into()
}

/// Decode an instance of `T` from a slice.
pub fn sbor_decode<'de, T: Decode>(buf: &'de [u8]) -> Result<T, DecodeError> {
    let mut dec = Decoder::with_metadata(buf);
    let v = T::decode(&mut dec);
    dec.check_end()?;
    v
}

/// Encode a `T` into byte array, with metadata stripped.
pub fn sbor_encode_no_metadata<T: Encode>(v: &T) -> Vec<u8> {
    let mut enc = Encoder::no_metadata();
    v.encode(&mut enc);
    enc.into()
}

/// Decode an instance of `T` from a slice which contains no metadata.
pub fn sbor_decode_no_metadata<'de, T: Decode>(buf: &'de [u8]) -> Result<T, DecodeError> {
    let mut dec = Decoder::no_metadata(buf);
    let v = T::decode(&mut dec);
    dec.check_end()?;
    v
}

// Re-export derives
extern crate sbor_derive;
pub use sbor_derive::*;

// This is to make derives work within this crate.
// See: https://users.rust-lang.org/t/how-can-i-use-my-derive-macro-from-the-crate-that-declares-the-trait/60502
extern crate self as sbor;