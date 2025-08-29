use crate::to_maybeuninit::ToMaybeUninit as _;
use std::cmp::Ordering;
use std::mem::{ self, MaybeUninit };
use std::slice;
use super::{ SliceRefChain, VecChain };

#[repr(transparent)]
pub struct SliceMutChain<'h, T> {
	inner: &'h mut [T]
}

impl<'h, T> SliceMutChain<'h, T> {
	pub fn into_inner(self) -> &'h mut [T] {
		self.inner
	}

	pub fn into_vec_chain(self) -> VecChain<T>
	where
		T: Clone
	{
		self.inner.to_vec().into()
	}

	pub fn nonchain_slice(&self) -> &[T] {
		self.inner
	}

	pub fn nonchain_slice_mut(&mut self) -> &mut [T] {
		self.inner
	}

	pub fn nonchain_slice_chainer_ref(&'h self) -> SliceRefChain<'h, T> {
		(*self.inner).into()
	}
}

// TODO: to_vec_chain_in (alloc)

impl<'h, T> SliceMutChain<'h, T> {
	pub fn first<CB>(self, cb: CB) -> Self
	where
		CB: FnOnce(Option<&T>)
	{
		cb(self.inner.first());
		self
	}

	pub fn first_mut<CB>(mut self, cb: CB) -> Self
	where
		CB: FnOnce(Option<&mut T>)
	{
		cb(self.inner.first_mut());
		self
	}

	pub fn last<CB>(self, cb: CB) -> Self
	where
		CB: FnOnce(Option<&T>)
	{
		cb(self.inner.last());
		self
	}

	pub fn last_mut<CB>(mut self, cb: CB) -> Self
	where
		CB: FnOnce(Option<&mut T>)
	{
		cb(self.inner.last_mut());
		self
	}

	pub fn is_empty(self, out: &mut bool) -> Self {
		self.is_empty_uninit(unsafe { out.to_maybeuninit_drop() })
	}

	pub fn is_empty_uninit(self, out: &mut MaybeUninit<bool>) -> Self {
		out.write(self.inner.is_empty());
		self
	}

	pub fn len(self, out: &mut usize) -> Self {
		self.len_uninit(unsafe { out.to_maybeuninit_drop() })
	}

	pub fn len_uninit(self, out: &mut MaybeUninit<usize>) -> Self {
		out.write(self.inner.len());
		self
	}

	pub fn sort(mut self) -> Self
	where
		T: Ord
	{
		self.inner.sort();
		self
	}

	pub fn sort_by<F>(mut self, compare: F) -> Self
	where
		F: FnMut(&T, &T) -> Ordering
	{
		self.inner.sort_by(compare);
		self
	}

	pub fn sort_by_key<K, F>(mut self, f: F) -> Self
	where
		F: FnMut(&T) -> K,
		K: Ord
	{
		self.inner.sort_by_key(f);
		self
	}

	pub fn sort_by_cached_key<K, F>(mut self, f: F) -> Self
	where
		F: FnMut(&T) -> K,
		K: Ord
	{
		self.inner.sort_by_cached_key(f);
		self
	}

	pub fn sort_unstable(mut self) -> Self
	where
		T: Ord
	{
		self.inner.sort_unstable();
		self
	}

	pub fn sort_unstable_by<F>(mut self, compare: F) -> Self
	where
		F: FnMut(&T, &T) -> Ordering
	{
		self.inner.sort_unstable_by(compare);
		self
	}

	pub fn sort_unstable_by_key<K, F>(mut self, f: F) -> Self
	where
		F: FnMut(&T) -> K,
		K: Ord
	{
		self.inner.sort_unstable_by_key(f);
		self
	}

	// TODO: see SliceRefChain
}

impl<'h, T, const N: usize> SliceMutChain<'h, [T; N]> {
	pub fn flatten(self) -> SliceMutChain<'h, T> {
		// TODO: use SizedTypeProperties or slice `flatten`, whichever stabilises first
		let len = if mem::size_of::<T>() == 0 {
			self.inner.len()
				.checked_mul(N)
				.expect("slice len overflow")
		} else {
			// TODO: unchecked_mul when stable (1.79)
			self.inner.len() * N
		};

		let ptr = self.inner as *mut [[T; N]] as *mut T;
		unsafe { slice::from_raw_parts_mut(ptr, len).into() }
	}
}

impl<'h, T> From<&'h mut [T]> for SliceMutChain<'h, T> {
	fn from(value: &'h mut [T]) -> Self {
		Self { inner: value }
	}
}
