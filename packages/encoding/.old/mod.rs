use crate::prelude::*;

pub use self::generic_fn::{
	encode,
	decode,

	Encode,
	Encoding,

	Base16,
	Base32,
	Base64,
	Hex,
	RFC1751,
	Z85,
};

mod generic_fn;

pub mod base16;
pub mod base32;
pub mod base64;
pub mod hex;
pub mod rfc1751;
pub mod z85;

/// Helper for unsafe buffer operations, when the _exact_ total capacity needed
/// is known ahead of time and requested up front
///
/// Calling [`Vec::extend_from_slice`] on a vec would be the equivalent safe
/// version of this struct. The reason this struct exists then, is because
/// anything that pushes to a vec checks to make sure there is enough capacity.
/// That cost is probably negligible, but added up could be noticeable
/// (especially in hot loops), and if you can guarantee you know / can calculate
/// the exact amount you need, then allocate it all up front, you no longer need
/// those checks, so why even bother running those checks?
///
/// With debug assertions enabled, this struct will keep track of the amount of
/// bytes written and assert unsafe preconditions, like not overflowing the
/// allocated capacity, as well as having all preallocated capacity filled.
/// However, with debug assertions off (release mode has it off by default),
/// these checks are not run (the `bytes_written` field that tracks it is
/// gated behind `cfg(debug_assertions)`, so doesn't even exist!), and it becomes
/// essentially just a wrapper around a vec, its ptr, raw ptr copying operations,
/// and a method that unsafely sets the len of the vec before unwrapping it.
///
/// Creating one of these structs is not unsafe, but you can't
/// really do much with it in safe only code :p
struct UnsafeBufWriteGuard {
	/// The [`Vec`] that's being written to
	///
	/// Note: if `self` is prematurely dropped, this vec will be dropped by it's
	/// normal drop implementation. Additionally, `u8` is plain old data, so
	/// doesn't have any special drop behaviour (it's just bytes), so it's safe
	/// that the length is still set to 0.
	vec: Vec<u8>,
	/// The pointer into the vec
	///
	/// This pointer is set to the start of the `vec` upon creation, and is
	/// shifted forward with every write operation to it.
	///
	/// This pointer is guaranteed not to move throughout the lifespan of `self`,
	/// because as far as the vec itself is aware, we request it to allocate some
	/// memory, it gives us at least that much, then we are not touching it, until
	/// `self` gets unwrapped using [`into_full_vec`]. After that, we will never
	/// use this pointer again. We are then (unsafely) setting the len of the vec
	/// to the initially requested amount of capacity, which the caller of that
	/// method promises is initialised, since it is a safety invariant of
	/// [`into_full_vec`]. What happens to the vec after we hand it's ownership
	/// back to caller, is no longer on us to handle. The pointer has been dropped,
	/// and our job is done.
	///
	/// [`into_full_vec`]: Self::into_full_vec
	ptr: *mut u8,
	/// The amount of capacity that the caller initially requested
	///
	/// # Safety
	///
	/// Previously, in the unwrapping operation, we used the value returned by
	/// [`Vec::capacity`] to set the len of the vec. This is unsound, since
	/// [`Vec::with_capacity`] is allowed to over allocate. Because of this, we
	/// must store the initial requested capacity (that the caller promises to
	/// fill before taking the vec), and use that value to set the len instead.
	requested_capacity: usize,
	/// In debug mode, tracks the amount of bytes written, and uses it to perform
	/// assertions on preconditions. In release mode, this is not present
	///
	/// Because this value is not present, memory usage is theoretically reduced
	/// by a word size. Unless you're relying on the size of `Self`, and/or having
	/// this struct as a field of another struct where size matters, this field
	/// not being present in release mode should not be an issue.
	///
	/// # Safety
	///
	/// You shouldn't be relying on the layout of this struct anyways.
	#[cfg(debug_assertions)]
	bytes_written: usize
}

impl UnsafeBufWriteGuard {
	/// Create a new [`UnsafeBufWriteGuard`] with specified capacity
	///
	/// The amount of capacity specified must be _exactly_ calculated, and _all_
	/// capacity allocated here _must_ be initialised before calling
	/// [`into_full_vec`](Self::into_full_vec). See that function for more details
	/// and safety notes.
	///
	/// The act of creating one of these structs is not unsafe, but you can't
	/// really do much with it in safe only code :p
	#[inline]
	pub fn with_capacity(capacity: usize) -> Self {
		let mut vec = Vec::new();
		vec.reserve_exact(capacity);

		debug_assert!(vec.capacity() >= capacity);

		let ptr = vec.as_mut_ptr();

		Self {
			vec,
			ptr,
			requested_capacity: capacity,
			#[cfg(debug_assertions)]
			bytes_written: 0
		}
	}

	/// Writes an amount of bytes into `self`, determined by const param `N`
	///
	/// This does the same as [`write_bytes`](Self::write_bytes) in functionality,
	/// but maybe the const generic param `N` will enable more optimisations?
	///
	/// # Safety
	///
	/// You must not write, in total, more than the amount of capacity that you
	/// requested when creating `self`.
	#[inline]
	pub unsafe fn write_bytes_const<const N: usize>(&mut self, src: *const u8) {
		#[cfg(debug_assertions)] {
			// this has to be behind cfg because self.bytes_written
			// doesn't exist in not(debug_assertions)
			self.bytes_written += N;
			assert!(self.bytes_written <= self.requested_capacity)
		}

		// SAFETY: caller promises not to write more bytes than they requested
		// up front, which is what we also requested from the vec
		unsafe { ptr::copy_nonoverlapping(src, self.ptr, N) }

		// SAFETY: caller promises not to write more bytes than they requested
		// up front. In the case of this invocation writing the exact amount to
		// fill the remaining bytes, the pointer could be set to the end of the
		// allocation, which is valid
		unsafe { self.ptr = self.ptr.add(N) }
	}

	/// Writes an amount of bytes into `self`
	///
	/// # Safety
	///
	/// You must not write, in total, more than the amount of capacity that you
	/// requested when creating `self`.
	#[inline]
	pub unsafe fn write_bytes(&mut self, src: *const u8, n: usize) {
		#[cfg(debug_assertions)] {
			// this has to be behind cfg because self.bytes_written
			// doesn't exist in not(debug_assertions)
			self.bytes_written += n;
			assert!(self.bytes_written <= self.requested_capacity)
		}

		// SAFETY: caller promises not to write more bytes than they requested
		// up front, which is what we also requested from the vec
		unsafe { ptr::copy_nonoverlapping(src, self.ptr, n) }

		// SAFETY: caller promises not to write more bytes than they requested
		// up front. In the case of this invocation writing the exact amount to
		// fill the remaining bytes, the pointer could be set to the end of the
		// allocation, which is valid
		unsafe { self.ptr = self.ptr.add(n) }
	}

	/// Get the pointer pointing to the start of the uninitialised memory in the
	/// buffer (to operate on the raw pointer directly)
	///
	/// If/when you are done writing to the pointer, you should call
	/// [`add_byte_count`](Self::add_byte_count). This offsets the internally
	/// stored pointer by that amount. If you don't, calling any other write
	/// function on this struct will clobber over what you just wrote.
	#[inline]
	pub fn as_mut_ptr(&mut self) -> *mut u8 {
		self.ptr
	}

	/// Declare that `n` bytes have been written
	///
	/// Call this after writing to the raw pointer (which you can get using
	/// [`as_mut_ptr`](Self::as_mut_ptr)), ensuring the pointer stored internally
	/// is still going to point at the start of the uninitialised chunk (or the end).
	///
	/// # Safety
	///
	/// You must have written the amount of bytes that you say you have written,
	/// and that you have not written too many bytes.
	///
	/// Calling this function without writing to the amount of memory you say you
	/// did will leave uninitialised memory "holes", which will cause undefined
	/// behaviour when you unwrap the vec.
	#[expect(dead_code, reason = "bweh")]
	#[inline]
	pub unsafe fn add_byte_count(&mut self, n: usize) {
		#[cfg(debug_assertions)] {
			// this has to be behind cfg because self.bytes_written
			// doesn't exist in not(debug_assertions)
			self.bytes_written += n;
			assert!(self.bytes_written <= self.requested_capacity)
		}

		// SAFETY: caller promises to have written the amount
		// of bytes that they say they did
		unsafe { self.ptr = self.ptr.add(n) }
	}

	/// Declare that the amount of bytes requested up front, has been written to,
	/// then unwraps and returns the internal vec
	///
	/// # Safety
	///
	/// You must have written to all the bytes that you have requested up front.
	/// Calling this function without doing so will leave a "tail" of uninitialised
	/// bytes in the vec, causing undefined behaviour.
	#[inline]
	pub unsafe fn into_full_vec(mut self) -> Vec<u8> {
		#[cfg(debug_assertions)] {
			// this has to be behind cfg because self.bytes_written
			// doesn't exist in not(debug_assertions)
			assert!(self.bytes_written == self.requested_capacity);
		}

		// SAFETY: caller promises to have written to all
		// the capacity they requested
		unsafe { self.vec.set_len(self.requested_capacity) }

		self.vec
	}
}

/// Utility to emit fixed size (const) chunks, in an unchecked manner, from
/// a slice
///
/// Contains debug assertions to assert preconditions.
// I cannot remember if I rely on this being repr(transparent) anywhere
#[repr(transparent)]
struct ChunkedSlice<'h, const N: usize> {
	/// The slice to pull bytes from
	bytes: &'h [u8]
}

impl<'h, const N: usize> ChunkedSlice<'h, N> {
	/// Creates a new [`ChunkedSlice`] instance from the given
	/// borrowed, byte slice
	#[inline]
	pub fn new(bytes: &'h [u8]) -> Self {
		Self { bytes }
	}

	/// Removes, without checking, `N` bytes off the front of the internal slice,
	/// then returns a reference to that slice
	///
	/// I believe the reason this function returns a reference rather than an
	/// array by value is performance? if I remember correctly, changing it to
	/// return the array by value caused a quite heavy performance regression
	/// in z85 encode speed. My not-very-educated guess is the alignment? since
	/// references are aligned to word size, which the CPU likes, while the byte
	/// array is only aligned to 1 ~vt
	///
	/// # Safety
	///
	/// There must be at least `N` bytes left, otherwise a reference to invalid
	/// memory will be created, causing undefined behaviour.
	#[inline]
	pub unsafe fn next_frame_unchecked(&mut self) -> &'h [u8; N] {
		debug_assert!(self.bytes.len() >= N, "enough bytes left to form another whole frame");

		let self_ptr = self.bytes.as_ptr();
		let self_len = self.bytes.len();

		// SAFETY: caller asserts there is at least `N` bytes left,
		// so this reference will point to valid memory
		let new_slice = unsafe { &*self_ptr.cast::<[u8; N]>() };

		// SAFETY: caller asserts there is at least `N` bytes left,
		// so this ptr will still point in range
		let self_ptr = unsafe { self_ptr.add(N) };

		// SAFETY: caller asserts there is at least `N` bytes left,
		// so the subtraction won't overflow (pointer is offset above)
		self.bytes = unsafe { slice::from_raw_parts(self_ptr, self_len - N) };

		new_slice
	}

	/// Consumes self, takes the remainder slice, copies it into a temporary
	/// buffer of length `N`, and calls the provided closure with the temporary
	/// buffer
	///
	/// This does _not_ indicate anywhere how many were padding bytes vs actual
	/// data. In the few places that this utility struct is used, the remainder
	/// has been calculated already.
	///
	/// # Safety
	///
	/// There must be strictly N or less bytes left, otherwise invalid memory
	/// (past the end of the temporary buffer created) will be written to.
	#[inline]
	pub unsafe fn with_remainder_unchecked<F>(self, f: F)
	where
		F: FnOnce(&[u8; N])
	{
		let len = self.bytes.len();

		debug_assert!(len < N, "(strictly) less than a whole frame remaining");

		// temp buffer of correct length, to add padding
		let mut slice = [0u8; N];

		// ptr to self
		let self_ptr = self.bytes.as_ptr();
		// ptr to temp buffer
		let slice_ptr = slice.as_mut_ptr();

		// SAFETY: caller promises that there is strictly less than N bytes
		// remaining, so the amount of data copied will always be less than
		// the temp buffer length. `len` comes from the same slice we are
		// copying from, so we must be able to copy that much over
		unsafe { ptr::copy_nonoverlapping(self_ptr, slice_ptr, len) }

		f(&slice);
	}

	/// Returns the slice left in `self`
	#[inline]
	pub fn to_slice(&self) -> &'h [u8] {
		self.bytes
	}
}
