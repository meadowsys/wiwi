use std::collections::VecDeque;
use super::{ SliceRefChain, SliceMutChain };

#[repr(transparent)]
pub struct VecDequeChain<T> {
	inner: VecDeque<T>
}

// TODO: eventually ref/mut versions

impl<T> VecDequeChain<T> {
	pub fn new() -> Self {
		VecDeque::new().into()
	}

	pub fn with_capacity(capacity: usize) -> Self {
		VecDeque::with_capacity(capacity).into()
	}
}

impl<T> VecDequeChain<T> {
	pub fn into_inner(self) -> VecDeque<T> {
		self.inner
	}

	pub fn nonchain_contiguous_mut(&mut self) -> &mut [T] {
		self.inner.make_contiguous()
	}

	pub fn nonchain_vec_deque(&self) -> &VecDeque<T> {
		&self.inner
	}

	pub fn nonchain_vec_deque_mut(&mut self) -> &mut VecDeque<T> {
		&mut self.inner
	}

	pub fn nonchain_slices(&self) -> (&[T], &[T]) {
		self.inner.as_slices()
	}

	pub fn nonchain_slices_mut(&mut self) -> (&mut [T], &mut [T]) {
		self.inner.as_mut_slices()
	}

	pub fn nonchain_slice_chainers_ref(&self) -> (SliceRefChain<T>, SliceRefChain<T>) {
		let (s1, s2) = self.nonchain_slices();
		(s1.into(), s2.into())
	}

	pub fn nonchain_slice_chainers_mut(&mut self) -> (SliceMutChain<T>, SliceMutChain<T>) {
		let (s1, s2) = self.nonchain_slices_mut();
		(s1.into(), s2.into())
	}
}

impl<T> From<VecDeque<T>> for VecDequeChain<T> {
	fn from(value: VecDeque<T>) -> Self {
		Self { inner: value }
	}
}
