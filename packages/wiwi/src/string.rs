use crate::prelude::*;

use crate::num::*;

const STRING_SIZE_BYTES: usize = size_of::<usize>() * 3;
const MAX_INLINE_LEN: usize = {
	// we'll be fine even on (hypothetical, at time of writing)
	// 128bit and 256bit computers, 512bit will cause issues, so this is
	// (quite extreme, probably unnecessary) future proofing
	let container_size = STRING_SIZE_BYTES - 1;
	#[expect(clippy::as_conversions, reason = "u8 to usize is fine (also we are in const context)")]
	let len_size = (u8::MAX >> 1) as usize;

	if len_size < container_size {
		len_size
	} else {
		container_size
	}
};
const CAP_MARKER_BE: usize = (!(usize::MAX >> 1)).to_be();
const CAP_MARKER_U8: u8 = !(u8::MAX >> 1);

const _: () = assert!(size_of::<StringInlineable>() == STRING_SIZE_BYTES);
const _: () = assert!(size_of::<StringInline>() == STRING_SIZE_BYTES);
const _: () = assert!(size_of::<StringHeap>() == STRING_SIZE_BYTES);
const _: () = assert!(MAX_INLINE_LEN > 0);

pub union StringInlineable {
	inline: ManuallyDrop<StringInline>,
	heap: ManuallyDrop<StringHeap>
}

impl StringInlineable {
	#[inline]
	pub const fn new() -> Self {
		let inline = StringInline::new();
		Self { inline: ManuallyDrop::new(inline) }
	}
}

impl StringInlineable {
	#[inline]
	pub fn len(&self) -> usize {
		self.do_thing(|s| s.len(), |s| s.len())
	}

	#[inline]
	pub fn capacity(&self) -> usize {
		self.do_thing(|s| s.capacity(), |s| s.capacity())
	}

	#[inline]
	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}

	#[inline]
	pub fn as_str(&self) -> &str {
		self.do_thing(|s| s.as_str(), |s| s.as_str())
	}

	#[inline]
	pub fn as_str_mut(&mut self) -> &mut str {
		self.do_thing_mut(|s| s.as_str_mut(), |s| s.as_str_mut())
	}
}

impl StringInlineable {
	#[inline]
	fn is_inline(&self) -> bool {
		// SAFETY: all memory-valid instancees of `StringHeap` satisfy memory
		// invariants of `StringInline`, so union field access `self.inline` is fine
		let len = unsafe { self.inline.len };

		len & CAP_MARKER_U8 == 0
	}

	#[inline]
	fn do_thing<'h, T, FInline, FHeap>(&'h self, f_inline: FInline, f_heap: FHeap) -> T
	where
		FInline: FnOnce(&'h StringInline) -> T,
		FHeap: FnOnce(&'h StringHeap) -> T
	{
		match self.is_inline() {
			// SAFETY: we just checked `self.is_inline()`
			true => unsafe { f_inline(&self.inline) }
			// SAFETY: we just checked `self.is_inline()`
			false => unsafe { f_heap(&self.heap) }
		}
	}

	#[inline]
	fn do_thing_mut<'h, T, FInline, FHeap>(&'h mut self, f_inline: FInline, f_heap: FHeap) -> T
	where
		FInline: FnOnce(&'h mut StringInline) -> T,
		FHeap: FnOnce(&'h mut StringHeap) -> T
	{
		match self.is_inline() {
			// SAFETY: we just checked `self.is_inline()`
			true => unsafe { f_inline(&mut self.inline) }
			// SAFETY: we just checked `self.is_inline()`
			false => unsafe { f_heap(&mut self.heap) }
		}
	}
}

impl Default for StringInlineable {
	#[inline]
	fn default() -> Self {
		Self::new()
	}
}

impl Deref for StringInlineable {
	type Target = str;

	#[inline]
	fn deref(&self) -> &str {
		self.as_str()
	}
}

impl From<&str> for StringInlineable {
	#[inline]
	fn from(s: &str) -> Self {
		match s.len() <= MAX_INLINE_LEN {
			true => {
				// SAFETY: just checked `s.len() <= MAX_INLINE_LEN`
				let inline = unsafe { StringInline::from_str_unchecked(s) };
				Self { inline: ManuallyDrop::new(inline) }
			}
			false => {
				// SAFETY: just checked `s.len() > MAX_INLINE_LEN`
				// (which is also not zero)
				let heap = unsafe { StringHeap::from_str_unchecked(s) };
				Self { heap: ManuallyDrop::new(heap) }
			}
		}
	}
}

#[repr(C)]
struct StringInline {
	/// regular u8, represented as is
	len: u8,
	rest: MaybeUninit<[u8; MAX_INLINE_LEN]>
}

impl StringInline {
	#[inline]
	const fn new() -> Self {
		Self { len: 0, rest: MaybeUninit::uninit() }
	}

	/// # Safety
	///
	/// The passed in `str` must have length less than or equal to [`MAX_INLINE_LEN`].
	#[inline]
	unsafe fn from_str_unchecked(s: &str) -> Self {
		debug_assert!(s.len() <= MAX_INLINE_LEN);

		let mut inline = Self {
			len: 0,
			rest: MaybeUninit::uninit()
		};

		// SAFETY:
		// - ptr obtained from `value` is valid, and for `s.len()` reads
		// - ptr obtained from `inline.rest` is valid
		// - caller promises `s.len()` is lte `MAX_INLINE_LEN`
		// - ptrs obtained from aligned sources
		// - reference to memory outside local stack memory in `value`
		//   cannot overlap with local stack memory in `inline`
		unsafe {
			ptr::copy_nonoverlapping(
				s.as_ptr(),
				inline.rest.as_mut_ptr().cast::<u8>(),
				s.len()
			)
		}

		// we just initialised `s.len()` amount of
		// memory with that `copy` call above
		inline.len = s.len().into_u8_lossy();

		inline
	}
}

impl StringInline {
	#[inline]
	fn len(&self) -> usize {
		usize::from_u8(self.len)
	}

	#[inline]
	fn capacity(&self) -> usize {
		MAX_INLINE_LEN
	}

	#[inline]
	fn as_str(&self) -> &str {
		let ptr = self.rest.as_ptr().cast::<u8>();
		let len = self.len.into_usize();

		// SAFETY: relying on invariant that `self.rest` must have
		// at least `self.len` elements initialised
		let slice = unsafe { slice::from_raw_parts(ptr, len) };
		// SAFETY: relying on invariant that `self` contains valid utf-8
		unsafe { str::from_utf8_unchecked(slice) }
	}

	#[inline]
	fn as_str_mut(&mut self) -> &mut str {
		let ptr = self.rest.as_ptr().cast::<u8>();
		let len = self.len.into_usize();

		// SAFETY: relying on invariant that `self.rest` must have
		// at least `self.len` elements initialised
		let slice = unsafe { slice::from_raw_parts_mut(ptr.cast_mut(), len) };
		// SAFETY: relying on invariant that `self` contains valid utf-8
		unsafe { str::from_utf8_unchecked_mut(slice) }
	}
}

#[repr(C)]
struct StringHeap {
	/// This value needs processing in order to be a valid capacity
	///
	/// This stores the capacity, in big endian, with the highest bit set. Just
	/// use [`capacity`](Self::capacity) function to get the capacity.
	cap_be_and_marker: usize,
	len: usize,
	ptr: *const u8
}

impl StringHeap {
	/// # Safety
	///
	/// The passed in `str` must have length greater than zero. (The passed in
	/// `str` _should_ have greater than `MAX_INLINE_LEN` len, which is larger
	/// than zero, and `StringInlineable` already ensures this)
	unsafe fn from_str_unchecked(s: &str) -> Self {
		let layout = alloc_mod::Layout::for_value(s);
		// SAFETY: layout is nonzero (caller promises `s` is not zero length)
		let ptr = unsafe { alloc(layout) };

		let mut heap = Self {
			cap_be_and_marker: 0,
			len: 0,
			ptr
		};
		// SAFETY:
		// - we just allocated the ptr inside with this layout
		// - existing `&str` cannot have memory larger than `isize::MAX`
		unsafe { heap.set_capacity(layout.size()) }

		// SAFETY:
		// - ptr obtained from `value` is valid, and for `s.len()` reads
		// - we just allocated ptr in `heap.ptr` for `s.len()` bytes
		// - ptrs obtained from aligned sources
		// - reference to existing memory in `value` cannot overlap with
		//   memory we just allocated
		unsafe {
			ptr::copy_nonoverlapping(
				s.as_ptr(),
				heap.ptr.cast_mut(),
				s.len()
			)
		}

		// we just initialised `s.len()` amount of
		// memory with that `copy` call above
		heap.len = s.len();

		heap
	}
}

impl StringHeap {
	#[inline]
	fn len(&self) -> usize {
		self.len
	}

	#[inline]
	fn capacity(&self) -> usize {
		usize::from_be(self.cap_be_and_marker ^ CAP_MARKER_BE)
	}

	/// Helper for setting capacity (since it's stored in a... nonstandard way)
	///
	/// # Safety
	///
	/// - Capacity in `self` must actually be `capacity`
	/// - `capacity` must be less than or equal to `isize::MAX`. This is
	///   required by rust's allocation APIs, as well as needed for the heap marker
	///   to be set properlu
	#[inline]
	unsafe fn set_capacity(&mut self, capacity: usize) {
		self.cap_be_and_marker = capacity.to_be() ^ CAP_MARKER_BE
	}

	#[inline]
	fn as_str(&self) -> &str {
		// SAFETY: relying on invariant that `self.rest` must have
		// at least `self.len` elements initialised
		let slice = unsafe { slice::from_raw_parts(self.ptr, self.len) };
		// SAFETY: relying on invariant that `self` contains valid utf-8
		unsafe { str::from_utf8_unchecked(slice) }
	}

	#[inline]
	fn as_str_mut(&mut self) -> &mut str {
		// SAFETY: relying on invariant that `self.rest` must have
		// at least `self.len` elements initialised
		let slice = unsafe { slice::from_raw_parts_mut(self.ptr.cast_mut(), self.len) };
		// SAFETY: relying on invariant that `self` contains valid utf-8
		unsafe { str::from_utf8_unchecked_mut(slice) }
	}
}
