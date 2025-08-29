use crate::num::{ IntoU8Lossy, IntoUsizeLossless };
use crate::ptr::coerce_ptr;
use crate::rust_std::{ debug_assert, panic, ptr, slice, str };
use crate::rust_std::alloc::{ Layout, alloc, dealloc, realloc };
use crate::rust_std::convert::From;
use crate::rust_std::default::Default;
use crate::rust_std::mem::{ drop, ManuallyDrop, MaybeUninit, size_of };
use crate::rust_std::ops::{ Deref, DerefMut, Drop };

// the struct is 3 words in size (ptr, len, cap)
// 24 bytes on 64bit, 12 on 32bit, 6 on 16bit
// so the len is that amount minus one
const INLINE_MAX_LEN: usize = size_of::<Heap>() - 1;
const INLINE_MARKER: u8 = 1 << 7;

pub union GermanString {
	inline: ManuallyDrop<Inline>,
	heap: ManuallyDrop<Heap>
}

impl GermanString {
	#[inline]
	pub const fn new() -> Self {
		let inline = ManuallyDrop::new(Inline {
			len: INLINE_MARKER,
			str: MaybeUninit::uninit()
		});
		Self { inline }
	}

	/// # Safety
	///
	/// The provided string must have a length less than or equal to
	/// [`INLINE_MAX_LEN`].
	#[inline]
	unsafe fn new_inline(s: &str) -> Self {
		// SAFETY: caller promises to uphold invariants of `Inline::new`
		Self { inline: ManuallyDrop::new(unsafe { Inline::new(s) }) }
	}

	/// # Safety
	///
	/// The passed in string should ideally actually need to be put
	/// on the heap, but in strict requirements, its length must not
	/// be zero or overflow `isize::MAX`.
	#[inline]
	unsafe fn new_heap(s: &str) -> Self {
		// SAFETY: caller promises to uphold invariants of `Heap::new`
		Self { heap: ManuallyDrop::new(unsafe { Heap::new(s) }) }
	}

	/// # Safety
	///
	/// Conservatively marked `unsafe`, `inline` variant has no initialisation
	/// requirements as long as the first byte is initialised, which all
	/// constructors of this struct guarantee.
	#[inline]
	unsafe fn to_inline_unchecked(&self) -> &Inline {
		// SAFETY: caller promises the string is stored inline
		unsafe { &self.inline }
	}

	/// # Safety
	///
	/// `self` must be inline.
	#[inline]
	unsafe fn to_inline_unchecked_mut(&mut self) -> &mut Inline {
		// SAFETY: caller promises the string is stored inline
		unsafe { &mut self.inline }
	}

	/// # Safety
	///
	/// `self` must be on the heap.
	#[inline]
	unsafe fn to_heap_unchecked(&self) -> &Heap {
		// SAFETY: caller promises the string is stored on the heap
		unsafe { &self.heap }
	}

	/// # Safety
	///
	/// `self` must be on the heap.
	#[inline]
	unsafe fn to_heap_unchecked_mut(&mut self) -> &mut Heap {
		// SAFETY: caller promises the string is stored on the heap
		unsafe { &mut self.heap }
	}


	#[inline]
	pub const fn is_inline(&self) -> bool {
		// SAFETY: accessing field `inlined` is always safe,
		// the first byte is always initialised (heap variant has a usize
		// taking that up), and the rest can be anything because `MaybeUninit`
		let inline = unsafe { &self.inline };

		// SAFETY: derived pointer from `&self`, casting it to a struct
		// with the same layout, dereference is safe. we do this because
		// cannot use `Deref` to access `Inlined` inner value, and no
		// other way.
		let inline = unsafe { &*coerce_ptr(inline).cast::<Inline>() };

		inline.len >> 7 == 1
	}

	#[inline]
	pub fn capacity(&self) -> usize {
		if self.is_inline() {
			// SAFETY: just checked we're inline
			let s = unsafe { self.to_inline_unchecked() };
			s.capacity()
		} else {
			// SAFETY: just checked we're on the heap
			let s = unsafe { self.to_heap_unchecked() };
			s.capacity()
		}
	}
	#[inline]
	pub fn len(&self) -> usize {
		if self.is_inline() {
			// SAFETY: just checked we're inline
			let s = unsafe { self.to_inline_unchecked() };
			s.len()
		} else {
			// SAFETY: just checked we're on the heap
			let s = unsafe { self.to_heap_unchecked() };
			s.len()
		}
	}

	#[inline]
	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}
}

impl Default for GermanString {
	#[inline]
	fn default() -> Self {
		Self::new()
	}
}

impl Deref for GermanString {
	type Target = str;

	#[inline]
	fn deref(&self) -> &str {
		if self.is_inline() {
			// SAFETY: just checked that we're inline
			unsafe { &self.inline }
		} else {
			// SAFETY: just checked that we're on the heap
			unsafe { &self.heap }
		}
	}
}

impl DerefMut for GermanString {
	#[inline]
	fn deref_mut(&mut self) -> &mut str {
		if self.is_inline() {
			// SAFETY: just checked that we're inline
			unsafe { &mut self.inline }
		} else {
			// SAFETY: just checked that we're on the heap
			unsafe { &mut self.heap }
		}
	}
}

impl Drop for GermanString {
	#[inline]
	fn drop(&mut self) {
		if !self.is_inline() {
			// SAFETY: just checked that we're on the heap
			let heap = unsafe { self.to_heap_unchecked_mut() };

			// SAFETY: we pass in the reference read from self, and we do not
			// drop heap anywhere else
			unsafe { drop(ptr::read(heap)) }
		}
	}
}

impl From<&str> for GermanString {
	#[inline]
	fn from(s: &str) -> Self {
		if s.len() > INLINE_MAX_LEN {
			// SAFETY: I (vt) am pretty sure `str` cannot be longer than `isize::MAX`.
			// and its not zero (would have been covered by previous branch)
			unsafe { Self::new_heap(s) }
		} else {
			// SAFETY: just checked len is lte `INLINE_MAX_LEN`
			unsafe { Self::new_inline(s) }
		}
	}
}

#[repr(C)]
struct Inline {
	len: u8,
	str: MaybeUninit<[u8; INLINE_MAX_LEN]>
}

impl Inline {
	/// # Safety
	///
	/// The provided string must have a length less than or equal to
	/// [`INLINE_MAX_LEN`].
	#[inline]
	unsafe fn new(s: &str) -> Self {
		debug_assert!(s.len() <= INLINE_MAX_LEN);

		let len = s.len().into_u8_lossy() | INLINE_MARKER;
		let mut str = MaybeUninit::<[u8; INLINE_MAX_LEN]>::uninit();

		// SAFETY: caller promises that `s.len()` is lte `INLINE_MAX_LEN`,
		// so we will not overflow
		unsafe {
			str.as_mut_ptr()
				.cast::<u8>()
				.copy_from_nonoverlapping(s.as_ptr(), s.len())
		}

		Self { len, str }
	}

	#[inline]
	fn capacity(&self) -> usize {
		INLINE_MAX_LEN
	}

	#[inline]
	fn len(&self) -> usize {
		(self.len & !INLINE_MARKER).into_usize()
	}
}

impl Deref for Inline {
	type Target = str;

	#[inline]
	fn deref(&self) -> &str {
		let ptr = self.str.as_ptr().cast::<u8>();
		let len = (self.len ^ INLINE_MARKER).into_usize();

		// SAFETY: we got ptr from `self`
		let slice = unsafe { slice::from_raw_parts(ptr, len) };

		// SAFETY: `self` can only hold valid UTF-8
		unsafe { str::from_utf8_unchecked(slice) }
	}
}

impl DerefMut for Inline {
	#[inline]
	fn deref_mut(&mut self) -> &mut str {
		let ptr = self.str.as_mut_ptr().cast::<u8>();
		let len = (self.len ^ INLINE_MARKER).into_usize();

		// SAFETY: we got ptr from `self`
		let slice = unsafe { slice::from_raw_parts_mut(ptr, len) };

		// SAFETY: `self` can only hold valid UTF-8
		unsafe { str::from_utf8_unchecked_mut(slice) }
	}
}

#[repr(C)]
struct Heap {
	cap_be: usize,
	len: usize,
	ptr: *const u8
}

impl Heap {
	/// # Safety
	///
	/// The passed in string should ideally actually need to be put
	/// on the heap, but in strict requirements, its length must not
	/// be zero or overflow `isize::MAX`.
	#[inline]
	unsafe fn new(s: &str) -> Self {
		// SAFETY: caller promises `s.len() > 0`, and `s.len()` doesn't overflow
		// `isize::MAX`. Additionally, `Self::alloc` checks/panics on failed allocation
		let ptr = unsafe { Self::alloc(s.len()) };

		// SAFETY: freshly allocated memory, and `Self::alloc` ensures that if
		// it returns, its a valid block of memory
		unsafe { ptr.copy_from_nonoverlapping(s.as_ptr(), s.len()) }

		let len = s.len();
		let cap_be = s.len().to_be();

		Self { cap_be, len, ptr }
	}

	/// # Safety
	///
	/// `len` must not be zero or overflow `isize::MAX`
	#[inline]
	unsafe fn try_alloc(len: usize) -> *mut u8 {
		// SAFETY: align is 1 (power of two, and not zero), and caller promises
		// `len` does not overflow `isize::MAX`
		let layout = unsafe { Layout::from_size_align_unchecked(len, 1) };

		// SAFETY: caller promises `len` is not zero
		unsafe { alloc(layout) }
	}

	/// # Safety
	///
	/// `len` must not be zero or overflow `isize::MAX`
	#[inline]
	unsafe fn alloc(len: usize) -> *mut u8 {
		// SAFETY: caller promises to uphold variants of `try_alloc`
		let ptr = unsafe { Self::try_alloc(len) };

		if ptr.is_null() {
			panic!("allocation failed");
		} else {
			ptr
		}
	}

	#[inline]
	fn capacity(&self) -> usize {
		usize::from_be(self.cap_be)
	}

	#[inline]
	fn len(&self) -> usize {
		self.len
	}
}

impl Deref for Heap {
	type Target = str;

	#[inline]
	fn deref(&self) -> &str {
		let ptr = self.ptr;
		let len = self.len;

		// SAFETY: we got ptr from `self`
		let slice = unsafe { slice::from_raw_parts(ptr, len) };

		// SAFETY: `self` can only hold valid UTF-8
		unsafe { str::from_utf8_unchecked(slice) }
	}
}

impl DerefMut for Heap {
	#[inline]
	fn deref_mut(&mut self) -> &mut str {
		let ptr = self.ptr.cast_mut();
		let len = self.len;

		// SAFETY: we got ptr from `self`
		let slice = unsafe { slice::from_raw_parts_mut(ptr, len) };

		// SAFETY: `self` can only hold valid UTF-8
		unsafe { str::from_utf8_unchecked_mut(slice) }
	}
}

impl Drop for Heap {
	#[inline]
	fn drop(&mut self) {
		// SAFETY: align is 1 (power of two, and not zero), and we couldn't
		// have allocated more than `isize::MAX` bytes
		let layout = unsafe { Layout::from_size_align_unchecked(usize::from_be(self.cap_be), 1) };

		// SAFETY: we have invariant that self must not be zero length, we
		// allocated using `alloc`, and capacity and align 1 is what we
		// used to allocate this
		unsafe { dealloc(self.ptr.cast_mut(), layout) }
	}
}
