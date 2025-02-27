use super::{ ArrayRefChain, SliceRefChain, SliceMutChain };

#[repr(transparent)]
pub struct ArrayMutChain<'h, T, const N: usize> {
	inner: &'h mut [T; N]
}

impl<'h, T, const N: usize> ArrayMutChain<'h, T, N> {
	pub fn into_inner(self) -> &'h mut [T; N] {
		self.inner
	}

	pub fn nonchain_array(&'h self) -> &'h [T; N] {
		self.inner
	}

	pub fn nonchain_array_mut(&'h mut self) -> &'h mut [T; N] {
		self.inner
	}

	pub fn nonchain_array_chainer_ref(&'h self) -> ArrayRefChain<'h, T, N> {
		(&*self.inner).into()
	}

	pub fn nonchain_slice(&'h self) -> &'h [T] {
		self.inner
	}

	pub fn nonchain_slice_mut(&'h mut self) -> &'h mut [T] {
		self.inner
	}

	pub fn nonchain_slice_chainer_ref(&'h self) -> SliceRefChain<'h, T> {
		(self.inner as &[T]).into()
	}

	pub fn nonchain_slice_chainer_mut(&'h mut self) -> SliceMutChain<'h, T> {
		(self.inner as &mut [T]).into()
	}
}

impl<'h, T, const N: usize> From<&'h mut [T; N]> for ArrayMutChain<'h, T, N> {
	fn from(value: &'h mut [T; N]) -> Self {
		Self { inner: value }
	}
}
