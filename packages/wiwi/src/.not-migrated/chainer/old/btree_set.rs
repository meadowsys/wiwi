use std::collections::BTreeSet;

#[repr(transparent)]
pub struct BTreeSetChain<T> {
	inner: BTreeSet<T>
}

// TODO: eventually ref/mut versions

impl<T> From<BTreeSet<T>> for BTreeSetChain<T> {
	fn from(value: BTreeSet<T>) -> Self {
		Self { inner: value }
	}
}
