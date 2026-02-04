use crate::prelude_internal::*;

#[chain_fn]
impl<T> Chain<Vec<T>> {
	fn new() -> Self;
	fn push(&mut self, value: T);
}
