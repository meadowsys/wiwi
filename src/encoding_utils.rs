use ::std::ptr;

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
