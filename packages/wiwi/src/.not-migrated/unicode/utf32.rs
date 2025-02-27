use super::{ _internal, Char };
use std::mem::transmute;
use std::ops::{ Deref, DerefMut };

#[repr(transparent)]
pub struct StrUtf32 {
	inner: [u32]
}

pub struct StringUtf32 {
	inner: Vec<u32>
}

impl StrUtf32 {
	#[inline]
	pub const fn from_utf32(code_units: &[u32]) -> Option<&Self> {
		if _internal::validate_utf32(code_units) {
			// SAFETY: we just validated
			Some(unsafe { Self::from_utf32_unchecked(code_units) })
		} else {
			None
		}
	}

	#[inline]
	pub fn from_utf32_mut(code_units: &mut [u32]) -> Option<&mut Self> {
		if _internal::validate_utf32(code_units) {
			// SAFETY: we just validated
			Some(unsafe { Self::from_utf32_unchecked_mut(code_units) })
		} else {
			None
		}
	}

	#[inline]
	pub const unsafe fn from_utf32_unchecked(utf32: &[u32]) -> &Self {
		// SAFETY: [u32] and Self have same layout
		unsafe { transmute(utf32) }
	}

	#[inline]
	pub unsafe fn from_utf32_unchecked_mut(utf32: &mut [u32]) -> &mut Self {
		// SAFETY: [u32] and Self have same layout
		unsafe { transmute(utf32) }
	}

	#[inline]
	pub const fn to_utf32_code_units(&self) -> &[u32] {
		// SAFETY: [u32] and Self have same layout
		unsafe { transmute(self) }
	}

	#[inline]
	pub const fn len_code_units(&self) -> usize {
		self.to_utf32_code_units().len()
	}

	#[inline]
	pub const fn is_empty(&self) -> bool {
		self.len_code_units() == 0
	}

	#[inline]
	pub const fn is_char_boundary(&self, index: usize) -> bool {
		// SAFETY: `to_utf32_code_units` returns valid UTF-32 code units
		// (well, `self` must be valid UTF-32)
		unsafe { _internal::is_char_boundary_utf32_unchecked(self.to_utf32_code_units(), index) }
	}
}

impl StringUtf32 {
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
		let cp = unsafe { _internal::codepoint_to_utf32_unchecked(c.to_u32()) };
		let cp = _internal::codepoint_utf32_to_slice(&cp);
		self.inner.extend_from_slice(cp);
	}
}

impl Default for StringUtf32 {
	#[inline]
	fn default() -> Self {
		Self::new()
	}
}

impl Deref for StringUtf32 {
	type Target = StrUtf32;

	#[inline]
	fn deref(&self) -> &StrUtf32 {
		// SAFETY: `self` must contain valid UTF-32
		unsafe { StrUtf32::from_utf32_unchecked(&self.inner) }
	}
}

impl DerefMut for StringUtf32 {
	#[inline]
	fn deref_mut(&mut self) -> &mut StrUtf32 {
		// SAFETY: `self` must contain valid UTF-32
		unsafe { StrUtf32::from_utf32_unchecked_mut(&mut self.inner) }
	}
}

impl FromIterator<Char> for StringUtf32 {
	#[inline]
	fn from_iter<T: IntoIterator<Item = Char>>(iter: T) -> Self {
		let mut this = Self::new();
		iter.into_iter().for_each(|c| this.push_char(c));
		this
	}
}
