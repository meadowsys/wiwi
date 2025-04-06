use wiwi_util::prelude::*;

pub struct UnsafeBufWriteGuard<T> {
	/// just do operations on a plain vector in debug mode
	#[cfg(debug_assertions)]
	vec: Vec<T>,

	/// keep track of requested capacity in debug mode to assert on
	#[cfg(debug_assertions)]
	requested: usize,

	/// write to a pointer in release
	#[cfg(not(debug_assertions))]
	ptr: *const T,

	/// seperately keep track of capacity in release, cause it
	/// can be different than requested length
	#[cfg(not(debug_assertions))]
	actual_cap: usize
}

impl<T> UnsafeBufWriteGuard<T> {
	#[inline]
	pub fn with_capacity(capacity: usize) -> Self {
		#[cfg(debug_assertions)]
		let rv = Self {
			vec: Vec::with_capacity(capacity),
			requested: capacity
		};

		#[cfg(not(debug_assertions))]
		let rv = {
			let vec = Vec::with_capacity(capacity);
			let mut vec = ManuallyDrop::new(vec);

			let ptr = vec.as_mut_ptr();
			let actual_cap = vec.capacity();

			Self { ptr, actual_cap }
		};

		rv
	}

	#[inline]
	pub unsafe fn write_bytes(&mut self, src: *const T, count: usize) {
		unsafe { self.as_mut_ptr().copy_from_nonoverlapping(src, count) }
		unsafe { self.add_byte_count(count) }
	}

	#[inline]
	pub unsafe fn write_bytes_const<const N: usize>(&mut self, src: *const T) {
		unsafe { self.write_bytes(src, N) }
	}

	#[inline]
	pub unsafe fn as_mut_ptr(&mut self) -> *mut T {
		#[cfg(debug_assertions)]
		let rv = self.vec.as_mut_ptr();

		#[cfg(not(debug_assertions))]
		let rv = self.ptr.cast_mut();

		rv
	}

	#[inline]
	pub unsafe fn add_byte_count(&mut self, count: usize) {
		#[cfg(debug_assertions)]
		unsafe { self.vec.set_len(self.vec.len() + count) }

		#[cfg(not(debug_assertions))]
		unsafe { self.ptr = self.ptr.add(count) }
	}

	#[inline]
	pub unsafe fn into_full_vec(self, len: usize) -> Vec<T> {
		let this = ManuallyDrop::new(self);

		#[cfg(debug_assertions)]
		let rv = {
			assert!(this.requested == len);
			unsafe { ptr::read(&raw const this.vec) }
		};

		#[cfg(not(debug_assertions))]
		let rv = {
			unsafe { Vec::from_raw_parts(this.ptr.cast_mut(), len, this.actual_cap) }
		};

		rv
	}

	/// Drops the vec, deallocating it, but possibly
	#[inline]
	pub fn drop_leak_elements(self) {
		#[cfg(debug_assertions)] {
			// nothing...
		}

		#[cfg(not(debug_assertions))] {
			// len is 0 here, caller says its fine to leak the elements in the vec
			// though, if the elements have no drop hook, this won't matter either way
			let vec = unsafe { Vec::from_raw_parts(self.ptr.cast_mut(), 0, self.actual_cap) };
			drop(vec);
		}
	}
}

impl<T> Drop for UnsafeBufWriteGuard<T> {
	fn drop(&mut self) {
		#[cfg(debug_assertions)] {
			// nothing... except, behaviour should be the same in debug,
			// so it should panic too to indicate a bug
			panic!("dropped UnsafeBufWriteGuard<T> instance (this isn't fine in release, please use `into_full_vec` or `drop_leak`)");
		}

		#[cfg(not(debug_assertions))] {
			panic!("dropped UnsafeBufWriteGuard<T> instance (please use `into_full_vec` or `drop_leak`)");
		}
	}
}
