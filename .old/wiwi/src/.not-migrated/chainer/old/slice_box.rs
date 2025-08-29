use std::mem::MaybeUninit;
use super::{ SliceRefChain, SliceMutChain, VecChain };

#[repr(transparent)]
pub struct SliceBoxChain<T> {
	inner: Box<[T]>
}

impl<T> SliceBoxChain<T> {
	pub fn new_uninit(len: usize) -> SliceBoxChain<MaybeUninit<T>> {
		unsafe {
			VecChain::with_capacity(len)
				.set_len(len)
				.nonchain_boxed_slice()
				.into()
		}
	}

	pub fn new_zeroed(len: usize) -> SliceBoxChain<MaybeUninit<T>> {
		let mut this = Self::new_uninit(len);
		unsafe { this.nonchain_ptr_mut().write_bytes(0, len) }
		this
	}
}

impl<T> SliceBoxChain<T> {
	pub fn into_inner(self) -> Box<[T]> {
		self.inner
	}

	pub fn nonchain_ptr(&self) -> *const T {
		self.inner.as_ptr()
	}

	pub fn nonchain_ptr_mut(&mut self) -> *mut T {
		self.inner.as_mut_ptr()
	}

	pub fn nonchain_slice(&self) -> &[T] {
		&self.inner
	}

	pub fn nonchain_slice_mut(&mut self) -> &mut [T] {
		&mut self.inner
	}

	pub fn nonchain_slice_chainer_ref(&self) -> SliceRefChain<T> {
		(*self.inner).into()
	}

	pub fn nonchain_slice_chainer_mut(&mut self) -> SliceMutChain<T> {
		(&mut *self.inner).into()
	}
}

impl<T> SliceBoxChain<MaybeUninit<T>> {
	pub unsafe fn assume_init(self) -> SliceBoxChain<T> {
		let raw = Box::into_raw(self.inner);
		let raw = raw as *mut [T];
		Box::from_raw(raw).into()
	}
}

impl<T> From<Box<[T]>> for SliceBoxChain<T> {
	fn from(value: Box<[T]>) -> Self {
		Self { inner: value }
	}
}
