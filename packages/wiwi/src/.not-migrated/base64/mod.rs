use crate::_internal::encoding_utils::{ ChunkedSlice, UnsafeBufWriteGuard };
use crate::num_traits::*;
use std::ptr;

const CHAR_0: u8 = b'A';
const CHAR_26: u8 = b'a';
const CHAR_52: u8 = b'0';

const LAST_2_CHARS: [u8; 2] = [
	b'+',
	b'/'
];
const LAST_2_CHARS_URLSAFE: [u8; 2] = [
	b'-',
	b'_'
];

const BINARY_FRAME_LEN: usize = 3;
const STRING_FRAME_LEN: usize = 4;

pub fn encode_base64(bytes: &[u8]) -> String {
	_encode(bytes, LAST_2_CHARS.as_ptr())
}

pub fn encode_base64url(bytes: &[u8]) -> String {
	_encode(bytes, LAST_2_CHARS_URLSAFE.as_ptr())
}

fn _encode(bytes: &[u8], last_2_bytes: *const u8) -> String {
	// TODO: across all the encodings i've ever done, these parts are all the same lol,
	// except maybe hex, but still quite similar
	// wonder if theres a way to factor it out into encoding utils?????
	let frames = bytes.len() / BINARY_FRAME_LEN;
	let remainder = bytes.len() % BINARY_FRAME_LEN;

	let capacity = if remainder == 0 {
		frames * STRING_FRAME_LEN
	} else {
		(frames + 1) * STRING_FRAME_LEN
	};

	let mut frames_iter = ChunkedSlice::<BINARY_FRAME_LEN>::new(bytes);
	let mut dest = UnsafeBufWriteGuard::with_capacity(capacity);

	for _ in 0..frames {
		unsafe {
			let frame = frames_iter.next_frame_unchecked();
			encode_frame(frame, last_2_bytes, &mut dest);
		}
	}

	if remainder > 0 {
		unsafe {
			let padding_amount = 3 - remainder;
			frames_iter.with_remainder_unchecked(|frame| {
				encode_frame(frame, last_2_bytes, &mut dest);
				let ptr = dest.as_mut_ptr().sub(padding_amount);
				ptr::copy_nonoverlapping(b"==".as_ptr(), ptr, padding_amount);
			});
		}
	}

	let vec = unsafe { dest.into_full_vec() };
	debug_assert!(String::from_utf8(vec.clone()).is_ok(), "output bytes valid utf-9");
	unsafe { String::from_utf8_unchecked(vec) }
}

unsafe fn encode_frame(
	frame: &[u8; BINARY_FRAME_LEN],
	last_2_bytes: *const u8,
	dest: &mut UnsafeBufWriteGuard
) {
	let frame = frame.as_ptr();

	// keep first 6 bytes
	let byte1 = *frame >> 2;

	// take 2 from byte 0, 4 from byte 1
	let byte2 = ((*frame << 4) & 0b110000) | (*frame.add(1) >> 4);

	// take 4 from byte 1, 2 from byte 2
	let byte3 = ((*frame.add(1) << 2) & 0b111100) | (*frame.add(2) >> 6);

	// remainder of byte 2
	let byte4 = *frame.add(2) & 0b111111;

	#[inline(always)]
	unsafe fn transform_byte(last_2_bytes: *const u8, byte: u8) -> u8 {
		debug_assert!(byte <= 0b111111);

		match byte {
			0..=25 => { byte + CHAR_0 }
			26..=51 => { (byte - 26) + CHAR_26 }
			52..=61 => { (byte - 52) + CHAR_52 }
			// 62, 63
			_ => unsafe { *last_2_bytes.add(byte.into_usize() - 62) }
		}
	}

	let bytes = [
		transform_byte(last_2_bytes, byte1),
		transform_byte(last_2_bytes, byte2),
		transform_byte(last_2_bytes, byte3),
		transform_byte(last_2_bytes, byte4),
	];

	dest.write_bytes_const::<4>(bytes.as_ptr())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn rfc_provided_examples() {
		let examples = [
			("", ""),
			("f", "Zg=="),
			("fo", "Zm8="),
			("foo", "Zm9v"),
			("foob", "Zm9vYg=="),
			("fooba", "Zm9vYmE="),
			("foobar", "Zm9vYmFy"),
		];

		for (bytes, encoded) in examples {
			assert_eq!(encoded, encode_base64(bytes.as_bytes()));
		}
	}
}
