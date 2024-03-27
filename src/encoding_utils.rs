use ::std::{ slice, ptr };

/// In debug mode, keeps track of the amount of bytes written, and asserts
/// preconditions like not writing over capacity and having all preallocated
/// capacity filled. However, in release mode, its just a wrapper around a vec,
/// its ptr, raw ptr operations ([`ptr::copy_nonoverlapping`] etc), and a method
/// that unsafetly sets the len of the vec before unwrapping it.
pub struct UnsafeBufWriteGuard {
	vec: Vec<u8>,
	ptr: *mut u8,
	#[cfg(debug_assertions)]
	bytes_written: usize
}

impl UnsafeBufWriteGuard {
	#[inline(always)]
	pub fn with_capacity(capacity: usize) -> Self {
		let mut vec = Vec::with_capacity(capacity);
		let ptr = vec.as_mut_ptr();

		#[cfg(debug_assertions)]
		assert_eq!(vec.capacity(), capacity);

		Self {
			vec,
			ptr,
			#[cfg(debug_assertions)]
			bytes_written: 0
		}
	}

	#[inline(always)]
	pub unsafe fn write_bytes_const<const N: usize>(&mut self, src: *const u8) {
		#[cfg(debug_assertions)] {
			self.bytes_written += N;
			assert!(self.bytes_written <= self.vec.capacity())
		}

		ptr::copy_nonoverlapping(src, self.ptr, N);
		self.ptr = self.ptr.add(N);
	}

	#[inline(always)]
	pub unsafe fn write_bytes(&mut self, src: *const u8, n: usize) {
		#[cfg(debug_assertions)] {
			self.bytes_written += n;
			assert!(self.bytes_written <= self.vec.capacity())
		}

		ptr::copy_nonoverlapping(src, self.ptr, n);
		self.ptr = self.ptr.add(n);
	}

	/// Make sure to also call `add_byte_count` function afterwards, to keep
	/// proper track of the ptr inside.
	#[inline(always)]
	pub unsafe fn as_ptr(&mut self) -> *mut u8 {
		self.ptr
	}

	#[inline(always)]
	pub unsafe fn add_byte_count(&mut self, n: usize) {
		#[cfg(debug_assertions)] {
			self.bytes_written += n;
			assert!(self.bytes_written <= self.vec.capacity())
		}

		self.ptr = self.ptr.add(n);
	}

	#[inline(always)]
	pub unsafe fn into_full_vec(mut self) -> Vec<u8> {
		#[cfg(debug_assertions)]
		assert!(self.bytes_written == self.vec.capacity());

		self.vec.set_len(self.vec.capacity());
		self.vec
	}
}

#[repr(transparent)]
pub struct ChunkedSlice<'h, const N: usize> {
	bytes: &'h [u8]
}

impl<'h, const N: usize> ChunkedSlice<'h, N> {
	#[inline(always)]
	pub fn new(bytes: &'h [u8]) -> Self {
		Self { bytes }
	}

	/// Takes N bytes off the front of the internal slice, returning that slice,
	/// and saving the rest for future calls.
	///
	/// # Safety
	///
	/// `self.bytes` must have `N` or more bytes left in it,
	/// otherwise invalid memory will be read from.
	pub unsafe fn next_frame_unchecked(&mut self) -> &[u8; N] {
		debug_assert!(self.bytes.len() >= N, "enough bytes left to form another whole frame");

		let self_ptr = self.bytes as *const [u8] as *const u8;
		let self_len = self.bytes.len();

		// new slice
		let new_slice = &*(self_ptr as *const [u8; N]);

		// new ptr to self (with N bytes removed from front)
		// SAFETY: see function doc comment. Caller asserts self has at least N bytes and
		// `self_len - N` and `self_ptr.add(N)` is correct because we just took N bytes out above.
		let slice_ptr = slice::from_raw_parts(self_ptr.add(N), self_len - N);
		self.bytes = slice_ptr;

		new_slice
	}

	/// Consumes self, takes the remainder slice, copies it into a temporary
	/// buffer of length `N`, and calls the provided function with this buffer.
	/// Returns the amount of bytes in that buffer that aren't padding (ie. the
	/// amount of bytes that are actual data bytes).
	///
	/// # Safety
	///
	/// `self.bytes` must have N or less bytes left in it,
	/// otherwise invalid memory will be written to.
	pub unsafe fn with_remainder_unchecked<F>(self, mut f: F)
	where
		F: FnMut(&[u8; N])
	{
		let len = self.bytes.len();
		debug_assert!(len < N, "less than a whole frame remaining");

		// temp buffer of correct length, to add padding
		let mut slice = [0u8; N];

		// ptr to self
		let self_ptr = self.bytes as *const [u8] as *const u8;
		// ptr to temp buffer
		let slice_ptr = &mut slice as *mut [u8] as *mut u8;

		// SAFETY: slice in self has less than N bytes remaining as guaranteed by
		// caller. therefore, the amount of bytes copied will be the correct
		// amount, and always fit in the temp buffer.
		ptr::copy_nonoverlapping(self_ptr, slice_ptr, len);

		f(&slice);
	}

	/// If debug assertions are enabled, this asserts that the slice contained in
	/// `self` is empty (ie. len 0), and panics if not. Otherwise, this does nothing.
	///
	/// When building with debug assertions off (ie. release mode), no assert
	/// happens, and ideally (hopefully?) this function call just gets optimised
	/// away into nothing (since its empty without that assertion).
	#[inline(always)]
	pub fn debug_assert_is_empty(&self) {
		#[cfg(debug_assertions)]
		assert!(self.bytes.is_empty(), "all bytes were consumed");
	}
}
