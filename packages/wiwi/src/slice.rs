extern crate std;

pub use std::marker::Sized;
pub use std::slice::*;

#[inline]
pub fn unsize<T>(val: &T) -> &T::Unsized
where
	T: Unsize
{
	val.unsize()
}

pub trait Unsize {
	type Unsized: ?Sized;

	fn unsize(&self) -> &Self::Unsized;
	fn unsize_mut(&mut self) -> &mut Self::Unsized;
}

impl<T, const N: usize> Unsize for [T; N] {
	type Unsized = [T];

	#[inline]
	fn unsize(&self) -> &[T] {
		self
	}

	#[inline]
	fn unsize_mut(&mut self) -> &mut [T] {
		self
	}
}
