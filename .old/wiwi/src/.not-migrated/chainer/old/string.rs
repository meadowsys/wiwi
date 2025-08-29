use crate::to_maybeuninit::ToMaybeUninit as _;
use std::mem::MaybeUninit;
use std::ops::{ Bound, Range, RangeBounds };
use std::string;
use super::{ IntoChainer, VecChain };

#[repr(transparent)]
pub struct StringChain {
	inner: String
}

impl StringChain {
	pub unsafe fn from_raw_parts(buf: *mut u8, length: usize, capacity: usize) -> Self {
		String::from_raw_parts(buf, length, capacity).into()
	}

	pub fn from_utf8(vec: Vec<u8>) -> Result<Self, string::FromUtf8Error> {
		String::from_utf8(vec).map(Into::into)
	}

	pub unsafe fn from_utf8_unchecked(bytes: Vec<u8>) -> Self {
		String::from_utf8_unchecked(bytes).into()
	}

	// pub fn from_utf8_lossy(v: &[u8]) -> Cow<str>

	// TODO: provide reasoning this is Option instead of Result with FromUtf16Error
	pub fn from_utf16(v: &[u16]) -> Option<Self> {
		String::from_utf16(v).ok().map(Into::into)
	}

	pub fn from_utf16_lossy(v: &[u16]) -> Self {
		String::from_utf16_lossy(v).into()
	}

	// pub fn from_utf16le(v: &[u8]) -> Option<Self> {
	// 	if v.len() % 2 != 0 { return None }
	//
	// 	match (cfg!(target_endian = "little"), unsafe { v.align_to::<u16>() }) {
	// 		(true, ([], v, [])) => { Self::from_utf16(v) }
	// 		_ => {
	// 			// need array chunks impl
	// 			// char::decode_utf16(v.into_chainer())
	// 		}
	// 	}
	// }
	// pub fn from_utf16le_lossy
	// pub fn from_utf16be
	// pub fn from_utf16be_lossy

	pub fn new() -> Self {
		String::new().into()
	}

	pub fn with_capacity(capacity: usize) -> Self {
		String::with_capacity(capacity).into()
	}

	// TODO: nightly try_with_capacity
}

impl StringChain {
	pub fn into_bytes(self) -> Vec<u8> {
		self.inner.into_bytes()
	}

	pub fn into_bytes_chainer(self) -> VecChain<u8> {
		self.into_bytes().into()
	}

	pub fn into_raw_parts(self) -> (*mut u8, usize, usize) {
		// std's into_raw_parts is unstable, so chainer shall provide
		self.inner
			.into_bytes()
			.into_chainer()
			.nonchain_raw_parts()
	}

	pub fn nonchain_bytes(&self) -> &[u8] {
		self.inner.as_bytes()
	}

	pub fn nonchain_str(&self) -> &str {
		&self.inner
	}

	pub fn nonchain_str_mut(&mut self) -> &mut str {
		&mut self.inner
	}

	pub unsafe fn nonchain_vec_mut(&mut self) -> &mut Vec<u8> {
		self.inner.as_mut_vec()
	}
}

impl StringChain {
	pub fn capacity(self, out: &mut usize) -> Self {
		*out = self.inner.capacity();
		self
	}

	pub fn capacity_uninit(self, out: &mut MaybeUninit<usize>) -> Self {
		out.write(self.inner.capacity());
		self
	}

	pub fn clear(mut self) -> Self {
		self.inner.clear();
		self
	}

	pub fn extend_from_within<R>(mut self, src: R) -> Self
	where
		R: RangeBounds<usize>
	{
		let start = match src.start_bound() {
			Bound::Included(start) => { *start }
			Bound::Excluded(start) => { start.checked_add(1).expect("range start overflow") }
			Bound::Unbounded => { 0 }
		};

		let end = match src.end_bound() {
			Bound::Included(end) => { end.checked_add(1).expect("range end overflow") }
			Bound::Excluded(end) => { *end }
			Bound::Unbounded => { self.inner.len() }
		};

		assert!(self.inner.is_char_boundary(start));
		assert!(self.inner.is_char_boundary(end));

		unsafe { self.nonchain_vec_mut().extend_from_within(Range { start, end }) }
		self
	}

	pub fn insert(mut self, idx: usize, ch: char) -> Self {
		self.inner.insert(idx, ch);
		self
	}

	pub fn insert_str(mut self, idx: usize, string: &str) -> Self {
		self.inner.insert_str(idx, string);
		self
	}

	pub fn is_empty(self, out: &mut bool) -> Self {
		*out = self.inner.is_empty();
		self
	}

	pub fn is_empty_uninit(self, out: &mut MaybeUninit<bool>) -> Self {
		out.write(self.inner.is_empty());
		self
	}

	pub fn len(self, out: &mut usize) -> Self {
		*out = self.inner.len();
		self
	}

	pub fn len_uninit(self, out: &mut MaybeUninit<usize>) -> Self {
		out.write(self.inner.len());
		self
	}

	pub fn pop(mut self, out: &mut Option<char>) -> Self {
		*out = self.inner.pop();
		self
	}

	pub fn pop_uninit(mut self, out: &mut MaybeUninit<Option<char>>) -> Self {
		out.write(self.inner.pop());
		self
	}

	pub fn push(mut self, ch: char) -> Self {
		self.inner.push(ch);
		self
	}

	pub fn push_str(mut self, string: &str) -> Self {
		self.inner.push_str(string);
		self
	}

	pub fn remove(mut self, idx: usize, out: &mut char) -> Self {
		*out = self.inner.remove(idx);
		self
	}

	pub fn remove_uninit(mut self, idx: usize, out: &mut MaybeUninit<char>) -> Self {
		out.write(self.inner.remove(idx));
		self
	}

	// TODO: nightly remove matches (also std::str::pattern::Pattern is unstable too)

	pub fn reserve(mut self, additional: usize) -> Self {
		self.inner.reserve(additional);
		self
	}

	pub fn reserve_exact(mut self, additional: usize) -> Self {
		self.inner.reserve_exact(additional);
		self
	}

	pub fn retain<F>(mut self, f: F) -> Self
	where
		F: FnMut(char) -> bool
	{
		self.inner.retain(f);
		self
	}

	pub fn shrink_to(mut self, min_capacity: usize) -> Self {
		self.inner.shrink_to(min_capacity);
		self
	}

	pub fn shrink_to_fit(mut self) -> Self {
		self.inner.shrink_to_fit();
		self
	}

	pub fn split(mut self, at: usize) -> (Self, Self) {
		let r = self.inner.split_off(at);
		(self, r.into())
	}

	/// Splits the left side off and writes it to another location, keeping the
	/// right side in `self`.
	pub fn split_left_off(self, at: usize, out: &mut Self) -> Self {
		self.split_left_off_uninit(at, unsafe { out.to_maybeuninit_drop() })
	}

	/// Splits the left side off and writes it to another location, keeping the
	/// right side in `self`.
	pub fn split_left_off_uninit(self, at: usize, out: &mut MaybeUninit<Self>) -> Self {
		let (left, right) = self.split(at);
		out.write(left);
		right
	}

	/// Splits the right side off and writes it to another location, keeping the
	/// left side in `self`.
	pub fn split_right_off(self, at: usize, out: &mut Self) -> Self {
		self.split_right_off_uninit(at, unsafe { out.to_maybeuninit_drop() })
	}

	/// Splits the right side off and writes it to another location, keeping the
	/// left side in `self`.
	pub fn split_right_off_uninit(self, at: usize, out: &mut MaybeUninit<Self>) -> Self {
		let (left, right) = self.split(at);
		out.write(right);
		left
	}

	pub fn truncate(mut self, new_len: usize) -> Self {
		self.inner.truncate(new_len);
		self
	}

	/// Unsafely truncates the string, with no checks at all...
	///
	/// Unlike the [`truncate`](Self::truncate) method or it's [`String`]
	/// equivalent, passing a new len that's past the end of a string
	/// (`index == self.len()` is valid) is not allowed and will cause
	/// undefined behaviour.
	///
	/// # Safety
	///
	/// - `new_len` must be within the range `0..=len`
	/// - `new_len` must lie on a character boundary
	pub unsafe fn truncate_unchecked(mut self, new_len: usize) -> Self {
		debug_assert!(self.nonchain_str().is_char_boundary(new_len));
		self.nonchain_vec_mut().set_len(new_len);
		self
	}

	// TODO: try_reserve/exact
}

impl From<String> for StringChain {
	fn from(value: String) -> Self {
		Self { inner: value }
	}
}
