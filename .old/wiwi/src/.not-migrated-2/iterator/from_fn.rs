use crate::function::FnMut;
use crate::option::Option;
use super::Iter;

// TODO: make a fused version of this?

#[repr(transparent)]
pub struct FromFn<F> {
	inner: F
}

#[inline]
pub fn from_fn<T, F>(f: F) -> FromFn<F>
where
	F: FnMut() -> Option<T>
{
	FromFn { inner: f }
}

impl<T, F> Iter for FromFn<F>
where
	F: FnMut() -> Option<T>
{
	type Item = T;

	#[inline]
	fn next(&mut self) -> Option<T> {
		(self.inner)()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn from_fn() {
		use super::*;

		let mut i = 0;
		let mut iter = from_fn(|| (i < 5).then(|| {
			let old = i;
			i += 1;
			old
		}));

		assert_eq!(iter.next(), Some(0));
		assert_eq!(iter.next(), Some(1));
		assert_eq!(iter.next(), Some(2));
		assert_eq!(iter.next(), Some(3));
		assert_eq!(iter.next(), Some(4));
		assert_eq!(iter.next(), None);
	}
}
