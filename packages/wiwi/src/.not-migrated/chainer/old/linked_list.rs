use std::collections::LinkedList;

#[repr(transparent)]
pub struct LinkedListChain<T> {
	inner: LinkedList<T>
}

// TODO: eventually ref/mut versions

impl<T> LinkedListChain<T> {
	pub fn new() -> Self {
		LinkedList::new().into()
	}
}

impl<T> LinkedListChain<T> {
	// TODO: append/chainer
}

impl<T> From<LinkedList<T>> for LinkedListChain<T> {
	fn from(value: LinkedList<T>) -> Self {
		Self { inner: value }
	}
}
