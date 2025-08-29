use crate::prelude::*;

pub use self::generic_fn::{
	encode,
	decode,

	Encode,
	Encoding,

	Base16,
	Base32,
	Base64,
	Hex,
	RFC1751,
	Z85,
};

mod generic_fn;

pub mod base16;
pub mod base32;
pub mod base64;
pub mod hex;
pub mod rfc1751;
pub mod z85;
