use crate::prelude::*;
use self::atomic::Ordering::*;

/// Trait for structs that can count references
///
/// `wiwi` includes two implementations: one for single threaded access (akin
/// to `std`'s [`Rc`]), and the other for atomic multithreaded access (akin to
/// `std`'s [`Arc`]).
///
/// # Safety
///
/// You must implement this trait correctly (ie. functions must return correct
/// values), as values returned from functions are directly used to control the
/// allocation/deallocation of memory and dropping of values.
pub unsafe trait Counter: Sized {
	/// Create a new couter with strong and weak count both set to 1
	fn new() -> Self;

	/// Get the strong reference count
	fn strong_count(&self) -> usize;

	/// Get the weak reference count
	///
	/// Don't subtract the "fake" weak reference that
	/// is held by all the strong references.
	fn weak_count(&self) -> usize;

	/// Increment the strong count for creation of a new strong reference
	fn inc_strong_for_new_ref(&self);

	/// Decrements the strong count for dropping a reference, returning `true`
	/// if there are no more strong pointers left (and the value and items in
	/// the slice should be dropped)
	fn dec_strong_for_drop(&self) -> bool;

	/// Increments the weak count for creation of a new weak reference
	fn inc_weak_for_new_ref(&self);

	/// Decrements the weak count for dropping a reference, returning `true`
	/// if there are no more weak pointers left (and the allocation should be
	/// deallocated)
	fn dec_weak_for_drop(&self) -> bool;

	/// Increment the strong count if it is possible to upgrade a weak pointer
	/// to strong, and return `true`, otherwise return `false` and do nothing
	fn try_inc_strong_for_upgrade(&self) -> bool;
}

pub struct ThreadCounter {
	strong: cell::Cell<usize>,
	weak: cell::Cell<usize>,
	__not_thread_safe: PhantomData<*const ()>
}

// SAFETY: we implement everything correctly
unsafe impl Counter for ThreadCounter {
	#[inline]
	fn new() -> Self {
		Self {
			strong: cell::Cell::new(1),
			weak: cell::Cell::new(1),
			__not_thread_safe: PhantomData
		}
	}

	#[inline]
	fn strong_count(&self) -> usize {
		self.strong.get()
	}

	#[inline]
	fn weak_count(&self) -> usize {
		self.weak.get()
	}

	#[inline]
	fn inc_strong_for_new_ref(&self) {
		let old = self.strong.get();
		self.strong.set(old + 1);
	}

	#[inline]
	fn dec_strong_for_drop(&self) -> bool {
		let old = self.strong.get();
		self.strong.set(old - 1);
		old == 1
	}

	#[inline]
	fn inc_weak_for_new_ref(&self) {
		let old = self.weak.get();
		self.weak.set(old + 1);
	}

	#[inline]
	fn dec_weak_for_drop(&self) -> bool {
		let old = self.weak.get();
		self.weak.set(old - 1);
		old == 1
	}

	#[inline]
	fn try_inc_strong_for_upgrade(&self) -> bool {
		let old = self.strong.get();
		let should_upgrade = old > 0;

		if should_upgrade {
			self.strong.set(old + 1)
		}

		should_upgrade
	}
}

pub struct AtomicCounter {
	strong: AtomicUsize,
	weak: AtomicUsize
}

// SAFETY: we implement everything correctly
unsafe impl Counter for AtomicCounter {
	#[inline]
	fn new() -> Self {
		Self {
			strong: AtomicUsize::new(1),
			weak: AtomicUsize::new(1)
		}
	}

	#[inline]
	fn strong_count(&self) -> usize {
		self.strong.load(Relaxed)
	}

	#[inline]
	fn weak_count(&self) -> usize {
		self.weak.load(Relaxed)
	}

	#[inline]
	fn inc_strong_for_new_ref(&self) {
		self.strong.fetch_add(1, Relaxed);
	}

	#[inline]
	fn dec_strong_for_drop(&self) -> bool {
		let old = self.strong.fetch_sub(1, Release);
		if old != 1 { return false }

		atomic::fence(Acquire);
		true
	}

	#[inline]
	fn inc_weak_for_new_ref(&self) {
		self.weak.fetch_add(1, Relaxed);
	}

	#[inline]
	fn dec_weak_for_drop(&self) -> bool {
		let old = self.weak.fetch_sub(1, Release);
		if old != 1 { return false }

		atomic::fence(Acquire);
		true
	}

	#[inline]
	fn try_inc_strong_for_upgrade(&self) -> bool {
		self.strong
			.fetch_update(
				Acquire,
				Relaxed,
				|old| (old > 0).then(move || old + 1)
			)
			.is_ok()
	}
}
