use super::{ _internal, Char };
use std::mem::transmute;
use std::ops::{ Deref, DerefMut };

#[repr(transparent)]
pub struct StrUtf8 {
	inner: [u8]
}

pub struct StringUtf8 {
	inner: Vec<u8>
}

impl StrUtf8 {
	#[inline]
	pub const fn from_utf8(code_units: &[u8]) -> Option<&Self> {
		if _internal::validate_utf8(code_units) {
			// SAFETY: we just validated
			Some(unsafe { Self::from_utf8_unchecked(code_units) })
		} else {
			None
		}
	}

	#[inline]
	pub fn from_utf8_mut(code_units: &mut [u8]) -> Option<&mut Self> {
		if _internal::validate_utf8(code_units) {
			// SAFETY: we just validated
			Some(unsafe { Self::from_utf8_unchecked_mut(code_units) })
		} else {
			None
		}
	}

	#[inline]
	pub const unsafe fn from_utf8_unchecked(utf8: &[u8]) -> &Self {
		// SAFETY: [u8] and Self have same layout
		unsafe { transmute(utf8) }
	}

	#[inline]
	pub unsafe fn from_utf8_unchecked_mut(utf8: &mut [u8]) -> &mut Self {
		// SAFETY: [u8] and Self have same layout
		unsafe { transmute(utf8) }
	}

	pub const fn from_std_str(s: &str) -> &Self {
		// SAFETY: `str` also requires that itself contain only valid UTF-8
		unsafe { Self::from_utf8_unchecked(s.as_bytes()) }
	}

	pub fn from_std_str_mut(s: &mut str) -> &mut Self {
		// SAFETY: we're only going to expose this as `Self`, and `Self` has same
		// invariant that it must contain valid UTF-8
		let cu = unsafe { s.as_bytes_mut() };

		// SAFETY: `str` also requires that itself contain only valid UTF-8
		unsafe { Self::from_utf8_unchecked_mut(cu) }
	}

	#[inline]
	pub const fn to_utf8_code_units(&self) -> &[u8] {
		// SAFETY: [u8] and Self have same layout
		unsafe { transmute(self) }
	}

	#[inline]
	pub unsafe fn to_utf8_code_units_mut(&mut self) -> &mut [u8] {
		// SAFETY: [u8] and Self have same layout
		unsafe { transmute(self) }
	}

	#[inline]
	pub const fn to_std_str(&self) -> &str {
		// SAFETY: `self` must contain valid UTF-8
		unsafe { std::str::from_utf8_unchecked(self.to_utf8_code_units()) }
	}

	#[inline]
	pub fn to_std_str_mut(&mut self) -> &mut str {
		// SAFETY: we're only going to expose this as `str`, and `str` also
		// has invariant that it must contain valid UTF-8
		let cu = unsafe { self.to_utf8_code_units_mut() };

		// SAFETY: `self` must contain valid UTF-8
		unsafe { std::str::from_utf8_unchecked_mut(cu) }
	}

	#[inline]
	pub const fn len_code_units(&self) -> usize {
		self.to_utf8_code_units().len()
	}

	#[inline]
	pub const fn is_empty(&self) -> bool {
		self.len_code_units() == 0
	}

	#[inline]
	pub const fn is_char_boundary(&self, index: usize) -> bool {
		// SAFETY: `to_utf8_code_units` returns valid UTF-8 code units
		// (well, `self` must be valid UTF-8)
		unsafe { _internal::is_char_boundary_utf8_unchecked(self.to_utf8_code_units(), index) }
	}

	#[inline]
	pub fn chars(&self) -> CharsUtf8<'_> {
		CharsUtf8::new(self)
	}
}

impl StringUtf8 {
	#[inline]
	pub const fn new() -> Self {
		Self { inner: Vec::new() }
	}

	#[inline]
	pub fn with_capacity(capacity: usize) -> Self {
		Self { inner: Vec::with_capacity(capacity) }
	}

	#[inline]
	pub fn push_char(&mut self, c: Char) {
		// SAFETY: Char is always valid unicode codepoint
		let cp = unsafe { _internal::codepoint_to_utf8_unchecked(c.to_u32()) };
		let cp = _internal::codepoint_utf8_to_slice(&cp);
		self.inner.extend_from_slice(cp);
	}
}

impl Default for StringUtf8 {
	#[inline]
	fn default() -> Self {
		Self::new()
	}
}

impl Deref for StringUtf8 {
	type Target = StrUtf8;

	#[inline]
	fn deref(&self) -> &StrUtf8 {
		// SAFETY: `self` must contain valid UTF-8
		unsafe { StrUtf8::from_utf8_unchecked(&self.inner) }
	}
}

impl DerefMut for StringUtf8 {
	#[inline]
	fn deref_mut(&mut self) -> &mut StrUtf8 {
		// SAFETY: `self` must contain valid UTF-8
		unsafe { StrUtf8::from_utf8_unchecked_mut(&mut self.inner) }
	}
}

impl FromIterator<Char> for StringUtf8 {
	#[inline]
	fn from_iter<T: IntoIterator<Item = Char>>(iter: T) -> Self {
		let mut this = Self::new();
		iter.into_iter().for_each(|c| this.push_char(c));
		this
	}
}

pub struct CharsUtf8<'h> {
	inner: _internal::CharsUtf8Raw<'h>
}

impl<'h> CharsUtf8<'h> {
	#[inline]
	fn new(s: &'h StrUtf8) -> Self {
		// SAFETY: `to_utf8_code_units` returns valid UTF-8 codepoint slice
		let inner = unsafe { _internal::new_chars_utf8_raw(s.to_utf8_code_units()) };
		Self { inner }
	}
}

impl<'h> Iterator for CharsUtf8<'h> {
	type Item = Char;

	#[inline]
	fn next(&mut self) -> Option<Char> {
		_internal::chars_utf8_raw_next(&mut self.inner)
			// SAFETY: `chars_utf8_raw_next` returns valid codepoints
			.map(|c| unsafe { Char::from_codepoint_unchecked(c) })
	}
}

impl<'h> DoubleEndedIterator for CharsUtf8<'h> {
	fn next_back(&mut self) -> Option<Char> {
		_internal::chars_utf8_raw_next_back(&mut self.inner)
			// SAFETY: `chars_utf8_raw_next_back` returns valid codepoints
			.map(|c| unsafe { Char::from_codepoint_unchecked(c) })
	}
}
