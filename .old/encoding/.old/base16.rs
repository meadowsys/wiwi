use crate::prelude::*;
use super::hex::{
	encode_hex,
	encode_hex_upper,
	decode_hex
};

pub use super::hex::{
	DecodeError,
	TABLE_ENCODER_LEN,
	TABLE_ENCODER_LOWER,
	TABLE_ENCODER_UPPER
};

#[inline]
pub fn encode_base16(bytes: &[u8]) -> String {
	encode_hex(bytes)
}

#[inline]
pub fn encode_base16_upper(bytes: &[u8]) -> String {
	encode_hex_upper(bytes)
}

#[inline]
pub fn decode_base16(bytes: &[u8]) -> Result<Vec<u8>, DecodeError> {
	decode_hex(bytes)
}
