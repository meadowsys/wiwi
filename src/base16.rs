// base 16 and hex are the same thing. lol
#[doc(inline)]
pub use crate::hex::{
	encode_hex as encode_base16,
	encode_hex_upper as encode_base16_upper,
	decode_hex as decode_base16,
	DecodeError,
	TABLE_ENCODER_LEN,
	TABLE_ENCODER_LOWER,
	TABLE_ENCODER_UPPER
};
