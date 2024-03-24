use crate::encoding_utils::UnsafeBufWriteGuard;
use ::std::{ ptr, slice };

pub(super) unsafe fn generic<const UPPER: bool>(
	mut bytes_ptr: *const u8,
	dest: &mut UnsafeBufWriteGuard,
	num_rounds: usize
) {
	let char_a = if UPPER { b'A' } else { b'a' } - 10;
	let char_0 = b'0';

	for _ in 0..num_rounds {
		let byte = *bytes_ptr;
		bytes_ptr = bytes_ptr.add(1);

		let char1 = byte >> 4;
		let char2 = byte & 0xf;

		let chars = [
			if char1 > 9 { char_a } else { char_0 } + char1,
			if char2 > 9 { char_a } else { char_0 } + char2
		];

		let chars = &chars as *const [u8] as *const u8;
		dest.write_bytes_const::<2>(chars);
	}
}

/// num_rounds should be equivalent to number of bytes divided by 16.
/// This function processes 16 bytes at a time
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
pub(super) unsafe fn neon_uint8x16<const UPPER: bool>(
	mut bytes_ptr: *const u8,
	mut dest_ptr: *mut u8,
	rounds: usize
) -> *const u8 {
	use ::std::arch::aarch64::*;

	let four_lower_bits = vdupq_n_u8(0xf);
	let nine = vdupq_n_u8(9);
	let char_a = vdupq_n_u8(if UPPER { b'A' } else { b'a' } - 10);
	let char_0 = vdupq_n_u8(b'0');

	for _ in 0..rounds {
		// load 16 u8 vals
		let vec = vld1q_u8(bytes_ptr);

		// get upper 4 bits and lower 4 bits
		// into 2 seperate vecs
		let upper_vals = vshrq_n_u8::<4>(vec);
		let lower_vals = vandq_u8(vec, four_lower_bits);

		// compare the vec with 9 (where transition to chars happens,
		// so needs 2 seperate char ranges)
		let upper_cmp = vcgtq_u8(upper_vals, nine);
		let lower_cmp = vcgtq_u8(lower_vals, nine);

		// add A or 0 to base depending on if gt 9 or not
		let upper = vbslq_u8(upper_cmp, char_a, char_0);
		let lower = vbslq_u8(lower_cmp, char_a, char_0);

		// add values to base
		let upper = vaddq_u8(upper_vals, upper);
		let lower = vaddq_u8(lower_vals, lower);

		// zip bytes together
		let zipped = vzipq_u8(upper, lower);
		// write to out ptr
		vst1q_u8_x2(dest_ptr, zipped);

		bytes_ptr = bytes_ptr.add(16);
		dest_ptr = dest_ptr.add(32);
	}

	bytes_ptr
}
