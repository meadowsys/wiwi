use super::_internal;
use std::fmt;

#[derive(Clone, Copy)]
pub struct Char {
	inner: u32
}

impl Char {
	#[inline]
	pub unsafe fn from_codepoint_unchecked(c: u32) -> Char {
		Char { inner: c }
	}

	#[inline]
	pub fn to_u32(self) -> u32 {
		self.inner
	}
}

impl fmt::Display for Char {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		// SAFETY: Char is always valid unicode codepoint
		let cp = unsafe { _internal::codepoint_to_utf8_unchecked(self.inner) };

		// SAFETY: Char is always valid unicode codepoint, so its encoded output is valid UTF-8
		f.write_str(unsafe { std::str::from_utf8_unchecked(_internal::codepoint_utf8_to_slice(&cp)) })
	}
}
