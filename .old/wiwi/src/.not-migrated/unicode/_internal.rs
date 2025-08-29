//! Internal implementations

use std::{ hint, slice };
use std::marker::PhantomData;
use std::ops::Range;

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub(super) enum CodepointUtf8 {
	One { values: [u8; 1] },
	Two { values: [u8; 2] },
	Three { values: [u8; 3] },
	Four { values: [u8; 4] }
}

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub(super) enum CodepointUtf16 {
	One { values: [u16; 1] },
	Two { values: [u16; 2] }
}

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub(super) enum CodepointUtf32 {
	One { values: [u32; 1] }
}

pub(super) struct CharsUtf8Raw<'h> {
	start: *const u8,
	end: *const u8,
	__marker: PhantomData<&'h [u8]>
}

pub(super) struct CharsUtf16Raw<'h> {
	start: *const u16,
	end: *const u16,
	__marker: PhantomData<&'h [u16]>
}

pub(super) struct CharsUtf32Raw<'h> {
	start: *const u32,
	end: *const u32,
	__marker: PhantomData<&'h [u32]>
}

pub(super) struct CharsIndicesUtf8Raw<'h> {
	inner: CharsUtf8Raw<'h>,
	offset_start: usize
}

pub(super) struct CharsIndicesUtf16Raw<'h> {
	inner: CharsUtf16Raw<'h>,
	offset_start: usize
}

pub(super) struct CharsIndicesUtf32Raw<'h> {
	inner: CharsUtf32Raw<'h>,
	offset_start: usize
}

/// Returns whether a codepoint is a valid unicode codepoint
#[inline]
pub(super) const fn validate_codepoint(c: u32) -> bool {
	matches!(c, 0x0000..=0xd7ff | 0xe000..=0x10ffff)
}

#[inline]
pub(super) const fn validate_utf8(code_units: &[u8]) -> bool {
	// table 3-7

	let len = code_units.len();
	let mut i = 0;

	macro_rules! next_must_match {
		($range:pat) => {
			{
				i += 1;
				if i >= len || !matches!(code_units[i], $range) {
					return false
				}
			}
		}
	}

	while i < len {
		// table 3-7
		match code_units[i] {
			0x00..=0x7f if i % (2 * size_of::<usize>()) == 0 => {
				const _ASSERT_USIZE_ALIGN_LTE_SIZE: () = assert!(size_of::<usize>() >= align_of::<usize>());
				// special ASCII case
				// we attempt to skip ahead quickly for ASCII in usize-sized chunks
				// but try this only every 2-usize chunks

				let remaining = code_units.len() - i;
				if remaining >= (2 * size_of::<usize>()) {
					unsafe {
						const MASK: usize = usize::from_ne_bytes([0x80; size_of::<usize>()]);

						// shift to current i first, then cast to pointer of usize
						// SAFETY: this is sound because the if condition above guarantees
						// sufficient alignment of usize
						let mut ptr = code_units
							.as_ptr()
							.add(i)
							.cast::<usize>();

						// truncating division, so will only return the amount of 2-usize
						// values we can dereference before we go out of bounds. this is
						// always at least one (because of the check above)
						let max = remaining / (2 * size_of::<usize>());

						let mut usize_i = 0;
						while usize_i < max {
							// SAFETY: loop does not loop more than we can read,
							// so ptr will not dereference out of bounds
							let units = *ptr;
							let units2 = *ptr.add(1);

							// if it isn't valid, this break (which leads to the continue
							// statement after it) would not update the counter, so next
							// loop would start at good spot
							if units & MASK != 0 { break }
							if units2 & MASK != 0 { break }

							usize_i += 2;
							ptr = ptr.add(2);
							i += 2 * size_of::<usize>();
						}

						// if loop did not even run (ie. first usize amount of bytes
						// were not ascii), we need to increment 1, so we only
						// continue/skip forward if usize_i is gt 0 (we looped at least once)
						if usize_i > 0 { continue }
					}
				}

				// else... we fall through as normal (`i` is incremented normally at the end)
				// so we always increment by at least 1, so this does not infinite-loop/deadlock
			}
			0x00..=0x7f => {
				// ASCII (non `2 * size_of::<usize>()` aligned)
			}
			0xc2..=0xdf => {
				next_must_match!(0x80..=0xbf);
			}
			0xe0 => {
				next_must_match!(0xa0..=0xbf);
				next_must_match!(0x80..=0xbf);
			}
			0xe1..=0xec => {
				next_must_match!(0x80..=0xbf);
				next_must_match!(0x80..=0xbf);
			}
			0xed => {
				next_must_match!(0x80..=0x9f);
				next_must_match!(0x80..=0xbf);
			}
			0xee..=0xef => {
				next_must_match!(0x80..=0xbf);
				next_must_match!(0x80..=0xbf);
			}
			0xf0 => {
				next_must_match!(0x90..=0xbf);
				next_must_match!(0x80..=0xbf);
				next_must_match!(0x80..=0xbf);
			}
			0xf1..=0xf3 => {
				next_must_match!(0x80..=0xbf);
				next_must_match!(0x80..=0xbf);
				next_must_match!(0x80..=0xbf);
			}
			0xf4 => {
				next_must_match!(0x80..=0x8f);
				next_must_match!(0x80..=0xbf);
				next_must_match!(0x80..=0xbf);
			}
			_ => { return false }
		}

		i += 1;
	}

	true
}

#[inline]
pub(super) const fn validate_utf16(code_units: &[u16]) -> bool {
	let len = code_units.len();
	let mut i = 0;

	while i < len {
		match code_units[i] {
			0x0000..=0xd7ff | 0xe000..=0xffff => {
				// BMP
			}
			0xd800..=0xdbff => {
				// leading surrogate code unit
				i += 1;

				if i >= len || !matches!(code_units[i], 0xdc00..=0xdfff) {
					// the next one isn't trailing surrogate code unit
					// isloated leading surrogate (invalid)
					return false
				}
			}
			0xdc00..=0xdfff => {
				// any leading surrogate code units would be checked in previous
				// match arm, and the trailing one would be checked in there as well
				// isloated trailing surrogate (invalid)
				return false
			}
		}

		i += 1;
	}

	true
}

#[inline]
pub(super) const fn validate_utf32(code_units: &[u32]) -> bool {
	let len = code_units.len();
	let mut i = 0;

	while i < len {
		let c = code_units[i];
		// every utf32 code unit is just a codepoint
		if !validate_codepoint(c) { return false }
		i += 1;
	}

	true
}

/// # Safety
///
/// `c` must be a valid unicode codepoint
#[inline]
pub(super) const unsafe fn codepoint_to_utf8_unchecked(c: u32) -> CodepointUtf8 {
	use CodepointUtf8::*;

	if c & 0x7f == c {
		One { values: [c as _] }
	} else if c & 0x7ff == c {
		let c1 = 0xc0 | (c >> 6) as u8;
		let c2 = 0x80 | (c & 0x3f) as u8;
		Two { values: [c1, c2] }
	} else if c & 0xffff == c {
		let c1 = 0xe0 | (c >> 12) as u8;
		let c2 = 0x80 | ((c >> 6) & 0x3f) as u8;
		let c3 = 0x80 | (c & 0x3f) as u8;
		Three { values: [c1, c2, c3] }
	} else {
		let c1 = 0xf0 | (c >> 18) as u8;
		let c2 = 0x80 | ((c >> 12) & 0x3f) as u8;
		let c3 = 0x80 | ((c >> 6) & 0x3f) as u8;
		let c4 = 0x80 | (c & 0x3f) as u8;
		Four { values: [c1, c2, c3, c4] }
	}
}

/// # Safety
///
/// `c` must be a valid unicode codepoint
#[inline]
pub(super) const unsafe fn codepoint_to_utf16_unchecked(c: u32) -> CodepointUtf16 {
	use CodepointUtf16::*;

	if c & 0xffff == c {
		One { values: [c as _] }
	} else {
		let c_offset = c - 0x10000;
		let c1 = 0xd800 | (c_offset >> 10) as u16;
		let c2 = 0xdc00 | (c_offset as u16 & 0x3ff);
		Two { values: [c1, c2] }
	}
}

/// # Safety
///
/// `c` must be a valid unicode codepoint
#[inline]
pub(super) const unsafe fn codepoint_to_utf32_unchecked(c: u32) -> CodepointUtf32 {
	CodepointUtf32::One { values: [c] }
}

#[inline]
pub(super) const fn codepoint_utf8_to_slice(c: &CodepointUtf8) -> &[u8] {
	match c {
		CodepointUtf8::One { values } => { values }
		CodepointUtf8::Two { values } => { values }
		CodepointUtf8::Three { values } => { values }
		CodepointUtf8::Four { values } => { values }
	}
}

#[inline]
pub(super) const fn codepoint_utf16_to_slice(c: &CodepointUtf16) -> &[u16] {
	match c {
		CodepointUtf16::One { values } => { values }
		CodepointUtf16::Two { values } => { values }
	}
}

#[inline]
pub(super) const fn codepoint_utf32_to_slice(c: &CodepointUtf32) -> &[u32] {
	match c {
		CodepointUtf32::One { values } => { values }
	}
}

/// # Safety
///
/// `utf8` must contain valid data for a UTF-8 codepoint. If it is returned from
/// [`codepoint_to_utf8_unchecked`] (assuming its preconditions were met of
/// course), it is valid.
#[inline]
pub(super) const unsafe fn utf8_to_codepoint_unchecked(utf8: CodepointUtf8) -> u32 {
	match utf8 {
		CodepointUtf8::One { values: [c1] } => { c1 as _ }
		CodepointUtf8::Two { values: [c1, c2] } => {
			let c1 = ((c1 & 0x1f) as u32) << 6;
			let c2 = (c2 & 0x3f) as u32;
			c1 | c2
		}
		CodepointUtf8::Three { values: [c1, c2, c3] } => {
			let c1 = ((c1 & 0xf) as u32) << 12;
			let c2 = ((c2 & 0x3f) as u32) << 6;
			let c3 = (c3 & 0x3f) as u32;
			c1 | c2 | c3
		}
		CodepointUtf8::Four { values: [c1, c2, c3, c4] } => {
			let c1 = ((c1 & 0x7) as u32) << 18;
			let c2 = ((c2 & 0x3f) as u32) << 12;
			let c3 = ((c3 & 0x3f) as u32) << 6;
			let c4 = (c4 & 0x3f) as u32;
			c1 | c2 | c3 | c4
		}
	}
}

/// # Safety
///
/// `utf16` must contain valid data for a UTF-16 codepoint. If it is returned from
/// [`codepoint_to_utf16_unchecked`] (assuming its preconditions were met of
/// course), it is valid.
#[inline]
pub(super) const unsafe fn utf16_to_codepoint_unchecked(utf16: CodepointUtf16) -> u32 {
	match utf16 {
		CodepointUtf16::One { values: [c1] } => { c1 as _ }
		CodepointUtf16::Two { values: [c1, c2] } => {
			let c1 = ((c1 & 0x3ff) as u32) << 10;
			let c2 = (c2 & 0x3ff) as u32;
			(c1 | c2) + 0x10000
		}
	}
}

/// # Safety
///
/// `utf32` must contain valid data for a UTF-32 codepoint. If it is returned from
/// [`codepoint_to_utf32_unchecked`] (assuming its preconditions were met of
/// course), it is valid.
#[inline]
pub(super) const unsafe fn utf32_to_codepoint_unchecked(utf32: CodepointUtf32) -> u32 {
	match utf32 {
		CodepointUtf32::One { values: [c1] } => { c1 }
	}
}

/// # Safety
///
/// The provided code unit slice must be valid UTF-8
#[inline]
pub(super) const unsafe fn is_char_boundary_utf8_unchecked(utf8: &[u8], i: usize) -> bool {
	if i > utf8.len() { return false }
	if i == 0 || i == utf8.len() { return true }

	matches!(
		*utf8.as_ptr().add(i),
		0x00..=0x7f | 0xc2..=0xdf | 0xe0..=0xef | 0xf0..=0xf4
	)
}

/// # Safety
///
/// The provided code unit slice must be valid UTF-16
#[inline]
pub(super) const unsafe fn is_char_boundary_utf16_unchecked(utf16: &[u16], i: usize) -> bool {
	if i > utf16.len() { return false }
	if i == 0 || i == utf16.len() { return true }

	// check that it's _not_ a trailing surrogate
	!matches!(*utf16.as_ptr().add(i), 0xdc00..=0xdfff)
}

/// # Safety
///
/// The provided code unit slice must be valid UTF-32
#[inline]
pub(super) const unsafe fn is_char_boundary_utf32_unchecked(utf32: &[u32], i: usize) -> bool {
	// there are no 2+ unit sequences in UTF-32
	// so the only `false` condition is if i is OOB
	// ie. if i is in bounds, this is true
	i <= utf32.len()
}

/// # Safety
///
/// The provided code unit slice must be valid UTF-8, and have a length greater
/// than 0. If both of those preconditions are satisfied, it must mean the slice
/// also has at least one UTF-8 character.
#[inline]
pub(super) const unsafe fn next_codepoint_utf8_unchecked(utf8: &[u8]) -> (u32, &[u8]) {
	debug_assert!(!utf8.is_empty());

	let (cp, consumed) = next_codepoint_utf8_ptr_unchecked(utf8.as_ptr());
	(cp, slice::from_raw_parts(utf8.as_ptr().add(consumed), utf8.len() - consumed))
}

/// # Safety
///
/// The provided code unit slice must be valid UTF-16, and have a length greater
/// than 0. If both of those preconditions are satisfied, it must mean the slice
/// also has at least one UTF-16 character.
#[inline]
pub(super) const unsafe fn next_codepoint_utf16_unchecked(utf16: &[u16]) -> (u32, &[u16]) {
	debug_assert!(!utf16.is_empty());

	let (cp, consumed) = next_codepoint_utf16_ptr_unchecked(utf16.as_ptr());
	(cp, slice::from_raw_parts(utf16.as_ptr().add(consumed), utf16.len() - consumed))
}

/// # Safety
///
/// The provided code unit slice must be valid UTF-32, and have a length greater
/// than 0. If both of those preconditions are satisfied, it must mean the slice
/// also has at least one UTF-32 character.
#[inline]
pub(super) const unsafe fn next_codepoint_utf32_unchecked(utf32: &[u32]) -> (u32, &[u32]) {
	debug_assert!(!utf32.is_empty());

	let (cp, consumed) = next_codepoint_utf32_ptr_unchecked(utf32.as_ptr());
	(cp, slice::from_raw_parts(utf32.as_ptr().add(consumed), utf32.len() - consumed))
}

#[inline]
pub(super) const unsafe fn next_codepoint_back_utf8_unchecked(utf8: &[u8]) -> (u32, &[u8]) {
	debug_assert!(!utf8.is_empty());

	let (cp, consumed) = next_codepoint_back_utf8_ptr_unchecked(utf8.as_ptr().add(utf8.len()));
	(cp, slice::from_raw_parts(utf8.as_ptr(), utf8.len() - consumed))
}

#[inline]
pub(super) const unsafe fn next_codepoint_back_utf16_unchecked(utf16: &[u16]) -> (u32, &[u16]) {
	debug_assert!(!utf16.is_empty());

	let (cp, consumed) = next_codepoint_back_utf16_ptr_unchecked(utf16.as_ptr().add(utf16.len()));
	(cp, slice::from_raw_parts(utf16.as_ptr(), utf16.len() - consumed))
}

#[inline]
pub(super) const unsafe fn next_codepoint_back_utf32_unchecked(utf32: &[u32]) -> (u32, &[u32]) {
	debug_assert!(!utf32.is_empty());

	let (cp, consumed) = next_codepoint_back_utf32_ptr_unchecked(utf32.as_ptr().add(utf32.len()));
	(cp, slice::from_raw_parts(utf32.as_ptr(), utf32.len() - consumed))
}

#[inline]
pub(super) const unsafe fn next_codepoint_utf8_ptr_unchecked(front_ptr: *const u8) -> (u32, usize) {
	let first_cu = *front_ptr;
	let (cp, consumed) = match first_cu {
		0x00..=0x7f => {
			let cp = CodepointUtf8::One { values: [first_cu] };
			(cp, 1)
		}
		0xc2..=0xdf => {
			let values = [first_cu, *front_ptr.add(1)];
			let cp = CodepointUtf8::Two { values };
			(cp, 2)
		}
		0xe0..=0xef => {
			let values = [first_cu, *front_ptr.add(1), *front_ptr.add(2)];
			let cp = CodepointUtf8::Three { values };
			(cp, 3)
		}
		0xf0..=0xf4 => {
			let values = [first_cu, *front_ptr.add(1), *front_ptr.add(2), *front_ptr.add(3)];
			let cp = CodepointUtf8::Four { values };
			(cp, 4)
		}
		_ => { hint::unreachable_unchecked() }
	};

	let cp = utf8_to_codepoint_unchecked(cp);
	(cp, consumed)
}

#[inline]
pub(super) const unsafe fn next_codepoint_utf16_ptr_unchecked(front_ptr: *const u16) -> (u32, usize) {
	let first_cu = *front_ptr;
	let (cp, consumed) = match first_cu {
		0x0000..=0xd7ff | 0xe000..=0xffff => {
			let cp = CodepointUtf16::One { values: [first_cu] };
			(cp, 1)
		}
		0xd800..=0xdbff => {
			let values = [first_cu, *front_ptr.add(1)];
			let cp = CodepointUtf16::Two { values };
			(cp, 2)
		}
		0xdc00..=0xdfff => { hint::unreachable_unchecked() }
	};

	let cp = utf16_to_codepoint_unchecked(cp);
	(cp, consumed)
}

#[inline]
pub(super) const unsafe fn next_codepoint_utf32_ptr_unchecked(front_ptr: *const u32) -> (u32, usize) {
	let first_cu = *front_ptr;
	let (cp, consumed) = match first_cu {
		0x0000..=0xd7ff | 0xe000..=0x10ffff => {
			let cp = CodepointUtf32::One { values: [first_cu] };
			(cp, 1)
		}
		_ => { hint::unreachable_unchecked() }
	};

	let cp = utf32_to_codepoint_unchecked(cp);
	(cp, consumed)
}

#[inline]
pub(super) const unsafe fn next_codepoint_back_utf8_ptr_unchecked(mut back_ptr: *const u8) -> (u32, usize) {
	macro_rules! next_cu_back {
		() => {
			{
				back_ptr = back_ptr.sub(1);
				*back_ptr
			}
		}
	}

	let (cp, consumed) = match next_cu_back!() {
		c1 @ 0x00..=0x7f => {
			let cp = CodepointUtf8::One { values: [c1] };
			(cp, 1)
		}
		c1 @ 0x80..=0xbf => match next_cu_back!() {
			c2 @ 0xc2..=0xdf => {
				let values = [c2, c1];
				let cp = CodepointUtf8::Two { values };
				(cp, 2)
			}
			c2 @ 0x80..=0xbf => match next_cu_back!() {
				c3 @ 0xe0..=0xef => {
					let values = [c3, c2, c1];
					let cp = CodepointUtf8::Three { values };
					(cp, 3)
				}
				c3 @ 0x80..=0xbf => match next_cu_back!() {
					c4 @ 0xf0..=0xf4 => {
						let values = [c4, c3, c2, c1];
						let cp = CodepointUtf8::Four { values };
						(cp, 4)
					}
					_ => { hint::unreachable_unchecked() }
				}
				_ => { hint::unreachable_unchecked() }
			}
			_ => { hint::unreachable_unchecked() }
		}
		_ => { hint::unreachable_unchecked() }
	};

	let cp = utf8_to_codepoint_unchecked(cp);
	(cp, consumed)
}

#[inline]
pub(super) const unsafe fn next_codepoint_back_utf16_ptr_unchecked(mut back_ptr: *const u16) -> (u32, usize) {
	macro_rules! next_cu_back {
		() => {
			{
				back_ptr = back_ptr.sub(1);
				*back_ptr
			}
		}
	}

	let (cp, consumed) = match next_cu_back!() {
		c1 @ (0x0000..=0xd7ff | 0xe000..=0xffff) => {
			let cp = CodepointUtf16::One { values: [c1] };
			(cp, 1)
		}
		c1 @ 0xdc00..=0xdfff => match next_cu_back!() {
			c2 @ 0xd800..=0xdbff => {
				let values = [c2, c1];
				let cp = CodepointUtf16::Two { values };
				(cp, 2)
			}
			_ => { hint::unreachable_unchecked() }
		}
		_ => { hint::unreachable_unchecked() }
	};

	let cp = utf16_to_codepoint_unchecked(cp);
	(cp, consumed)
}

#[inline]
pub(super) const unsafe fn next_codepoint_back_utf32_ptr_unchecked(mut back_ptr: *const u32) -> (u32, usize) {
	macro_rules! next_cu_back {
		() => {
			{
				back_ptr = back_ptr.sub(1);
				*back_ptr
			}
		}
	}

	let (cp, consumed) = match next_cu_back!() {
		cu @ (0x0000..=0xd7ff | 0xe000..=0x10ffff) => {
			let cp = CodepointUtf32::One { values: [cu] };
			(cp, 1)
		}
		_ => { hint::unreachable_unchecked() }
	};

	let cp = utf32_to_codepoint_unchecked(cp);
	(cp, consumed)
}

#[inline]
pub(super) unsafe fn new_chars_utf8_raw(utf8: &[u8]) -> CharsUtf8Raw<'_> {
	let Range { start, end } = utf8.as_ptr_range();
	CharsUtf8Raw { start, end, __marker: PhantomData }
}

#[inline]
pub(super) unsafe fn new_chars_utf16_raw(utf16: &[u16]) -> CharsUtf16Raw<'_> {
	let Range { start, end } = utf16.as_ptr_range();
	CharsUtf16Raw { start, end, __marker: PhantomData }
}

#[inline]
pub(super) unsafe fn new_chars_utf32_raw(utf32: &[u32]) -> CharsUtf32Raw<'_> {
	let Range { start, end } = utf32.as_ptr_range();
	CharsUtf32Raw { start, end, __marker: PhantomData }
}

#[inline]
pub(super) fn chars_utf8_raw_next(chars: &mut CharsUtf8Raw) -> Option<u32> {
	if chars.start < chars.end {
		unsafe {
			let (cp, consumed) = next_codepoint_utf8_ptr_unchecked(chars.start);
			chars.start = chars.start.add(consumed);
			Some(cp)
		}
	} else {
		None
	}
}

#[inline]
pub(super) fn chars_utf16_raw_next(chars: &mut CharsUtf16Raw) -> Option<u32> {
	if chars.start < chars.end {
		unsafe {
			let (cp, consumed) = next_codepoint_utf16_ptr_unchecked(chars.start);
			chars.start = chars.start.add(consumed);
			Some(cp)
		}
	} else {
		None
	}
}

#[inline]
pub(super) fn chars_utf32_raw_next(chars: &mut CharsUtf32Raw) -> Option<u32> {
	if chars.start < chars.end {
		unsafe {
			let (cp, consumed) = next_codepoint_utf32_ptr_unchecked(chars.start);
			chars.start = chars.start.add(consumed);
			Some(cp)
		}
	} else {
		None
	}
}

#[inline]
pub(super) fn chars_utf8_raw_next_back(chars: &mut CharsUtf8Raw) -> Option<u32> {
	if chars.start < chars.end {
		unsafe {
			let (cp, consumed) = next_codepoint_back_utf8_ptr_unchecked(chars.end);
			chars.end = chars.end.sub(consumed);
			Some(cp)
		}
	} else {
		None
	}
}

#[inline]
pub(super) fn chars_utf16_raw_next_back(chars: &mut CharsUtf16Raw) -> Option<u32> {
	if chars.start < chars.end {
		unsafe {
			let (cp, consumed) = next_codepoint_back_utf16_ptr_unchecked(chars.end);
			chars.end = chars.end.sub(consumed);
			Some(cp)
		}
	} else {
		None
	}
}

#[inline]
pub(super) fn chars_utf32_raw_next_back(chars: &mut CharsUtf32Raw) -> Option<u32> {
	if chars.start < chars.end {
		unsafe {
			let (cp, consumed) = next_codepoint_back_utf32_ptr_unchecked(chars.end);
			chars.end = chars.end.sub(consumed);
			Some(cp)
		}
	} else {
		None
	}
}

#[inline]
pub(super) fn chars_indices_utf8_raw_next(chars_indices: &mut CharsIndicesUtf8Raw) -> Option<(usize, u32)> {
	if chars_indices.inner.start < chars_indices.inner.end {
		unsafe {
			let (cp, consumed) = next_codepoint_utf8_ptr_unchecked(chars_indices.inner.start);
			chars_indices.inner.start = chars_indices.inner.start.add(consumed);

			let i = chars_indices.offset_start;
			chars_indices.offset_start += consumed;

			Some((i, cp))
		}
	} else {
		None
	}
}

#[inline]
pub(super) fn chars_indices_utf16_raw_next(chars_indices: &mut CharsIndicesUtf16Raw) -> Option<(usize, u32)> {
	if chars_indices.inner.start < chars_indices.inner.end {
		unsafe {
			let (cp, consumed) = next_codepoint_utf16_ptr_unchecked(chars_indices.inner.start);
			chars_indices.inner.start = chars_indices.inner.start.add(consumed);

			let i = chars_indices.offset_start;
			chars_indices.offset_start += consumed;

			Some((i, cp))
		}
	} else {
		None
	}
}

#[inline]
pub(super) fn chars_indices_utf32_raw_next(chars_indices: &mut CharsIndicesUtf32Raw) -> Option<(usize, u32)> {
	if chars_indices.inner.start < chars_indices.inner.end {
		unsafe {
			let (cp, consumed) = next_codepoint_utf32_ptr_unchecked(chars_indices.inner.start);
			chars_indices.inner.start = chars_indices.inner.start.add(consumed);

			let i = chars_indices.offset_start;
			chars_indices.offset_start += consumed;

			Some((i, cp))
		}
	} else {
		None
	}
}

#[inline]
pub(super) fn chars_indices_utf8_raw_next_back(chars_indices: &mut CharsIndicesUtf8Raw) -> Option<(usize, u32)> {
	if chars_indices.inner.start < chars_indices.inner.end {
		unsafe {
			let (cp, consumed) = next_codepoint_back_utf8_ptr_unchecked(chars_indices.inner.end);
			chars_indices.inner.end = chars_indices.inner.end.sub(consumed);

			debug_assert!((chars_indices.inner.end as usize - chars_indices.inner.start as usize) % 8 == 0);
			let i = chars_indices.offset_start + ((chars_indices.inner.end as usize - chars_indices.inner.start as usize) >> 3);

			Some((i, cp))
		}
	} else {
		None
	}
}

#[inline]
pub(super) fn chars_indices_utf16_raw_next_back(chars_indices: &mut CharsIndicesUtf16Raw) -> Option<(usize, u32)> {
	if chars_indices.inner.start < chars_indices.inner.end {
		unsafe {
			let (cp, consumed) = next_codepoint_back_utf16_ptr_unchecked(chars_indices.inner.end);
			chars_indices.inner.end = chars_indices.inner.end.sub(consumed);

			debug_assert!((chars_indices.inner.end as usize - chars_indices.inner.start as usize) % 16 == 0);
			let i = chars_indices.offset_start + ((chars_indices.inner.end as usize - chars_indices.inner.start as usize) >> 4);

			Some((i, cp))
		}
	} else {
		None
	}
}

#[inline]
pub(super) fn chars_indices_utf32_raw_next_back(chars_indices: &mut CharsIndicesUtf32Raw) -> Option<(usize, u32)> {
	if chars_indices.inner.start < chars_indices.inner.end {
		unsafe {
			let (cp, consumed) = next_codepoint_back_utf32_ptr_unchecked(chars_indices.inner.end);
			chars_indices.inner.end = chars_indices.inner.end.sub(consumed);

			debug_assert!((chars_indices.inner.end as usize - chars_indices.inner.start as usize) % 32 == 0);
			let i = chars_indices.offset_start + ((chars_indices.inner.end as usize - chars_indices.inner.start as usize) >> 5);

			Some((i, cp))
		}
	} else {
		None
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn codepoint_to_utf8() {
		// testing endpoints of the code point ranges in
		// table 3-7

		trait MkCodepointUtf8 {
			fn make_codepoint_utf8(self) -> CodepointUtf8;
		}

		impl MkCodepointUtf8 for [u8; 1] {
			fn make_codepoint_utf8(self) -> CodepointUtf8 {
				CodepointUtf8::One { values: self }
			}
		}

		impl MkCodepointUtf8 for [u8; 2] {
			fn make_codepoint_utf8(self) -> CodepointUtf8 {
				CodepointUtf8::Two { values: self }
			}
		}

		impl MkCodepointUtf8 for [u8; 3] {
			fn make_codepoint_utf8(self) -> CodepointUtf8 {
				CodepointUtf8::Three { values: self }
			}
		}

		impl MkCodepointUtf8 for [u8; 4] {
			fn make_codepoint_utf8(self) -> CodepointUtf8 {
				CodepointUtf8::Four { values: self }
			}
		}

		fn check<T: MkCodepointUtf8>(codepoint: u32, expected: T) {
			assert!(validate_codepoint(codepoint));
			assert_eq!(
				// SAFETY: just asserted codepoint is valid above
				unsafe { codepoint_to_utf8_unchecked(codepoint) },
				expected.make_codepoint_utf8()
			);
		}

		check(0x0000,   [0x00]);
		check(0x007f,   [0x7f]);
		check(0x0080,   [0xc2, 0x80]);
		check(0x07ff,   [0xdf, 0xbf]);
		check(0x0800,   [0xe0, 0xa0, 0x80]);
		check(0x0fff,   [0xe0, 0xbf, 0xbf]);
		check(0x1000,   [0xe1, 0x80, 0x80]);
		check(0xcfff,   [0xec, 0xbf, 0xbf]);
		check(0xd000,   [0xed, 0x80, 0x80]);
		check(0xd7ff,   [0xed, 0x9f, 0xbf]);
		check(0xe000,   [0xee, 0x80, 0x80]);
		check(0xffff,   [0xef, 0xbf, 0xbf]);
		check(0x10000,  [0xf0, 0x90, 0x80, 0x80]);
		check(0x3ffff,  [0xf0, 0xbf, 0xbf, 0xbf]);
		check(0x40000,  [0xf1, 0x80, 0x80, 0x80]);
		check(0xfffff,  [0xf3, 0xbf, 0xbf, 0xbf]);
		check(0x100000, [0xf4, 0x80, 0x80, 0x80]);
		check(0x10ffff, [0xf4, 0x8f, 0xbf, 0xbf]);
	}
}
