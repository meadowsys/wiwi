#[repr(transparent)]
pub struct VecMutChain<'h, T> {
	inner: &'h mut Vec<T>
}

impl<'h, T> From<&'h mut Vec<T>> for VecMutChain<'h, T> {
	fn from(value: &'h mut Vec<T>) -> Self {
		Self { inner: value }
	}
}
