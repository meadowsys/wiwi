#[repr(transparent)]
pub struct VecRefChain<'h, T> {
	inner: &'h Vec<T>
}

impl<'h, T> From<&'h Vec<T>> for VecRefChain<'h, T> {
	fn from(value: &'h Vec<T>) -> Self {
		Self { inner: value }
	}
}
