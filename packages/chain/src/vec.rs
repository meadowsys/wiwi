use crate::prelude_internal::*;

#[chain_fn]
impl<T> Chain<Vec<T>> {
	#[chain_doc("Vec::new")]
	pub fn new() -> Self {}

	#[chain_doc("Vec::with_capacity")]
	pub fn with_capacity(capacity: usize) -> Self {}

	#[chain_doc("Vec::from_raw_parts")]
	pub unsafe fn from_raw_parts(ptr: *mut T, length: usize, capacity: usize) -> Self {}

	pub fn append(&mut self, other: &mut Vec<T>) {}
	// pub fn binary_search(&self, x: &T) -> Result<usize, usize>
	// where
	// 	T: Ord
	// {}
	pub fn capacity(&self) -> usize {}
	pub fn clear(&mut self) {}
	// pub fn clone_from_slice(&mut self, src: &[T])
	// where
	// 	T: Clone
	// {}
	// pub fn copy_from_slice(&mut self, src: &[T])
	// where
	// 	T: Copy
	// {}
	pub fn push(&mut self, value: T) {}
}
