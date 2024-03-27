use crate::encoding_utils::UnsafeBufWriteGuard;
use ::std::{ ptr, slice };

pub const TABLE_ENCODER_LEN: usize = 16;
pub const TABLE_ENCODER_LOWER: [u8; TABLE_ENCODER_LEN] = *b"0123456789abcdef";
pub const TABLE_ENCODER_UPPER: [u8; TABLE_ENCODER_LEN] = *b"0123456789ABCDEF";

mod encode;
mod decode;

#[inline]
pub fn encode_hex(bytes: &[u8]) -> String {
	_encode::<false>(bytes)
}

#[inline]
pub fn encode_hex_upper(bytes: &[u8]) -> String {
	_encode::<true>(bytes)
}

// mut is used by cfg(target_arch) which might be inactive
#[allow(unused_mut)]
fn _encode<const UPPER: bool>(bytes: &[u8]) -> String {
	let bytes_len = bytes.len();
	let capacity = bytes_len * 2;

	let mut bytes_ptr = bytes as *const [u8] as *const u8;
	let mut dest = UnsafeBufWriteGuard::with_capacity(capacity);
	let mut rounds = bytes_len;

	#[cfg(target_arch = "aarch64")] {
		if ::std::arch::is_aarch64_feature_detected!("neon") {
			// we handle the big chunks, but leave enough info for the below generic
			// to continue the uneven chunks
			// divide by 16
			let neon_rounds = rounds >> 4;
			// mod 16
			let remainder = bytes_len & 0b1111;

			bytes_ptr = unsafe { encode::neon_uint8x16::<UPPER>(bytes_ptr, dest.as_ptr(), neon_rounds) };

			// multiply by 32
			// multiply by num rounds (^) times two, which is shift one more
			let amount_written = neon_rounds << 5;
			rounds = remainder;
			unsafe { dest.add_byte_count(amount_written) }
		}
	}

	unsafe { encode::generic::<UPPER>(bytes_ptr, &mut dest, rounds) };

	let vec = unsafe { dest.into_full_vec() };
	debug_assert!(String::from_utf8(vec.clone()).is_ok(), "output bytes are valid utf-8");
	unsafe { String::from_utf8_unchecked(vec) }
}

pub fn decode_hex(bytes: &[u8]) -> Result<Vec<u8>, DecodeError> {
	// AND 0b1 is chopping off all the other bits; last bit will
	// always be 0 or 1, depending on odd or even
	if bytes.len() & 0b1 != 0 { return Err(DecodeError::InvalidLength) }

	// shr 1 is same as div 2
	let capacity = bytes.len() >> 1;
	let mut dest = UnsafeBufWriteGuard::with_capacity(capacity);
	// num rounds is same as capacity, since each round outputs one byte.

	let bytes_ptr = bytes as *const [u8] as *const u8;

	unsafe { decode::generic(bytes_ptr, &mut dest, capacity)? }

	Ok(unsafe { dest.into_full_vec() })
}

#[derive(Debug, ::thiserror::Error)]
pub enum DecodeError {
	#[error("invalid length")]
	InvalidLength,
	#[error("invalid character")]
	InvalidChar
}

#[cfg(test)]
mod tests {
	use super::*;
	use ::rand::{ Rng, thread_rng };

	#[test]
	fn rfc_provided_examples() {
		let examples = [
			("", ""),
			("f", "66"),
			("fo", "666F"),
			("foo", "666F6F"),
			("foob", "666F6F62"),
			("fooba", "666F6F6261"),
			("foobar", "666F6F626172")
		];

		for (bytes, encoded) in examples {
			assert_eq!(encoded, encode_hex_upper(bytes.as_bytes()));
			assert_eq!(encoded.to_lowercase(), encode_hex(bytes.as_bytes()));
		}
	}

	#[test]
	fn randomised() {
		// (bytes_len, encoded_len)
		// (expected_input_len, expected_output_len)
		let expected_lengths = [
			(0usize, 0usize),
			(1, 2),
			(2, 4),
			(3, 6),
			(4, 8),
			(5, 10),
			(6, 12),
			(7, 14),
			(8, 16),
			(9, 18),
			(10, 20),
			(11, 22),
			(12, 24),
			(13, 26),
			(14, 28),
			(15, 30),
			(16, 32),
			(17, 34),
			(18, 36),
			(19, 38),
			(20, 40),

			(50, 100),
			(100, 200),
			(500, 1000),
			(1000, 2000),
			(100_000, 200_000),
			(1_000_000, 2_000_000),
		];
		let mut rng = thread_rng();

		for (expected_input_len, expected_output_len) in expected_lengths {
			for _ in 0usize..5 {
				let mut original_input = vec![0u8; expected_input_len];
				rng.fill(&mut *original_input);
				assert_eq!(original_input.len(), expected_input_len);

				let encoded_lower = encode_hex(&original_input);
				assert_eq!(encoded_lower.len(), expected_output_len);
				let encoded_upper = encode_hex_upper(&original_input);
				assert_eq!(encoded_upper.len(), expected_output_len);

				let decoded_lower = decode_hex(encoded_lower.as_bytes())
					.expect("can round trip decode just encoded data");
				assert_eq!(decoded_lower.len(), expected_input_len);
				assert_eq!(original_input, decoded_lower);

				let decoded_upper = decode_hex(encoded_upper.as_bytes())
					.expect("can round trip decode just encoded data");
				assert_eq!(decoded_upper.len(), expected_input_len);
				assert_eq!(original_input, decoded_upper);

				// this is enforced by debug_assert! in the code, so this already
				// is validated if tests are run in debug, but still,
				assert_eq!(encoded_lower.len(), encoded_lower.capacity());
				assert_eq!(decoded_lower.len(), decoded_lower.capacity());
				assert_eq!(encoded_upper.len(), encoded_upper.capacity());
				assert_eq!(decoded_upper.len(), decoded_upper.capacity());
			}
		}
	}

	#[test]
	fn hex_crate_compat() {
		let mut rng = thread_rng();

		let mut bytes = vec![0u8; 1000];
		rng.fill(&mut *bytes);
		let bytes = &*bytes;

		let wiwi_encoded = encode_hex(bytes);
		let hex_encoded = ::hex::encode(bytes);
		assert_eq!(wiwi_encoded, hex_encoded);

		let wiwi_decoded_hex = decode_hex(hex_encoded.as_bytes())
			.expect("wiwi can decode hex");
		let hex_decoded_wiwi = ::hex::decode(wiwi_encoded.as_bytes())
			.expect("hex can decode wiwi");

		assert_eq!(wiwi_decoded_hex, hex_decoded_wiwi);
	}

	// #[test]
	// fn test_neon_impl() {
	// 	const IN_SIZE: usize = 1024 * 1024;
	// 	const NUM_ROUNDS: usize = IN_SIZE / 8;

	// 	let mut rng = thread_rng();

	// 	let mut bytes = vec![0u8; IN_SIZE];
	// 	rng.fill(&mut *bytes);
	// 	let bytes = &*bytes;

	// 	let regular_encoded = encode_hex(bytes);
	// 	let neon_encoded = unsafe {
	// 		let capacity = bytes.len() * 2;
	// 		let mut dest = Vec::with_capacity(capacity);
	// 		_encode_neon_uint8x8(
	// 			bytes as *const [u8] as *const u8,
	// 			dest.as_mut_ptr(),
	// 			NUM_ROUNDS
	// 		);
	// 		dest.set_len(capacity);
	// 		unsafe { String::from_utf8_unchecked(dest) }
	// 	};

	// 	assert_eq!(regular_encoded, neon_encoded);
	// }
}
