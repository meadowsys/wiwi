use crate::memory::{ Drop, ManuallyDrop, size_of };
use crate::option::{ Option, Option::Some, Option::None };
use crate::phantom::PhantomData;
use crate::pointer;
use super::{ IntoIter, Iter, SizeHintImpl, SizeHintMarker };

pub struct VecIntoIter<T> {
	/// - if ZST: this is a pointer to the start (vec.as_ptr()) and never changed
	/// - if not ZST: this ptr gets offset by 1 every iteration, so it points at the
	///   next element. We keep track of amount of remaining elements (in addition to
	///   capacity and (original) len) so we can offset it back to the original spot
	///   upon drop
	ptr: *const T,
	/// Vec capacity
	capacity: usize,
	/// Vec len at the time of `into_iter` call.
	///
	/// Used in drop to unshift `ptr` for non-ZSTs
	len: usize,
	/// Remaining elements to emit
	remaining: usize,
	__marker: PhantomData<T>
}

impl<T> Iter for VecIntoIter<T> {
	type Item = T;

	#[inline]
	fn next(&mut self) -> Option<T> {
		if self.remaining == 0 { return None }

		let ptr = self.ptr;
		if size_of::<T>() != 0 {
			self.ptr = unsafe { self.ptr.add(1) }
		}

		self.remaining -= 1;
		Some(unsafe { ptr.read() })
	}

	#[inline]
	unsafe fn size_hint_impl(&self, _: SizeHintMarker) -> SizeHintImpl {
		// SAFETY: we have self.remaining elements remaining (invariant)
		unsafe { SizeHintImpl::hard(self.remaining) }
	}
}

impl<T> Drop for VecIntoIter<T> {
	fn drop(&mut self) {
		let ptr = if size_of::<T>() == 0 || self.capacity == 0 {
			self.ptr.cast_mut()
		} else {
			unsafe {
				let consumed = self.len - self.remaining;
				let original_ptr = self.ptr.sub(consumed).cast_mut();

				// copy remaining elements to the front of ptr
				pointer::copy(self.ptr, original_ptr, self.remaining);

				original_ptr
			}
		};

		// This is probably the safest way to do this...
		// TODO: improve this with alloc api when it's stabilised?

		// ... let Vec handle the rest (including dropping contained values
		// and deallocating)
		let _ = unsafe { Vec::from_raw_parts(ptr, self.remaining, self.capacity) };
	}
}

impl<T> IntoIter for Vec<T> {
	type Item = T;
	type Iter = VecIntoIter<T>;

	#[inline]
	fn into_wiwi_iter(self) -> VecIntoIter<T> {
		let me = ManuallyDrop::new(self);

		let ptr = me.as_ptr();
		let capacity = me.capacity();
		let len = me.len();
		let remaining = len;
		let __marker = PhantomData;

		VecIntoIter { ptr, capacity, len, remaining, __marker }
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::iter::IntoStdIterator;

	#[test]
	fn vec_into_iter() {
		let mut iter = vec![1, 2, 3, 4, 5]
			.into_wiwi_iter();

		assert_eq!(iter.next(), Some(1));
		assert_eq!(iter.next(), Some(2));
		assert_eq!(iter.next(), Some(3));
		assert_eq!(iter.next(), Some(4));
		assert_eq!(iter.next(), Some(5));
		assert_eq!(iter.next(), None);

		let mut iter = vec![1, 2, 3, 4, 5]
			.into_wiwi_iter();
		let _ = iter.next();
		let _ = iter.next();

		let vec = Vec::from_iter(iter.convert_wiwi_into_std_iterator());
		assert_eq!(vec, [3, 4, 5]);

		let mut iter = vec
			.into_wiwi_iter()
			.map(|i| i * 2);
		assert_eq!(iter.next(), Some(6));
		assert_eq!(iter.next(), Some(8));
		assert_eq!(iter.next(), Some(10));
		assert_eq!(iter.next(), None);

		let mut iter = vec![1, 2, 3, 4, 5].into_wiwi_iter();
		let _ = iter.next();
		let _ = iter.next();
		let _ = iter.next();

		// well, I'm not sure how to test this lol?
		drop(iter);
	}
}
