use crate::num_traits::*;
use super::colour;
use std::str;

static HEX_CHARS: &[u8; 16] = b"0123456789abcdef";

#[inline]
pub fn hex_from_argb(argb: u32, leading_hash_sign: bool) -> String {
	let mut s = if leading_hash_sign {
		let mut s = String::with_capacity(7);
		s.push('#');
		s
	} else {
		String::with_capacity(6)
	};

	hex_into(colour::red_from_argb(argb), &mut s, HEX_CHARS);
	hex_into(colour::green_from_argb(argb), &mut s, HEX_CHARS);
	hex_into(colour::blue_from_argb(argb), &mut s, HEX_CHARS);

	s
}

pub fn argb_from_hex(hex: &str) -> Option<u32> {
	let hex = match hex.as_bytes() {
		[b'#', hex @ ..] if hex.len() == 6 => { hex }
		hex if hex.len() == 6 => { hex }
		_ => { return None }
	};

	// SAFETY: we took this from a str slice, so this is fine
	let s = unsafe { str::from_utf8_unchecked(hex) };

	u32::from_str_radix(s, 16).ok()
}

pub fn hex(val: u8) -> String {
	let mut s = String::with_capacity(2);
	hex_into(val, &mut s, HEX_CHARS);
	s
}

fn hex_into(val: u8, s: &mut String, hex_chars: &[u8; 16]) {
	let mut chars_ptr = hex_chars.as_ptr();
	unsafe {
		let s = s.as_mut_vec();
		s.push(*chars_ptr.add((val >> 4).into_usize()));
		s.push(*chars_ptr.add((val & 0xf).into_usize()));
	}
}
