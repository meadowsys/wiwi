use crate::encoding_utils::UnsafeBufWriteGuard;
use ::std::{ ptr, slice };

pub(super) unsafe fn generic(
	mut bytes_ptr: *const u8,
	dest: &mut UnsafeBufWriteGuard,
	num_rounds: usize
) {
	const CHAR_A: u8 = b'a' - 10;
	const CHAR_0: u8 = b'0';

	for _ in 0..num_rounds {
		let byte = *bytes_ptr;
		bytes_ptr = bytes_ptr.add(1);

		let char1 = byte >> 4;
		let char2 = byte & 0xf;

		let chars = [
			if char1 > 9 { CHAR_A } else { CHAR_0 } + char1,
			if char2 > 9 { CHAR_A } else { CHAR_0 } + char2
		];

		let chars = &chars as *const [u8] as *const u8;
		dest.write_bytes_const::<2>(chars);
	}
}


// #[cfg(target_arch = "aarch64")]
// #[target_feature(enable = "neon")]
// unsafe fn _encode_neon_uint8x8(
// 	mut bytes_ptr: *const u8,
// 	mut dest_ptr: *mut u8,
// 	num_rounds: usize
// ) -> *const u8 {
// 	use ::std::arch::aarch64::*;
//
// 	let four_lower_bits = vdup_n_u8(0xf);
// 	let nine = vdup_n_u8(9);
// 	let char_0 = vdup_n_u8(b'0');
// 	let char_a = vdup_n_u8(b'a' - 10);
//
// 	for _ in 0..num_rounds {
// 		let vec = vld1_u8(bytes_ptr);
//
// 		let upper_vals = vshr_n_u8::<4>(vec);
// 		let lower_vals = vand_u8(vec, four_lower_bits);
//
// 		let upper_cmp = vcgt_u8(upper_vals, nine);
// 		let lower_cmp = vcgt_u8(lower_vals, nine);
//
// 		let upper = vbsl_u8(upper_cmp, char_a, char_0);
// 		let lower = vbsl_u8(lower_cmp, char_a, char_0);
//
// 		let upper = vadd_u8(upper_vals, upper);
// 		let lower = vadd_u8(lower_vals, lower);
//
// 		let zipped = vzip_u8(upper, lower);
// 		vst1_u8_x2(dest_ptr, zipped);
//
// 		bytes_ptr = bytes_ptr.add(8);
// 		dest_ptr = dest_ptr.add(16);
// 	}
//
// 	bytes_ptr
// }
