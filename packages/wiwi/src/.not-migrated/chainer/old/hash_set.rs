use std::collections::HashSet;
use std::hash::RandomState;

#[repr(transparent)]
pub struct HashSetChain<T, S = RandomState> {
	inner: HashSet<T, S>
}

// TODO: eventually ref/mut versions

impl<T> HashSetChain<T> {
	pub fn new() -> Self {
		HashSet::new().into()
	}

	pub fn with_capacity(capacity: usize) -> Self {
		HashSet::with_capacity(capacity).into()
	}
}

impl<T, S> From<HashSet<T, S>> for HashSetChain<T, S> {
	fn from(value: HashSet<T, S>) -> Self {
		Self { inner: value }
	}
}
