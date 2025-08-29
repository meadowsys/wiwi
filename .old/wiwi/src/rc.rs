use crate::prelude::*;
use crate::num::*;

pub use self::inner::{ Counter, ThreadCounter, AtomicCounter };
pub use self::str::{
	RcStr,
	RcStrWeak,
	RcStrThread,
	RcStrThreadWeak,
	RcStrAtomic,
	RcStrAtomicWeak
};

/// This module encapsulates and provides low level manipulation APIs on
/// a raw and opaque inner reference counting pointer
mod inner;
mod str;

/// Reference counted thin pointer, that can hold one sized
/// value and one (dynamically sized) slice
#[repr(transparent)]
pub struct Rc<C, V, S>
where
	C: Counter
{
	/// Opaque reference to inner data
	inner: inner::RcInner<C, V, S>
}

/// Weak pointer to a reference counted thin pointer [`Rc`]
#[repr(transparent)]
pub struct RcWeak<C, V, S>
where
	C: Counter
{
	/// Opaque reference to inner data
	inner: inner::RcInner<C, V, S>
}

/// Single threaded reference counting thin pointer
pub type RcThread<V, S = ()> = Rc<ThreadCounter, V, S>;

/// Weak pointer to a single threaded reference counted thin pointer [`RcThread`]
pub type RcThreadWeak<V, S = ()> = RcWeak<ThreadCounter, V, S>;

/// Atomically counted reference counting thin pointer
pub type RcAtomic<V, S = ()> = Rc<AtomicCounter, V, S>;

/// Weak pointer to an atomically counted reference counted thin pointer [`RcAtomic`]
pub type RcAtomicWeak<V, S = ()> = RcWeak<AtomicCounter, V, S>;

impl<C, V> Rc<C, V, ()>
where
	C: Counter
{
	/// Creates a reference counter from a (sized) value, storing it in the `value`
	/// field
	#[inline]
	pub fn from_value(value: V) -> Self {
		Self { inner: inner::new_from_value(value) }
	}
}

impl<C, S> Rc<C, (), S>
where
	C: Counter
{
	/// Creates a reference counter from an array, storing it in the `slice` field
	/// and erasing the array length from the type
	///
	/// If you want to keep the const length of the array, you should use
	/// [`from_value`](Rc::from_value) instead. The memory usage of the RC
	/// allocation actually does not differ between these two.
	#[inline]
	pub fn from_array_into_slice<const N: usize>(array: [S; N]) -> Self {
		Self { inner: inner::new_from_array_into_slice(array) }
	}
}

impl<C, S> Rc<C, (), S>
where
	C: Counter,
	S: Clone
{
	/// Creates a reference counter from a slice, cloning all elements into the
	/// `slice` field
	///
	/// If your slice contains elements that implement copy, you should use
	/// [`from_slice_copy`](Rc::from_slice_copy) instead, which can be more
	/// efficient. (Rust, specialisation when?)
	#[inline]
	pub fn from_slice_clone(slice: &[S]) -> Self {
		Self { inner: inner::new_from_slice_clone(slice) }
	}
}

impl<C, S> Rc<C, (), S>
where
	C: Counter,
	S: Copy
{
	/// Creates a reference counter from a slice, copying all elements into
	/// the `slice` field
	#[inline]
	pub fn from_slice_copy(slice: &[S]) -> Self {
		Self { inner: inner::new_from_slice_copy(slice) }
	}
}

impl<C, V, S> Rc<C, V, S>
where
	C: Counter
{
	/// Creates a reference counter from a value and an array, with the array
	/// being stored in the `slice` field
	#[inline]
	pub fn from_value_and_array_into_slice<const N: usize>(value: V, array: [S; N]) -> Self {
		Self { inner: inner::new_from_value_and_array_into_slice(value, array) }
	}
}

impl<C, V, S> Rc<C, V, S>
where
	C: Counter,
	S: Clone
{
	/// Creates a reference counter from a value and a slice, cloning all
	/// elements of the slice into the `slice` field
	///
	/// If your slice contains elements that implement copy, you should use
	/// [`from_value_and_slice_copy`](Rc::from_value_and_slice_copy) instead,
	/// which can be more efficient. (Rust, specialisation when?)
	#[inline]
	pub fn from_value_and_slice_clone(value: V, slice: &[S]) -> Self {
		Self { inner: inner::new_from_value_and_slice_clone(value, slice) }
	}
}

impl<C, V, S> Rc<C, V, S>
where
	C: Counter,
	S: Copy
{
	/// Creates a reference counter from a value and a slice, copying all
	/// elements of the slice into the `slice` field
	#[inline]
	pub fn from_value_and_slice_copy(value: V, slice: &[S]) -> Self {
		Self { inner: inner::new_from_value_and_slice_copy(value, slice) }
	}
}

impl<C, V, S> Rc<C, V, S>
where
	C: Counter
{
	/// Gets an immurable reference to the value stored in the `value` field
	#[inline]
	pub fn as_value_ref(&self) -> &V {
		// SAFETY: `self.inner` is a valid instance
		unsafe { inner::value_ref(self.inner) }
	}

	/// Gets an immurable reference to the slice stored in the `slice` field
	#[inline]
	pub fn as_slice_ref(&self) -> &[S] {
		// SAFETY: `self.inner` is a valid instance
		unsafe { inner::slice_ref(self.inner) }
	}

	/// Gets the strong pointer count
	#[inline]
	pub fn strong_count(&self) -> usize {
		// SAFETY: `self.inner` is a valid instance
		unsafe { inner::counter_ref(self.inner).strong_count() }
	}

	/// Gets the weak pointer count
	#[inline]
	pub fn weak_count(&self) -> usize {
		// SAFETY: `self.inner` is a valid instance
		unsafe { inner::counter_ref(self.inner).weak_count() - 1 }
	}

	/// "Downgrades" this pointer, returning a weak pointer [`RcWeak`] to the data
	#[inline]
	pub fn downgrade(&self) -> RcWeak<C, V, S> {
		// SAFETY: `self.inner` is a valid instance
		unsafe { inner::counter_ref(self.inner).inc_weak_for_new_ref() }

		RcWeak { inner: self.inner }
	}
}

impl<C, V, S> Clone for Rc<C, V, S>
where
	C: Counter
{
	/// Creates a new strong pointer to the same allocation,
	/// incrementing the strong count
	#[inline]
	fn clone(&self) -> Self {
		// SAFETY: `self.inner` is a valid instance
		unsafe { inner::counter_ref(self.inner).inc_strong_for_new_ref() }

		Self { inner: self.inner }
	}
}

impl<C, V, S> Drop for Rc<C, V, S>
where
	C: Counter
{
	#[inline]
	fn drop(&mut self) {
		// SAFETY: `self.inner` is a valid instance
		let should_drop = unsafe { inner::counter_ref(self.inner).dec_strong_for_drop() };

		if !should_drop { return }

		// SAFETY: we checked we should drop, and early exited if we shouldn't
		unsafe { inner::drop_instance(self.inner) }

		// take care of the "fake" weak ptr collectively held by strong ptrs
		drop(RcWeak { inner: self.inner });
	}
}

impl<C, V, S> RcWeak<C, V, S>
where
	C: Counter
{
	/// Gets the strong pointer count
	#[inline]
	pub fn strong_count(&self) -> usize {
		// SAFETY: `self.inner` is a valid instance
		unsafe { inner::counter_ref(self.inner).strong_count() }
	}

	/// Gets the weak pointer count
	#[inline]
	pub fn weak_count(&self) -> usize {
		// SAFETY: `self.inner` is a valid instance
		let weak = unsafe { inner::counter_ref(self.inner).weak_count() };

		// SAFETY: same as above
		let strong = unsafe { inner::counter_ref(self.inner).strong_count() };

		weak - (strong > 0).into_usize()
	}

	/// "Upgrades" this pointer, returning a strong pointer [`Rc`] to the data
	/// if there are still other strong pointers to it
	#[inline]
	pub fn upgrade(&self) -> Option<Rc<C, V, S>> {
		// SAFETY: `self.inner` is a valid instance
		let should_upgrade = unsafe { inner::counter_ref(self.inner).try_inc_strong_for_upgrade() };

		should_upgrade.then(|| Rc { inner: self.inner })
	}
}

impl<C, V, S> Clone for RcWeak<C, V, S>
where
	C: Counter
{
	/// Creates a new weak pointer to the same allocation,
	/// incrementing the weak count
	#[inline]
	fn clone(&self) -> Self {
		// SAFETY: `self.inner` is a valid instance
		unsafe { inner::counter_ref(self.inner).inc_weak_for_new_ref() }

		Self { inner: self.inner }
	}
}

impl<C, V, S> Drop for RcWeak<C, V, S>
where
	C: Counter
{
	#[inline]
	fn drop(&mut self) {
		// SAFETY: `self.inner` is a valid instance
		let should_dealloc = unsafe { inner::counter_ref(self.inner).dec_weak_for_drop() };

		if !should_dealloc { return }

		// SAFETY: we checked we should dealloc, and early exit if we shouldn't
		unsafe { inner::dealloc_instance(self.inner) }
	}
}

// SAFETY: we are `Send` if the counter/value/slice are all `Send`
unsafe impl<C, V, S> Send for Rc<C, V, S>
where
	C: Counter + Send,
	V: Send,
	S: Send
{}

// SAFETY: same as above
unsafe impl<C, V, S> Send for RcWeak<C, V, S>
where
	C: Counter + Send,
	V: Send,
	S: Send
{}

// SAFETY: we are `Sync` if the counter/value/slice are all `Sync`
unsafe impl<C, V, S> Sync for Rc<C, V, S>
where
	C: Counter + Sync,
	V: Sync,
	S: Sync
{}

// SAFETY: same as above
unsafe impl<C, V, S> Sync for RcWeak<C, V, S>
where
	C: Counter + Sync,
	V: Sync,
	S: Sync
{}
