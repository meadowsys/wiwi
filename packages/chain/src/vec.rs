use crate::prelude_internal::*;

#[chain_fn]
impl<T> Chain<Vec<T>> {
	pub fn new() -> Self {}
	pub fn push(&mut self, value: T) {}
}
