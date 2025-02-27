use crate::to_maybeuninit::ToMaybeUninit;
use std::collections::BinaryHeap;
use std::collections::binary_heap::PeekMut;
use std::mem::MaybeUninit;
use super::VecChain;

#[repr(transparent)]
pub struct BinaryHeapChain<T> {
	inner: BinaryHeap<T>
}

// TODO: eventually ref/mut versions

impl<T: Ord> BinaryHeapChain<T> {
	pub fn new() -> Self {
		BinaryHeap::new().into()
	}

	pub fn with_capacity(capacity: usize) -> Self {
		BinaryHeap::with_capacity(capacity).into()
	}
}

impl<T: Ord> BinaryHeapChain<T> {
	pub fn into_binary_heap(self) -> BinaryHeap<T> {
		self.inner
	}

	pub fn into_sorted_vec(self) -> Vec<T> {
		self.inner.into_sorted_vec()
	}

	pub fn into_sorted_vec_chainer(self) -> VecChain<T> {
		self.into_sorted_vec().into()
	}
}

impl<T: Ord> BinaryHeapChain<T> {
	// TODO: alloc methods
	// new_in
	// with_capacity_in
}

impl<T: Ord> BinaryHeapChain<T> {
	pub fn with_peek_mut<F>(mut self, f: F) -> Self
	where
		F: FnOnce(Option<PeekMut<T>>)
	{
		f(self.inner.peek_mut());
		self
	}

	pub fn pop(mut self, out: &mut Option<T>) -> Self {
		self.pop_uninit(unsafe { out.to_maybeuninit_drop() })
	}

	pub fn pop_uninit(mut self, out: &mut MaybeUninit<Option<T>>) -> Self {
		out.write(self.inner.pop());
		self
	}

	pub fn push(mut self, item: T) -> Self {
		self.inner.push(item);
		self
	}

	pub fn append(mut self, other: &mut BinaryHeap<T>) -> Self {
		self.inner.append(other);
		self
	}

	// TODO: append_chainer
	// pub fn append_chainer(mut self, other: BinaryHeapChain<T>) -> Self {
	// 	// self.inner.append(other.as)
	// }

	// TODO: drain_sorted

	pub fn retain<F>(mut self, f: F) -> Self
	where
		F: FnMut(&T) -> bool
	{
		self.inner.retain(f);
		self
	}
}

impl<T> BinaryHeapChain<T> {
	// TODO: iter, into_iter_sorted

	pub fn with_peek<F>(self, f: F) -> Self
	where
		F: FnOnce(Option<&T>)
	{
		f(self.inner.peek());
		self
	}

	pub fn capacity(self, out: &mut usize) -> Self {
		self.capacity_uninit(unsafe { out.to_maybeuninit_drop() })
	}

	pub fn capacity_uninit(self, out: &mut MaybeUninit<usize>) -> Self {
		out.write(self.inner.capacity());
		self
	}

	pub fn reserve(mut self, additional: usize) -> Self {
		self.inner.reserve(additional);
		self
	}

	pub fn reserve_exact(mut self, additional: usize) -> Self {
		self.inner.reserve_exact(additional);
		self
	}
}

impl<T> From<BinaryHeap<T>> for BinaryHeapChain<T> {
	fn from(value: BinaryHeap<T>) -> Self {
		Self { inner: value }
	}
}
