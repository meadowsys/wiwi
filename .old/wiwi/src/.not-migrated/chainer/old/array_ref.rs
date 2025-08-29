use super::SliceRefChain;

#[repr(transparent)]
pub struct ArrayRefChain<'h, T, const N: usize> {
	inner: &'h [T; N]
}

impl<'h, T, const N: usize> ArrayRefChain<'h, T, N> {
	pub fn into_inner(self) -> &'h [T; N] {
		self.inner
	}

	pub fn nonchain_array(&'h self) -> &'h [T; N] {
		self.inner
	}

	pub fn nonchain_slice(&'h self) -> &'h [T] {
		self.inner
	}

	pub fn nonchain_slice_chainer_ref(&'h self) -> SliceRefChain<'h, T> {
		(self.inner as &[T]).into()
	}
}

impl<'h, T, const N: usize> From<&'h [T; N]> for ArrayRefChain<'h, T, N> {
	fn from(value: &'h [T; N]) -> Self {
		Self { inner: value }
	}
}
